use std::error::Error;

use audio_game_engine::{Application, Color, Engine, WindowCanvas};

struct App {
    col: u8,
    flipper: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            col: 255,
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
        if self.col == 0 || self.col == 255 {
            self.flipper = !self.flipper;
        }
        canvas.set_draw_color(Color::RGB(
            self.col,
            self.col,
            self.col,
        ));
        canvas.clear();
        if !self.flipper {
            self.col -= 1;
        } else {
            self.col += 1;
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
