use std::collections::HashMap;

use sdl2::keyboard::{Scancode, ScancodeIterator};

/// Represents the state of a key.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Key {
    /// Was the key pressed on this frame?
    pub pressed: bool,
    /// Was the key released on this frame?
    pub released: bool,
    /// Is the key held down?
    pub held: bool,
}

impl Key {
    /// Create a new Key struct from an initial state.
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
    /// Create an initial state.
    pub fn new(scancodes: ScancodeIterator) -> Self {
        Self(
            scancodes
                .map(|(key, state)| (key, Key::new(state)))
                .collect(),
        )
    }

    /// Update the previous state. This is called by the engine every frame to determine which keys
    /// have been pressed / released.
    pub fn update(&mut self, scancodes: ScancodeIterator) {
        for (key, state) in scancodes {
            if let Some(prev_state) = self.0.get_mut(&key) {
                prev_state.update(state);
            } else {
                self.0.insert(key, Key::new(state));
            }
        }
    }

    /// Get the state of a specific key.
    pub fn get(&self, scancode: &Scancode) -> &Key {
        self.0.get(scancode).unwrap()
    }

    /// Returns if the specified key was pressed on this frame.
    pub fn pressed(&self, scancode: &Scancode) -> bool {
        self.0.get(scancode).unwrap().pressed
    }

    /// Returns if the specified key was released on this frame.
    pub fn released(&self, scancode: &Scancode) -> bool {
        self.0.get(scancode).unwrap().released
    }

    /// Returns if the specified key is held down.
    pub fn held(&self, scancode: &Scancode) -> bool {
        self.0.get(scancode).unwrap().held
    }
}
