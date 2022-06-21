mod address;
mod control;
mod mask;
mod scroll;
mod status;

use address::AddressRegister;
use control::ControlRegister;
use mask::MaskRegister;
use scroll::ScrollRegister;
use status::StatusRegister;

#[derive(Debug, Default)]
pub struct PpuRegisters {
    pub address: AddressRegister,
    pub control: ControlRegister,
    pub mask: MaskRegister,
    pub scroll: ScrollRegister,
    pub status: StatusRegister,
}
