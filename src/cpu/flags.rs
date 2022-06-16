bitflags! {
    pub struct CpuFlags: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const BREAK2            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIVE          = 0b10000000;
    }
}

impl Default for CpuFlags {
    fn default() -> Self {
        Self::from_bits_truncate(0b100100)
    }
}

impl CpuFlags {
    pub fn reset(&self) -> Self {
        Self::default()
    }
}
