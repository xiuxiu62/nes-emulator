use std::collections::HashMap;

lazy_static! {
    pub static ref INTERRUPT_DESCRIPTOR_TABLE: HashMap<InterruptType, Interrupt> =
        HashMap::from([(InterruptType::NMI, Interrupt::new(0xFFFA, 0b0010_0000, 2))]);
}

#[derive(PartialEq, Eq, Hash)]
pub enum InterruptType {
    NMI,
}

#[derive(PartialEq, Eq)]
pub struct Interrupt {
    vector_address: u16,
    b_flag_mask: u8,
    cpu_cycles: u8,
}

impl Interrupt {
    pub fn new(vector_address: u16, b_flag_mask: u8, cpu_cycles: u8) -> Self {
        Self {
            vector_address,
            b_flag_mask,
            cpu_cycles,
        }
    }
}
