#[derive(Debug, Default)]
pub struct ScrollRegister {
    x: u8,
    y: u8,
    latch: bool,
}

impl ScrollRegister {
    pub fn write(&mut self, data: u8) {
        match self.latch {
            true => self.y = data,
            false => self.x = data,
        };

        self.latch = !self.latch;
    }

    pub fn reset_latch(&mut self) {
        self.latch = false;
    }
}
