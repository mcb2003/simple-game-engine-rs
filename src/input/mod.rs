//! Data structures for inspecting user input.

mod button;
pub use button::Button;
mod state;
use state::ButtonState;

mod mouse;
pub use mouse::MouseState;

pub use sdl2::{keyboard::Scancode, mouse::MouseButton};

pub(crate) type KeyboardState = ButtonState<Scancode>;

/// The state of all supported input devices.
pub struct InputState {
    /// State of every SDL2 supported key on the keyboard
    pub keyboard: KeyboardState,
    /// State of every SDL2 supported mouse button, as well as the cursor's *x* and *y* coordinates
    pub mouse: MouseState,
}
