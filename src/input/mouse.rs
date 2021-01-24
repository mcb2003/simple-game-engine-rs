use std::ops::Deref;

use sdl2::mouse::MouseState as SdlMouseState;

use super::{ButtonState, MouseButton};

pub struct MouseState {
    pub buttons: ButtonState<MouseButton>,
    pub x: i32,
    pub y: i32,
}

impl MouseState {
    pub(crate) fn new(state: SdlMouseState) -> Self {
        Self {
            buttons: ButtonState::new(state.mouse_buttons()),
            x: state.x(),
            y: state.y(),
        }
    }

    pub(crate) fn update(&mut self, state: SdlMouseState) {
        self.buttons.update(state.mouse_buttons());
        self.x = state.x();
        self.y = state.y();
    }
}
