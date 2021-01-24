use std::error::Error;

use simple_game_engine::{self as sge, prelude::*};

const CYCLE_SPEED: f32 = 130.0;
const SCREEN_WIDTH: u32 = 480;
const SCREEN_HEIGHT: u32 = 360;

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

impl sge::Application for App {
    fn on_update(
        &mut self,
        canvas: &mut WindowCanvas,
        input: &InputState,
        elapsed_time: f64,
    ) -> Result<(), Box<dyn Error>> {
        // Handle keyboard input
        if input.keyboard.pressed(Scancode::Q) {
            std::process::exit(0);
        }
        // If we're at the bounds for a colour value, change direction
        if self.col <= 0.0 || self.col >= 255.0 {
            self.flipper = !self.flipper;
        }
        // Fill the screen with the current colour
        canvas.set_draw_color(Color::RGB(self.col as u8, 0, 255 - self.col as u8));
        canvas.clear();
        // Change the colour
        if !self.flipper {
            self.col -= CYCLE_SPEED * elapsed_time as f32;
        } else {
            self.col += CYCLE_SPEED * elapsed_time as f32;
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
    let mut engine = sge::Engine::new(&mut app, "Test App", SCREEN_WIDTH, SCREEN_HEIGHT)?;
    engine.start(false)
}
