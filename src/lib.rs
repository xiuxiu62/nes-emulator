#![allow(dead_code)]

pub mod core;
pub mod error;
pub mod io;
mod macros;
mod trace;

pub use trace::trace;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod test {
    use crate::core::Bus;

    use super::{
        core::{Cpu, Rom},
        error::Result,
        rom,
    };

    lazy_static! {
        pub static ref TEST_DATA: Vec<Rom> = vec![
            rom![0xA9, 0x05, 0x00],
            rom![0xA9, 0x00, 0x00],
            rom![0xa9, 0x02, 0xaa, 0x00],
            rom![0xa9, 0x02, 0xaa, 0xe8, 0x00]
        ];
    }

    // Executes the Rom and returns the Cpu
    fn run(rom: &'static Rom) -> Result<Cpu> {
        let bus = Bus::new(rom);
        let mut cpu = Cpu::new(bus);

        cpu.load()?;
        cpu.run()?;

        println!("{cpu}");

        Ok(cpu)
    }

    #[test]
    fn ensure_0xa9_lda_immidiate_load_data() -> Result<()> {
        let cpu = run(&TEST_DATA[0])?;

        let status = cpu.status.bits();
        let register_a = cpu.register_a.get();

        assert_eq!(register_a, 0x05);
        assert!(status & 0b0000_0010 == 0b00);
        assert!(status & 0b1000_0000 == 0);

        Ok(())
    }

    #[test]
    fn ensure_0xa9_lda_zero_flag() -> Result<()> {
        let cpu = run(&TEST_DATA[1])?;

        let status = cpu.status.bits();
        assert!(status & 0b0000_0010 == 0b10);

        Ok(())
    }

    #[test]
    fn ensure_0xaa_moves_a_to_x() -> Result<()> {
        let cpu = run(&TEST_DATA[2])?;

        let register_x = cpu.register_x.get();
        assert_eq!(register_x, 2);

        Ok(())
    }

    #[test]
    fn ensure_0xe8_increments_the_x_register() -> Result<()> {
        let cpu = run(&TEST_DATA[3])?;

        let register_x = cpu.register_x.get();
        assert_eq!(register_x, 3);
        Ok(())
    }
}
