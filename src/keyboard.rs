use std::collections::HashMap;

use sdl2::keyboard::{Scancode, ScancodeIterator};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
/// Represents the state of a key.
pub struct Key {
    /// Was the key pressed on this frame?
    pub pressed: bool,
    /// Was the key released on this frame?
    pub released: bool,
    /// Is the key held down?
    pub held: bool,
}

impl Key {
    pub fn new(state: bool) -> Self {
        Self {
            pressed: state,
            released: false,
            held: state,
        }
    }
    /// Change the state based on if the key is currently held.
    /// This is called internally by the engine every frame.
    pub fn update(&mut self, state: bool) {
        self.pressed = state && !self.held;
        self.released = !state && self.held;
        self.held = state;
    }
}

pub struct KeyboardState(HashMap<Scancode, Key>);

impl KeyboardState {
    pub fn new(scancodes: ScancodeIterator) -> Self {
        Self(
            scancodes
                .map(|(key, state)| (key, Key::new(state)))
                .collect(),
        )
    }

    pub fn update(&mut self, scancodes: ScancodeIterator) {
        for (key, state) in scancodes {
            if let Some(prev_state) = self.0.get_mut(&key) {
                prev_state.update(state);
            } else {
                self.0.insert(key, Key::new(state));
            }
        }
    }

    pub fn get(&self, scancode: &Scancode) -> Option<&Key> {
        self.0.get(scancode)
    }
}
