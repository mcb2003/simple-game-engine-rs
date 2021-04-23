use simple_game_engine::{self as sge, prelude::*};
use std::error::Error;

struct App {}

impl sge::Application for App {
    fn on_create(
        &mut self,
        _canvas: &mut WindowCanvas,
        _input: &InputState,
    ) -> sge::ApplicationResult {
        Ok(true) // `true` indicates to continue running the application
    }

    fn on_update(
        &mut self,
        canvas: &mut WindowCanvas,
        _input: &InputState,
        _elapsed_time: f64,
    ) -> sge::ApplicationResult {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::WHITE);
        canvas.draw_text("Hello, world!", (10, 10))?;
        Ok(true) // `true` indicates to continue running the application
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App {};
    let mut engine = sge::Engine::new(
        &mut app,   // Application instance
        "Test App", // Window title
        320,        // Window width
        240,        // Window height
    )?;
    engine.start(true)
}
