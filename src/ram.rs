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
