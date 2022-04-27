use crate::bus::Bus;
use crate::channel::Channel;
use crate::pcan;

///
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PccBus {
    ///
    PCC1,
    ///
    PCC2,
}

impl From<PccBus> for u16 {
    fn from(value: PccBus) -> Self {
        let ret = match value {
            PccBus::PCC1 => pcan::PCAN_PCCBUS1,
            PccBus::PCC2 => pcan::PCAN_PCCBUS2,
        } as u16;
        ret
    }
}

impl TryFrom<u16> for PccBus {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value as u32 {
            pcan::PCAN_PCCBUS1 => Ok(PccBus::PCC1),
            pcan::PCAN_PCCBUS2 => Ok(PccBus::PCC2),
            _ => Err(()),
        }
    }
}

/* Bus trait implementation */

impl Bus for PccBus {
    fn channel(&self) -> u16 {
        u16::from(*self)
    }
}

/* Channel trait implementation */

impl Channel for PccBus {
    fn channel(&self) -> u16 {
        Bus::channel(self)
    }
}

/* HARDWARE IDENTIFICATION */

/* SPECIAL BEHAVIOR */
