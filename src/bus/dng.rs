use crate::bus::Bus;
use crate::channel::Channel;
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

/* SPECIAL BEHAVIOR */
