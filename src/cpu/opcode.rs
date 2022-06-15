use super::{AddressingMode, Cpu, Message};
use crate::{Error, Result};
use std::collections::HashMap;

#[allow(unused)]
pub struct OpCode {
    code: u8,
    mnemonic: &'static str,
    len: u8,
    cycles: u8,
    mode: AddressingMode,
}

impl OpCode {
    pub fn new(
        code: u8,
        mnemonic: &'static str,
        len: u8,
        cycles: u8,
        mode: AddressingMode,
    ) -> Self {
        Self {
            code,
            mnemonic,
            len,
            cycles,
            mode,
        }
    }

    pub fn len(&self) -> u8 {
        self.len
    }
}

lazy_static! {
    pub static ref OPCODES: Vec<OpCode> = vec![
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),
        OpCode::new(0xaa, "TAX", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xe8, "INX", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xa9, "LDA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa5, "LDA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb5, "LDA", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0xad, "LDA", 3, 4, AddressingMode::Absolute),
        OpCode::new(
            0xbd,
            "LDA",
            3,
            4, /*+1 if page crossed*/
            AddressingMode::AbsoluteX,
        ),
        OpCode::new(
            0xb9,
            "LDA",
            3,
            4, /*+1 if page crossed*/
            AddressingMode::AbsoluteY,
        ),
        OpCode::new(0xa1, "LDA", 2, 6, AddressingMode::IndirectX),
        OpCode::new(
            0xb1,
            "LDA",
            2,
            5, /*+1 if page crossed*/
            AddressingMode::IndirectY,
        ),
        OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x8d, "STA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x9d, "STA", 3, 5, AddressingMode::AbsoluteX),
        OpCode::new(0x99, "STA", 3, 5, AddressingMode::AbsoluteY),
        OpCode::new(0x81, "STA", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x91, "STA", 2, 6, AddressingMode::IndirectY),
    ];

    pub static ref OPCODE_MAP: HashMap<u8, &'static OpCode> = {
        let mut map = HashMap::new();
        let populator = |op: &'static OpCode| {
            map.insert(op.code, op);
        };

        OPCODES.iter().for_each(populator);

        map
    };
}

impl Cpu {
    pub fn handle_opcode(&mut self, opcode: &OpCode) -> Result<Message> {
        let code = opcode.code;
        match code {
            0x00 => return Ok(Message::Break),
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(&opcode.mode)?,
            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => self.sta(&opcode.mode)?,
            0xAA => self.tax(),
            0xE8 => self.inx(),
            _ => {
                return Err(Error::Unsupported(format!(
                    r#"opcode "{code:#x}" is not supported"#
                )))
            }
        };

        Ok(Message::Continue)
    }

    fn lda(&mut self, mode: &AddressingMode) -> Result<()> {
        let addr = self.get_operand_address(mode)?;
        let value = self.mem_read_byte(addr);

        self.register_a.set(value);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        Ok(())
    }

    fn sta(&mut self, mode: &AddressingMode) -> Result<()> {
        let addr = self.get_operand_address(mode)?;
        self.mem_write_byte(addr, self.register_a.get());

        Ok(())
    }

    fn tax(&mut self) {
        self.register_x.set(self.register_a.get());

        let register_x = self.register_x.get();
        self.update_zero_flag(register_x);
        self.update_negative_flag(register_x);
    }

    fn inx(&mut self) {
        self.register_x.increment();

        let register_x = self.register_x.get();
        self.update_zero_flag(register_x);
        self.update_negative_flag(register_x);
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> Result<u16> {
        match mode {
            AddressingMode::Immediate => Ok(self.program_counter.get()),
            AddressingMode::ZeroPage => Ok(self.mem_read_byte(self.program_counter.get()) as u16),
            AddressingMode::Absolute => Ok(self.mem_read_word(self.program_counter.get())),
            AddressingMode::ZeroPageX => {
                let pos = self.mem_read_byte(self.program_counter.get());
                let addr = pos.wrapping_add(self.register_x.get()) as u16;

                Ok(addr)
            }
            AddressingMode::ZeroPageY => {
                let pos = self.mem_read_byte(self.program_counter.get());
                let addr = pos.wrapping_add(self.register_y.get()) as u16;

                Ok(addr)
            }
            AddressingMode::AbsoluteX => {
                let base = self.mem_read_word(self.program_counter.get());
                let addr = base.wrapping_add(self.register_x.get() as u16);

                Ok(addr)
            }
            AddressingMode::AbsoluteY => {
                let base = self.mem_read_word(self.program_counter.get());
                let addr = base.wrapping_add(self.register_y.get() as u16);

                Ok(addr)
            }

            AddressingMode::IndirectX => {
                let base = self.mem_read_byte(self.program_counter.get());

                let ptr: u8 = (base as u8).wrapping_add(self.register_x.get());
                let lo = self.mem_read_byte(ptr as u16);
                let hi = self.mem_read_byte(ptr.wrapping_add(1) as u16);

                Ok((hi as u16) << 8 | (lo as u16))
            }
            AddressingMode::IndirectY => {
                let base = self.mem_read_byte(self.program_counter.get());

                let lo = self.mem_read_byte(base as u16);
                let hi = self.mem_read_byte((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y.get() as u16);

                Ok(deref)
            }
            AddressingMode::NoneAddressing => Err(Error::Unsupported(format!(
                "addressing mode {mode:?} is not supported"
            ))),
        }
    }
    // register_opcode! [
    //     (oc_0xa9, "LDA (Immediate Mode)") => self {
    //         self.lda(AddressingMode::Immediate)?;
    //         self.program_counter.increment();
    //     },
    //     (oc_0xa5, "LDA (Zero Page Mode)") => self {
    //         self.lda(AddressingMode::ZeroPage)?;
    //         self.program_counter.increment();
    //     },
    //     (oc_0xad, "LDA (Absolute Mode)") => self {
    //         self.lda(AddressingMode::Absolute)?;
    //         (0..2).for_each(|_| self.program_counter.increment());
    //     },
    //     (oc_0xaa, "TAX: copies the a register into the x register") => self {
    //         self.register_x.set(self.register_a.get());

    //         let register_x = self.register_x.get();
    //         self.update_zero_flag(register_x);
    //         self.update_negative_flag(register_x);
    //     },
    //     (oc_0xe8, "INX: increments the x register") => self {
    //         self.register_x.increment();

    //         let register_x = self.register_x.get();
    //         self.update_zero_flag(register_x);
    //         self.update_negative_flag(register_x);
    //     }
    // ];
}
