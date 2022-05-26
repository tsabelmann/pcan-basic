//!
//!
//!

use crate::channel::Channel;
use crate::bus::Bus;
use crate::error::{PcanError, PcanOkError};
use crate::pcan;
use std::ffi::c_void;
use std::net::Ipv4Addr;

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

/* ChannelCondition trait */

pub(crate) trait HasChannelCondition {}

pub trait ChannelCondition {
    fn channel_condition(&self) -> Result<ChannelConditionStatus, PcanError>;
}

impl<T: HasChannelCondition + Channel> ChannelCondition for T {
    fn channel_condition(&self) -> Result<ChannelConditionStatus, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
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

/* ChannelIdentifying trait */

pub(crate) trait HasChannelIdentifying {}

pub trait ChannelIdentifying {
    fn set_channel_identifying(&self, value: bool) -> Result<(), PcanError>;
    fn is_channel_identifying(&self) -> Result<bool, PcanError>;
}

impl<T: HasChannelIdentifying + Channel> ChannelIdentifying for T {
    fn set_channel_identifying(&self, value: bool) -> Result<(), PcanError> {
        let mut data = match value {
            true => pcan::PCAN_PARAMETER_ON.to_le_bytes(),
            false => pcan::PCAN_PARAMETER_OFF.to_le_bytes(),
        };

        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_IDENTIFYING as u8,
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

    fn is_channel_identifying(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_IDENTIFYING as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::PCAN_PARAMETER_ON == pcan::PCAN_PARAMETER_ON {
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

pub(crate) trait HasDeviceId {}

pub trait DeviceId {
    fn device_id(&self) -> Result<u32, PcanError>;
}

impl<T: HasDeviceId + Channel> DeviceId for T {
    fn device_id(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_DEVICE_ID as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(u32::from_le_bytes(data)),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub(crate) trait HasSetDeviceId {}

pub trait SetDeviceId {
    type Item;
    fn set_device_id(&self, value: Self::Item) -> Result<(), PcanError>;
}

impl<T: HasSetDeviceId + Channel> SetDeviceId for T {
    type Item = u32;
    fn set_device_id(&self, value: Self::Item) -> Result<(), PcanError> {
        let mut data = value.to_le_bytes();
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
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

pub(crate) trait HasHardwareName {}

pub trait HardwareName {
    fn hardware_name(&self) -> Result<String, PcanError>;
}

impl<T: HasHardwareName + Channel> HardwareName for T {
    fn hardware_name(&self) -> Result<String, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_HARDWARE_NAME as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
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

pub(crate) trait HasControllerNumber {}

pub trait ControllerNumber {
    fn controller_number(&self) -> Result<u32, PcanError>;
}

impl<T: HasControllerNumber + Channel> ControllerNumber for T {
    fn controller_number(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CONTROLLER_NUMBER as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(u32::from_le_bytes(data)),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub(crate) trait HasSetControllerNumber {}

pub trait SetControllerNumber {
    type Item;
    fn set_controller_number(&self, value: Self::Item) -> Result<(), PcanError>;
}

impl<T: HasSetControllerNumber + Channel> SetControllerNumber for T {
    type Item = u32;
    fn set_controller_number(&self, value: Self::Item) -> Result<(), PcanError> {
        let mut data = value.to_le_bytes();
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
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

/* IpAddress trait */

pub(crate) trait HasIpAddress {}

pub trait IpAddress {
    fn ip_address(&self) -> Result<Ipv4Addr, PcanError>;
}

impl<T: HasIpAddress + Channel> IpAddress for T {
    fn ip_address(&self) -> Result<Ipv4Addr, PcanError> {
        let mut data = [0u8; 20];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
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

/* ATTACHED CHANNEL COUNT */

pub fn attached_channels_count() -> Result<u32, PcanError> {
    let mut data = [0u8; 4];
    let code = unsafe {
        pcan::CAN_GetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan::PCAN_ATTACHED_CHANNELS_COUNT as u8,
            data.as_mut_ptr() as *mut c_void,
            data.len() as u32,
        )
    };

    match PcanOkError::try_from(code) {
        Ok(PcanOkError::Ok) => Ok(u32::from_le_bytes(data)),
        Ok(PcanOkError::Err(err)) => Err(err),
        Err(_) => Err(PcanError::Unknown),
    }
}

/* DevicePartNumber trait */

pub(crate) trait HasDevicePartNumber {}

pub trait DevicePartNumber {
    fn device_part_number(&self) -> Result<String, PcanError>;
}

impl<T: HasDevicePartNumber + Channel> DevicePartNumber for T {
    fn device_part_number(&self) -> Result<String, PcanError> {
        let mut data = [0u8; 100];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
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
