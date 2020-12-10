mod engine;
pub use engine::Engine;

use std::error::Error;

use sdl2::{render::Canvas, video::Window};

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
