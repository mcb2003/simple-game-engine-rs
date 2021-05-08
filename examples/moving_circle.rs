use std::error::Error;

use simple_game_engine::{self as sge, prelude::*};

const MOVEMENT_SPEED: f64 = 200.0;
const SCREEN_WIDTH: u32 = 480;
const SCREEN_HEIGHT: u32 = 360;
const CIRCLE_RADIUS: u32 = 50;

struct App {
    x: f64,
    y: f64,
}

impl App {
    pub fn new() -> Self {
        Self { x: 60.0, y: 60.0 }
    }
}

impl sge::Application for App {
    fn on_update(
        &mut self,
        canvas: &mut WindowCanvas,
        input: &InputState,
        elapsed_time: f64,
    ) -> sge::ApplicationResult {
        // Move the rectangle with the keyboard
        if input.keyboard.held(Scancode::Up) {
            self.y = (self.y - MOVEMENT_SPEED * elapsed_time).max(0.0);
        } else if input.keyboard.held(Scancode::Down) {
            self.y =
                (self.y + MOVEMENT_SPEED * elapsed_time).min((SCREEN_HEIGHT - CIRCLE_RADIUS) as f64);
        }
        if input.keyboard.held(Scancode::Left) {
            self.x = (self.x - MOVEMENT_SPEED * elapsed_time).max(0.0);
        } else if input.keyboard.held(Scancode::Right) {
            self.x =
                (self.x + MOVEMENT_SPEED * elapsed_time).min((SCREEN_WIDTH - CIRCLE_RADIUS) as f64);
        }
        // Move the rectangle with the mouse
        if input.mouse.buttons.held(MouseButton::Left) {
            self.x = input.mouse.x as f64;
            self.y = input.mouse.y as f64;
        }
        // Draw the screen
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::GRAY);
        canvas.fill_circle((self.x as i32, self.y as i32), CIRCLE_RADIUS as i32)?;
        Ok(true)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    let mut engine = sge::Engine::new(&mut app, "Test App", SCREEN_WIDTH, SCREEN_HEIGHT)?;
    engine.start(false)
}
