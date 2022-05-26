use crate::bus::Bus;
use crate::channel::Channel;
use crate::hw::{HasChannelCondition, HasDeviceId};
use crate::pcan;

///
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LanBus {
    ///
    LAN1,
    ///
    LAN2,
    ///
    LAN3,
    ///
    LAN4,
    ///
    LAN5,
    ///
    LAN6,
    ///
    LAN7,
    ///
    LAN8,
    ///
    LAN9,
    ///
    LAN10,
    ///
    LAN11,
    ///
    LAN12,
    ///
    LAN13,
    ///
    LAN14,
    ///
    LAN15,
    ///
    LAN16,
}

impl From<LanBus> for u16 {
    fn from(value: LanBus) -> Self {
        let ret = match value {
            LanBus::LAN1 => pcan::PCAN_LANBUS1,
            LanBus::LAN2 => pcan::PCAN_LANBUS2,
            LanBus::LAN3 => pcan::PCAN_LANBUS3,
            LanBus::LAN4 => pcan::PCAN_LANBUS4,
            LanBus::LAN5 => pcan::PCAN_LANBUS5,
            LanBus::LAN6 => pcan::PCAN_LANBUS6,
            LanBus::LAN7 => pcan::PCAN_LANBUS7,
            LanBus::LAN8 => pcan::PCAN_LANBUS8,
            LanBus::LAN9 => pcan::PCAN_LANBUS9,
            LanBus::LAN10 => pcan::PCAN_LANBUS10,
            LanBus::LAN11 => pcan::PCAN_LANBUS11,
            LanBus::LAN12 => pcan::PCAN_LANBUS12,
            LanBus::LAN13 => pcan::PCAN_LANBUS13,
            LanBus::LAN14 => pcan::PCAN_LANBUS14,
            LanBus::LAN15 => pcan::PCAN_LANBUS15,
            LanBus::LAN16 => pcan::PCAN_LANBUS16,
        } as u16;
        ret
    }
}

impl TryFrom<u16> for LanBus {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value as u32 {
            pcan::PCAN_LANBUS1 => Ok(LanBus::LAN1),
            pcan::PCAN_LANBUS2 => Ok(LanBus::LAN2),
            pcan::PCAN_LANBUS3 => Ok(LanBus::LAN3),
            pcan::PCAN_LANBUS4 => Ok(LanBus::LAN4),
            pcan::PCAN_LANBUS5 => Ok(LanBus::LAN5),
            pcan::PCAN_LANBUS6 => Ok(LanBus::LAN6),
            pcan::PCAN_LANBUS7 => Ok(LanBus::LAN7),
            pcan::PCAN_LANBUS8 => Ok(LanBus::LAN8),
            pcan::PCAN_LANBUS9 => Ok(LanBus::LAN9),
            pcan::PCAN_LANBUS10 => Ok(LanBus::LAN10),
            pcan::PCAN_LANBUS11 => Ok(LanBus::LAN11),
            pcan::PCAN_LANBUS12 => Ok(LanBus::LAN12),
            pcan::PCAN_LANBUS13 => Ok(LanBus::LAN13),
            pcan::PCAN_LANBUS14 => Ok(LanBus::LAN14),
            pcan::PCAN_LANBUS15 => Ok(LanBus::LAN15),
            pcan::PCAN_LANBUS16 => Ok(LanBus::LAN16),
            _ => Err(()),
        }
    }
}

/* Bus trait implementation */

impl Bus for LanBus {
    fn channel(&self) -> u16 {
        u16::from(*self)
    }
}

/* Channel trait implementation */

impl Channel for LanBus {
    fn channel(&self) -> u16 {
        Bus::channel(self)
    }
}

/* HARDWARE IDENTIFICATION */

impl HasChannelCondition for LanBus {}

impl HasDeviceId for LanBus {}

/* SPECIAL BEHAVIOR */
