#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug, Default)]
pub struct Cpu {
    register_a: u8,
    status: u8,
    program_counter: u16,
}

impl Cpu {
    pub fn get_register_a(&self) -> u8 {
        self.register_a
    }

    pub fn set_register_a(&mut self, value: u8) {
        self.register_a = value;
    }

    pub fn get_status(&self) -> u8 {
        self.status
    }

    pub fn set_status(&mut self, value: u8) {
        self.status = value;
    }

    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    pub fn set_program_coutner(&mut self, value: u16) {
        self.program_counter = value;
    }

    /// Increments the program counter, returning the previous value
    pub fn increment_program_counter(&mut self) -> u16 {
        let old_value = self.program_counter;
        self.program_counter += 1;

        old_value
    }

    /// Decrements the program counter, returning the previous value
    pub fn decrement_program_counter(&mut self) -> u16 {
        let old_value = self.program_counter;
        self.program_counter -= 1;

        old_value
    }
}

#[derive(Debug)]
pub struct Interpreter<'a> {
    source: Option<&'a [u8]>,
    cpu: &'a mut Cpu,
}

macro_rules! opcode {
    ($id:ident, $self:ident $f:block) => {
        fn $id(&mut $self) -> Result<()> {
            $f();

            Ok(())
        }
    };
}

// macro_rules! opcode {
//     ($self:ident, $id:ident, $f:tt) => {
//         fn $id(&self, cpu: &mut Cpu) -> Result<()> {
//             $f(cpu: &mut Cpu);

//             Ok(())
//         }
//     };
// }

impl<'a> Interpreter<'a> {
    pub fn new(source: Option<&'a [u8]>, cpu: &'a mut Cpu) -> Self {
        Self { source, cpu }
    }

    pub fn load(&mut self, source: &'a [u8]) {
        self.source = Some(source)
    }

    pub fn interpret(&mut self) -> Result<()> {
        if self.source.is_none() {
            return Err(Error::NoCode("No source code has been loaded".to_owned()));
        }

        loop {
            match self.get_current_opcode() {
                Some(opcode) => {
                    if opcode == 0x00 {
                        break;
                    }

                    self.step(opcode)?
                }
                None => break,
            }
        }

        return Ok(());
    }

    fn step(&mut self, opcode: u8) -> Result<()> {
        self.cpu.increment_program_counter();

        self.handle_opcode(opcode)
    }

    fn handle_opcode(&mut self, opcode: u8) -> Result<()> {
        match opcode {
            0xA9 => self.oc_0xa9(),
            code => Err(Error::UnsupportedOpcode(format!(
                r#"opcode "{code:#x}" not supported"#
            ))),
        }
    }

    // SAFETY: we've already ensured self.source is Some at the top of the interpret method
    fn get_current_opcode(&self) -> Option<u8> {
        self.source
            .unwrap()
            .get(self.cpu.program_counter as usize)
            .map(|opcode| *opcode)
    }

    opcode!(oc_0xa9, self {
        match self.get_current_opcode() {
            Some(param) => {
                self.cpu.increment_program_counter();
                self.cpu.set_register_a(param);
            },
            None => return Err(Error::ExpectedParameter(self.cpu.get_program_counter())),
        };

        let register_a = self.cpu.get_register_a();
        let mut status = self.cpu.get_status();

        status = match register_a  {
            0 => status | 0b0000_0010,
            _ => status & 0b1111_1101,
        };

        status = match register_a & 0b1000_0000 {
            0 => status & 0b0111_1111,
            _ => status | 0b1000_0000,
        };

        self.cpu.set_status(status);
    });
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoCode(String),
    UnsupportedOpcode(String),
    ExpectedParameter(u16),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod test {
    use super::{Cpu, Interpreter, Result};

    // Interprets the source code and returns (Register_A, Status)
    fn interpret(source: Vec<u8>) -> Result<(u8, u8)> {
        let mut cpu = Cpu::default();
        let mut interpreter = Interpreter::new(Some(&source), &mut cpu);

        interpreter.interpret()?;

        Ok((cpu.get_register_a(), cpu.get_status()))
    }

    #[test]
    fn ensure_0xa9_lda_immidiate_load_data() -> Result<()> {
        let source = vec![0xa9, 0x05, 0x00];
        let (register_a, status) = interpret(source)?;

        println!("{:#x}", status & 0b0000_0010);

        assert_eq!(register_a, 0x05);
        assert!(status & 0b0000_0010 == 0b00);
        assert!(status & 0b1000_0000 == 0);

        Ok(())
    }

    #[test]
    fn ensure_0xa9_lda_zero_flag() -> Result<()> {
        let source = vec![0xA9, 0x00, 0x00];
        let (_register_a, status) = interpret(source)?;

        assert!(status & 0b0000_0010 == 0b10);

        Ok(())
    }
}
