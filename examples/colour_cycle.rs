use std::error::Error;

use simple_game_engine::{Application, Color, Engine, WindowCanvas};

struct App {
    col: f32,
    flipper: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            col: 255.0,
            flipper: true,
        }
    }
}

impl Application for App {
    fn on_update(
        &mut self,
        canvas: &mut WindowCanvas,
        elapsed_time: f64,
    ) -> Result<(), Box<dyn Error>> {
        // If we're at the bounds for a colour value, change direction
        if self.col <= 0.0 || self.col >= 255.0 {
            self.flipper = !self.flipper;
            self.col = self.col.min(255.0).max(0.0);
        }
        // Fill the screen with the current colour
        canvas.set_draw_color(Color::RGB(self.col as u8, 0, 255 - self.col as u8));
        canvas.clear();
        if !self.flipper {
            self.col -= 130.0 * elapsed_time as f32;
        } else {
            self.col += 130.0 * elapsed_time as f32;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = Box::new(App::new());
    let mut engine = Engine::new(app, "Test App", 400, 300)?;
    engine.start()
}
