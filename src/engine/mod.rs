//! Contains the `Engine` type, which manages all resources to do with the game engine, and calls
//! the functions defined in the `Application` trait.

mod fps;
use fps::FpsCounter;

use std::error::Error;

use sdl2::event::Event;

use crate::{
    input::{InputState, KeyboardState, MouseState},
    Application, WindowCanvas,
};

/// The main game engine, which manages the display and input.
pub struct Engine<'a> {
    app: &'a mut dyn Application<WindowCanvas>,
    title: &'a str,
    width: u32,
    height: u32,
    /// Whether the FPS should be calculated and displayed in the titlebar of the window.
    pub show_fps: bool,
    ctx: sdl2::Sdl,
}

impl<'a> Engine<'a> {
    /// Create a new engine.
    /// # Parameters
    ///* `app`: Defines the application's logic.
    ///* `title`: Title of the window.
    /// * `width`: Width (in pixels) of the window.
    /// * `height`: Height (in pixels) of the window.
    /// # Example
    /// ```
    /// # struct App;
    /// # impl simple_game_engine::Application for App {}
    /// use simple_game_engine::Engine;
    /// # fn main() -> Result<(), String> {
    /// let mut app = App {}; // Some `Application` implementation
    /// let engine = Engine::new(&mut app, "Window Title", 640, 480)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(
        app: &'a mut dyn Application,
        title: &'a str,
        width: u32,
        height: u32,
    ) -> Result<Engine<'a>, String> {
        Ok(Engine {
            app,
            title,
            width,
            height,
            show_fps: true,
            ctx: sdl2::init()?,
        })
    }

    /// Create and show the window and start the main event loop.
    /// # Parameters
    /// * `present_vsync`: Whether to limit the frame rate of the application to the frame rate of
    ///   the display.
    /// # Example
    /// ```no_run
    /// # struct App;
    /// # impl simple_game_engine::Application for App {}
    /// use simple_game_engine::Engine;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut app = App {}; // Some `Application` implementation
    /// let mut engine = Engine::new(&mut app, "Window Title", 640, 480)?;
    /// engine.start(true)?; // Starts with vsync enabled
    /// # Ok(())
    /// # }
    /// ```
    pub fn start(&mut self, present_vsync: bool) -> Result<(), Box<dyn Error>> {
        let video = self.ctx.video()?;
        let mut fps_counter = FpsCounter::new(self.ctx.timer()?);
        let mut canvas = video
            .window(self.title, self.width, self.height)
            .position_centered()
            .build()?
            .into_canvas()
            .accelerated();
        if present_vsync {
            canvas = canvas.present_vsync();
        }
        let mut canvas = WindowCanvas::new(canvas.build()?);
        // Event handling
        let mut event_pump = self.ctx.event_pump()?;
        // Input state
        let mut input = InputState {
            keyboard: KeyboardState::new(event_pump.keyboard_state().scancodes()),
            mouse: MouseState::new(event_pump.mouse_state()),
        };

        // Call the app.on_create() function so the user can perform one-time initialisation of
        // their application.
        if !self.app.on_create(&mut canvas, &input)? {
            return self.app.on_quit();
        }

        // These variables are used to determine the elapsed time between frames, to allow for
        // time-regulated things like animation and to calculate average frame rates
        loop {
            let elapsed_time = fps_counter.update(self.show_fps);
            if self.show_fps && fps_counter.time_acc() >= 1.0 {
                let fps = fps_counter.fps();
                let title = format!("{} ({} FPS)", self.title, fps.round() as u32);
                // This fails silently on error
                canvas.window_mut().set_title(title.as_str()).ok();
                fps_counter.reset_average();
            }

            // Process next frame and exit if `Ok(false)` is returned
            if !self.app.on_update(&mut canvas, &input, elapsed_time)? {
                return self.app.on_quit();
            }

            // Handle events
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        return self.app.on_quit();
                    }
                    _ => {}
                }
            }
            // Refresh the input state
            input
                .keyboard
                .update(event_pump.keyboard_state().scancodes());
            input.mouse.update(event_pump.mouse_state());

            // Flip the double buffer
            canvas.present();
        }
    }
}
