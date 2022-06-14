use std::fmt::Display;

use crate::{ram::RAM_SIZE, Component, Ram};

#[derive(Debug, Default)]
pub struct Cpu {
    pub register_a: Component<u8>,
    pub register_x: Component<u8>,
    pub status: Component<u8>,
    pub program_counter: Component<u16>,
    memory: Ram,
}

impl Cpu {
    pub fn load_program(&mut self, data: &[u8]) {
        self.load(0x8000, data);
    }

    pub fn load(&mut self, offset: u16, data: &[u8]) {
        self.memory.load(offset, data);
        self.program_counter.set(offset);
        self.mem_write_word(0xFFFC, offset);
    }

    pub fn reset(&mut self) {
        self.register_a.set(0);
        self.register_x.set(0);
        self.status.set(0);

        let start_addr = self.mem_read_word(0xFFFC);
        self.program_counter.set(start_addr);
    }

    pub fn mem_read_byte(&self, addr: u16) -> u8 {
        self.memory.read(addr)
    }

    pub fn mem_write_byte(&mut self, addr: u16, byte: u8) {
        self.memory.write(addr, byte);
    }

    pub fn mem_read_word(&self, addr: u16) -> u16 {
        let lo = self.mem_read_byte(addr) as u16;
        let hi = self.mem_read_byte(addr + 1) as u16;

        hi << 8 | lo
    }

    pub fn mem_write_word(&mut self, addr: u16, word: u16) {
        let lo = (word & 0xff) as u8;
        let hi = (word >> 8) as u8;

        self.mem_write_byte(addr, lo);
        self.mem_write_byte(addr + 1, hi);
    }

    pub fn mem_dump(&self) -> [u8; RAM_SIZE] {
        self.memory.dump()
    }

    pub fn update_zero_flag(&mut self, result: u8) {
        let old_status = self.status.get();
        self.status.set(match result {
            0 => old_status | 0b0000_0010,
            _ => old_status & 0b1111_1101,
        });
    }

    pub fn update_negative_flag(&mut self, result: u8) {
        let old_status = self.status.get();
        self.status.set(match result & 0b1000_0000 {
            0 => old_status & 0b0111_1111,
            _ => old_status | 0b1000_0000,
        });
    }
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = format!(
            "CPU {{
    registers: {{
        a: {:#x}
        x: {:#x}            
    }}
    status: {:#x}
    program_counter: {:#x}
}}",
            self.register_a.get(),
            self.register_x.get(),
            self.status.get(),
            self.program_counter.get(),
        );

        write!(f, "{}", message)
    }
}
