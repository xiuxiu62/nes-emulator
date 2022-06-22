mod register;

use super::{Mirroring, Ram, Rom, SubComponent};
use crate::{
    error::{Error, Result},
    io::Write,
    rom,
};
use register::PpuRegisters;

#[derive(Debug)]
pub struct Ppu {
    character_rom: Rom,
    mirroring: Mirroring,
    registers: PpuRegisters,
    vram: Ram,

    oam_address: SubComponent<u8>,
    oam_data: [u8; 256],
    palette_table: [u8; 32],

    data_buffer: SubComponent<u8>,

    scanline: SubComponent<u16>,
    cycles: SubComponent<usize>,
    pub nmi_interrupt: Option<u8>,
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new(rom![0; 2048], Mirroring::Horizontal)
    }
}

impl Ppu {
    pub fn new(character_rom: Rom, mirroring: Mirroring) -> Self {
        Self {
            character_rom,
            mirroring,
            registers: PpuRegisters::default(),
            vram: Ram::default(),
            oam_address: SubComponent::default(),
            oam_data: [0; 64 * 4],
            palette_table: [0; 32],
            data_buffer: SubComponent::default(),
            scanline: SubComponent::default(),
            cycles: SubComponent::default(),
            nmi_interrupt: None,
        }
    }

    pub fn mirror_vram_address(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b10111111111111;
        let vram_index = mirrored_vram - 0x2000;
        let name_table = vram_index / 0x400;

        match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_index - 0x800,
            (Mirroring::Horizontal, 2) => vram_index - 0x400,
            (Mirroring::Horizontal, 1) => vram_index - 0x400,
            (Mirroring::Horizontal, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }

    fn increment_vram_addr(&mut self) {
        self.registers
            .address
            .add(self.registers.control.vram_address_increment());
    }

    pub fn tick(&mut self, cycles: usize) -> bool {
        self.cycles.wrapping_add(cycles);
        if self.cycles.get() >= 341 {
            self.cycles.wrapping_sub(341);
            self.scanline.increment();

            if self.scanline.get() == 241 {
                self.registers.status.set_vblank_status(true);
                self.registers.status.set_sprite_zero_hit(false);

                if self.registers.control.generate_vblank_nmi() {
                    self.nmi_interrupt = Some(1);
                }
            }

            if self.scanline.get() >= 262 {
                self.scanline.set(0);
                self.nmi_interrupt = None;

                self.registers.status.set_sprite_zero_hit(false);
                self.registers.status.reset_vblank_status();

                return true;
            }
        }

        false
    }

    pub fn poll_nmi_interrupt(&mut self) -> Option<u8> {
        self.nmi_interrupt.take()
    }

    pub fn write_to_ctrl(&mut self, value: u8) {
        let before_nmi_status = self.registers.control.generate_vblank_nmi();
        self.registers.control.update(value);

        if !before_nmi_status
            && self.registers.control.generate_vblank_nmi()
            && self.registers.status.is_in_vblank()
        {
            self.nmi_interrupt = Some(1);
        }
    }

    pub fn write_to_mask(&mut self, value: u8) {
        self.registers.mask.update(value);
    }

    pub fn read_status(&mut self) -> u8 {
        let data = self.registers.status.snapshot();
        self.registers.status.reset_vblank_status();
        self.registers.address.reset_latch();
        self.registers.scroll.reset_latch();

        data
    }

    pub fn write_to_oam_addr(&mut self, value: u8) {
        self.oam_address.set(value);
    }

    pub fn write_to_oam_data(&mut self, value: u8) {
        self.oam_data[self.oam_address.get() as usize] = value;
        self.oam_address.wrapping_add(1);
    }

    pub fn read_oam_data(&self) -> u8 {
        self.oam_data[self.oam_address.get() as usize]
    }

    pub fn write_to_scroll(&mut self, value: u8) {
        self.registers.scroll.write(value);
    }

    pub fn write_to_ppu_addr(&mut self, value: u8) {
        self.registers.address.update(value);
    }

    pub fn write_to_data(&mut self, value: u8) -> Result<()> {
        let addr = self.registers.address.get();
        match addr {
            0..=0x1fff => println!("attempt to write to chr rom space {}", addr),
            0x2000..=0x2fff => {
                let address = self.mirror_vram_address(addr);
                self.vram.write_byte(address, value)?;
            }
            0x3000..=0x3eff => unimplemented!("addr {} shouldn't be used in reallity", addr),
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize] = value;
            }
            0x3f00..=0x3fff => {
                self.palette_table[(addr - 0x3f00) as usize] = value;
            }
            _ => {
                return Err(Error::Illegal(format!(
                    "unexpected access to mirrored space {addr}"
                )))
            }
        }

        self.increment_vram_addr();

        Ok(())
    }

    pub fn read_data(&mut self) -> Result<u8> {
        let addr = self.registers.address.get();
        self.increment_vram_addr();

        match addr {
            0..=0x1fff => {
                let result = self.data_buffer.get();
                self.data_buffer
                    .set(self.character_rom.as_ref()[addr as usize]);

                Ok(result)
            }
            0x2000..=0x2fff => {
                let result = self.data_buffer.get();
                self.data_buffer
                    .set(self.vram.as_ref()[self.mirror_vram_address(addr) as usize]);

                Ok(result)
            }
            0x3000..=0x3eff => unimplemented!("addr {} shouldn't be used in reallity", addr),

            //Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;

                Ok(self.palette_table[(add_mirror - 0x3f00) as usize])
            }

            0x3f00..=0x3fff => Ok(self.palette_table[(addr - 0x3f00) as usize]),
            _ => Err(Error::Illegal(format!(
                "unexpected access to mirrored space {addr}"
            ))),
        }
    }

    fn write_oam_dma(&mut self, data: &[u8; 256]) {
        data.iter().for_each(|n| {
            self.oam_data[self.oam_address.get() as usize] = *n;
            self.oam_address.wrapping_add(1);
        })
    }
}
