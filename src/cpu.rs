use std::fmt::Display;

#[derive(Debug, Default)]
pub struct Cpu {
    register_a: u8,
    status: u8,
    program_counter: u16,
}

impl Cpu {
    pub fn get_register_a(&self) -> u8 {
        self.register_a
    }

    pub fn set_register_a(&mut self, value: u8) {
        self.register_a = value;
    }

    pub fn get_status(&self) -> u8 {
        self.status
    }

    pub fn set_status(&mut self, value: u8) {
        self.status = value;
    }

    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    pub fn set_program_coutner(&mut self, value: u16) {
        self.program_counter = value;
    }

    /// Increments the program counter, returning the previous value
    pub fn increment_program_counter(&mut self) -> u16 {
        let old_value = self.program_counter;
        self.program_counter += 1;

        old_value
    }

    /// Decrements the program counter, returning the previous value
    pub fn decrement_program_counter(&mut self) -> u16 {
        let old_value = self.program_counter;
        self.program_counter -= 1;

        old_value
    }
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = format!(
            "CPU {{
    register_a: {},
    status: {},
    program_counter: {}
}}",
            self.get_register_a(),
            self.get_status(),
            self.get_program_counter()
        );

        write!(f, "{}", message)
    }
}
