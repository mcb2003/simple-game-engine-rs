use std::error::Error;

use sdl2::event::Event;

use super::Application;

pub struct Engine<'a> {
    app: Box<dyn Application>,
    title: &'a str,
    width: u32,
    height: u32,
    ctx: sdl2::Sdl,
}

impl<'a> Engine<'a> {
    pub fn new(
        app: Box<dyn Application>,
        title: &'a str,
        width: u32,
        height: u32,
    ) -> Result<Engine, Box<dyn Error>> {
        let ctx = sdl2::init()?;

        Ok(Engine {
            app,
            title,
            width,
            height,
            ctx,
        })
    }

    pub fn start(&mut self, present_vsync: bool) -> Result<(), Box<dyn Error>> {
        let video = self.ctx.video()?;
        let timer = self.ctx.timer()?;
        let mut canvas = video
            .window(self.title, self.width, self.height)
            .position_centered()
            .build()?
        .into_canvas();
        if present_vsync {
            canvas = canvas.present_vsync();
        }
        let mut canvas = canvas.build()?;
        // These variables are used to determine the elapsed time between frames, to allow for
        // time-regulated things like animation
        let mut last: u64;
        let mut now = timer.performance_counter();
        let mut elapsed_time: f64;
        // These variables are used to calculate and display average frame rates
        let mut time_acc = 0.0;
        let mut fps_acc = 0.0;
        let mut fps_counter = 0;
        // Event handling
        let mut event_pump = self.ctx.event_pump()?;
        loop {
            // Calculate and display FPS
            last = now;
            now = timer.performance_counter();
            // Note: I have no idea whether this actually works, so if anyone would like to confirm
            // or deny this, please do
            elapsed_time = (now - last) as f64 / timer.performance_frequency() as f64;
            time_acc += elapsed_time;
            fps_acc += 1.0 / elapsed_time;
            fps_counter += 1;
            if time_acc >= 1.0 {
                let fps = fps_acc / fps_counter as f64;
                let title = format!("{} ({} FPS)", self.title, fps.round() as u32);
                // This fails silently on error
                canvas
                    .window_mut()
                    .set_title(title.as_str())
                    .unwrap_or(());
                time_acc -= 1.0;
                fps_acc = 0.0;
                fps_counter = 0;
            }

            self.app.on_update(&mut canvas, elapsed_time)?;
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => return Ok(()),
                    _ => {}
                }
            }
            canvas.present();
        }
    }
}
