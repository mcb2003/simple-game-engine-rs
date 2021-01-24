//! Types related to the state of various input devices.

use std::{hash::Hash, ops::Index};

use fnv::FnvHashMap;

use super::Button;

/// The state (pressed, held, released) of a collection of buttons.
pub struct ButtonState<T>(FnvHashMap<T, Button>);

impl<T> ButtonState<T>
where
    T: Eq + Hash,
{
    /// Create an initial state.
    pub fn new(buttons: impl Iterator<Item = (T, bool)>) -> Self {
        Self(buttons.map(|(b, state)| (b, Button::new(state))).collect())
    }

    /// Update the previous state. This is called by the engine every frame to determine which buttons
    /// have been pressed / released.
    pub fn update(&mut self, buttons: impl Iterator<Item = (T, bool)>) {
        for (button, state) in buttons {
            if let Some(prev_state) = self.0.get_mut(&button) {
                prev_state.update(state);
            } else {
                self.0.insert(button, Button::new(state));
            }
        }
    }

    /// Get the state of a specific button.
    pub fn get(&self, button: T) -> &Button {
        self.0.get(&button).unwrap()
    }

    /// Returns if the specified button was pressed on this frame.
    pub fn pressed(&self, button: T) -> bool {
        self.0.get(&button).unwrap().pressed
    }

    /// Returns if the specified button was released on this frame.
    pub fn released(&self, button: T) -> bool {
        self.0.get(&button).unwrap().released
    }

    /// Returns if the specified button is held.
    pub fn held(&self, button: T) -> bool {
        self.0.get(&button).unwrap().held
    }
}

impl<T> Index<T> for ButtonState<T>
where
    T: Eq + Hash,
{
    type Output = Button;

    fn index(&self, button: T) -> &Self::Output {
        self.get(button)
    }
}
