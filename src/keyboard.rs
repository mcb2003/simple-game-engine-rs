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
    /// Change the state based on if the key is currently held.
    /// This is called internally by the engine every frame.
    pub fn update(&mut self, state: bool) {
        self.pressed = state && !self.held;
        self.released = !state && self.held;
        self.held = state;
    }
}
