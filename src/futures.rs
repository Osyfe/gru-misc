use std::{task::{Waker, Context, Poll}, pin::Pin, future::Future};
use pin_project::pin_project;

pub enum Query<T>
{
    Wait,
    Done(T),
    Finished
}

pub struct State<'a, T>
{
    fut: Pin<Box<dyn Future<Output = T> + 'a>>,
    finished: bool,
    cx: Context<'static>
}

impl<'a, T> State<'a, T>
{
    pub fn from<F>(fut: F) -> Self where F: Future<Output = T> + 'a
    {
        let fut = Box::pin(fut);
        let finished = false;
        let cx = Context::from_waker(Waker::noop());
        Self { fut, finished, cx }
    }

    pub fn query(&mut self) -> Query<T>
    {
        if self.finished { return Query::Finished; }
        match self.fut.as_mut().poll(&mut self.cx)
        {
            Poll::Pending => Query::Wait,
            Poll::Ready(val) =>
            {
                self.finished = true;
                Query::Done(val)
            }
        }
    }
}

pub async fn yield_now()
{
    struct YieldNow(bool);

    impl Future for YieldNow
    {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<()>
        {
            if self.0 { return Poll::Ready(()); }
            self.0 = true;
            Poll::Pending
        }
    }

    YieldNow(false).await;
}

#[pin_project(project = JoinedProjection)]
enum Joined<F>
{
    Pending(#[pin] F),
    Done
}

macro_rules! make_join
{
    ($name: ident, $(($f: ident, $f_val: ident, $F: ident)),+) =>
    {
        #[pin_project]
        pub struct $name<$($F: Future),+>
        {
            $(
                #[pin] $f: Joined<$F>,
                $f_val: Option<$F::Output>
            ),+
        }

        impl<$($F: Future),+> Future for $name<$($F),+>
        {
            type Output = ($($F::Output),+);

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
            {
                let mut this = self.project();
                $(
                    if let JoinedProjection::Pending($f) = this.$f.as_mut().project()
                    {
                        if let Poll::Ready(val) = $f.poll(cx)
                        {
                            *this.$f_val = Some(val);
                            this.$f.as_mut().set(Joined::Done);
                        }
                    }
                )+
                if $(this.$f_val.is_some())&&+ { Poll::Ready(($(this.$f_val.take().unwrap()),+)) }
                else { Poll::Pending }
            }
        }

        impl<$($F: Future),+> $name<$($F),+>
        {
            #[allow(unused)]
            pub fn new($($f: $F),+) -> Self
            {
                Self { $($f: Joined::Pending($f), $f_val: None),+ }
            }
        }
    };
}

make_join!(Join2, (f1, f1_val, F1), (f2, f2_val, F2));
make_join!(Join3, (f1, f1_val, F1), (f2, f2_val, F2), (f3, f3_val, F3));

#[macro_export]
macro_rules! join
{
    ($f1: expr, $f2: expr) => { Join2::new($f1, $f2) };
    ($f1: expr, $f2: expr, $f3: expr) => { Join3::new($f1, $f2, $f3) };
}
pub use join;

#[pin_project]
pub struct JoinAll<F: Future>
{
    #[pin]
    futs: Box<[Joined<F>]>,
    vals: Box<[Option<F::Output>]>
}

impl<F: Future> Future for JoinAll<F>
{
    type Output = Vec<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
    {
        let mut this = self.project();
        for (i, fut) in this.futs.iter_mut().enumerate()
        {
            let mut fut = unsafe { Pin::new_unchecked(fut) }; //fine because everything in boxed slice...
            if let JoinedProjection::Pending(f) = fut.as_mut().project()
            {
                if let Poll::Ready(val) = f.poll(cx)
                {
                    this.vals[i] = Some(val);
                    fut.set(Joined::Done);
                }
            }
        }
        if this.vals.iter().all(|val| val.is_some())
        {
            let mut vals = Vec::with_capacity(this.vals.len());
            this.vals.iter_mut().for_each(|val| vals.push(val.take().unwrap()));
            Poll::Ready(vals)
        } else { Poll::Pending }
    }
}

impl<F: Future> JoinAll<F>
{
    pub fn new<I: IntoIterator<Item = F>>(iter: I) -> Self
    {
        let futs = iter.into_iter().map(|fut| Joined::Pending(fut)).collect::<Vec<_>>().into_boxed_slice();
        let vals = (0..futs.as_ref().len()).map(|_| None).collect::<Vec<_>>().into_boxed_slice();
        Self { futs, vals }
    }
}

pub fn join_all<F: Future, I: IntoIterator<Item = F>>(iter: I) -> JoinAll<F> { JoinAll::new(iter) }
