use std::error::Error;

use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::video::Window;

pub type WindowCanvas = Canvas<Window>;
pub use sdl2::rect::Rect;
pub use sdl2::rect::Point;
pub use sdl2::pixels::Color;

pub trait Application {
    fn on_update(&mut self, canvas: &mut WindowCanvas, elapsed_time: f64) -> Result<(), Box<dyn Error>>;
}

pub struct Engine {
    app: Box<dyn Application>,
       ctx: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    timer: sdl2::TimerSubsystem,
    canvas: Canvas<Window>,
}

impl Engine {
    pub fn new(app: Box<dyn Application>, title: &str, width: u32, height: u32) -> Result<Engine, Box<dyn Error>> {
        let ctx = sdl2::init()?;
        let video = ctx.video()?;
        let timer = ctx.timer()?;
        let win = video.window(title, width, height).position_centered().build()?;
        let canvas = win.into_canvas().build()?;

        Ok(Engine { app, ctx, video, timer, canvas })
    }
    
    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        // These variables are used to determine the elapsed time between frames, to allow for
        // time-regulated things like animation
        let mut last: u64;
        let mut now = self.timer.performance_counter();
        let mut elapsed_time: f64;
        // Event handling
        let mut event_pump = self.ctx.event_pump()?;
        loop {
            last = now;
            now = self.timer.performance_counter();
            elapsed_time = ((now - last)) as f64 / self.timer.performance_frequency() as f64;

            self.app.on_update(&mut self.canvas, elapsed_time)?;
                    for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}  => return Ok(()),
                _ => {}
            }
        }
                    self.canvas.present();
        }
    }
}
