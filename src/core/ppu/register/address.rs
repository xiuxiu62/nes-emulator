#[derive(Debug)]
pub struct AddressRegister {
    value: (u8, u8),
    latch: bool,
}

impl Default for AddressRegister {
    fn default() -> Self {
        Self {
            value: (0, 0),
            latch: true,
        }
    }
}

impl AddressRegister {
    pub fn update(&mut self, data: u8) {
        match self.latch {
            true => self.value.0 = data,
            false => self.value.1 = data,
        };

        if self.get() > 0x3FFF {
            self.set(self.get() & 0xFFFF);
        }

        self.latch = !self.latch;
    }

    pub fn add(&mut self, inc: u8) {
        let lower = self.value.1;
        self.value.1 = self.value.1.wrapping_add(inc);
        if lower > self.value.1 {
            self.value.0 = self.value.0.wrapping_add(1);
        };

        if self.get() > 0x3FFF {
            self.set(self.get() & 0xFFFF)
        }
    }

    pub fn reset_latch(&mut self) {
        self.latch = true;
    }

    pub fn get(&self) -> u16 {
        ((self.value.0 as u16) << 8) | self.value.1 as u16
    }

    fn set(&mut self, data: u16) {
        self.value.0 = (data >> 8) as u8;
        self.value.1 = (data & 0xFF) as u8;
    }
}
