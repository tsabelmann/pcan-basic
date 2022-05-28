//!
//!
//!

pub mod dng;
pub mod isa;
pub mod lan;
pub mod pcc;
pub mod pci;
pub mod usb;

///
pub trait Bus {
    ///
    fn channel(&self) -> u16;
}

pub use dng::DngBus;
pub use isa::IsaBus;
pub use lan::LanBus;
pub use pcc::PccBus;
pub use pci::PciBus;
pub use usb::UsbBus;
