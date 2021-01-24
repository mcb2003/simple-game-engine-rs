//! Types related to the state of the mouse.

use std::ops::Deref;

use sdl2::mouse::MouseState as SdlMouseState;

use super::{ButtonState, MouseButton};

/// The cursor position and state of the mouse buttons.
pub struct MouseState {
    /// The state of every SDL2 supported mouse button.
    pub buttons: ButtonState<MouseButton>,
    /// *X* coordinate of the mouse cursor.
    pub x: i32,
    /// *Y* coordinate of the mouse cursor.
    pub y: i32,
}

impl MouseState {
    /// Create an initial state from an `sdl2::mouse::MouseState`. Called internally by the engine.
    pub(crate) fn new(state: SdlMouseState) -> Self {
        Self {
            buttons: ButtonState::new(state.mouse_buttons()),
            x: state.x(),
            y: state.y(),
        }
    }

    /// Update the existing state from the current `sdl2::mouse::MouseState`. This is called
    /// internally by the engine on every frame.
    pub(crate) fn update(&mut self, state: SdlMouseState) {
        self.buttons.update(state.mouse_buttons());
        self.x = state.x();
        self.y = state.y();
    }
}
