use crate::error::{Error, Result};

bitflags! {
    pub struct ControlRegister: u8 {
        const NAMETABLE1              = 0b00000001;
        const NAMETABLE2              = 0b00000010;
        const VRAM_ADD_INCREMENT      = 0b00000100;
        const SPRITE_PATTERN_ADDR     = 0b00001000;
        const BACKROUND_PATTERN_ADDR  = 0b00010000;
        const SPRITE_SIZE             = 0b00100000;
        const MAIN_SUB_SELECT     = 0b01000000;
        const GENERATE_NMI            = 0b10000000;
    }
}

impl Default for ControlRegister {
    fn default() -> Self {
        Self::from_bits_truncate(0b0000_0000)
    }
}

impl ControlRegister {
    pub fn update(&mut self, data: u8) {
        self.bits = data;
    }

    pub fn nametable_address(&self) -> Result<u16> {
        match self.bits() & 0b11 {
            0 => Ok(0x2000),
            1 => Ok(0x2400),
            2 => Ok(0x2800),
            3 => Ok(0x2C00),
            address => Err(Error::Unsupported(format!(
                "Invalid nametable address: {address}",
            ))),
        }
    }

    pub fn vram_address_increment(&self) -> u8 {
        match self.contains(ControlRegister::VRAM_ADD_INCREMENT) {
            true => 32,
            false => 1,
        }
    }

    pub fn sprite_pattern_address(&self) -> u16 {
        match self.contains(ControlRegister::SPRITE_PATTERN_ADDR) {
            true => 16,
            false => 8,
        }
    }

    pub fn main_sub_select(&self) -> u16 {
        match self.contains(ControlRegister::MAIN_SUB_SELECT) {
            true => 1,
            false => 0,
        }
    }

    pub fn generate_vblank_nmi(&self) -> bool {
        self.contains(ControlRegister::GENERATE_NMI)
    }
}
