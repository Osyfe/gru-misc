use std::{thread, any, marker::PhantomData, collections::VecDeque};

type Key = (usize, u32);
type Res = Box<dyn any::Any + Send>;
type Task = Box<dyn FnOnce() -> Res + Send>;

struct Worker
{
    submit: flume::Sender<(usize, Key, Task)>,
    free: bool
}

enum ResSlot
{
    Free,
    Pending,
    Done(Option<Res>)
}

#[derive(PartialEq, Eq)]
pub struct TaskKey<T>
{
    key: Key,
    _phantom: PhantomData<T>
}

pub enum Query<T>
{
    InvalidKey,
    Pending,
    Done(T)
}

pub struct Pool
{
    worker: Vec<Worker>,
    send: flume::Sender<(usize, Key, Res)>,
    recv: flume::Receiver<(usize, Key, Res)>,
    task: VecDeque<(Key, Task)>,
    res: Vec<(u32, ResSlot)>,
    free: Vec<usize>,
    available: u32
}

impl Pool
{
    pub fn new(num_threads: usize) -> Self
    {
        let (s2, r2) = flume::bounded(num_threads);
        let mut worker = Vec::with_capacity(num_threads);
        for i in 0..num_threads { worker.push(Worker::new(i, s2.clone())); }
        Self { worker, send: s2, recv: r2, task: VecDeque::new(), res: Vec::new(), free: Vec::new(), available: 0 }
    }

    pub fn poll(&mut self)
    {
        for (i, key, res) in self.recv.try_iter()
        {
            self.worker[i].free = true;
            let entry = &mut self.res[key.0];
            if cfg!(debug_assertions)
            {
                if entry.0 != key.1 { panic!("Pool::poll: Generation mismatch (expected {}, found {}).", key.1, self.res[key.0].0); }
                if !entry.1.pending() { panic!("Pool::poll: ResSlot not pending."); }
            }
            *entry = (key.1, ResSlot::Done(Some(res)));
            self.available += 1;
        }
        for (i, worker) in self.worker.iter_mut().enumerate()
        {
            if worker.free //filtering the iterator does not work
            {
                if let Some((key, task)) = self.task.pop_front()
                {
                    worker.free = false;
                    if let Err(flume::SendError(task)) = worker.submit.send((i, key, task))
                    { //replace broken worker and submit again
                        *worker = Worker::new(i, self.send.clone());
                        worker.free = false;
                        worker.submit.send(task).unwrap();
                    }
                }
            }
        }
    }

    pub fn spawn<T, F: FnOnce() -> T>(&mut self, task: F) -> TaskKey<T> where T: Send + 'static, F: Send + 'static
    {
        if self.free.is_empty()
        {
            self.free.push(self.res.len());
            self.res.push((0, ResSlot::Free));
        }
        let i = self.free.pop().unwrap();
        if cfg!(debug_assertions) && !self.res[i].1.free() { panic!("Pool::spawn: Free list error."); }
        let key = (i, self.res[i].0);
        self.res[i].1 = ResSlot::Pending;
        self.task.push_back((key, Box::new(move || Box::new(task()))));
        TaskKey { key, _phantom: PhantomData }
    }

    pub fn available(&self) -> u32
    {
        self.available
    }

    pub fn query<T: 'static>(&mut self, key: TaskKey<T>) -> Query<T>
    {
        let key = key.key;
        let (gen, res) = &mut self.res[key.0]; //should not panic because self.res does never shrink
        if *gen != key.1 { Query::InvalidKey }
        else if let ResSlot::Done(res_box) = res
        {
            let ret = *res_box.take().unwrap().downcast::<T>().unwrap(); //should not panic if the generation logic is correct
            *gen = gen.wrapping_add(1);
            *res = ResSlot::Free;
            self.free.push(key.0);
            self.available -= 1;
            Query::Done(ret)
        } else { Query::Pending }
    }
}

impl Worker
{
    fn new(id: usize, send: flume::Sender<(usize, Key, Res)>) -> Self
    {
        let (s1, r1) = flume::bounded::<(usize, Key, Task)>(1);
        thread::Builder::new().name(format!("Worker {id}")).spawn(move ||
        {
            for (i, key, task) in r1
            {
                let res = task();
                if send.send((i, key, res)).is_err() { break; }
            }
        }).unwrap();
        Self { submit: s1, free: true }
    }
}

impl ResSlot
{
    fn free(&self) -> bool {  if let Self::Free = self { true } else { false } }
    fn pending(&self) -> bool {  if let Self::Pending = self { true } else { false } }
}

impl<T> Clone for TaskKey<T>
{
	fn clone(&self) -> Self
	{
		Self { key: self.key, _phantom: PhantomData }
	}
}

#[cfg(test)]
mod tests //cargo test --features thread -- --nocapture
{
    use super::*;

    #[test]
    fn timer()
    {
        fn check(pool: &mut Pool, keys: &mut Vec<(usize, TaskKey<usize>)>)
        {
            if pool.available > 0
            {
                for i in (0..keys.len()).rev()
                {
                    let res = pool.query(keys[i].1.clone());
                    if let Query::Done(id) = res
                    {
                        if id != keys[i].0 { panic!("task mismatch"); }
                        println!("job {id} done");
                        keys.remove(i);
                    }
                }
            }
            pool.poll();
        }

        let pool_size = 10;
        let job_size_sqrt = 15;
        let base_wait = 1000;
        let mut pool = Pool::new(pool_size);
        let mut keys = Vec::with_capacity(job_size_sqrt * job_size_sqrt);
        let t1 = std::time::Instant::now();
        for i in 0..job_size_sqrt
        {
            for j in 0..job_size_sqrt
            {
                let id = i * job_size_sqrt + j;
                keys.push((id, pool.spawn(move || { thread::sleep(std::time::Duration::from_millis(base_wait + id as u64)); id })));
                println!("job {id} spawned");
            }
            check(&mut pool, &mut keys);
            thread::sleep(std::time::Duration::from_millis(base_wait / 2));
        }
        while !keys.is_empty()
        {
            check(&mut pool, &mut keys);
            thread::sleep(std::time::Duration::from_millis(base_wait / 100));
        }
        let t2 = std::time::Instant::now();
        let t_tot = (t2 - t1).as_millis() as usize;
        let t_th = base_wait as usize * job_size_sqrt.pow(2) / pool_size;
        println!("finished in {}ms", t_tot);
        println!("minimum time ~{}ms", t_th);
        println!("len res queue = {} vs {}", pool.res.len(), job_size_sqrt.pow(2));
        if t_tot > 2 * t_th { panic!("execution takes too much time"); }
    }
}
