use std::error::Error;

use sdl2::event::Event;

use super::Application;

/// The main game engine, which manages the display and input.
pub struct Engine<'a> {
    app: &'a mut dyn Application,
    title: &'a str,
    width: u32,
    height: u32,
    ctx: sdl2::Sdl,
}

impl<'a> Engine<'a> {
    /// Create a new engine.
    /// # Parameters
    ///* `app`: Defines the application's logic.
    ///* `title`: Title of the window.
    /// * `width`: Width (in pixels) of the window.
    /// * `height`: Height (in pixels) of the window.
    pub fn new(
        app: &'a mut dyn Application,
        title: &'a str,
        width: u32,
        height: u32,
    ) -> Result<Engine<'a>, Box<dyn Error>> {
        let ctx = sdl2::init()?;

        Ok(Engine {
            app,
            title,
            width,
            height,
            ctx,
        })
    }

    /// Create and show the window and start the main event loop.
    /// # Parameters
    /// * `present_vsync`: Whether to limit the frame rate of the application to the frame rate of
    ///   the display.
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
                canvas.window_mut().set_title(title.as_str()).unwrap_or(());
                time_acc -= 1.0;
                fps_acc = 0.0;
                fps_counter = 0;
            }

            self.app.on_update(&mut canvas, elapsed_time)?;
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        return self.app.on_quit();
                    }
                    _ => {}
                }
            }
            canvas.present();
        }
    }
}
