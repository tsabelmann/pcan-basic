use crate::bus::{DngBus, IsaBus, LanBus, PccBus, PciBus, UsbBus};
use crate::channel::{InitializedChannel, UninitializedChannel};
use crate::error::{PcanError, PcanOkError};
use crate::pcan;
use std::net::Ipv4Addr;
use std::os::raw::c_void;

#[derive(Debug, PartialEq)]
pub enum ChannelConditionStatus {
    Unavailable,
    Available,
    Occupied,
    PcanView,
}

impl From<ChannelConditionStatus> for u32 {
    fn from(value: ChannelConditionStatus) -> Self {
        match value {
            ChannelConditionStatus::Unavailable => pcan::PCAN_CHANNEL_UNAVAILABLE,
            ChannelConditionStatus::Available => pcan::PCAN_CHANNEL_AVAILABLE,
            ChannelConditionStatus::Occupied => pcan::PCAN_CHANNEL_OCCUPIED,
            ChannelConditionStatus::PcanView => pcan::PCAN_CHANNEL_PCANVIEW,
        }
    }
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

impl<T: HasChannelCondition + UninitializedChannel> ChannelCondition for T {
    fn channel_condition(&self) -> Result<ChannelConditionStatus, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.uninitialized_channel(),
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
    fn is_identifying(&self) -> Result<bool, PcanError>;
}

impl<T: HasChannelIdentifying + UninitializedChannel> ChannelIdentifying for T {
    fn enable_identifying(&self) -> Result<(), PcanError> {
        let mut data = u32::to_le_bytes(pcan::PCAN_PARAMETER_ON);
        let code = unsafe {
            pcan::CAN_SetValue(
                self.uninitialized_channel(),
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
                self.uninitialized_channel(),
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

    fn is_identifying(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.uninitialized_channel(),
                pcan::PCAN_CHANNEL_IDENTIFYING as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::PCAN_PARAMETER_ON {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl<T: HasChannelIdentifying + InitializedChannel> ChannelIdentifying for T {
    fn enable_identifying(&self) -> Result<(), PcanError> {
        let mut data = u32::to_le_bytes(pcan::PCAN_PARAMETER_ON);
        let code = unsafe {
            pcan::CAN_SetValue(
                self.initialized_channel(),
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
                self.initialized_channel(),
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

    fn is_identifying(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.initialized_channel(),
                pcan::PCAN_CHANNEL_IDENTIFYING as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::PCAN_PARAMETER_ON {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
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

trait HasSetDeviceId {}

pub trait SetDeviceId {
    type Item;
    fn set_device_id(&self, Self::Item) -> Result<(), PcanError>;
}

impl<T: HasDeviceId + UninitializedChannel> DeviceId for T {
    fn device_id(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.uninitialized_channel(),
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

impl<T: HasDeviceId + InitializedChannel> DeviceId for T {
    fn device_id(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.initialized_channel(),
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

impl<T: HasSetDeviceId + InitializedChannel> SetDeviceId for T {
    type Item = u32;
    fn set_device_id(&self, value: Self::Item) -> Result<(), PcanError> {
        let mut data = value.to_le_bytes();
        let code = unsafe {
            pcan::CAN_SetValue(
                self.initialized_channel(),
                pcan::PCAN_DEVICE_ID as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(()),
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

impl<T: HasHardwareName + UninitializedChannel> HardwareName for T {
    fn hardware_name(&self) -> Result<String, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_HARDWARE_NAME as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.uninitialized_channel(),
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

impl<T: HasHardwareName + InitializedChannel> HardwareName for T {
    fn hardware_name(&self) -> Result<String, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_HARDWARE_NAME as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.initialized_channel(),
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

trait HasSetControllerNumber {}

pub trait SetControllerNumber {
    type Item;
    fn set_controller_number(&self, value: Self::Item) -> Result<(), PcanError>;
}

impl<T: HasControllerNumber + UninitializedChannel> ControllerNumber for T {
    fn controller_number(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.uninitialized_channel(),
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

impl<T: HasControllerNumber + InitializedChannel> ControllerNumber for T {
    fn controller_number(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.initialized_channel(),
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


impl<T: HasSetControllerNumber + InitializedChannel> SetControllerNumber for T {
    type Item = u32;
    fn set_controller_number(&self, value: Self::Item) -> Result<(), PcanError> {
        let mut data = value.to_le_bytes();
        let code = unsafe {
            pcan::CAN_SetValue(
                self.initialized_chanwnel(),
                pcan::PCAN_CONTROLLER_NUMBER as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(()),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

trait HasIpAddress {}

trait IpAddress {
    fn ip_address(&self) -> Result<Ipv4Addr, PcanError>;
}

impl<T: HasIpAddress + InitializedChannel> IpAddress for T {
    fn ip_address(&self) -> Result<Ipv4Addr, PcanError> {
        let mut data = [0u8; 20];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.handle,
                pcan::PCAN_IP_ADDRESS as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let s = s.trim_matches(char::from(0));
                    match s.parse() {
                        Ok(ip) => Ok(ip),
                        Err(_) => Err(PcanError::Unknown),
                    }
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            _ => Err(PcanError::Unknown),
        }
    }
}

/* Device Part Number */

trait HasDevicePartNumber {}

pub trait DevicePartNumber {
    fn device_part_number(&self) -> Result<String, PcanError>;
}

impl<T: HasDevicePartNumber + UninitializedChannel> DevicePartNumber for T {
    fn device_part_number(&self) -> Result<String, PcanError> {
        let mut data = [0u8; 100];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.uninitialized_channel(),
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
