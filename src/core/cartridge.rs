use super::Rom;
use crate::{
    error::{Error, Result},
    kb,
};

const NES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
const PROGRAM_ROM_PAGE_SIZE: usize = kb!(16);
const CHARACTER_ROM_PAGE_SIZE: usize = kb!(8);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}

pub struct Cartridge {
    program_rom: Rom,
    character_rom: Rom,
    mapper: u8,
    screen_mirroring: Mirroring,
}

impl Cartridge {
    pub fn new(data: &Vec<u8>) -> Result<Self> {
        if data[0..4] != NES_TAG {
            return Err(Error::Unsupported(
                "File is not in iNES file format".to_owned(),
            ));
        }

        let mapper = (data[7] & 0b1111_0000) | (data[6] >> 4);

        let ines_ver = (data[7] >> 2) & 0b11;
        if ines_ver != 0 {
            return Err(Error::Unsupported(
                "NES2.0 format is not supported".to_owned(),
            ));
        }

        let four_screen = data[6] & 0b1000 != 0;
        let vertical_mirroring = data[6] & 0b1 != 0;
        let screen_mirroring = match (four_screen, vertical_mirroring) {
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
            (true, _) => Mirroring::FourScreen,
        };

        let program_rom_size = data[4] as usize * PROGRAM_ROM_PAGE_SIZE;
        let character_rom_size = data[5] as usize * CHARACTER_ROM_PAGE_SIZE;

        let skip_trainer = data[6] & 0b100 != 0;

        let program_rom_start = 16 + if skip_trainer { 512 } else { 0 };
        let character_rom_start = program_rom_start + program_rom_size;
        let character_rom_end = character_rom_start + character_rom_size;

        let program_rom = Rom::new(data[program_rom_start..character_rom_start].to_vec());
        let character_rom = Rom::new(data[character_rom_start..character_rom_end].to_vec());

        Ok(Self {
            program_rom,
            character_rom,
            mapper,
            screen_mirroring,
        })
    }

    pub fn program_rom(&self) -> &Rom {
        &self.program_rom
    }

    pub fn character_rom(&self) -> &Rom {
        &self.character_rom
    }

    pub fn mapper(&self) -> u8 {
        self.mapper
    }

    pub fn screen_mirroring(&self) -> Mirroring {
        self.screen_mirroring
    }
}
