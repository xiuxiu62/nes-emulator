use crate::{
    core::{
        opcode::{OpCodeMap, OPCODE_MAP},
        AddressingMode, Cpu,
    },
    error::{Error, Result},
    io::Read,
};

pub fn trace(cpu: &mut Cpu) -> Result<String> {
    let ref opcode_map: OpCodeMap = *OPCODE_MAP;

    let program_counter = cpu.program_counter.get();
    let code = cpu.read_byte(program_counter)?;
    let opcode = opcode_map
        .get(&code)
        .ok_or_else(|| Error::Unsupported(format!(r#"opcode "{code:#x}" is not supported"#)))?;

    let begin = cpu.program_counter.get();
    let mut hex_dump = vec![];
    hex_dump.push(code);

    let (mem_addr, stored_value) = match opcode.mode() {
        AddressingMode::Immediate | AddressingMode::NoneAddressing => (0, 0),
        _ => {
            let addr = cpu.get_absolute_address(&opcode.mode(), begin + 1)?;
            (addr, cpu.read_byte(addr)?)
        }
    };

    let tmp = match opcode.len() {
        1 => match opcode.code() {
            0x0a | 0x4a | 0x2a | 0x6a => format!("A "),
            _ => String::from(""),
        },
        2 => {
            let address = cpu.read_byte(begin + 1)?;
            hex_dump.push(address);

            match opcode.mode() {
                AddressingMode::Immediate => format!("#${:02x}", address),
                AddressingMode::ZeroPage => format!("${:02x} = {:02x}", mem_addr, stored_value),
                AddressingMode::ZeroPageX => format!(
                    "${:02x},X @ {:02x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                AddressingMode::ZeroPageY => format!(
                    "${:02x},Y @ {:02x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                AddressingMode::IndirectX => format!(
                    "(${:02x},X) @ {:02x} = {:04x} = {:02x}",
                    address,
                    (address.wrapping_add(cpu.register_x.get())),
                    mem_addr,
                    stored_value
                ),
                AddressingMode::IndirectY => format!(
                    "(${:02x}),Y = {:04x} @ {:04x} = {:02x}",
                    address,
                    (mem_addr.wrapping_sub(cpu.register_y.get() as u16)),
                    mem_addr,
                    stored_value
                ),
                AddressingMode::NoneAddressing => {
                    let address: usize =
                        (begin as usize + 2).wrapping_add((address as i8) as usize);
                    format!("${:04x}", address)
                }

                _ => panic!(
                    "unexpected addressing mode {:?} has ops-len 2. code {:02x}",
                    opcode.mode(),
                    opcode.code()
                ),
            }
        }
        3 => {
            let address_lo = cpu.read_byte(begin + 1)?;
            let address_hi = cpu.read_byte(begin + 2)?;
            hex_dump.push(address_lo);
            hex_dump.push(address_hi);

            let address = cpu.read_word(begin + 1)?;

            match opcode.mode() {
                AddressingMode::NoneAddressing => {
                    if opcode.code() == 0x6c {
                        //jmp indirect
                        let jmp_addr = match address & 0x00FF {
                            0x00FF => {
                                let lo = cpu.read_byte(address)?;
                                let hi = cpu.read_byte(address & 0xFF00)?;

                                (hi as u16) << 8 | (lo as u16)
                            }
                            _ => cpu.read_word(address)?,
                        };

                        // let jmp_addr = cpu.mem_read_u16(address);
                        format!("(${:04x}) = {:04x}", address, jmp_addr)
                    } else {
                        format!("${:04x}", address)
                    }
                }
                AddressingMode::Absolute => format!("${:04x} = {:02x}", mem_addr, stored_value),
                AddressingMode::AbsoluteX => format!(
                    "${:04x},X @ {:04x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                AddressingMode::AbsoluteY => format!(
                    "${:04x},Y @ {:04x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                _ => panic!(
                    "unexpected addressing mode {:?} has ops-len 3. code {:02x}",
                    opcode.mode(),
                    opcode.code()
                ),
            }
        }
        _ => String::from(""),
    };

    let hex_str = hex_dump
        .iter()
        .map(|z| format!("{:02x}", z))
        .collect::<Vec<String>>()
        .join(" ");

    let asm_str = format!(
        "{:04x}  {:8} {: >4} {}",
        begin,
        hex_str,
        opcode.mnemonic(),
        tmp
    )
    .trim()
    .to_string();

    Ok(format!(
        "{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x}",
        asm_str,
        cpu.register_a.get(),
        cpu.register_x.get(),
        cpu.register_y.get(),
        cpu.status,
        cpu.stack_pointer.get(),
    )
    .to_ascii_uppercase())
}
