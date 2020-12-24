mod engine;
pub use engine::Engine;
mod keyboard;
pub use keyboard::{Key, KeyboardState};

use std::error::Error;

use sdl2::{render::Canvas, video::Window};

pub use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
};

/// A draw target for a window on screen.
pub type WindowCanvas = Canvas<Window>;

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
        elapsed_time: f64,
    ) -> Result<(), Box<dyn Error>>;
    /// Called when the window's close button is clicked.
    /// Does nothing by default.
    fn on_quit(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
