use crate::{
    error::Result,
    io::{Read, Write},
};
use std::fmt::Display;

pub const RAM_SIZE: usize = 0xFFFF;

#[derive(Debug)]
pub struct Ram([u8; RAM_SIZE]);

impl Ram {
    pub fn read(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    pub fn write(&mut self, addr: u16, byte: u8) {
        self.0[addr as usize] = byte;
    }

    pub fn load(&mut self, offset: u16, data: &[u8]) {
        self.0[offset as usize..(offset as usize + data.len())].copy_from_slice(data);
    }

    pub fn dump(&self) -> [u8; RAM_SIZE] {
        self.0
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self([0x00; 0xFFFF])
    }
}

impl Read for Ram {
    fn read_byte(&self, addr: u16) -> Result<u8> {
        Ok(self.0[addr as usize])
    }

    fn read_word(&self, addr: u16) -> Result<u16> {
        let lower = self.read_byte(addr)? as u16;
        let upper = self.read_byte(addr + 1)? as u16;

        Ok(upper << 8 | lower)
    }
}

impl Write for Ram {
    fn write_byte(&mut self, addr: u16, byte: u8) -> Result<()> {
        self.0[addr as usize] = byte;

        Ok(())
    }

    fn write_word(&mut self, addr: u16, word: u16) -> Result<()> {
        let lower = (word & 0xFF) as u8;
        let upper = (word >> 8) as u8;

        self.write_byte(addr, lower)?;
        self.write_byte(addr + 1, upper)
    }
}

impl Display for Ram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:#}]",
            self.0
                .iter()
                .fold(String::new(), |acc, byte| format!("{acc} {byte}"))
        )
    }
}
