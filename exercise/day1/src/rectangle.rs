pub(crate) struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}
