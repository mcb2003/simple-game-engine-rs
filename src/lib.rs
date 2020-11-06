use std::error::Error;

use sdl2::{event::Event, render::Canvas, video::Window};

pub use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
};

pub type WindowCanvas = Canvas<Window>;

pub trait Application {
    fn on_update(
        &mut self,
        canvas: &mut WindowCanvas,
        elapsed_time: f64,
    ) -> Result<(), Box<dyn Error>>;
}

pub struct Engine<'a> {
    app: Box<dyn Application>,
    title: &'a str,
    ctx: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    timer: sdl2::TimerSubsystem,
    canvas: Canvas<Window>,
}

impl<'a> Engine<'a> {
    pub fn new(
        app: Box<dyn Application>,
        title: &'a str,
        width: u32,
        height: u32,
    ) -> Result<Engine, Box<dyn Error>> {
        let ctx = sdl2::init()?;
        let video = ctx.video()?;
        let timer = ctx.timer()?;
        let win = video
            .window(title, width, height)
            .position_centered()
            .build()?;
        let canvas = win.into_canvas().build()?;

        Ok(Engine {
            app,
            title,
            ctx,
            video,
            timer,
            canvas,
        })
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        // These variables are used to determine the elapsed time between frames, to allow for
        // time-regulated things like animation
        let mut last: u64;
        let mut now = self.timer.performance_counter();
        let mut elapsed_time: f64;
        // These variables are used to calculate and display average frame rates
        let mut time_acc = 0.0;
        let mut fps_acc = 0.0;
        let mut fps_counter = 0;
        // Event handling
        let mut event_pump = self.ctx.event_pump()?;
        loop {
            last = now;
            now = self.timer.performance_counter();
            elapsed_time = (now - last) as f64 / self.timer.performance_frequency() as f64;
            // Calculate and display FPS
            // Note: I have no idea whether this actually works, so if anyone would like to confirm
            // or deny this, please do
            time_acc += elapsed_time;
            fps_acc += 1.0 / elapsed_time;
            fps_counter += 1;
            if time_acc >= 1.0 {
                let fps = fps_acc / fps_counter as f64;
                let title = format!("{} ({} FPS)", self.title, fps.round() as u32);
                self.canvas.window_mut().set_title(title.as_str()).unwrap_or(()); // Fail silently
                time_acc -= 1.0;
                fps_acc = 0.0;
                fps_counter = 0;
            }

            self.app.on_update(&mut self.canvas, elapsed_time)?;
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => return Ok(()),
                    _ => {}
                }
            }
            self.canvas.present();
        }
    }
}
