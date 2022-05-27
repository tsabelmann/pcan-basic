use crate::bus::Bus;
use crate::channel::Channel;
use crate::hw::{HasChannelCondition, HasControllerNumber, HasDevicePartNumber, HasHardwareName};
use crate::info::{HasBitrateInfo, HasBitrateInfoFd, HasChannelFeatures, HasChannelVersion};
use crate::pcan;

///
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DngBus {
    ///
    DNG1,
}

impl From<DngBus> for u16 {
    fn from(value: DngBus) -> Self {
        let ret = match value {
            DngBus::DNG1 => pcan::PCAN_DNGBUS1,
        } as u16;
        ret
    }
}

impl TryFrom<u16> for DngBus {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value as u32 {
            pcan::PCAN_DNGBUS1 => Ok(DngBus::DNG1),
            _ => Err(()),
        }
    }
}

/* Bus trait implementation */

impl Bus for DngBus {
    fn channel(&self) -> u16 {
        u16::from(*self)
    }
}

/* Channel trait implementation */

impl Channel for DngBus {
    fn channel(&self) -> u16 {
        Bus::channel(self)
    }
}

/* HARDWARE IDENTIFICATION */

impl HasChannelCondition for DngBus {}

impl HasHardwareName for DngBus {}

impl HasControllerNumber for DngBus {}

impl HasDevicePartNumber for DngBus {}

/* INFORMATIONAL PARAMETERS */

impl HasChannelVersion for DngBus {}

impl HasChannelFeatures for DngBus {}

impl HasBitrateInfo for DngBus {}

impl HasBitrateInfoFd for DngBus {}

/* SPECIAL BEHAVIOR */
