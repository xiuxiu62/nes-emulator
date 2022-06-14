use std::fmt::Display;

use crate::Component;

#[derive(Debug, Default)]
pub struct Cpu {
    pub register_a: Component<u8>,
    pub register_x: Component<u8>,
    pub status: Component<u8>,
    pub program_counter: Component<u16>,
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
