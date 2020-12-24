#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Key {
    pub pressed: bool,
    pub released: bool,
    pub held: bool
}

impl Key {
    pub fn update(&mut self, state: bool) {
        self.pressed = state && !self.held;
        self.released = !state && self.held;
        self.held = state;
    }
}
