mod button;
pub use button::Button;
mod state;
use state::ButtonState;

pub use sdl2::{keyboard::Scancode, mouse::MouseButton};

pub type KeyboardState = ButtonState<Scancode>;
