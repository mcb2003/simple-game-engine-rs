mod button;
pub use button::Button;
mod state;
use state::ButtonState;

mod mouse;
pub use mouse::MouseState;

pub use sdl2::{keyboard::Scancode, mouse::MouseButton};

pub(crate) type KeyboardState = ButtonState<Scancode>;

pub struct InputState {
    pub keyboard: KeyboardState,
    pub mouse: MouseState,
}
