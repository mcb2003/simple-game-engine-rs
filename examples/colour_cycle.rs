use std::error::Error;

use simple_game_engine::{Application, Color, Engine, KeyboardState, Scancode, WindowCanvas};

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
        keyboard: &KeyboardState,
        elapsed_time: f64,
    ) -> Result<(), Box<dyn Error>> {
        // If we're at the bounds for a colour value, change direction
        if self.col <= 0.0 || self.col >= 255.0 {
            self.flipper = !self.flipper;
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

    fn on_quit(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Quitting ...");
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    let mut engine = Engine::new(&mut app, "Test App", 400, 300)?;
    engine.start(false)
}
