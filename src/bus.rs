use crate::error::{PcanError, PcanOkError};
use crate::pcan;
use std::os::raw::c_void;

/* TRAITS */

pub trait ToHandle {
    fn handle(&self) -> u16;
}

#[derive(Debug, PartialEq)]
pub enum ChannelConditionStatus {
    Unavailable,
    Available,
    Occupied,
    PcanView,
}

impl TryFrom<u32> for ChannelConditionStatus {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            pcan::PCAN_CHANNEL_AVAILABLE => Ok(ChannelConditionStatus::Available),
            pcan::PCAN_CHANNEL_UNAVAILABLE => Ok(ChannelConditionStatus::Unavailable),
            pcan::PCAN_CHANNEL_OCCUPIED => Ok(ChannelConditionStatus::Occupied),
            pcan::PCAN_CHANNEL_PCANVIEW => Ok(ChannelConditionStatus::PcanView),
            _ => Err(()),
        }
    }
}

trait HasChannelCondition {}

pub trait ChannelCondition {
    fn channel_condition(&self) -> Result<ChannelConditionStatus, PcanError>;
}

impl<T: HasChannelCondition + ToHandle> ChannelCondition for T {
    fn channel_condition(&self) -> Result<ChannelConditionStatus, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.handle(),
                pcan::PCAN_CHANNEL_CONDITION as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };
        let value: u32 = u32::from_le_bytes(data);

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match ChannelConditionStatus::try_from(value) {
                Ok(status) => Ok(status),
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Channel Identifying */

trait HasChannelIdentifying {}

pub trait ChannelIdentifying {
    fn enable_identifying(&self) -> Result<(), PcanError>;
    fn disable_identifying(&self) -> Result<(), PcanError>;
    fn identifying(&self, value: bool) -> Result<(), PcanError> {
        match value {
            false => self.disable_identifying(),
            true => self.enable_identifying(),
        }
    }
}

impl<T: HasChannelIdentifying + ToHandle> ChannelIdentifying for T {
    fn enable_identifying(&self) -> Result<(), PcanError> {
        let mut data = u32::to_le_bytes(pcan::PCAN_PARAMETER_ON);
        let code = unsafe {
            pcan::CAN_SetValue(
                self.handle(),
                pcan::PCAN_CHANNEL_IDENTIFYING as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(()),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn disable_identifying(&self) -> Result<(), PcanError> {
        let mut data = u32::to_le_bytes(pcan::PCAN_PARAMETER_OFF);
        let code = unsafe {
            pcan::CAN_SetValue(
                self.handle(),
                pcan::PCAN_CHANNEL_IDENTIFYING as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(()),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Device Id */

trait HasDeviceId {}

pub trait DeviceId {
    fn device_id(&self) -> Result<u32, PcanError>;
}

impl<T: HasDeviceId + ToHandle> DeviceId for T {
    fn device_id(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.handle(),
                pcan::PCAN_DEVICE_ID as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(u32::from_le_bytes(data)),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Hardware Name */

trait HasHardwareName {}

pub trait HardwareName {
    fn hardware_name(&self) -> Result<String, PcanError>;
}

impl<T: HasHardwareName + ToHandle> HardwareName for T {
    fn hardware_name(&self) -> Result<String, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_HARDWARE_NAME as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.handle(),
                pcan::PCAN_HARDWARE_NAME as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let s = s.trim_matches(char::from(0));
                    Ok(String::from(s))
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Controller Number */

trait HasControllerNumber {}

pub trait ControllerNumber {
    fn controller_number(&self) -> Result<u32, PcanError>;
}

impl<T: HasControllerNumber + ToHandle> ControllerNumber for T {
    fn controller_number(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.handle(),
                pcan::PCAN_CONTROLLER_NUMBER as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(u32::from_le_bytes(data)),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Device Part Number */

trait HasDevicePartNumber {}

pub trait DevicePartNumber {
    fn device_part_number(&self) -> Result<String, PcanError>;
}

impl<T: HasDevicePartNumber + ToHandle> DevicePartNumber for T {
    fn device_part_number(&self) -> Result<String, PcanError> {
        let mut data = [0u8; 100];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.handle(),
                pcan::PCAN_DEVICE_PART_NUMBER as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let s = s.trim_matches(char::from(0));
                    Ok(String::from(s))
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* BUS SECTION */

///
#[derive(Debug, PartialEq)]
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

impl From<IsaBus> for u32 {
    fn from(value: IsaBus) -> Self {
        match value {
            IsaBus::ISA1 => pcan::PCAN_ISABUS1,
            IsaBus::ISA2 => pcan::PCAN_ISABUS2,
            IsaBus::ISA3 => pcan::PCAN_ISABUS3,
            IsaBus::ISA4 => pcan::PCAN_ISABUS4,
            IsaBus::ISA5 => pcan::PCAN_ISABUS5,
            IsaBus::ISA6 => pcan::PCAN_ISABUS6,
            IsaBus::ISA7 => pcan::PCAN_ISABUS7,
            IsaBus::ISA8 => pcan::PCAN_ISABUS8,
        }
    }
}

// impl TryFrom<u32> for IsaBus {
//     type Error = ();
//
//     fn try_from(value: u32) -> Result<Self, Self::Error> {
//         Ok(IsaBus::ISA1)
//     }
// }

impl ToHandle for IsaBus {
    fn handle(&self) -> u16 {
        match self {
            IsaBus::ISA1 => pcan::PCAN_ISABUS1 as u16,
            IsaBus::ISA2 => pcan::PCAN_ISABUS2 as u16,
            IsaBus::ISA3 => pcan::PCAN_ISABUS3 as u16,
            IsaBus::ISA4 => pcan::PCAN_ISABUS4 as u16,
            IsaBus::ISA5 => pcan::PCAN_ISABUS5 as u16,
            IsaBus::ISA6 => pcan::PCAN_ISABUS6 as u16,
            IsaBus::ISA7 => pcan::PCAN_ISABUS7 as u16,
            IsaBus::ISA8 => pcan::PCAN_ISABUS8 as u16,
        }
    }
}

///
#[derive(Debug, PartialEq)]
pub enum DngBus {
    ///
    DNG1,
}

impl From<DngBus> for u32 {
    fn from(value: DngBus) -> Self {
        match value {
            DngBus::DNG1 => pcan::PCAN_DNGBUS1,
        }
    }
}

impl TryFrom<u32> for DngBus {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            pcan::PCAN_DNGBUS1 => Ok(DngBus::DNG1),
            _ => Err(()),
        }
    }
}

impl ToHandle for DngBus {
    fn handle(&self) -> u16 {
        match self {
            DngBus::DNG1 => pcan::PCAN_DNGBUS1 as u16,
        }
    }
}

///
#[derive(Debug, PartialEq)]
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

impl ToHandle for PciBus {
    fn handle(&self) -> u16 {
        match self {
            PciBus::PCI1 => pcan::PCAN_PCIBUS1 as u16,
            PciBus::PCI2 => pcan::PCAN_PCIBUS2 as u16,
            PciBus::PCI3 => pcan::PCAN_PCIBUS3 as u16,
            PciBus::PCI4 => pcan::PCAN_PCIBUS4 as u16,
            PciBus::PCI5 => pcan::PCAN_PCIBUS5 as u16,
            PciBus::PCI6 => pcan::PCAN_PCIBUS6 as u16,
            PciBus::PCI7 => pcan::PCAN_PCIBUS7 as u16,
            PciBus::PCI8 => pcan::PCAN_PCIBUS8 as u16,
            PciBus::PCI9 => pcan::PCAN_PCIBUS9 as u16,
            PciBus::PCI10 => pcan::PCAN_PCIBUS10 as u16,
            PciBus::PCI11 => pcan::PCAN_PCIBUS11 as u16,
            PciBus::PCI12 => pcan::PCAN_PCIBUS12 as u16,
            PciBus::PCI13 => pcan::PCAN_PCIBUS13 as u16,
            PciBus::PCI14 => pcan::PCAN_PCIBUS14 as u16,
            PciBus::PCI15 => pcan::PCAN_PCIBUS15 as u16,
            PciBus::PCI16 => pcan::PCAN_PCIBUS16 as u16,
        }
    }
}

///
#[derive(Debug, PartialEq)]
pub enum UsbBus {
    ///
    USB1,
    ///
    USB2,
    ///
    USB3,
    ///
    USB4,
    ///
    USB5,
    ///
    USB6,
    ///
    USB7,
    ///
    USB8,
    ///
    USB9,
    ///
    USB10,
    ///
    USB11,
    ///
    USB12,
    ///
    USB13,
    ///
    USB14,
    ///
    USB15,
    ///
    USB16,
}

impl ToHandle for UsbBus {
    fn handle(&self) -> u16 {
        match self {
            UsbBus::USB1 => pcan::PCAN_USBBUS1 as u16,
            UsbBus::USB2 => pcan::PCAN_USBBUS2 as u16,
            UsbBus::USB3 => pcan::PCAN_USBBUS3 as u16,
            UsbBus::USB4 => pcan::PCAN_USBBUS4 as u16,
            UsbBus::USB5 => pcan::PCAN_USBBUS5 as u16,
            UsbBus::USB6 => pcan::PCAN_USBBUS6 as u16,
            UsbBus::USB7 => pcan::PCAN_USBBUS7 as u16,
            UsbBus::USB8 => pcan::PCAN_USBBUS8 as u16,
            UsbBus::USB9 => pcan::PCAN_USBBUS9 as u16,
            UsbBus::USB10 => pcan::PCAN_USBBUS10 as u16,
            UsbBus::USB11 => pcan::PCAN_USBBUS11 as u16,
            UsbBus::USB12 => pcan::PCAN_USBBUS12 as u16,
            UsbBus::USB13 => pcan::PCAN_USBBUS13 as u16,
            UsbBus::USB14 => pcan::PCAN_USBBUS14 as u16,
            UsbBus::USB15 => pcan::PCAN_USBBUS15 as u16,
            UsbBus::USB16 => pcan::PCAN_USBBUS16 as u16,
        }
    }
}

///
#[derive(Debug, PartialEq)]
pub enum PccBus {
    ///
    PCC1,
    ///
    PCC2,
}

impl ToHandle for PccBus {
    fn handle(&self) -> u16 {
        match self {
            PccBus::PCC1 => pcan::PCAN_PCCBUS1 as u16,
            PccBus::PCC2 => pcan::PCAN_PCCBUS2 as u16,
        }
    }
}

///
#[derive(Debug, PartialEq)]
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

impl ToHandle for LanBus {
    fn handle(&self) -> u16 {
        match self {
            LanBus::LAN1 => pcan::PCAN_LANBUS1 as u16,
            LanBus::LAN2 => pcan::PCAN_LANBUS2 as u16,
            LanBus::LAN3 => pcan::PCAN_LANBUS3 as u16,
            LanBus::LAN4 => pcan::PCAN_LANBUS4 as u16,
            LanBus::LAN5 => pcan::PCAN_LANBUS5 as u16,
            LanBus::LAN6 => pcan::PCAN_LANBUS6 as u16,
            LanBus::LAN7 => pcan::PCAN_LANBUS7 as u16,
            LanBus::LAN8 => pcan::PCAN_LANBUS8 as u16,
            LanBus::LAN9 => pcan::PCAN_LANBUS9 as u16,
            LanBus::LAN10 => pcan::PCAN_LANBUS10 as u16,
            LanBus::LAN11 => pcan::PCAN_LANBUS11 as u16,
            LanBus::LAN12 => pcan::PCAN_LANBUS12 as u16,
            LanBus::LAN13 => pcan::PCAN_LANBUS13 as u16,
            LanBus::LAN14 => pcan::PCAN_LANBUS14 as u16,
            LanBus::LAN15 => pcan::PCAN_LANBUS15 as u16,
            LanBus::LAN16 => pcan::PCAN_LANBUS16 as u16,
        }
    }
}

/* Has Channel Condition trait implementations */
impl HasChannelCondition for IsaBus {}
impl HasChannelCondition for DngBus {}
impl HasChannelCondition for PciBus {}
impl HasChannelCondition for UsbBus {}
impl HasChannelCondition for PccBus {}
impl HasChannelCondition for LanBus {}

/* Has Device ID trait implementations */
impl HasDeviceId for UsbBus {}

/* Has Hardware Name trait implementations */
impl HasHardwareName for IsaBus {}
impl HasHardwareName for DngBus {}
impl HasHardwareName for PciBus {}
impl HasHardwareName for UsbBus {}
impl HasHardwareName for PccBus {}
impl HasHardwareName for LanBus {}

/* Has Controller Number trait implementations */
impl HasControllerNumber for IsaBus {}
impl HasControllerNumber for DngBus {}
impl HasControllerNumber for PciBus {}
impl HasControllerNumber for UsbBus {}
impl HasControllerNumber for PccBus {}
impl HasControllerNumber for LanBus {}

/* Has Channel Identifying trait implementations */
impl HasChannelIdentifying for UsbBus {}

/* Has Ip Address trait implementations */

/* Has Available Channels trait implementations */

/* Has Device Part Number trait implementations */
impl HasDevicePartNumber for IsaBus {}
impl HasDevicePartNumber for DngBus {}
impl HasDevicePartNumber for PciBus {}
impl HasDevicePartNumber for UsbBus {}
impl HasDevicePartNumber for PccBus {}
impl HasDevicePartNumber for LanBus {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usb_bus_channel_condition_001() {
        let usb = UsbBus::USB1;
        let code = usb.channel_condition();
        match code {
            Ok(v) => println!("{:?}", v),
            Err(err) => println!("{:?}", err),
        }
    }
}
