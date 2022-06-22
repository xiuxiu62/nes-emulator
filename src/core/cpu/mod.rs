use crate::{
    core::{Bus, SubComponent, OPCODE_MAP},
    error::{Error, Result},
    io::{Read, Write},
};
use std::fmt::Display;

mod flags;
mod message;
mod opcode;

pub use flags::CpuFlags;
pub use message::CpuMessage;

use super::Cartridge;

pub const STACK_START_ADDR: u16 = 0x0100;
pub const STACK_RESET: u8 = 0xFD;

#[derive(Debug)]
pub struct Cpu {
    pub(crate) register_a: SubComponent<u8>,
    pub(crate) register_x: SubComponent<u8>,
    pub(crate) register_y: SubComponent<u8>,
    pub(crate) program_counter: SubComponent<u16>,
    pub(crate) stack_pointer: SubComponent<u8>,
    pub(crate) status: CpuFlags,
    pub(crate) bus: Bus,
}

impl Cpu {
    pub fn new(bus: Bus) -> Self {
        Self {
            register_a: SubComponent::default(),
            register_x: SubComponent::default(),
            register_y: SubComponent::default(),
            program_counter: SubComponent::default(),
            stack_pointer: SubComponent::new(STACK_RESET),
            status: CpuFlags::default(),
            bus,
        }
    }

    pub fn load_cartridge(&mut self, cartridge: &Cartridge) {
        self.bus.load_cartridge(cartridge);
    }

    pub fn load(&mut self) -> Result<()> {
        let offset = 0x0600;

        self.bus.load(offset)?;
        self.program_counter.set(offset);

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        self.run_with_callback(|_| Ok(()))
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F) -> Result<()>
    where
        F: FnMut(&mut Cpu) -> Result<()>,
    {
        loop {
            if self.bus.poll_nmi_status().is_some() {
                // self.interrupt(interrupt::NMI)
            }

            callback(self)?;

            let program_counter = self.program_counter.get();
            let code = self.read_byte(program_counter)?;

            self.program_counter.increment();
            let program_counter = self.program_counter.get();
            let opcode = OPCODE_MAP.get(&code).ok_or_else(|| {
                Error::Unsupported(format!(r#"opcode "{code:#x}" is not supported"#))
            })?;

            if let CpuMessage::Break = self.handle_opcode(opcode)? {
                break;
            }

            self.bus.tick(opcode.cycles as usize);

            if program_counter == self.program_counter.get() {
                (0..(opcode.len() - 1) as u16).for_each(|_| self.program_counter.increment())
            }
        }

        Ok(())
    }

    pub fn reset(&mut self) -> Result<()> {
        self.register_a.reset();
        self.register_x.reset();
        self.register_y.reset();
        self.status.reset();

        let start_addr = self.read_word(0xFFFC)?;
        self.program_counter.set(start_addr);

        Ok(())
    }

    fn set_carry_flag(&mut self) {
        self.status.insert(CpuFlags::CARRY);
    }

    fn clear_carry_flag(&mut self) {
        self.status.remove(CpuFlags::CARRY);
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

impl Read for Cpu {
    fn read_byte(&mut self, addr: u16) -> Result<u8> {
        self.bus.read_byte(addr)
    }
}

impl Write for Cpu {
    fn write_byte(&mut self, addr: u16, byte: u8) -> Result<()> {
        self.bus.write_byte(addr, byte)
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
