//! # Example
//! The simplest SGE program looks like this:
//! ```no_run
//! use simple_game_engine::{self as sge, prelude::*};
//! use std::error::Error;
//!
//! struct App {}
//!
//! impl sge::Application for App {
//!     fn on_create(
//!         &mut self,
//!         canvas: &mut WindowCanvas,
//!         input: &InputState,
//!     ) -> sge::ApplicationResult {
//!         // Do one-time initialisation here
//!         Ok(true) // `true` indicates to continue running the application
//!     }
//!
//!     fn on_update(
//!         &mut self,
//!         canvas: &mut WindowCanvas,
//!         input: &InputState,
//!         elapsed_time: f64,
//!     ) -> sge::ApplicationResult {
//!         // Handle user input, update the canvas, and perform any other tasks to be ran on each frame
//!         Ok(true) // `true` indicates to continue running the application
//!     }
//! }
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let mut app = App {};
//!     let mut engine = sge::Engine::new(
//!         &mut app,   // Application instance
//!         "Test App", // Window title
//!         640,        // Window width
//!         480,        // Window height
//!     )?;
//!     engine.start(true)
//! }
//! ```
//! `on_create` and `on_update` are optional, but their default implementation does nothing, so
//! you'll probably want to define some logic for at least `on_update`, which is called for every
//! frame.

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
pub type ApplicationResult = Result<bool, Box<dyn Error>>;

/// An application using this framework.
pub trait Application<Canvas = WindowCanvas> {
    /// Called once at the start of the program.
    /// # Parameters
    /// * `canvas`: A draw target representing the visible window.
    /// * `input`: a struct containing info about the state of input devices, such as the keyboard
    ///   and mouse.
    fn on_create(&mut self, _canvas: &mut Canvas, _input: &input::InputState) -> ApplicationResult {
        Ok(true)
    }
    /// Called once per frame.
    /// # Parameters
    /// * `canvas`: A draw target representing the visible window.
    /// * `input`: a struct containing info about the state of input devices, such as the keyboard
    ///   and mouse.
    /// * `elapsed_time`: Duration (in seconds) since the last frame. This can be used to keep
    ///   time-sensative routines, such as animation, running at a constant speed.
    fn on_update(
        &mut self,
        _canvas: &mut Canvas,
        _input: &input::InputState,
        _elapsed_time: f64,
    ) -> ApplicationResult {
        Ok(true)
    }
    /// Called when the window's close button is clicked.
    /// Be aware that this isn't called on `std::process::exit`, so do any essential
    /// cleanup in a `Drop` implementation instead.
    /// Does nothing by default.
    fn on_quit(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub mod prelude {
    //! Commonly used types.
    pub use crate::{
        input::{InputState, MouseButton, Scancode},
        Color, Point, Rect, WindowCanvas,
    };
}
