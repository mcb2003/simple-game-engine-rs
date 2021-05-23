use sdl2::TimerSubsystem;

pub struct FpsCounter {
    timer: TimerSubsystem,
    last: u64,
    now: u64,
    time_acc: f64,
    fps_acc: f64,
    fps_count: u64,
}

impl FpsCounter {
    pub fn new(timer: TimerSubsystem) -> Self {
        let now = timer.performance_counter();
        Self {
            last: 0,
            timer,
            now,
            time_acc: 0.0,
            fps_acc: 0.0,
            fps_count: 0,
        }
    }

    pub fn update(&mut self) -> f64 {
        self.last = self.now;
        self.now = self.timer.performance_counter();
            // Note: I have no idea whether this actually works, so if anyone would like to confirm
            // or deny this, please do
            let elapsed_time = (self.now - self.last) as f64 / self.timer.performance_frequency() as f64;
            self.time_acc += elapsed_time;
            self.fps_acc += elapsed_time.recip();
            self.fps_count += 1;
            elapsed_time
    }

    #[inline]
    pub fn time_acc(&self) -> f64 {
        self.time_acc
    }

    #[inline]
    pub fn fps(&self) -> f64 {
self.fps_acc / self.fps_count as f64
    }

    pub fn reset_average(&mut self) {
                self.time_acc -= 1.0;
                self.fps_acc = 0.0;
                self.fps_count = 0;
    }
}
