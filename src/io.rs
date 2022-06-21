use crate::error::Result;

pub trait Read {
    fn read_byte(&self, addr: u16) -> Result<u8>;

    fn read_word(&self, addr: u16) -> Result<u16> {
        let lower = self.read_byte(addr)? as u16;
        let upper = self.read_byte(addr + 1)? as u16;

        Ok(upper << 8 | lower)
    }
}

pub trait Write {
    fn write_byte(&mut self, addr: u16, byte: u8) -> Result<()>;

    fn write_word(&mut self, addr: u16, word: u16) -> Result<()> {
        let lower = (word & 0xFF) as u8;
        let upper = (word >> 8) as u8;

        self.write_byte(addr, lower)?;
        self.write_byte(addr + 1, upper)
    }
}
