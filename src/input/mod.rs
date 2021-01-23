mod button;
pub use button::Button;
mod state;
use state::ButtonState;

pub use sdl2::{keyboard::Scancode, mouse::MouseButton};

pub(crate) type KeyboardState = ButtonState<Scancode>;
pub(crate) type MouseState = ButtonState<MouseButton>;

pub struct InputState {
    pub keyboard: KeyboardState,
    pub mouse: MouseState,
}
