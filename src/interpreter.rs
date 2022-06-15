use crate::{cpu::AddressingMode, ram::RAM_SIZE, Cpu, Error, Result};

#[derive(Debug)]
pub struct Interpreter<'a> {
    cpu: &'a mut Cpu,
}

// Implements an opcode handler for Interpreter<'a>
// macro_rules! opcode {
//     ($id:ident, $doc_string:expr, $self:ident $f:block) => {
//         #[doc=$doc_string]
//         pub fn $id(&mut $self) -> Result<()> {
//             $f();

//             Ok(())
//         }
//     };

//     [$(($id:ident, $doc_string:expr) => $self:ident $f:block),*] => {
//         $(opcode!($id, $doc_string, $self $f);)*
//     }
// }

impl<'a> Interpreter<'a> {
    pub fn new(cpu: &'a mut Cpu) -> Self {
        Self { cpu }
    }

    pub fn load(&mut self, source: &'a [u8]) {
        self.cpu.load_program(source);
    }

    pub fn mem_dump(&self) -> [u8; RAM_SIZE] {
        self.cpu.mem_dump()
    }

    pub fn interpret(&mut self) -> Result<()> {
        loop {
            let opcode = self.get_current_opcode();
            self.cpu.program_counter.increment();

            if opcode == 0x00 {
                return Ok(());
            }

            // self.handle_opcode(opcode)?;
        }
    }

    // fn handle_opcode(&mut self, opcode: u8) -> Result<()> {
    //     match opcode {
    //         0xA9 => self.oc_0xa9(),
    //         0xA5 => self.oc_0xa5(),
    //         0xAD => self.oc_0xad(),
    //         0xAA => self.oc_0xaa(),
    //         0xE8 => self.oc_0xe8(),
    //         code => Err(Error::Unsupported(format!(
    //             r#"opcode "{code:#x}" is not supported"#
    //         ))),
    //     }
    // }

    fn get_current_opcode(&self) -> u8 {
        let program_counter = self.cpu.program_counter.get();
        self.cpu.mem_read_byte(program_counter)
    }

    // opcode![
    //     (oc_0xa9, "LDA (Immediate Mode)") => self {
    //         self.cpu.lda(AddressingMode::Immediate)?;
    //         self.cpu.program_counter.increment();
    //     },
    //     (oc_0xa5, "LDA (Zero Page Mode)") => self {
    //         self.cpu.lda(AddressingMode::ZeroPage)?;
    //         self.cpu.program_counter.increment();
    //     },
    //     (oc_0xad, "LDA (Absolute Mode)") => self {
    //         self.cpu.lda(AddressingMode::Absolute)?;
    //         (0..2).for_each(|_| self.cpu.program_counter.increment());
    //     },
    //     (oc_0xaa, "TAX: copies the a register into the x register") => self {
    //         self.cpu.register_x.set(self.cpu.register_a.get());

    //         let register_x = self.cpu.register_x.get();
    //         self.cpu.update_zero_flag(register_x);
    //         self.cpu.update_negative_flag(register_x);
    //     },
    //     (oc_0xe8, "INX: increments the x register") => self {
    //         self.cpu.register_x.increment();

    //         let register_x = self.cpu.register_x.get();
    //         self.cpu.update_zero_flag(register_x);
    //         self.cpu.update_negative_flag(register_x);
    //     }
    // ];
}
