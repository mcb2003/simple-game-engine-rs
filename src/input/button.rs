//! Contains the `Button` type, which stores the state (pressed, held, released) of a single button
//! or key.

/// Represents the state of a button, such as a keyboard key or mouse button.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Button {
    /// Was the key pressed on this frame?
    pub pressed: bool,
    /// Was the key released on this frame?
    pub released: bool,
    /// Is the key held down?
    pub held: bool,
}

impl Button {
    /// Create a new Button struct from an initial state.
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Check initial state on construction.
    #[test]
    fn test_initial_state() {
        assert_eq!(
            Button::new(false),
            Button {
                pressed: false,
                released: false,
                held: false
            }
        );
        assert_eq!(
            Button::new(true),
            Button {
                pressed: true,
                released: false,
                held: true
            }
        );
    }

    /// Test updating a Button's state.
    #[test]
    fn test_update_state() {
        let mut btn = Button::new(false);
        // Pressed (first frame)
        btn.update(true);
        assert_eq!(
            btn,
            Button {
                pressed: true,
                released: false,
                held: true,
            }
        );
        // Held (subsequent frames)
        btn.update(true);
        assert_eq!(
            btn,
            Button {
                pressed: false,
                released: false,
                held: true,
            }
        );
        // Released (first frame)
        btn.update(false);
        assert_eq!(
            btn,
            Button {
                pressed: false,
                released: true,
                held: false,
            }
        );
        // Not held (subsequent frames)
        btn.update(false);
        assert_eq!(
            btn,
            Button {
                pressed: false,
                released: false,
                held: false,
            }
        );
    }
}
