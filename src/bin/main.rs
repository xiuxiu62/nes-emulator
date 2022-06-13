use nes_emulator::{Cpu, Interpreter};

fn main() -> nes_emulator::Result<()> {
    let source = vec![0xa9, 0x01];

    let cpu = interpret(&source)?;
    println!("{cpu}");

    Ok(())
}

fn interpret(source: &[u8]) -> nes_emulator::Result<Cpu> {
    let mut cpu = Cpu::default();
    let mut interpreter = Interpreter::new(Some(source), &mut cpu);

    interpreter.interpret()?;

    Ok(cpu)
}
