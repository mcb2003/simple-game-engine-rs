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
