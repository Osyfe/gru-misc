use std::time::{Instant, Duration};

struct Limiter
{
    target_dt: Duration,
    sleeper: spin_sleep::SpinSleeper
}

impl Limiter
{
    fn new(old: Option<Self>, max_fps: u32) -> Self
    {
        Self
        {
            target_dt: Duration::from_secs_f64(1.0 / max_fps as f64),
            sleeper: old.map(|old| old.sleeper).unwrap_or_default()
        }
    }
}

pub struct FPS
{
    limiter: Option<Limiter>,
    last_time: Instant,
    current_fps: u32
}

impl FPS
{
    pub fn new(max_fps: Option<u32>) -> Self
    {
        Self
        {
            limiter: max_fps.map(|max_fps| Limiter::new(None, max_fps)),
            last_time: Instant::now(),
            current_fps: 0
        }
    }

    pub fn renew(&mut self, max_fps: Option<u32>)
    {
        self.limiter = max_fps.map(|max_fps| Limiter::new(self.limiter.take(), max_fps));
    }

    pub fn dt(&mut self) -> f32
    {
        if let Some(limiter) = &self.limiter
        {
            let deadline = self.last_time + limiter.target_dt;
            limiter.sleeper.sleep_until(deadline);
        }
        let now = Instant::now();
        let dt = (now - self.last_time).as_secs_f32();
        self.last_time = now;
        self.current_fps = (1.0 / dt).round() as u32;
        dt
    }

    pub fn current_fps(&self) -> u32
    {
        self.current_fps
    }
}
