pub struct FPS
{
    loop_helper: spin_sleep::LoopHelper,
    current_fps: u32
}

impl FPS
{
    pub fn new(max_fps: Option<usize>) -> Self
    {
        let loop_helper = spin_sleep::LoopHelper::builder().report_interval_s(1.0);
        let loop_helper = match max_fps
        {
            Some(max_fps) => loop_helper.build_with_target_rate(max_fps as f64),
            None => loop_helper.build_without_target_rate()
        };
        Self { loop_helper, current_fps: 0 }
    }

    pub fn renew(&mut self, max_fps: Option<usize>)
    {
        *self = Self::new(max_fps);
    }

    pub fn dt(&mut self) -> f32
    {
        if let Some(fps) = self.loop_helper.report_rate()
        {
            self.current_fps = fps.round() as u32;
        }
        self.loop_helper.loop_sleep();
        self.loop_helper.loop_start_s() as f32
    }

    pub fn current_fps(&self) -> u32
    {
        self.current_fps
    }
}
