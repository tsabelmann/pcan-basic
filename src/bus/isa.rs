use crate::bus::Bus;
use crate::channel::Channel;
use crate::hw::{HasChannelCondition, HasControllerNumber, HasHardwareName};
use crate::pcan;

///
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum IsaBus {
    ///
    ISA1,
    ///
    ISA2,
    ///
    ISA3,
    ///
    ISA4,
    ///
    ISA5,
    ///
    ISA6,
    ///
    ISA7,
    ///
    ISA8,
}

impl From<IsaBus> for u16 {
    fn from(value: IsaBus) -> Self {
        let ret = match value {
            IsaBus::ISA1 => pcan::PCAN_ISABUS1,
            IsaBus::ISA2 => pcan::PCAN_ISABUS2,
            IsaBus::ISA3 => pcan::PCAN_ISABUS3,
            IsaBus::ISA4 => pcan::PCAN_ISABUS4,
            IsaBus::ISA5 => pcan::PCAN_ISABUS5,
            IsaBus::ISA6 => pcan::PCAN_ISABUS6,
            IsaBus::ISA7 => pcan::PCAN_ISABUS7,
            IsaBus::ISA8 => pcan::PCAN_ISABUS8,
        } as u16;
        ret
    }
}

impl TryFrom<u16> for IsaBus {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value as u32 {
            pcan::PCAN_ISABUS1 => Ok(IsaBus::ISA1),
            pcan::PCAN_ISABUS2 => Ok(IsaBus::ISA2),
            pcan::PCAN_ISABUS3 => Ok(IsaBus::ISA3),
            pcan::PCAN_ISABUS4 => Ok(IsaBus::ISA4),
            pcan::PCAN_ISABUS5 => Ok(IsaBus::ISA5),
            pcan::PCAN_ISABUS6 => Ok(IsaBus::ISA6),
            pcan::PCAN_ISABUS7 => Ok(IsaBus::ISA7),
            pcan::PCAN_ISABUS8 => Ok(IsaBus::ISA8),
            _ => Err(()),
        }
    }
}

/* Bus trait implementation */

impl Bus for IsaBus {
    fn channel(&self) -> u16 {
        u16::from(*self)
    }
}

/* Channel trait implementation */

impl Channel for IsaBus {
    fn channel(&self) -> u16 {
        Bus::channel(self)
    }
}

/* HARDWARE IDENTIFICATION */

impl HasChannelCondition for IsaBus {}

impl HasHardwareName for IsaBus {}

impl HasControllerNumber for IsaBus {}

/* SPECIAL BEHAVIOR */
