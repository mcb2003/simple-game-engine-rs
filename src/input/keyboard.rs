use std::{collections::HashMap, ops::Index};

use sdl2::keyboard::{Scancode, ScancodeIterator};

use super::Button;

pub struct KeyboardState(HashMap<Scancode, Button>);

impl KeyboardState {
    /// Create an initial state.
    pub fn new(scancodes: ScancodeIterator) -> Self {
        Self(
            scancodes
                .map(|(key, state)| (key, Button::new(state)))
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
                self.0.insert(key, Button::new(state));
            }
        }
    }

    /// Get the state of a specific key.
    pub fn get(&self, scancode: Scancode) -> &Button {
        self.0.get(&scancode).unwrap()
    }

    /// Returns if the specified key was pressed on this frame.
    pub fn pressed(&self, scancode: Scancode) -> bool {
        self.0.get(&scancode).unwrap().pressed
    }

    /// Returns if the specified key was released on this frame.
    pub fn released(&self, scancode: Scancode) -> bool {
        self.0.get(&scancode).unwrap().released
    }

    /// Returns if the specified key is held down.
    pub fn held(&self, scancode: Scancode) -> bool {
        self.0.get(&scancode).unwrap().held
    }
}

impl Index<Scancode> for KeyboardState {
    type Output = Button;

    fn index(&self, key: Scancode) -> &Self::Output {
        self.get(key)
    }
}
