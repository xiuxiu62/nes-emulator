use std::path::Path;

use nes_emulator::{
    core::{Bus, Cartridge, Cpu},
    trace,
};

const TEST_DATA_DIRECTORY: &'static str = "test_data";

fn main() -> nes_emulator::error::Result<()> {
    let filename = format!("./{TEST_DATA_DIRECTORY}/nestest.nes");
    let path = Path::new(&filename);
    let cartridge = Cartridge::try_from(path)?;
    // let cartridge = Cartridge::new(vec![
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // 0x00,
    // ])?;

    let mut nes = Nes::new(&cartridge)?;
    nes.run()?;

    Ok(())
}

struct Nes(Cpu);

impl Nes {
    pub fn new(cartridge: &Cartridge) -> nes_emulator::error::Result<Self> {
        let bus = Bus::new(cartridge);
        let mut cpu = Cpu::new(bus);
        cpu.load()?;

        Ok(Self(cpu))
    }

    pub fn run(&mut self) -> nes_emulator::error::Result<()> {
        self.0.run_with_callback(|cpu| {
            println!("{}", trace(cpu)?);

            Ok(())
        })
    }
}
