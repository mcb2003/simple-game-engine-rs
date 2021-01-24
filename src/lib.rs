mod engine;
pub use engine::Engine;
pub mod input;

use std::error::Error;

pub use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::WindowCanvas,
};

/// The return type of `Application::on_update()`
pub type UpdateResult = Result<bool, Box<dyn Error>>;

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
        input: &input::InputState,
        elapsed_time: f64,
    ) -> UpdateResult;
    /// Called when the window's close button is clicked.
    /// Be aware that this isn't called on `std::process::exit`, so do any essential
    /// cleanup in a `Drop` implementation instead.
    /// Does nothing by default.
    fn on_quit(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub mod prelude {
    pub use crate::{
        input::{InputState, MouseButton, Scancode},
        Color, Point, Rect, WindowCanvas,
    };
}
