mod addressing_mode;
mod flags;
mod message;
mod opcode;

use crate::{
    components::{ram::RAM_SIZE, Ram, Rom, SubComponent},
    error::{Error, Result},
};
use opcode::OPCODE_MAP;
use std::fmt::Display;

pub use addressing_mode::AddressingMode;
pub use flags::CpuFlags;
pub use message::CpuMessage;

#[derive(Debug, Default)]
pub struct Cpu {
    pub(crate) register_a: SubComponent<u8>,
    pub(crate) register_x: SubComponent<u8>,
    pub(crate) register_y: SubComponent<u8>,
    pub(crate) program_counter: SubComponent<u16>,
    pub(crate) stack_pointer: SubComponent<u8>,
    pub(crate) status: CpuFlags,
    pub(crate) memory: Ram,
}

impl Cpu {
    pub fn load_rom(&mut self, rom: &Rom) {
        self.load(0x8000, rom);
    }

    pub fn load(&mut self, offset: u16, rom: &Rom) {
        let data = rom.as_ref();

        self.memory.load(offset, data);
        self.program_counter.set(offset);
        self.mem_write_word(0xFFFC, offset);
    }

    pub fn run(&mut self) -> Result<()> {
        self.run_with_callback(|_| {})
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F) -> Result<()>
    where
        F: FnMut(&mut Cpu),
    {
        loop {
            let code = self.mem_read_byte(self.program_counter.get());

            self.program_counter.increment();
            let program_counter_state = self.program_counter.get();
            let opcode = OPCODE_MAP.get(&code).ok_or_else(|| {
                Error::Unsupported(format!(r#"opcode "{code:#x}" is not supported"#))
            })?;

            if let CpuMessage::Break = self.handle_opcode(opcode)? {
                break;
            }

            if program_counter_state == self.program_counter.get() {
                (0..(opcode.len() - 1) as u16).for_each(|_| self.program_counter.increment())
            }

            callback(self)
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

    fn update_zero_flag(&mut self, result: u8) {
        match result {
            0 => self.status.insert(CpuFlags::ZERO),
            _ => self.status.remove(CpuFlags::ZERO),
        };
    }

    fn update_negative_flag(&mut self, result: u8) {
        match result & 0b1000_0000 {
            0 => self.status.remove(CpuFlags::NEGATIVE),
            _ => self.status.insert(CpuFlags::NEGATIVE),
        };
    }
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = format!(
            "CPU {{
    registers: {{
        a: {:#x}
        x: {:#x}            
        y: {:#x}
    }}
    program_counter: {:#x}
    stack_pointer: {:#x}
    status: {:#x}
}}",
            self.register_a.get(),
            self.register_x.get(),
            self.register_y.get(),
            self.program_counter.get(),
            self.stack_pointer.get(),
            self.status,
        );

        write!(f, "{}", message)
    }
}
