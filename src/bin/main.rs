use nes_emulator::{Cpu, Interpreter};

fn main() -> nes_emulator::Result<()> {
    let source = vec![0xa9, 0x01, 0xaa, 0xe8, 0x00];
    let cpu = interpret(&source)?;

    println!("{cpu}");

    Ok(())
}

fn interpret(source: &[u8]) -> nes_emulator::Result<Cpu> {
    let mut cpu = Cpu::default();
    cpu.load_program(source);

    let mut interpreter = Interpreter::new(&mut cpu);
    interpreter.interpret()?;

    Ok(cpu)
}
