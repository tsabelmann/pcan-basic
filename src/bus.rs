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

trait HasChannelCondition {}

pub trait ChannelCondition {
    fn channel_condition(&self) -> Result<ChannelConditionStatus, PcanError>;
}

impl<T: HasChannelCondition + ToHandle> ChannelCondition for T {
    fn channel_condition(&self) -> Result<ChannelConditionStatus, PcanError> {
        let value: u32 = 0;
        let code = unsafe {
            pcan::CAN_GetValue(
                self.handle(),
                pcan::PCAN_CHANNEL_CONDITION as u8,
                value as *mut c_void,
                4,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                if value & pcan::PCAN_CHANNEL_AVAILABLE == pcan::PCAN_CHANNEL_AVAILABLE {
                    return Ok(ChannelConditionStatus::Available);
                }

                if value & pcan::PCAN_CHANNEL_UNAVAILABLE == pcan::PCAN_CHANNEL_UNAVAILABLE {
                    return Ok(ChannelConditionStatus::Unavailable);
                }

                if value & pcan::PCAN_CHANNEL_OCCUPIED == pcan::PCAN_CHANNEL_OCCUPIED {
                    return Ok(ChannelConditionStatus::Occupied);
                }

                if value & pcan::PCAN_CHANNEL_PCANVIEW == pcan::PCAN_CHANNEL_PCANVIEW {
                    return Ok(ChannelConditionStatus::PcanView);
                }
                Err(PcanError::Unknown)
            }
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
        let activate = pcan::PCAN_PARAMETER_ON;
        let code = unsafe {
            pcan::CAN_SetValue(
                self.handle(),
                pcan::PCAN_CHANNEL_IDENTIFYING as u8,
                activate as *mut c_void,
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
        let activate = pcan::PCAN_PARAMETER_OFF;
        let code = unsafe {
            pcan::CAN_SetValue(
                self.handle(),
                pcan::PCAN_CHANNEL_IDENTIFYING as u8,
                activate as *mut c_void,
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
    fn device_id(&self) -> Result<u32, ()>;
}

impl<T: HasDeviceId + ToHandle> DeviceId for T {
    fn device_id(&self) -> Result<u32, ()> {
        let devive_id = 0u32;
        let flag = unsafe {
            pcan::CAN_GetValue(
                self.handle(),
                pcan::PCAN_DEVICE_ID as u8,
                devive_id as *mut c_void,
                4,
            )
        } == pcan::PCAN_ERROR_OK;

        match flag {
            true => Ok(devive_id),
            false => Err(()),
        }
    }
}

/* Hardware Name */

trait HasHardwareName {}

pub trait HardwareName {
    fn hardware_name(&self) -> Result<String, ()>;
}

impl<T: HasHardwareName + ToHandle> HardwareName for T {
    fn hardware_name(&self) -> Result<String, ()> {
        let mut hardware_name = [0u8; pcan::MAX_LENGTH_HARDWARE_NAME as usize];
        let flag = unsafe {
            pcan::CAN_GetValue(
                self.handle(),
                pcan::PCAN_HARDWARE_NAME as u8,
                hardware_name.as_mut_ptr() as *mut c_void,
                pcan::MAX_LENGTH_HARDWARE_NAME,
            )
        } == pcan::PCAN_ERROR_OK;

        match flag {
            true => {
                let s = std::str::from_utf8(&hardware_name);
                match s {
                    Ok(s) => Ok(String::from(s)),
                    Err(_) => Err(()),
                }
            }
            false => Err(()),
        }
    }
}

/* Controller Number */

trait HasControllerNumber {}

pub trait ControllerNumber {
    fn controller_number(&self) -> Result<u32, ()>;
}

impl<T: HasControllerNumber + ToHandle> ControllerNumber for T {
    fn controller_number(&self) -> Result<u32, ()> {
        let controller_number = 0u32;
        let flag = unsafe {
            pcan::CAN_GetValue(
                self.handle(),
                pcan::PCAN_CONTROLLER_NUMBER as u8,
                controller_number as *mut c_void,
                4,
            )
        } == pcan::PCAN_ERROR_OK;

        match flag {
            true => Ok(controller_number),
            false => Err(()),
        }
    }
}

/* Device Part Number */

trait HasDevicePartNumber {}

pub trait DevicePartNumber {
    fn device_part_number(&self) -> Result<String, ()>;
}

impl<T: HasDevicePartNumber + ToHandle> DevicePartNumber for T {
    fn device_part_number(&self) -> Result<String, ()> {
        let mut device_part_number = [0u8; 100];
        let flag = unsafe {
            pcan::CAN_GetValue(
                self.handle(),
                pcan::PCAN_DEVICE_PART_NUMBER as u8,
                device_part_number.as_mut_ptr() as *mut c_void,
                100,
            )
        } == pcan::PCAN_ERROR_OK;

        match flag {
            true => {
                let s = std::str::from_utf8(&device_part_number);
                match s {
                    Ok(s) => Ok(String::from(s)),
                    Err(_) => Err(()),
                }
            }
            false => Err(()),
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

impl HasChannelCondition for IsaBus {}
impl HasHardwareName for IsaBus {}
impl HasControllerNumber for IsaBus {}
impl HasDevicePartNumber for IsaBus {}

///
#[derive(Debug, PartialEq)]
pub enum DngBus {
    ///
    DNG1,
}

impl ToHandle for DngBus {
    fn handle(&self) -> u16 {
        match self {
            DngBus::DNG1 => pcan::PCAN_DNGBUS1 as u16,
        }
    }
}

impl HasChannelCondition for DngBus {}
impl HasHardwareName for DngBus {}
impl HasControllerNumber for DngBus {}
impl HasDevicePartNumber for DngBus {}

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

impl HasChannelCondition for PciBus {}
impl HasHardwareName for PciBus {}
impl HasControllerNumber for PciBus {}
impl HasDevicePartNumber for PciBus {}

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

impl HasChannelCondition for UsbBus {}
impl HasChannelIdentifying for UsbBus {}
impl HasDeviceId for UsbBus {}
impl HasHardwareName for UsbBus {}
impl HasControllerNumber for UsbBus {}
impl HasDevicePartNumber for UsbBus {}

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

impl HasChannelCondition for PccBus {}
impl HasHardwareName for PccBus {}
impl HasControllerNumber for PccBus {}
impl HasDevicePartNumber for PccBus {}

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

impl HasChannelCondition for LanBus {}
impl HasHardwareName for LanBus {}
impl HasControllerNumber for LanBus {}
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
