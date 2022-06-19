mod component;
mod cpu;
mod error;
mod ram;
#[macro_use]
mod rom;

pub use component::Component;
pub use cpu::Cpu;
pub use error::{Error, Result};
pub use ram::Ram;
pub use rom::Rom;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod test {
    use super::{Cpu, Result, Rom};

    // Interprets the source code and returns the Cpu
    fn run(rom: &Rom) -> Result<Cpu> {
        let mut cpu = Cpu::default();
        cpu.load_program(rom);
        cpu.run()?;

        Ok(cpu)
    }

    #[test]
    fn ensure_0xa9_lda_immidiate_load_data() -> Result<()> {
        let rom = rom![0xa9, 0x05, 0x00];
        let cpu = run(&rom)?;

        let status = cpu.status.bits();
        let register_a = cpu.register_a.get();

        assert_eq!(register_a, 0x05);
        assert!(status & 0b0000_0010 == 0b00);
        assert!(status & 0b1000_0000 == 0);

        Ok(())
    }

    #[test]
    fn ensure_0xa9_lda_zero_flag() -> Result<()> {
        let rom = rom![0xA9, 0x00, 0x00];
        let cpu = run(&rom)?;

        let status = cpu.status.bits();
        assert!(status & 0b0000_0010 == 0b10);

        Ok(())
    }

    #[test]
    fn ensure_0xaa_moves_a_to_x() -> Result<()> {
        let rom = rom![0xa9, 0x02, 0xaa, 0x00];
        let cpu = run(&rom)?;

        let register_x = cpu.register_x.get();
        assert_eq!(register_x, 2);

        Ok(())
    }

    #[test]
    fn ensure_0xe8_increments_the_x_register() -> Result<()> {
        let rom = rom![0xa9, 0x02, 0xaa, 0xe8, 0x00];
        let cpu = run(&rom)?;

        let register_x = cpu.register_x.get();
        assert_eq!(register_x, 3);
        Ok(())
    }
}
