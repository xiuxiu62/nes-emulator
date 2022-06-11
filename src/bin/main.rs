use nes_emulator::{Cpu, Interpreter};

fn main() -> Result<(), nes_emulator::Error> {
    let source = vec![0xa9];

    let mut cpu = Cpu::default();
    let mut interpreter = Interpreter::new(&source, &mut cpu);

    interpreter.interpret()
}
