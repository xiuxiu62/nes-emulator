use crate::{
    components::{Ram, Rom},
    error::{Error, Result},
    io::{Read, Write},
};

#[allow(unused)]
pub struct Bus<'a> {
    rom: &'a Rom,
    ram: &'a mut Ram,
    ppu: &'a mut (),
    apu: &'a mut (),
    keypad: &'a mut (),
    dma: &'a mut (),
    mmc: &'a mut (),
}

impl<'a> Bus<'a> {
    pub fn new(
        rom: &'a Rom,
        ram: &'a mut Ram,
        ppu: &'a mut (),
        apu: &'a mut (),
        keypad: &'a mut (),
        dma: &'a mut (),
        mmc: &'a mut (),
    ) -> Self {
        Self {
            rom,
            ram,
            ppu,
            apu,
            keypad,
            dma,
            mmc,
        }
    }
}

impl<'a> Read for Bus<'a> {
    fn read_byte(&self, addr: u16) -> Result<u8> {
        match addr {
            0x0000..=0x1FFF => todo!("self.ram.read_byte(addr & 0x07FF)"),
            0x2000..=0x3FFF => todo!("self.ppu.read_byte(addr - 0x2000)"),
            0x4016 => todo!("self.keypad.read_byte(addr)"),
            0x4017 => todo!("multi keypad"),
            0x4000..=0x401F => todo!("self.apu.read_byte(addr - 0x4000)"),
            0x6000..=0x7FFF => Err(Error::Unsupported(format!(
                "this region is for battery backup ram: {addr:#x}"
            ))),
            0x8000..=0xBFFF => todo!("self.rom.read_byte(addr - 0x8000)"),
            0xC000..=0xFFFF => match self.rom.len() {
                0x0000..=0x4000 => todo!("self.rom.read_byte(addr - 0xC000)"),
                _ => todo!("self.rom.read_byte(addr - 0x8000)"),
            },
            _ => Err(Error::Unsupported(format!(
                "[READ] illegal address: {addr:#x}"
            ))),
        }
    }

    fn read_word(&self, addr: u16) -> Result<u16> {
        let lower = self.read_byte(addr)? as u16;
        let upper = self.read_byte(addr + 1)? as u16;

        Ok(upper << 8 | lower)
    }
}

impl<'a> Write for Bus<'a> {
    fn write_byte(&mut self, addr: u16, _byte: u8) -> Result<()> {
        match addr {
            0x0000..=0x1FFF => todo!("self.ram.write_byte(addr & 0x07FF, byte)"),
            0x2000..=0x3FFF => todo!("self.ppu.write_byte(addr - 0x2000, byte)"),
            0x4014 => todo!("self.dma.write_byte(addr, byte)"),
            0x4016 => todo!("self.keypad.write_byte(addr, byte)"),
            0x4017 => todo!("multi keypad"),
            0x4000..=0x401F => todo!("self.apu.write_byte(addr - 0x4000, byte)"),
            0x6000..=0x7FFF => Err(Error::Unsupported(format!(
                "this region is for battery backup ram: {addr:#x}"
            ))),
            0x8000..=0xBFFF => todo!("self.mmc.set_bank(byte)"),
            _ => Err(Error::Unsupported(format!(
                "[WRITE] illegal address: {addr:#x}"
            ))),
        }
    }

    fn write_word(&mut self, addr: u16, word: u16) -> Result<()> {
        let lower = (word & 0xFF) as u8;
        let upper = (word >> 8) as u8;

        self.write_byte(addr, lower)?;
        self.write_byte(addr + 1, upper)
    }
}
