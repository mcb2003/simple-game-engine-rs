use std::error::Error;

use simple_game_engine::{prelude::*, Application, Engine};

const MOVEMENT_SPEED: f64 = 200.0;
const SCREEN_WIDTH: u32 = 480;
const SCREEN_HEIGHT: u32 = 360;
const RECT_SIZE: u32 = 100;

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
        input: &input::InputState,
        elapsed_time: f64,
    ) -> Result<(), Box<dyn Error>> {
        // Move the rectangle
        if input.keyboard.held(input::Scancode::Up) {
            self.y = (self.y - MOVEMENT_SPEED * elapsed_time).max(0.0);
        } else if input.keyboard.held(input::Scancode::Down) {
            self.y =
                (self.y + MOVEMENT_SPEED * elapsed_time).min((SCREEN_HEIGHT - RECT_SIZE) as f64);
        }
        if input.keyboard.held(input::Scancode::Left) {
            self.x = (self.x - MOVEMENT_SPEED * elapsed_time).max(0.0);
        } else if input.keyboard.held(input::Scancode::Right) {
            self.x =
                (self.x + MOVEMENT_SPEED * elapsed_time).min((SCREEN_WIDTH - RECT_SIZE) as f64);
        }
        // Fill the screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(127, 127, 127));
        canvas.fill_rect(Rect::new(
            self.x as i32,
            self.y as i32,
            RECT_SIZE,
            RECT_SIZE,
        ))?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    let mut engine = Engine::new(&mut app, "Test App", SCREEN_WIDTH, SCREEN_HEIGHT)?;
    engine.start(false)
}
