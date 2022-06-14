use crate::{Cpu, Error, Result};

#[derive(Debug)]
pub struct Interpreter<'a> {
    source: Option<&'a [u8]>,
    cpu: &'a mut Cpu,
}

// Implements an opcode handler for Interpreter<'a>
macro_rules! opcode {
    ($id:ident, $doc_string:expr, $self:ident $f:block) => {
        #[doc=$doc_string]
        pub fn $id(&mut $self) -> Result<()> {
            $f();

            Ok(())
        }
    };

    [$(($id:ident, $doc_string:expr) => $self:ident $f:block),*] => {
        $(opcode!($id, $doc_string, $self $f);)*
    }
}

impl<'a> Interpreter<'a> {
    pub fn new(source: Option<&'a [u8]>, cpu: &'a mut Cpu) -> Self {
        Self { source, cpu }
    }

    pub fn load(&mut self, source: &'a [u8]) {
        self.source = Some(source)
    }

    pub fn interpret(&mut self) -> Result<()> {
        if self.source.is_none() {
            return Err(Error::NoSource("No source code has been loaded".to_owned()));
        };

        while let Some(opcode) = self.get_current_opcode() {
            if opcode == 0x00 {
                break;
            }

            self.step(opcode)?;
        }

        Ok(())
    }

    fn step(&mut self, opcode: u8) -> Result<()> {
        self.cpu.program_counter.increment();

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
            .get(self.cpu.program_counter.get() as usize)
            .copied()
    }

    opcode![
        (oc_0xa9, "LDA") => self {
            match self.get_current_opcode() {
                Some(param) => {
                    self.cpu.program_counter.increment();
                    self.cpu.register_a.set(param);
                },
                None => return Err(Error::ExpectedParameter(self.cpu.program_counter.get())),
            };

            let register_a = self.cpu.register_a.get();
            let mut status = self.cpu.status.get();

            status = match register_a  {
                0 => status | 0b0000_0010,
                _ => status & 0b1111_1101,
            };

            status = match register_a & 0b1000_0000 {
                0 => status & 0b0111_1111,
                _ => status | 0b1000_0000,
            };

            self.cpu.status.set(status);
        }
    ];
}
