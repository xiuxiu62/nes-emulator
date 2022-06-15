mod addressing_mode;
mod message;
mod opcode;

use crate::{ram::RAM_SIZE, Component, Error, Ram, Result};
use opcode::OPCODE_MAP;
use std::fmt::Display;

pub use addressing_mode::AddressingMode;
pub use message::Message;

#[derive(Debug, Default)]
pub struct Cpu {
    register_a: Component<u8>,
    register_x: Component<u8>,
    register_y: Component<u8>,
    status: Component<u8>,
    program_counter: Component<u16>,
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

    pub fn run(&mut self) -> Result<()> {
        loop {
            let code = self.mem_read_byte(self.program_counter.get());

            self.program_counter.increment();
            let program_counter_state = self.program_counter.get();
            let opcode = OPCODE_MAP.get(&code).ok_or_else(|| {
                Error::Unsupported(format!(r#"opcode "{code:#x}" is not supported"#))
            })?;

            if let Message::Break = self.handle_opcode(opcode)? {
                break;
            }

            if program_counter_state == self.program_counter.get() {
                (0..(opcode.len() - 1) as u16).for_each(|_| self.program_counter.increment())
            }
        }

        Ok(())
    }

    pub fn reset(&mut self) {
        self.register_a.reset();
        self.register_x.reset();
        self.register_y.reset();
        self.status.reset();

        let start_addr = self.mem_read_word(0xFFFC);
        self.program_counter.set(start_addr);
    }

    pub fn mem_dump(&self) -> [u8; RAM_SIZE] {
        self.memory.dump()
    }
    fn mem_read_byte(&self, addr: u16) -> u8 {
        self.memory.read(addr)
    }

    fn mem_write_byte(&mut self, addr: u16, byte: u8) {
        self.memory.write(addr, byte);
    }

    fn mem_read_word(&self, addr: u16) -> u16 {
        let lo = self.mem_read_byte(addr) as u16;
        let hi = self.mem_read_byte(addr + 1) as u16;

        hi << 8 | lo
    }

    fn mem_write_word(&mut self, addr: u16, word: u16) {
        let lo = (word & 0xff) as u8;
        let hi = (word >> 8) as u8;

        self.mem_write_byte(addr, lo);
        self.mem_write_byte(addr + 1, hi);
    }

    fn update_zero_flag(&mut self, result: u8) {
        let old_status = self.status.get();
        self.status.set(match result {
            0 => old_status | 0b0000_0010,
            _ => old_status & 0b1111_1101,
        });
    }

    fn update_negative_flag(&mut self, result: u8) {
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
