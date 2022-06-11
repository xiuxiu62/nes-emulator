#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug, Default)]
pub struct Cpu {
    register: u8,
    status: u8,
    program_counter: u16,
}

// impl Cpu {
//     pub fn interpret(&mut self, interpreter: Interpreter) -> Result<()> {
//         self.program_counter = 0;

//         interpreter.interpret(self)
//     }
// }

#[derive(Debug)]
pub struct Interpreter<'a> {
    source: &'a [u8],
    cpu: &'a mut Cpu,
}

macro_rules! opcode {
    ($self:ident, $id:ident, $f:tt) => {
        fn $id(&self) -> Result<()> {
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
    pub fn new(source: &'a [u8], cpu: &'a mut Cpu) -> Self {
        Self { source, cpu }
    }

    pub fn interpret(&mut self) -> Result<()> {
        self.source.iter().try_for_each(|opcode| {
            self.cpu.program_counter += 1;

            self.handle_opcode(*opcode)
        })
    }

    fn handle_opcode(&self, opcode: u8) -> Result<()> {
        match opcode {
            0xA9 => self.oc_0xa9(),
            code => Err(Error::UnsupportedOpcode(format!(
                r#"opcode "{code:#x}" not supported"#
            ))),
        }
    }

    opcode!(self, oc_0xa9, {
        println!("hello world");
    });
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnsupportedOpcode(String),
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

    #[test]
    fn interpreter_works() -> Result<()> {
        let source = vec![0xA9];

        let mut cpu = Cpu::default();
        let mut interpreter = Interpreter::new(&source, &mut cpu);

        interpreter.interpret()
    }
}
