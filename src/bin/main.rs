use std::error::Error;

use audio_game_engine::{Application, Color, Engine, WindowCanvas};

struct App {
    col: f32,
    flipper: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            col: 0.0,
            flipper: false,
        }
    }
}

impl Application for App {
    fn on_update(
        &mut self,
        canvas: &mut WindowCanvas,
        elapsed_time: f64,
    ) -> Result<(), Box<dyn Error>> {
        if self.col <= 0.0 || self.col >= 255.0 {
            self.flipper = !self.flipper;
            self.col = self.col.max(0.0).min(255.0);
        }
        let col = self.col as u8;
        canvas.set_draw_color(Color::RGB(col, col, col));
        canvas.clear();
        if !self.flipper {
            self.col -= (130.0 * elapsed_time) as f32;
        } else {
            self.col += (130.0 * elapsed_time) as f32;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = Box::new(App::new());
    let mut engine = Engine::new(app, "Test App", 400, 300)?;
    engine.start()?;
    Ok(())
}
