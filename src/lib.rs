//! A minimal game engine that's really easy to get started with.
//!
//!  This project aims to create a minimal, yet usable, game engine. It is heavily inspired by [the Pixel Game Engine](https://github.com/OneLoneCoder/olcPixelGameEngine),
//! with the goal of creating something that abstracts away the complexities of creating graphical, interactive apps and
//! games. Right now, it's a thin wrapper around [SDL2](https://www.libsdl.org) (using the [sdl2
//! crate](https://crates.io/crates/sdl2)) for visuals.
//! # Features
//! * **Very simple to use:** Just implement the [`Application` trait][Application] on a type of your choice, then pass an instance of this type to [`Engine::new`].
//! * **Powerful:** Anything you can do with sdl2 from Rust, you can do with this library, and we provide thin abstractions over some of the more convoluted sdl2 interfaces.
//! * **Built-in text rendering:** No need to find a TTF font and distribute it with your application, just call the [`Canvas::draw_text`][canvas::Canvas::draw_text] method. ([see below](#caveats-with-text-rendering))
//! ## Caveats With Text Rendering
//! This crate uses the [GNU Unifont][unifont] for built-in text rendering. As such, if you wish to use this feature, you
//! must distribute your project under the [GPL][gpl]. As this is not desirable for many projects, this feature is only
//! enabled if this crate is built with the "unifont" cargo feature.
//! ```toml
//! [dependencies.simple-game-engine]
//! version = "0.8.1"
//! features = ["unifont"]
//! ```
//! If you'd like to render text without using this font, consider checking out the [SDL2 TTF module][sdl2-ttf].
//!
//! [unifont]: <http://unifoundry.com/unifont/index.html>
//! [gpl]: <https://www.gnu.org/licenses/old-licenses/gpl-2.0-standalone.html>
//! [sdl2-ttf]: <https://docs.rs/sdl2/latest/sdl2/ttf/index.html>
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
//!     engine.start(true) // `true` starts the app with vsync enabled
//! }
//! ```
//! `on_create` and `on_update` are optional, but their default implementation does nothing, so
//! you'll probably want to define some logic for at least `on_update`, which is called for every
//! frame.

#![warn(missing_docs)]

mod engine;
pub use engine::Engine;
pub mod canvas;
pub use canvas::WindowCanvas;
pub mod input;

use std::error::Error;

pub use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
};

/// The return type of [`Application::on_create`] and [`Application::on_update`].
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
        input::{InputState, MouseButton, Scancode}, WindowCanvas,
        Color, Point, Rect,
    };
}
