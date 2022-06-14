mod component;
mod cpu;
mod error;
mod interpreter;

pub use component::Component;
pub use cpu::Cpu;
pub use error::{Error, Result};
pub use interpreter::Interpreter;

#[cfg(test)]
mod test {
    use super::{Cpu, Interpreter, Result};

    // Interprets the source code and returns (Register_A, Status)
    fn interpret(source: Vec<u8>) -> Result<(u8, u8)> {
        let mut cpu = Cpu::default();
        let mut interpreter = Interpreter::new(Some(&source), &mut cpu);

        interpreter.interpret()?;

        Ok((cpu.register_a.get(), cpu.status.get()))
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
