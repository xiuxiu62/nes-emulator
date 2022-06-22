use std::mem;

use super::{Cartridge, Ppu, Ram, Rom, SubComponent};
use crate::{
    error::{Error, Result},
    io::{Read, Write},
};

const RAM_START: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS_START: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

#[allow(unused)]
#[derive(Debug)]
pub struct Bus {
    program_rom: Rom,
    ram: Ram,
    ppu: Ppu,
    apu: (),
    keypad: (),
    dma: (),
    mmc: (),
    cycles: SubComponent<usize>,
}

impl Bus {
    pub fn new(cartridge: &Cartridge) -> Self {
        let program_rom = cartridge.program_rom().to_owned();
        let character_rom = cartridge.character_rom().to_owned();
        let mirroring = cartridge.screen_mirroring().to_owned();
        let ppu = Ppu::new(character_rom, mirroring);

        Self {
            program_rom,
            ram: Ram::default(),
            ppu,
            apu: (),
            keypad: (),
            dma: (),
            mmc: (),
            cycles: SubComponent::default(),
        }
    }

    pub fn load_cartridge(&mut self, cartridge: &Cartridge) {
        let _ = mem::replace(self, Self::new(cartridge));
    }

    pub fn load(&mut self, offset: u16) -> Result<()> {
        self.program_rom
            .clone()
            .into_iter()
            .enumerate()
            .try_for_each(|(i, byte)| self.write_byte(offset + i as u16, byte))
    }

    pub fn tick(&mut self, cycles: usize) {
        self.cycles.wrapping_add(cycles);
        self.ppu.tick(cycles * 3);
    }

    pub fn poll_nmi_status(&mut self) -> Option<u8> {
        self.ppu.poll_nmi_interrupt()
    }
}

// UNWRAP: we've ensured that a rom is loaded
impl Read for Bus {
    fn read_byte(&mut self, addr: u16) -> Result<u8> {
        match addr {
            RAM_START..=RAM_MIRRORS_END => self.ram.read_byte(addr & 0x07FF),
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => Err(Error::Illegal(format!(
                "Attempted to read from write-only PPU address: {addr:#x}"
            ))),
            0x2002 => Ok(self.ppu.read_status()),
            0x2004 => Ok(self.ppu.read_oam_data()),
            0x2007 => self.ppu.read_data(),
            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0010_0000_0000_0111;

                self.read_byte(mirror_down_addr)
            }
            0x8000..=0xFFFF => {
                let mut addr = addr - 0x8000;
                if self.program_rom.len() == 0x4000 && addr >= 0x4000 {
                    addr %= 0x4000;
                }

                self.program_rom.read_byte(addr)
            }
            _ => Err(Error::Unsupported(format!(
                "[READ] illegal address: {addr:#x}"
            ))),
        }
    }
}

impl Write for Bus {
    fn write_byte(&mut self, addr: u16, byte: u8) -> Result<()> {
        match addr {
            RAM_START..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b11111111111;

                self.ram.write_byte(mirror_down_addr, byte)
            }
            0x2000 => {
                self.ppu.write_to_ctrl(byte);
                Ok(())
            },
            0x2001 => {
                self.ppu.write_to_mask(byte);
                Ok(())
            },
            0x2002 => Err(Error::Illegal(format!(
                "attempted to write to PPU status register: {addr:#x}"
            ))),
            0x2003 => {
                self.ppu.write_to_oam_addr(byte);
                Ok(())
            },
            0x2004 => {
                self.ppu.write_to_oam_data(byte);
                Ok(())
            },
            0x2005 => {
                self.ppu.write_to_scroll(byte);
                Ok(())
            },
            0x2006 => {
                self.ppu.write_to_ppu_addr(byte);
                Ok(())
            },
            0x2007 => self.ppu.write_to_data(byte),
            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0010_0000_0000_0111;

                self.write_byte(mirror_down_addr, byte)
            }
            0x8000..=0xFFFF => Err(Error::Illegal(format!(
                "attempted to write to Cartridge ROM: {addr:#x}"
            ))),
            _ => Err(Error::Illegal(format!(
                "ignoring mem write-access: {addr:#x}"
            ))),
        }
    }
}
