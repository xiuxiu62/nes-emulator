use std::fmt::Display;

use crate::Component;

#[derive(Debug, Default)]
pub struct Cpu {
    pub register_a: Component<u8>,
    pub register_x: Component<u8>,
    pub status: Component<u8>,
    pub program_counter: Component<u16>,
}

impl Cpu {
    pub fn update_zero_flag(&mut self, result: u8) {
        let old_status = self.status.get();
        self.status.set(match result {
            0 => old_status | 0b0000_0010,
            _ => old_status & 0b1111_1101,
        });
    }

    pub fn update_negative_flag(&mut self, result: u8) {
        let old_status = self.status.get();
        self.status.set(match result & 0b1000_0000 {
            0 => old_status & 0b0111_1111,
            _ => old_status | 0b1000_0000,
        });
    }
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = format!(
            "CPU {{
    registers: {{
        a: {}
        x: {}            
    }}
    status: {}
    program_counter: {}
}}",
            self.register_a.get(),
            self.register_x.get(),
            self.status.get(),
            self.program_counter.get()
        );

        write!(f, "{}", message)
    }
}
