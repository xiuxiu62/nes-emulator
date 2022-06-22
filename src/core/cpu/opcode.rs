use super::{Cpu, CpuFlags, CpuMessage, STACK_START_ADDR};
use crate::{
    core::{AddressingMode, OpCode},
    error::{Error, Result},
    io::{Read, Write},
};

impl Cpu {
    pub fn handle_opcode(&mut self, opcode: &OpCode) -> Result<CpuMessage> {
        let code = opcode.code();
        match code {
            // OFFICIAL OPCODES
            0x00 => return Ok(CpuMessage::Break),
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(&opcode.mode())?,
            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => self.sta(&opcode.mode())?,
            0xAA => self.tax(),
            0xE8 => self.inx(),
            0xD8 => self.status.remove(CpuFlags::DECIMAL_MODE),
            0x58 => self.status.remove(CpuFlags::INTERRUPT_DISABLE),
            0xB8 => self.status.remove(CpuFlags::OVERFLOW),
            0x18 => self.clear_carry_flag(),
            0x38 => self.set_carry_flag(),
            0x78 => self.status.insert(CpuFlags::INTERRUPT_DISABLE),
            0xF8 => self.status.insert(CpuFlags::DECIMAL_MODE),
            0x48 => self.stack_push_byte(self.register_a.get())?,
            0x68 => self.pla()?,
            0x08 => self.php()?,
            0x28 => self.plp()?,
            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => self.adc(&opcode.mode())?,
            0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => self.sbc(&opcode.mode())?,
            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.and(&opcode.mode())?,
            0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => self.eor(&opcode.mode())?,
            0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
                self.ora(&opcode.mode())?;
            }
            0x46 | 0x56 | 0x4E | 0x5E => {
                self.lsr(&opcode.mode())?;
            }
            0x06 | 0x16 | 0x0E | 0x1E => {
                self.asl(&opcode.mode())?;
            }
            0x26 | 0x36 | 0x2E | 0x3E => {
                self.rol(&opcode.mode())?;
            }
            0x66 | 0x76 | 0x6E | 0x7E => {
                self.ror(&opcode.mode())?;
            }
            0xE6 | 0xF6 | 0xEE | 0xFE => {
                self.inc(&opcode.mode())?;
            }
            0xC6 | 0xD6 | 0xCE | 0xDE => self.dec(&opcode.mode())?,
            0x4A => self.lsr_accumulator(),
            0x0A => self.asl_accumulator(),
            0x2A => self.rol_accumulator(),
            0x6A => self.ror_accumulator(),
            0xC8 => self.iny(),
            0xCA => self.dex(),
            0x88 => self.dey(),
            0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                self.compare(&opcode.mode(), self.register_a.get())?
            }
            0xC0 | 0xC4 | 0xCC => {
                self.compare(&opcode.mode(), self.register_y.get())?;
            }
            0xE0 | 0xE4 | 0xEC => {
                self.compare(&opcode.mode(), self.register_x.get())?;
            }
            0x4C => {
                let addr = self.read_word(self.program_counter.get())?;
                self.program_counter.set(addr);
            }
            0x6C => {
                let addr = self.read_word(self.program_counter.get())?;
                let indirect_ref = match addr & 0x00FF {
                    0x00FF => {
                        let lo = self.read_byte(addr)?;
                        let hi = self.read_byte(addr & 0xFF00)?;

                        (hi as u16) << 8 | (lo as u16)
                    }
                    _ => self.read_word(addr)?,
                };

                self.program_counter.set(indirect_ref);
            }
            0x20 => {
                let program_counter = self.program_counter.get();
                self.stack_push_word(program_counter + 1)?;

                let addr = self.read_word(program_counter)?;
                self.program_counter.set(addr);
            }
            0x60 => {
                let value = self.stack_pop_word()?;
                self.program_counter.set(value + 1)
            }
            0x40 => {
                self.status = CpuFlags::from_bits_truncate(self.stack_pop_byte()?);
                self.status.remove(CpuFlags::BREAK);
                self.status.insert(CpuFlags::BREAK2);

                let value = self.stack_pop_word()?;
                self.program_counter.set(value);
            }
            0xd0 => self.branch(!self.status.contains(CpuFlags::ZERO))?,
            0x70 => self.branch(self.status.contains(CpuFlags::OVERFLOW))?,
            0x50 => self.branch(!self.status.contains(CpuFlags::OVERFLOW))?,
            0x10 => self.branch(!self.status.contains(CpuFlags::NEGATIVE))?,
            0x30 => self.branch(self.status.contains(CpuFlags::NEGATIVE))?,
            0xF0 => self.branch(self.status.contains(CpuFlags::ZERO))?,
            0xB0 => self.branch(self.status.contains(CpuFlags::CARRY))?,
            0x90 => self.branch(!self.status.contains(CpuFlags::CARRY))?,
            0x24 | 0x2C => self.bit(&opcode.mode())?,
            0x86 | 0x96 | 0x8E => {
                let (addr, _) = self.get_operand_address(&opcode.mode())?;
                self.write_byte(addr, self.register_x.get())?;
            }
            0x84 | 0x94 | 0x8C => {
                let (addr, _) = self.get_operand_address(&opcode.mode())?;
                self.write_byte(addr, self.register_y.get())?;
            }
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(&opcode.mode())?,
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.ldy(&opcode.mode())?,
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

            // UNOFFICIAL OPCODES
            // NOP read
            0xc7 | 0xd7 | 0xCF | 0xdF | 0xdb | 0xd3 | 0xc3 => {
                let (addr, _) = self.get_operand_address(&opcode.mode)?;
                let byte = self.read_byte(addr)?.wrapping_add(1);
                self.write_byte(addr, byte)?;
                // self._update_zero_and_negative_flags(data);
                if byte <= self.register_a.get() {
                    self.status.insert(CpuFlags::CARRY);
                }

                self.update_zero_flag(self.register_a.get().wrapping_sub(byte));
                self.update_negative_flag(self.register_a.get().wrapping_sub(byte));
            }
            // RLA
            0x27 | 0x37 | 0x2F | 0x3F | 0x3b | 0x33 | 0x23 => {
                let data = self.rol(&opcode.mode)?;
                self.register_a.set(data & self.register_a.get());
            }
            // SLO
            0x07 | 0x17 | 0x0F | 0x1F | 0x1B | 0x03 | 0x13 => {
                let data = self.asl(&opcode.mode)?;
                self.register_a.set(data | self.register_a.get());
            }
            // SRE
            0x47 | 0x57 | 0x4F | 0x5F | 0x5B | 0x43 | 0x53 => {
                let data = self.lsr(&opcode.mode)?;
                self.register_a.set(data ^ self.register_a.get());
            }
            // SKB
            0x80 | 0x82 | 0x89 | 0xC2 | 0xE2 => {
                /* 2 byte NOP (immidiate ) */
                // todo: might be worth doing the read
            }
            // AXS
            0xCB => {
                let (addr, _) = self.get_operand_address(&opcode.mode)?;
                let data = self.read_byte(addr)?;
                let x_and_a = self.register_x.get() & self.register_a.get();
                let result = x_and_a.wrapping_sub(data);

                if data <= x_and_a {
                    self.status.insert(CpuFlags::CARRY);
                }

                self.update_zero_flag(result);
                self.update_negative_flag(result);

                self.register_x.set(result);
            }
            // ARR
            0x6B => {
                let (addr, _) = self.get_operand_address(&opcode.mode)?;
                let data = self.read_byte(addr)?;
                self.register_a.set(data & self.register_a.get());
                self.ror_accumulator();

                let result = self.register_a.get();
                let bit_5 = (result >> 5) & 1;
                let bit_6 = (result >> 6) & 1;

                if bit_6 == 1 {
                    self.status.insert(CpuFlags::CARRY)
                } else {
                    self.status.remove(CpuFlags::CARRY)
                }

                if bit_5 ^ bit_6 == 1 {
                    self.status.insert(CpuFlags::OVERFLOW);
                } else {
                    self.status.remove(CpuFlags::OVERFLOW);
                }

                self.update_zero_flag(result);
                self.update_negative_flag(result);
            }
            // SBC
            0xEB => {
                let (addr, _) = self.get_operand_address(&opcode.mode)?;
                let data = self.read_byte(addr)?;
                self.register_a.wrapping_sub(data + 1);
            }
            // ANC
            0x0B | 0x2b => {
                let (addr, _) = self.get_operand_address(&opcode.mode)?;
                let data = self.read_byte(addr)?;
                self.register_a.set(data & self.register_a.get());

                match self.status.contains(CpuFlags::NEGATIVE) {
                    true => self.status.insert(CpuFlags::CARRY),
                    false => self.status.remove(CpuFlags::CARRY),
                }
            }
            // ALR
            0x4B => {
                let (addr, _) = self.get_operand_address(&opcode.mode)?;
                let data = self.read_byte(addr)?;
                self.register_a.set(data & self.register_a.get());

                self.lsr_accumulator();
            }
            // NOP read
            0x04 | 0x44 | 0x64 | 0x14 | 0x34 | 0x54 | 0x74 | 0xD4 | 0xF4 | 0x0C | 0x1C | 0x3C
            | 0x5C | 0x7C | 0xDC | 0xFC => {
                let (addr, page_cross) = self.get_operand_address(&opcode.mode)?;
                let _data = self.read_byte(addr)?;
                if page_cross {
                    self.bus.tick(1);
                }
            }
            // RRA
            0x67 | 0x77 | 0x6F | 0x7F | 0x7B | 0x63 | 0x73 => {
                let data = self.ror(&opcode.mode)?;
                self.add_to_register_a(data);
            }
            // ISB
            0xE7 | 0xF7 | 0xEF | 0xFF | 0xFB | 0xE3 | 0xF3 => {
                let data = self.inc(&opcode.mode)?;
                self.register_a.wrapping_sub(data + 1);
            }
            // NOPs
            0x02 | 0x12 | 0x22 | 0x32 | 0x42 | 0x52 | 0x62 | 0x72 | 0x92 | 0xB2 | 0xD2 | 0xF2
            | 0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA => {}
            // LAX
            0xA7 | 0xB7 | 0xAF | 0xBF | 0xA3 | 0xB3 => {
                let (addr, _) = self.get_operand_address(&opcode.mode)?;
                let data = self.read_byte(addr)?;

                self.register_a.set(data);
                self.register_x.set(self.register_a.get());
            }
            // SAX
            0x87 | 0x97 | 0x8F | 0x83 => {
                let data = self.register_a.get() & self.register_x.get();
                let (addr, _) = self.get_operand_address(&opcode.mode)?;

                self.write_byte(addr, data)?;
            }
            // LXA
            0xAB => {
                self.lda(&opcode.mode)?;
                self.tax();
            }
            // XAA
            0x8B => {
                self.register_a.set(self.register_x.get());

                let register_a = self.register_a.get();
                self.update_zero_flag(register_a);
                self.update_negative_flag(register_a);

                let (addr, _) = self.get_operand_address(&opcode.mode)?;
                let data = self.read_byte(addr)?;
                self.register_a.set(data & self.register_a.get());
            }
            // LAS
            0xBB => {
                let (addr, _) = self.get_operand_address(&opcode.mode)?;
                let data = self.read_byte(addr)? & self.stack_pointer.get();

                self.register_a.set(data);
                self.register_x.set(data);
                self.stack_pointer.set(data);

                self.update_zero_flag(data);
                self.update_negative_flag(data);
            }
            // TAS
            0x9B => {
                let data = self.register_a.get() & self.register_x.get();
                self.stack_pointer.set(data);
                let mem_address =
                    self.read_word(self.program_counter.get())? + self.register_y.get() as u16;

                let data = ((mem_address >> 8) as u8 + 1) & self.stack_pointer.get();
                self.write_byte(mem_address, data)?;
            }
            // AHX  Indirect Y
            0x93 => {
                let pos = self.read_byte(self.program_counter.get())?;
                let mem_address = self.read_word(pos as u16)? + self.register_y.get() as u16;
                let data = self.register_a.get() & self.register_x.get() & (mem_address >> 8) as u8;
                self.write_byte(mem_address, data)?;
            }
            // AHX Absolute Y
            0x9F => {
                let mem_address =
                    self.read_word(self.program_counter.get())? + self.register_y.get() as u16;

                let data = self.register_a.get() & self.register_x.get() & (mem_address >> 8) as u8;
                self.write_byte(mem_address, data)?;
            }
            // SHX
            0x9E => {
                let mem_address =
                    self.read_word(self.program_counter.get())? + self.register_y.get() as u16;
                let data = self.register_x.get() & ((mem_address >> 8) as u8 + 1);
                self.write_byte(mem_address, data)?;
            }
            // SHY
            0x9C => {
                let mem_address =
                    self.read_word(self.program_counter.get())? + self.register_x.get() as u16;
                let data = self.register_y.get() & ((mem_address >> 8) as u8 + 1);
                self.write_byte(mem_address, data)?;
            }
        };

        Ok(CpuMessage::Continue)
    }

    fn lda(&mut self, mode: &AddressingMode) -> Result<()> {
        let (addr, page_cross) = self.get_operand_address(mode)?;
        let value = self.read_byte(addr)?;

        self.register_a.set(value);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        if page_cross {
            self.bus.tick(1);
        }

        Ok(())
    }

    fn ldx(&mut self, mode: &AddressingMode) -> Result<()> {
        let (addr, page_cross) = self.get_operand_address(mode)?;
        let value = self.read_byte(addr)?;

        self.register_x.set(value);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        if page_cross {
            self.bus.tick(1);
        }

        Ok(())
    }

    fn ldy(&mut self, mode: &AddressingMode) -> Result<()> {
        let (addr, page_cross) = self.get_operand_address(mode)?;
        let value = self.read_byte(addr)?;

        self.register_y.set(value);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        if page_cross {
            self.bus.tick(1);
        }

        Ok(())
    }

    fn sta(&mut self, mode: &AddressingMode) -> Result<()> {
        let (addr, _) = self.get_operand_address(mode)?;
        self.write_byte(addr, self.register_a.get())?;

        Ok(())
    }

    fn and(&mut self, mode: &AddressingMode) -> Result<()> {
        let (addr, page_cross) = self.get_operand_address(mode)?;
        let data = self.read_byte(addr)?;

        let result = data & self.register_a.get();
        self.register_a.set(result);

        if page_cross {
            self.bus.tick(1);
        }

        Ok(())
    }

    fn eor(&mut self, mode: &AddressingMode) -> Result<()> {
        let (addr, page_cross) = self.get_operand_address(mode)?;
        let data = self.read_byte(addr)?;

        let result = data ^ self.register_a.get();
        self.register_a.set(result);

        if page_cross {
            self.bus.tick(1);
        }

        Ok(())
    }

    fn ora(&mut self, mode: &AddressingMode) -> Result<()> {
        let (addr, page_cross) = self.get_operand_address(mode)?;
        let data = self.read_byte(addr)?;

        let result = data | self.register_a.get();
        self.register_a.set(result);

        if page_cross {
            self.bus.tick(1);
        }

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
        let (addr, page_cross) = self.get_operand_address(mode)?;
        let byte = self.read_byte(addr)?;

        self.add_to_register_a((byte as i8).wrapping_neg().wrapping_sub(1) as u8);

        if page_cross {
            self.bus.tick(1);
        }

        Ok(())
    }

    fn adc(&mut self, mode: &AddressingMode) -> Result<()> {
        let (addr, page_cross) = self.get_operand_address(mode)?;
        let byte = self.read_byte(addr)?;

        self.add_to_register_a(byte);

        if page_cross {
            self.bus.tick(1);
        }

        Ok(())
    }

    fn asl(&mut self, mode: &AddressingMode) -> Result<u8> {
        let (addr, _) = self.get_operand_address(mode)?;
        let value = self.read_byte(addr)?;

        match value >> 7 {
            1 => self.set_carry_flag(),
            _ => self.clear_carry_flag(),
        };

        let value = value << 1;
        self.write_byte(addr, value)?;
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        Ok(value)
    }

    fn asl_accumulator(&mut self) {
        let value = self.register_a.get();
        match value >> 7 {
            1 => self.set_carry_flag(),
            _ => self.clear_carry_flag(),
        };

        self.register_a.set(value << 1);
        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn lsr(&mut self, mode: &AddressingMode) -> Result<u8> {
        let (addr, _) = self.get_operand_address(mode)?;
        let value = self.read_byte(addr)?;

        match value & 1 {
            1 => self.set_carry_flag(),
            _ => self.clear_carry_flag(),
        };

        let value = value >> 1;
        self.write_byte(addr, value)?;
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        Ok(value)
    }

    fn lsr_accumulator(&mut self) {
        let value = self.register_a.get();
        match value & 1 {
            1 => self.set_carry_flag(),
            _ => self.clear_carry_flag(),
        };

        self.register_a.set(value >> 1);
        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn rol(&mut self, mode: &AddressingMode) -> Result<u8> {
        let (addr, _) = self.get_operand_address(mode)?;
        let value = self.read_byte(addr)?;
        let old_carry = self.status.contains(CpuFlags::CARRY);

        match value >> 7 {
            1 => self.set_carry_flag(),
            _ => self.clear_carry_flag(),
        };

        let value = value << 1;
        let value = match old_carry {
            true => value | 1,
            false => value,
        };

        self.write_byte(addr, value)?;
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        Ok(value)
    }

    fn rol_accumulator(&mut self) {
        let value = self.register_a.get();
        let old_carry = self.status.contains(CpuFlags::CARRY);

        match value >> 7 {
            1 => self.set_carry_flag(),
            _ => self.clear_carry_flag(),
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

    fn ror(&mut self, mode: &AddressingMode) -> Result<u8> {
        let (addr, _) = self.get_operand_address(mode)?;
        let value = self.read_byte(addr)?;
        let old_carry = self.status.contains(CpuFlags::CARRY);

        match value & 1 {
            1 => self.set_carry_flag(),
            _ => self.clear_carry_flag(),
        };

        let value = value >> 1;
        let value = match old_carry {
            true => value | 0b10000000,
            false => value,
        };

        self.write_byte(addr, value)?;
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        Ok(value)
    }

    fn ror_accumulator(&mut self) {
        let value = self.register_a.get();
        let old_carry = self.status.contains(CpuFlags::CARRY);

        match value & 1 {
            1 => self.set_carry_flag(),
            _ => self.clear_carry_flag(),
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

    fn inc(&mut self, mode: &AddressingMode) -> Result<u8> {
        let (addr, _) = self.get_operand_address(mode)?;
        let value = self.read_byte(addr)?.wrapping_add(1);

        self.write_byte(addr, value)?;
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        Ok(value)
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
        let (addr, _) = self.get_operand_address(mode)?;
        let value = self.read_byte(addr)?.wrapping_sub(1);

        self.write_byte(addr, value)?;
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        Ok(())
    }

    fn pla(&mut self) -> Result<()> {
        let value = self.stack_pop_byte()?;

        self.register_a.set(value);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        Ok(())
    }

    fn plp(&mut self) -> Result<()> {
        self.status = CpuFlags::from_bits_truncate(self.stack_pop_byte()?);

        self.status.remove(CpuFlags::BREAK);
        self.status.insert(CpuFlags::BREAK2);

        Ok(())
    }

    fn php(&mut self) -> Result<()> {
        let mut flags = self.status;

        flags.insert(CpuFlags::BREAK);
        flags.insert(CpuFlags::BREAK2);

        self.stack_push_byte(flags.bits())
    }

    fn bit(&mut self, mode: &AddressingMode) -> Result<()> {
        let (addr, _) = self.get_operand_address(mode)?;
        let value = self.read_byte(addr)?;

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
        let (addr, page_cross) = self.get_operand_address(mode)?;
        let value = self.read_byte(addr)?;

        match value <= compare_value {
            true => self.set_carry_flag(),
            false => self.clear_carry_flag(),
        };

        let compare_value = compare_value.wrapping_sub(value);

        self.update_zero_flag(compare_value);
        self.update_negative_flag(compare_value);

        if page_cross {
            self.bus.tick(1);
        }

        Ok(())
    }

    fn branch(&mut self, condition: bool) -> Result<()> {
        if condition {
            let program_counter = self.program_counter.get();
            let jump = self.read_byte(program_counter)? as i8;
            let jump_addr = program_counter.wrapping_add(jump as u16 + 1);

            if self.program_counter.get().wrapping_add(1) & 0xFF00 != jump_addr & 0xFF00 {
                self.bus.tick(1);
            }

            self.program_counter.set(jump_addr);
        }

        Ok(())
    }

    fn stack_push_byte(&mut self, value: u8) -> Result<()> {
        self.write_byte(
            (STACK_START_ADDR as u16) + self.stack_pointer.get() as u16,
            value,
        )?;
        self.stack_pointer.decrement();

        Ok(())
    }

    fn stack_push_word(&mut self, value: u16) -> Result<()> {
        let hi = value >> 8;
        let lo = value & 0xFF;

        self.stack_push_byte(hi as u8)?;
        self.stack_push_byte(lo as u8)
    }

    fn stack_pop_byte(&mut self) -> Result<u8> {
        self.stack_pointer.increment();

        self.read_byte(STACK_START_ADDR as u16 + self.stack_pointer.get() as u16)
    }

    fn stack_pop_word(&mut self) -> Result<u16> {
        let lo = self.stack_pop_byte()? as u16;
        let hi = self.stack_pop_byte()? as u16;

        Ok(hi << 8 | lo)
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
            true => self.set_carry_flag(),
            false => self.clear_carry_flag(),
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

    fn get_operand_address(&mut self, mode: &AddressingMode) -> Result<(u16, bool)> {
        let program_counter = self.program_counter.get();
        match mode {
            AddressingMode::Immediate => Ok((program_counter, false)),
            _ => self.get_absolute_address(mode, program_counter),
        }
    }

    // returns Result<(address, page_cross flag)>
    pub fn get_absolute_address(
        &mut self,
        mode: &AddressingMode,
        addr: u16,
    ) -> Result<(u16, bool)> {
        match mode {
            AddressingMode::ZeroPage => Ok((self.read_byte(addr)? as u16, false)),
            AddressingMode::Absolute => Ok((self.read_word(addr)?, false)),
            AddressingMode::ZeroPageX => {
                let pos = self.read_byte(addr)?;
                let addr = pos.wrapping_add(self.register_x.get()) as u16;

                Ok((addr, false))
            }
            AddressingMode::ZeroPageY => {
                let pos = self.read_byte(addr)?;
                let addr = pos.wrapping_add(self.register_y.get()) as u16;

                Ok((addr, false))
            }
            AddressingMode::AbsoluteX => {
                let base = self.read_word(addr)?;
                let addr = base.wrapping_add(self.register_x.get() as u16);

                Ok((addr, self.page_cross(base, addr)))
            }
            AddressingMode::AbsoluteY => {
                let base = self.read_word(addr)?;
                let addr = base.wrapping_add(self.register_y.get() as u16);

                Ok((addr, self.page_cross(base, addr)))
            }
            AddressingMode::IndirectX => {
                let base = self.read_byte(addr)?;

                let ptr: u8 = base.wrapping_add(self.register_x.get());
                let lo = self.read_byte(ptr as u16)?;
                let hi = self.read_byte(ptr.wrapping_add(1) as u16)?;

                Ok(((hi as u16) << 8 | (lo as u16), false))
            }
            AddressingMode::IndirectY => {
                let base = self.read_byte(addr)?;

                let lo = self.read_byte(base as u16)?;
                let hi = self.read_byte((base as u8).wrapping_add(1) as u16)?;
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y.get() as u16);

                Ok((deref, self.page_cross(deref, deref_base)))
            }
            _ => Err(Error::Unsupported(format!(
                "mode {:?} is not supported",
                mode
            ))),
        }
    }

    fn page_cross(&self, address_1: u16, address_2: u16) -> bool {
        address_1 & 0xFF00 != address_2 & 0xFF00
    }
}
