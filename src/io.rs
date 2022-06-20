use crate::error::Result;

pub trait Read {
    fn read_byte(&self, addr: u16) -> Result<u8>;

    fn read_word(&self, addr: u16) -> Result<u16>;
}

pub trait Write {
    fn write_byte(&mut self, addr: u16, byte: u8) -> Result<()>;

    fn write_word(&mut self, addr: u16, word: u16) -> Result<()>;
}
