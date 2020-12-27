mod engine;
pub use engine::Engine;
pub mod input;

use std::error::Error;

pub use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::WindowCanvas,
};

/// An application using this framework.
pub trait Application {
    /// Called once per frame.
    /// # Parameters
    /// * `canvas`: A draw target representing the visible window.
    /// * `elapsed_time`: Duration (in seconds) since the last frame. This can be used to keep
    ///   time-sensative routines, such as animation, running at a constant speed.
    fn on_update(
        &mut self,
        canvas: &mut WindowCanvas,
        keyboard: &input::KeyboardState,
        elapsed_time: f64,
    ) -> Result<(), Box<dyn Error>>;
    /// Called when the window's close button is clicked.
    /// Does nothing by default.
    fn on_quit(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
