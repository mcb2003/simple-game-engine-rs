use std::error::Error;

use simple_game_engine::{prelude::*, Application, Engine};

struct App {
    x: f64,
    y: f64,
}

impl App {
    pub fn new() -> Self {
        Self { x: 10.0, y: 10.0 }
    }
}

impl Application for App {
    fn on_update(
        &mut self,
        canvas: &mut WindowCanvas,
        keyboard: &input::KeyboardState,
        elapsed_time: f64,
    ) -> Result<(), Box<dyn Error>> {
        // Move the rectangle
        if keyboard.held(input::Scancode::Up) {
            self.y = (self.y - 130.0 * elapsed_time).max(0.0);
        } else if keyboard.held(input::Scancode::Down) {
            self.y = (self.y + 130.0 * elapsed_time).min(300.0);
        }
        if keyboard.held(input::Scancode::Left) {
            self.x = (self.x - 130.0 * elapsed_time).max(0.0);
        } else if keyboard.held(input::Scancode::Right) {
            self.x = (self.x + 130.0 * elapsed_time).min(300.0);
        }
        // Fill the screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(127, 127, 127));
        canvas.fill_rect(Rect::new(self.x as i32, self.y as i32, 100, 100))?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    let mut engine = Engine::new(&mut app, "Test App", 400, 300)?;
    engine.start(false)
}
