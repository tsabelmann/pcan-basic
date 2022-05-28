use crate::bus::Bus;
use crate::channel::Channel;
use crate::df::{HasReceiveStatus, HasSetReceiveStatus};
use crate::hw::{
    HasChannelCondition, HasControllerNumber, HasDeviceId, HasDevicePartNumber, HasHardwareName,
};
use crate::info::{HasBitrateInfo, HasBitrateInfoFd, HasChannelFeatures, HasChannelVersion};
use crate::pcan;

///
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PciBus {
    ///
    PCI1,
    ///
    PCI2,
    ///
    PCI3,
    ///
    PCI4,
    ///
    PCI5,
    ///
    PCI6,
    ///
    PCI7,
    ///
    PCI8,
    ///
    PCI9,
    ///
    PCI10,
    ///
    PCI11,
    ///
    PCI12,
    ///
    PCI13,
    ///
    PCI14,
    ///
    PCI15,
    ///
    PCI16,
}

impl From<PciBus> for u16 {
    fn from(value: PciBus) -> Self {
        let ret = match value {
            PciBus::PCI1 => pcan::PCAN_PCIBUS1,
            PciBus::PCI2 => pcan::PCAN_PCIBUS2,
            PciBus::PCI3 => pcan::PCAN_PCIBUS3,
            PciBus::PCI4 => pcan::PCAN_PCIBUS4,
            PciBus::PCI5 => pcan::PCAN_PCIBUS5,
            PciBus::PCI6 => pcan::PCAN_PCIBUS6,
            PciBus::PCI7 => pcan::PCAN_PCIBUS7,
            PciBus::PCI8 => pcan::PCAN_PCIBUS8,
            PciBus::PCI9 => pcan::PCAN_PCIBUS9,
            PciBus::PCI10 => pcan::PCAN_PCIBUS10,
            PciBus::PCI11 => pcan::PCAN_PCIBUS11,
            PciBus::PCI12 => pcan::PCAN_PCIBUS12,
            PciBus::PCI13 => pcan::PCAN_PCIBUS13,
            PciBus::PCI14 => pcan::PCAN_PCIBUS14,
            PciBus::PCI15 => pcan::PCAN_PCIBUS15,
            PciBus::PCI16 => pcan::PCAN_PCIBUS16,
        } as u16;
        ret
    }
}

impl TryFrom<u16> for PciBus {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value as u32 {
            pcan::PCAN_PCIBUS1 => Ok(PciBus::PCI1),
            pcan::PCAN_PCIBUS2 => Ok(PciBus::PCI2),
            pcan::PCAN_PCIBUS3 => Ok(PciBus::PCI3),
            pcan::PCAN_PCIBUS4 => Ok(PciBus::PCI4),
            pcan::PCAN_PCIBUS5 => Ok(PciBus::PCI5),
            pcan::PCAN_PCIBUS6 => Ok(PciBus::PCI6),
            pcan::PCAN_PCIBUS7 => Ok(PciBus::PCI7),
            pcan::PCAN_PCIBUS8 => Ok(PciBus::PCI8),
            pcan::PCAN_PCIBUS9 => Ok(PciBus::PCI1),
            pcan::PCAN_PCIBUS10 => Ok(PciBus::PCI10),
            pcan::PCAN_PCIBUS11 => Ok(PciBus::PCI11),
            pcan::PCAN_PCIBUS12 => Ok(PciBus::PCI12),
            pcan::PCAN_PCIBUS13 => Ok(PciBus::PCI13),
            pcan::PCAN_PCIBUS14 => Ok(PciBus::PCI14),
            pcan::PCAN_PCIBUS15 => Ok(PciBus::PCI15),
            pcan::PCAN_PCIBUS16 => Ok(PciBus::PCI16),
            _ => Err(()),
        }
    }
}

/* Bus trait implementation */

impl Bus for PciBus {
    fn channel(&self) -> u16 {
        u16::from(*self)
    }
}

/* Channel trait implementation */

impl Channel for PciBus {
    fn channel(&self) -> u16 {
        Bus::channel(self)
    }
}

/* HARDWARE IDENTIFICATION */

impl HasChannelCondition for PciBus {}

impl HasDeviceId for PciBus {}

impl HasHardwareName for PciBus {}

impl HasControllerNumber for PciBus {}

impl HasDevicePartNumber for PciBus {}

/* INFORMATIONAL PARAMETERS */

impl HasChannelVersion for PciBus {}

impl HasChannelFeatures for PciBus {}

impl HasBitrateInfo for PciBus {}

impl HasBitrateInfoFd for PciBus {}

/* SPECIAL BEHAVIOR */

/* CONTROLLING DATA FLOW */

impl HasReceiveStatus for PciBus {}
impl HasSetReceiveStatus for PciBus {}
