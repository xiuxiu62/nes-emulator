use super::{AddressingMode, Cpu, CpuFlags, CpuMessage};
use crate::error::{Error, Result};
use std::collections::HashMap;

const STACK_START_ADDR: u16 = 0x0100;
// const STACK_RESET: u8 = 0xFD;

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
                OpCode::new(0xEA, "NOP", 1, 2, AddressingMode::NoneAddressing),

                /* Arithmetic */
                OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
                OpCode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),
                OpCode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPageX),
                OpCode::new(0x6D, "ADC", 3, 4, AddressingMode::Absolute),
                OpCode::new(0x7D, "ADC", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteX),
                OpCode::new(0x79, "ADC", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteY),
                OpCode::new(0x61, "ADC", 2, 6, AddressingMode::IndirectX),
                OpCode::new(0x71, "ADC", 2, 5/*+1 if page crossed*/, AddressingMode::IndirectY),

                OpCode::new(0xE9, "SBC", 2, 2, AddressingMode::Immediate),
                OpCode::new(0xE5, "SBC", 2, 3, AddressingMode::ZeroPage),
                OpCode::new(0xF5, "SBC", 2, 4, AddressingMode::ZeroPageX),
                OpCode::new(0xED, "SBC", 3, 4, AddressingMode::Absolute),
                OpCode::new(0xFD, "SBC", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteX),
                OpCode::new(0xf9, "SBC", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteY),
                OpCode::new(0xE1, "SBC", 2, 6, AddressingMode::IndirectX),
                OpCode::new(0xF1, "SBC", 2, 5/*+1 if page crossed*/, AddressingMode::IndirectY),

                OpCode::new(0x29, "AND", 2, 2, AddressingMode::Immediate),
                OpCode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage),
                OpCode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPageX),
                OpCode::new(0x2D, "AND", 3, 4, AddressingMode::Absolute),
                OpCode::new(0x3D, "AND", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteX),
                OpCode::new(0x39, "AND", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteY),
                OpCode::new(0x21, "AND", 2, 6, AddressingMode::IndirectX),
                OpCode::new(0x31, "AND", 2, 5/*+1 if page crossed*/, AddressingMode::IndirectY),

                OpCode::new(0x49, "EOR", 2, 2, AddressingMode::Immediate),
                OpCode::new(0x45, "EOR", 2, 3, AddressingMode::ZeroPage),
                OpCode::new(0x55, "EOR", 2, 4, AddressingMode::ZeroPageX),
                OpCode::new(0x4D, "EOR", 3, 4, AddressingMode::Absolute),
                OpCode::new(0x5D, "EOR", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteX),
                OpCode::new(0x59, "EOR", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteY),
                OpCode::new(0x41, "EOR", 2, 6, AddressingMode::IndirectX),
                OpCode::new(0x51, "EOR", 2, 5/*+1 if page crossed*/, AddressingMode::IndirectY),

                OpCode::new(0x09, "ORA", 2, 2, AddressingMode::Immediate),
                OpCode::new(0x05, "ORA", 2, 3, AddressingMode::ZeroPage),
                OpCode::new(0x15, "ORA", 2, 4, AddressingMode::ZeroPageX),
                OpCode::new(0x0D, "ORA", 3, 4, AddressingMode::Absolute),
                OpCode::new(0x1D, "ORA", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteX),
                OpCode::new(0x19, "ORA", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteY),
                OpCode::new(0x01, "ORA", 2, 6, AddressingMode::IndirectX),
                OpCode::new(0x11, "ORA", 2, 5/*+1 if page crossed*/, AddressingMode::IndirectY),

                /* Shifts */
                OpCode::new(0x0A, "ASL", 1, 2, AddressingMode::NoneAddressing),
                OpCode::new(0x06, "ASL", 2, 5, AddressingMode::ZeroPage),
                OpCode::new(0x16, "ASL", 2, 6, AddressingMode::ZeroPageX),
                OpCode::new(0x0E, "ASL", 3, 6, AddressingMode::Absolute),
                OpCode::new(0x1E, "ASL", 3, 7, AddressingMode::AbsoluteX),

                OpCode::new(0x4A, "LSR", 1, 2, AddressingMode::NoneAddressing),
                OpCode::new(0x46, "LSR", 2, 5, AddressingMode::ZeroPage),
                OpCode::new(0x56, "LSR", 2, 6, AddressingMode::ZeroPageX),
                OpCode::new(0x4E, "LSR", 3, 6, AddressingMode::Absolute),
                OpCode::new(0x5E, "LSR", 3, 7, AddressingMode::AbsoluteX),

                OpCode::new(0x2A, "ROL", 1, 2, AddressingMode::NoneAddressing),
                OpCode::new(0x26, "ROL", 2, 5, AddressingMode::ZeroPage),
                OpCode::new(0x36, "ROL", 2, 6, AddressingMode::ZeroPageX),
                OpCode::new(0x2E, "ROL", 3, 6, AddressingMode::Absolute),
                OpCode::new(0x3E, "ROL", 3, 7, AddressingMode::AbsoluteX),

                OpCode::new(0x6A, "ROR", 1, 2, AddressingMode::NoneAddressing),
                OpCode::new(0x66, "ROR", 2, 5, AddressingMode::ZeroPage),
                OpCode::new(0x76, "ROR", 2, 6, AddressingMode::ZeroPageX),
                OpCode::new(0x6E, "ROR", 3, 6, AddressingMode::Absolute),
                OpCode::new(0x7E, "ROR", 3, 7, AddressingMode::AbsoluteX),

                OpCode::new(0xE6, "INC", 2, 5, AddressingMode::ZeroPage),
                OpCode::new(0xF6, "INC", 2, 6, AddressingMode::ZeroPageX),
                OpCode::new(0xEE, "INC", 3, 6, AddressingMode::Absolute),
                OpCode::new(0xFE, "INC", 3, 7, AddressingMode::AbsoluteX),

                OpCode::new(0xE8, "INX", 1, 2, AddressingMode::NoneAddressing),
                OpCode::new(0xC8, "INY", 1, 2, AddressingMode::NoneAddressing),

                OpCode::new(0xC6, "DEC", 2, 5, AddressingMode::ZeroPage),
                OpCode::new(0xD6, "DEC", 2, 6, AddressingMode::ZeroPageX),
                OpCode::new(0xCE, "DEC", 3, 6, AddressingMode::Absolute),
                OpCode::new(0xDE, "DEC", 3, 7, AddressingMode::AbsoluteX),

                OpCode::new(0xCA, "DEX", 1, 2, AddressingMode::NoneAddressing),
                OpCode::new(0x88, "DEY", 1, 2, AddressingMode::NoneAddressing),

                OpCode::new(0xC9, "CMP", 2, 2, AddressingMode::Immediate),
                OpCode::new(0xC5, "CMP", 2, 3, AddressingMode::ZeroPage),
                OpCode::new(0xD5, "CMP", 2, 4, AddressingMode::ZeroPageX),
                OpCode::new(0xCD, "CMP", 3, 4, AddressingMode::Absolute),
                OpCode::new(0xDD, "CMP", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteX),
                OpCode::new(0xD9, "CMP", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteY),
                OpCode::new(0xC1, "CMP", 2, 6, AddressingMode::IndirectX),
                OpCode::new(0xD1, "CMP", 2, 5/*+1 if page crossed*/, AddressingMode::IndirectY),

                OpCode::new(0xC0, "CPY", 2, 2, AddressingMode::Immediate),
                OpCode::new(0xC4, "CPY", 2, 3, AddressingMode::ZeroPage),
                OpCode::new(0xCC, "CPY", 3, 4, AddressingMode::Absolute),

                OpCode::new(0xE0, "CPX", 2, 2, AddressingMode::Immediate),
                OpCode::new(0xE4, "CPX", 2, 3, AddressingMode::ZeroPage),
                OpCode::new(0xEC, "CPX", 3, 4, AddressingMode::Absolute),


                /* Branching */
                OpCode::new(0x4C, "JMP", 3, 3, AddressingMode::NoneAddressing), //AddressingMode that acts as Immidiate
                OpCode::new(0x6C, "JMP", 3, 5, AddressingMode::NoneAddressing), //AddressingMode:Indirect with 6502 bug

                OpCode::new(0x20, "JSR", 3, 6, AddressingMode::NoneAddressing),
                OpCode::new(0x60, "RTS", 1, 6, AddressingMode::NoneAddressing),

                OpCode::new(0x40, "RTI", 1, 6, AddressingMode::NoneAddressing),

                OpCode::new(0xD0, "BNE", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),
                OpCode::new(0x70, "BVS", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),
                OpCode::new(0x50, "BVC", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),
                OpCode::new(0x30, "BMI", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),
                OpCode::new(0xF0, "BEQ", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),
                OpCode::new(0xB0, "BCS", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),
                OpCode::new(0x90, "BCC", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),
                OpCode::new(0x10, "BPL", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),

                OpCode::new(0x24, "BIT", 2, 3, AddressingMode::ZeroPage),
                OpCode::new(0x2C, "BIT", 3, 4, AddressingMode::Absolute),


                /* Stores, Loads */
                OpCode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate),
                OpCode::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage),
                OpCode::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPageX),
                OpCode::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute),
                OpCode::new(0xBD, "LDA", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteX),
                OpCode::new(0xB9, "LDA", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteY),
                OpCode::new(0xA1, "LDA", 2, 6, AddressingMode::IndirectX),
                OpCode::new(0xB1, "LDA", 2, 5/*+1 if page crossed*/, AddressingMode::IndirectY),

                OpCode::new(0xA2, "LDX", 2, 2, AddressingMode::Immediate),
                OpCode::new(0xA6, "LDX", 2, 3, AddressingMode::ZeroPage),
                OpCode::new(0xB6, "LDX", 2, 4, AddressingMode::ZeroPageY),
                OpCode::new(0xAE, "LDX", 3, 4, AddressingMode::Absolute),
                OpCode::new(0xBE, "LDX", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteY),

                OpCode::new(0xA0, "LDY", 2, 2, AddressingMode::Immediate),
                OpCode::new(0xA4, "LDY", 2, 3, AddressingMode::ZeroPage),
                OpCode::new(0xB4, "LDY", 2, 4, AddressingMode::ZeroPageX),
                OpCode::new(0xAC, "LDY", 3, 4, AddressingMode::Absolute),
                OpCode::new(0xBC, "LDY", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteX),


                OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
                OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPageX),
                OpCode::new(0x8D, "STA", 3, 4, AddressingMode::Absolute),
                OpCode::new(0x9d, "STA", 3, 5, AddressingMode::AbsoluteX),
                OpCode::new(0x99, "STA", 3, 5, AddressingMode::AbsoluteY),
                OpCode::new(0x81, "STA", 2, 6, AddressingMode::IndirectX),
                OpCode::new(0x91, "STA", 2, 6, AddressingMode::IndirectY),

                OpCode::new(0x86, "STX", 2, 3, AddressingMode::ZeroPage),
                OpCode::new(0x96, "STX", 2, 4, AddressingMode::ZeroPageY),
                OpCode::new(0x8E, "STX", 3, 4, AddressingMode::Absolute),

                OpCode::new(0x84, "STY", 2, 3, AddressingMode::ZeroPage),
                OpCode::new(0x94, "STY", 2, 4, AddressingMode::ZeroPageX),
                OpCode::new(0x8C, "STY", 3, 4, AddressingMode::Absolute),


                /* Flags clear */
                OpCode::new(0xD8, "CLD", 1, 2, AddressingMode::NoneAddressing),
                OpCode::new(0x58, "CLI", 1, 2, AddressingMode::NoneAddressing),
                OpCode::new(0xb8, "CLV", 1, 2, AddressingMode::NoneAddressing),
                OpCode::new(0x18, "CLC", 1, 2, AddressingMode::NoneAddressing),
                OpCode::new(0x38, "SEC", 1, 2, AddressingMode::NoneAddressing),
                OpCode::new(0x78, "SEI", 1, 2, AddressingMode::NoneAddressing),
                OpCode::new(0xf8, "SED", 1, 2, AddressingMode::NoneAddressing),

                OpCode::new(0xAA, "TAX", 1, 2, AddressingMode::NoneAddressing),
                OpCode::new(0xA8, "TAY", 1, 2, AddressingMode::NoneAddressing),
                OpCode::new(0xBA, "TSX", 1, 2, AddressingMode::NoneAddressing),
                OpCode::new(0x8A, "TXA", 1, 2, AddressingMode::NoneAddressing),
                OpCode::new(0x9A, "TXS", 1, 2, AddressingMode::NoneAddressing),
                OpCode::new(0x98, "TYA", 1, 2, AddressingMode::NoneAddressing),

                /* Stack */
                OpCode::new(0x48, "PHA", 1, 3, AddressingMode::NoneAddressing),
                OpCode::new(0x68, "PLA", 1, 4, AddressingMode::NoneAddressing),
                OpCode::new(0x08, "PHP", 1, 3, AddressingMode::NoneAddressing),
                OpCode::new(0x28, "PLP", 1, 4, AddressingMode::NoneAddressing),
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
    pub fn handle_opcode(&mut self, opcode: &OpCode) -> Result<CpuMessage> {
        let code = opcode.code;
        match code {
            0x00 => return Ok(CpuMessage::Break),
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(&opcode.mode)?,
            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => self.sta(&opcode.mode)?,
            0xAA => self.tax(),
            0xE8 => self.inx(),
            0xD8 => self.status.remove(CpuFlags::DECIMAL_MODE),
            0x58 => self.status.remove(CpuFlags::INTERRUPT_DISABLE),
            0xB8 => self.status.remove(CpuFlags::OVERFLOW),
            0x18 => self.status.remove(CpuFlags::CARRY),
            0x38 => self.status.insert(CpuFlags::CARRY),
            0x78 => self.status.insert(CpuFlags::INTERRUPT_DISABLE),
            0xF8 => self.status.insert(CpuFlags::DECIMAL_MODE),
            0x48 => self.stack_push_byte(self.register_a.get()),
            0x68 => self.pla(),
            0x08 => self.php(),
            0x28 => self.plp(),
            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => self.adc(&opcode.mode)?,
            0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => self.sbc(&opcode.mode)?,
            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.and(&opcode.mode)?,
            0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => self.eor(&opcode.mode)?,
            0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => self.ora(&opcode.mode)?,
            0x4A => self.lsr_accumulator(),
            0x46 | 0x56 | 0x4E | 0x5E => self.lsr(&opcode.mode)?,
            0x0A => self.asl_accumulator(),
            0x06 | 0x16 | 0x0E | 0x1E => self.asl(&opcode.mode)?,
            0x2A => self.rol_accumulator(),
            0x26 | 0x36 | 0x2E | 0x3E => self.rol(&opcode.mode)?,
            0x6A => self.ror_accumulator(),
            0x66 | 0x76 | 0x6E | 0x7E => self.ror(&opcode.mode)?,
            0xE6 | 0xF6 | 0xEE | 0xFE => self.inc(&opcode.mode)?,
            0xC8 => self.iny(),
            0xC6 | 0xD6 | 0xCE | 0xDE => self.dec(&opcode.mode)?,
            0xCA => self.dex(),
            0x88 => self.dey(),
            0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                let value = self.register_a.get();
                self.compare(&opcode.mode, value)?;
            }
            0xC0 | 0xC4 | 0xCC => {
                let value = self.register_y.get();
                self.compare(&opcode.mode, value)?;
            }
            0xE0 | 0xE4 | 0xEC => {
                let value = self.register_x.get();
                self.compare(&opcode.mode, value)?;
            }
            0x4C => {
                let addr = self.mem_read_word(self.program_counter.get());
                self.program_counter.set(addr);
            }
            0x6C => {
                let addr = self.mem_read_word(self.program_counter.get());
                let indirect_ref = match addr & 0x00FF {
                    0x00FF => {
                        let lo = self.mem_read_byte(addr);
                        let hi = self.mem_read_byte(addr & 0xFF00);

                        (hi as u16) << 8 | (lo as u16)
                    }
                    _ => self.mem_read_word(addr),
                };

                self.program_counter.set(indirect_ref);
            }
            0x20 => {
                let program_counter = self.program_counter.get();
                self.stack_push_word(program_counter + 1);

                let addr = self.mem_read_word(program_counter);
                self.program_counter.set(addr);
            }
            0x60 => {
                let value = self.stack_pop_word();
                self.program_counter.set(value + 1)
            }
            0x40 => {
                self.status = CpuFlags::from_bits_truncate(self.stack_pop_byte());
                self.status.remove(CpuFlags::BREAK);
                self.status.insert(CpuFlags::BREAK2);

                let value = self.stack_pop_word();
                self.program_counter.set(value);
            }
            0xd0 => self.branch(!self.status.contains(CpuFlags::ZERO)),
            0x70 => self.branch(self.status.contains(CpuFlags::OVERFLOW)),
            0x50 => self.branch(!self.status.contains(CpuFlags::OVERFLOW)),
            0x10 => self.branch(!self.status.contains(CpuFlags::NEGATIVE)),
            0x30 => self.branch(self.status.contains(CpuFlags::NEGATIVE)),
            0xF0 => self.branch(self.status.contains(CpuFlags::ZERO)),
            0xB0 => self.branch(self.status.contains(CpuFlags::CARRY)),
            0x90 => self.branch(!self.status.contains(CpuFlags::CARRY)),
            0x24 | 0x2C => self.bit(&opcode.mode)?,
            0x86 | 0x96 | 0x8E => {
                let addr = self.get_operand_address(&opcode.mode)?;
                self.mem_write_byte(addr, self.register_x.get());
            }
            0x84 | 0x94 | 0x8C => {
                let addr = self.get_operand_address(&opcode.mode)?;
                self.mem_write_byte(addr, self.register_y.get());
            }
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(&opcode.mode)?,
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.ldy(&opcode.mode)?,
            0xEA => {}
            0xA8 => {
                self.register_y.set(self.register_a.get());

                let register_y = self.register_y.get();
                self.update_zero_flag(register_y);
                self.update_negative_flag(register_y);
            }
            0xBA => {
                self.register_x.set(self.stack_pointer.get());

                let register_x = self.register_x.get();
                self.update_zero_flag(register_x);
                self.update_negative_flag(register_x);
            }
            0x8A => {
                self.register_a.set(self.register_x.get());

                let register_a = self.register_a.get();
                self.update_zero_flag(register_a);
                self.update_negative_flag(register_a);
            }
            0x9A => self.stack_pointer.set(self.register_x.get()),
            0x98 => {
                self.register_a.set(self.register_y.get());

                let register_a = self.register_a.get();
                self.update_zero_flag(register_a);
                self.update_negative_flag(register_a);
            }

            _ => {
                return Err(Error::Unsupported(format!(
                    r#"opcode "{code:#x}" is not supported"#
                )))
            }
        };

        Ok(CpuMessage::Continue)
    }

    fn lda(&mut self, mode: &AddressingMode) -> Result<()> {
        let addr = self.get_operand_address(mode)?;
        let value = self.mem_read_byte(addr);

        self.register_a.set(value);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        Ok(())
    }

    fn ldx(&mut self, mode: &AddressingMode) -> Result<()> {
        let addr = self.get_operand_address(mode)?;
        let value = self.mem_read_byte(addr);

        self.register_x.set(value);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        Ok(())
    }

    fn ldy(&mut self, mode: &AddressingMode) -> Result<()> {
        let addr = self.get_operand_address(mode)?;
        let value = self.mem_read_byte(addr);

        self.register_y.set(value);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        Ok(())
    }

    fn sta(&mut self, mode: &AddressingMode) -> Result<()> {
        let addr = self.get_operand_address(mode)?;
        self.mem_write_byte(addr, self.register_a.get());

        Ok(())
    }

    fn and(&mut self, mode: &AddressingMode) -> Result<()> {
        let addr = self.get_operand_address(mode)?;
        let data = self.mem_read_byte(addr);

        let result = data & self.register_a.get();
        self.register_a.set(result);

        Ok(())
    }

    fn eor(&mut self, mode: &AddressingMode) -> Result<()> {
        let addr = self.get_operand_address(mode)?;
        let data = self.mem_read_byte(addr);

        let result = data ^ self.register_a.get();
        self.register_a.set(result);

        Ok(())
    }

    fn ora(&mut self, mode: &AddressingMode) -> Result<()> {
        let addr = self.get_operand_address(mode)?;
        let data = self.mem_read_byte(addr);

        let result = data | self.register_a.get();
        self.register_a.set(result);

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

    fn iny(&mut self) {
        self.register_y.increment();

        let register_y = self.register_y.get();
        self.update_zero_flag(register_y);
        self.update_negative_flag(register_y);
    }

    fn sbc(&mut self, mode: &AddressingMode) -> Result<()> {
        let addr = self.get_operand_address(mode)?;
        let byte = self.mem_read_byte(addr);

        self.add_to_register_a((byte as i8).wrapping_neg().wrapping_sub(1) as u8);

        Ok(())
    }

    fn adc(&mut self, mode: &AddressingMode) -> Result<()> {
        let addr = self.get_operand_address(mode)?;
        let byte = self.mem_read_byte(addr);

        self.add_to_register_a(byte);

        Ok(())
    }

    fn asl(&mut self, mode: &AddressingMode) -> Result<()> {
        let addr = self.get_operand_address(mode)?;
        let value = self.mem_read_byte(addr);

        match value >> 7 {
            1 => self.status.insert(CpuFlags::CARRY),
            _ => self.status.remove(CpuFlags::CARRY),
        };

        let value = value << 1;
        self.mem_write_byte(addr, value);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        Ok(())
    }

    fn asl_accumulator(&mut self) {
        let value = self.register_a.get();
        match value >> 7 {
            1 => self.status.insert(CpuFlags::CARRY),
            _ => self.status.remove(CpuFlags::CARRY),
        };

        self.register_a.set(value << 1);
        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn lsr(&mut self, mode: &AddressingMode) -> Result<()> {
        let addr = self.get_operand_address(mode)?;
        let value = self.mem_read_byte(addr);

        match value & 1 {
            1 => self.status.insert(CpuFlags::CARRY),
            _ => self.status.remove(CpuFlags::CARRY),
        };

        let value = value >> 1;
        self.mem_write_byte(addr, value);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        Ok(())
    }

    fn lsr_accumulator(&mut self) {
        let value = self.register_a.get();
        match value & 1 {
            1 => self.status.insert(CpuFlags::CARRY),
            _ => self.status.remove(CpuFlags::CARRY),
        };

        self.register_a.set(value >> 1);
        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn rol(&mut self, mode: &AddressingMode) -> Result<()> {
        let addr = self.get_operand_address(mode)?;
        let value = self.mem_read_byte(addr);
        let old_carry = self.status.contains(CpuFlags::CARRY);

        match value >> 7 {
            1 => self.status.insert(CpuFlags::CARRY),
            _ => self.status.remove(CpuFlags::CARRY),
        };

        let value = value << 1;
        let value = match old_carry {
            true => value | 1,
            false => value,
        };

        self.mem_write_byte(addr, value);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        Ok(())
    }

    fn rol_accumulator(&mut self) {
        let value = self.register_a.get();
        let old_carry = self.status.contains(CpuFlags::CARRY);

        match value >> 7 {
            1 => self.status.insert(CpuFlags::CARRY),
            _ => self.status.remove(CpuFlags::CARRY),
        };

        let value = value << 1;
        let value = match old_carry {
            true => value | 1,
            false => value,
        };

        self.register_a.set(value);
        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn ror(&mut self, mode: &AddressingMode) -> Result<()> {
        let addr = self.get_operand_address(mode)?;
        let value = self.mem_read_byte(addr);
        let old_carry = self.status.contains(CpuFlags::CARRY);

        match value & 1 {
            1 => self.status.insert(CpuFlags::CARRY),
            _ => self.status.remove(CpuFlags::CARRY),
        };

        let value = value >> 1;
        let value = match old_carry {
            true => value | 0b10000000,
            false => value,
        };

        self.mem_write_byte(addr, value);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        Ok(())
    }

    fn ror_accumulator(&mut self) {
        let value = self.register_a.get();
        let old_carry = self.status.contains(CpuFlags::CARRY);

        match value & 1 {
            1 => self.status.insert(CpuFlags::CARRY),
            _ => self.status.remove(CpuFlags::CARRY),
        };

        let value = value >> 1;
        let value = match old_carry {
            true => value | 0b10000000,
            false => value,
        };

        self.register_a.set(value);
        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn inc(&mut self, mode: &AddressingMode) -> Result<()> {
        let addr = self.get_operand_address(mode)?;
        let value = self.mem_read_byte(addr).wrapping_add(1);

        self.mem_write_byte(addr, value);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        Ok(())
    }

    fn dex(&mut self) {
        self.register_x.decrement();

        let value = self.register_x.get();
        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn dey(&mut self) {
        self.register_y.decrement();

        let value = self.register_y.get();
        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn dec(&mut self, mode: &AddressingMode) -> Result<()> {
        let addr = self.get_operand_address(mode)?;
        let value = self.mem_read_byte(addr).wrapping_sub(1);

        self.mem_write_byte(addr, value);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        Ok(())
    }

    fn pla(&mut self) {
        let value = self.stack_pop_byte();

        self.register_a.set(value);
        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn plp(&mut self) {
        self.status = CpuFlags::from_bits_truncate(self.stack_pop_byte());

        self.status.remove(CpuFlags::BREAK);
        self.status.insert(CpuFlags::BREAK2);
    }

    fn php(&mut self) {
        let mut flags = self.status;

        flags.insert(CpuFlags::BREAK);
        flags.insert(CpuFlags::BREAK2);

        self.stack_push_byte(flags.bits());
    }

    fn bit(&mut self, mode: &AddressingMode) -> Result<()> {
        let addr = self.get_operand_address(mode)?;
        let value = self.mem_read_byte(addr);

        match self.register_a.get() & value {
            0 => self.status.insert(CpuFlags::ZERO),
            _ => self.status.remove(CpuFlags::ZERO),
        };

        self.status
            .set(CpuFlags::NEGATIVE, (value & 0b10000000) > 0);
        self.status
            .set(CpuFlags::OVERFLOW, (value & 0b01000000) > 0);

        Ok(())
    }

    fn compare(&mut self, mode: &AddressingMode, compare_value: u8) -> Result<()> {
        let addr = self.get_operand_address(mode)?;
        let value = self.mem_read_byte(addr);

        match value <= compare_value {
            true => self.status.insert(CpuFlags::CARRY),
            false => self.status.remove(CpuFlags::CARRY),
        };

        let compare_value = compare_value.wrapping_sub(value);

        self.update_zero_flag(compare_value);
        self.update_negative_flag(compare_value);

        Ok(())
    }

    fn branch(&mut self, condition: bool) {
        if condition {
            let jump = self.mem_read_byte(self.program_counter.get()) as i8;
            self.program_counter.wrapping_add(jump as u16 + 1);
        }
    }

    fn stack_push_byte(&mut self, value: u8) {
        self.mem_write_byte(
            (STACK_START_ADDR as u16) + self.stack_pointer.get() as u16,
            value,
        );
        self.stack_pointer.decrement();
    }

    fn stack_pop_byte(&mut self) -> u8 {
        self.stack_pointer.increment();

        self.mem_read_byte(STACK_START_ADDR as u16 + self.stack_pointer.get() as u16)
    }

    fn stack_push_word(&mut self, value: u16) {
        let hi = value >> 8;
        let lo = value & 0xFF;

        self.stack_push_byte(hi as u8);
        self.stack_push_byte(lo as u8);
    }

    fn stack_pop_word(&mut self) -> u16 {
        let lo = self.stack_pop_byte() as u16;
        let hi = self.stack_pop_byte() as u16;

        hi << 8 | lo
    }

    fn add_to_register_a(&mut self, value: u8) {
        let register_a = self.register_a.get();
        let sum = register_a as u16
            + value as u16
            + (if self.status.contains(CpuFlags::CARRY) {
                1
            } else {
                0
            }) as u16;

        match sum > 0xff {
            true => self.status.insert(CpuFlags::CARRY),
            false => self.status.remove(CpuFlags::CARRY),
        }

        let result = sum as u8;

        match (value ^ result) & (result ^ register_a) & 0x80 {
            0 => self.status.remove(CpuFlags::OVERFLOW),
            _ => self.status.insert(CpuFlags::OVERFLOW),
        }

        self.register_a.set(result);
        self.update_negative_flag(result);
        self.update_negative_flag(result);
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
}
