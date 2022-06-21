use crate::{
    core::{Ram, Rom},
    error::{Error, Result},
    io::{Read, Write},
};

#[allow(unused)]
#[derive(Debug)]
pub struct Bus {
    rom: &'static Rom,
    ram: Ram,
    ppu: (),
    apu: (),
    keypad: (),
    dma: (),
    mmc: (),
}

impl Bus {
    pub fn new(rom: &'static Rom) -> Self {
        Self {
            rom,
            ram: Ram::default(),
            ppu: (),
            apu: (),
            keypad: (),
            dma: (),
            mmc: (),
        }
    }

    // pub fn load_rom(&'a mut self, rom: &'a Rom) {
    // self.rom = rom;
    // }

    pub fn load(&mut self, offset: u16) -> Result<()> {
        for (i, byte) in self.rom.as_ref().iter().enumerate() {
            self.write_byte(offset + i as u16, *byte)?;
        }

        Ok(())
    }
}

// UNWRAP: we've ensured that a rom is loaded
impl Read for Bus {
    fn read_byte(&self, addr: u16) -> Result<u8> {
        match addr {
            0x0000..=0x1FFF => self.ram.read_byte(addr & 0x07FF),
            0x2000..=0x3FFF => todo!("self.ppu.read_byte(addr - 0x2000)"),
            0x4016 => todo!("self.keypad.read_byte(addr)"),
            0x4017 => todo!("multi keypad"),
            0x4000..=0x401F => todo!("self.apu.read_byte(addr - 0x4000)"),
            0x6000..=0x7FFF => Err(Error::Unsupported(format!(
                "this region is for battery backup ram: {addr:#x}"
            ))),
            0x8000..=0xBFFF => self.rom.read_byte(addr - 0x8000),
            0xC000..=0xFFFF => match self.rom.len() {
                0x0000..=0x4000 => self.rom.read_byte(addr - 0xC000),
                _ => self.rom.read_byte(addr - 0x8000),
            },
            _ => Err(Error::Unsupported(format!(
                "[READ] illegal address: {addr:#x}"
            ))),
        }
    }
}

impl Write for Bus {
    fn write_byte(&mut self, addr: u16, byte: u8) -> Result<()> {
        match addr {
            0x0000..=0x1FFF => self.ram.write_byte(addr & 0x07FF, byte),
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
}
