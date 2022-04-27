use crate::bus::Bus;
use crate::channel::Channel;
use crate::error::{PcanError, PcanOkError};
use crate::hw_ident::{
    ChannelCondition, ChannelConditionStatus, ControllerNumber, DevicePartNumber, HardwareName,
};
use crate::info::{ChannelFeatures, ChannelVersion, Version};
use crate::pcan;
use crate::special::{BitrateAdapting, SetBitrateAdapting};
use std::os::raw::c_void;

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

/* Hardware Identification */

impl ChannelCondition for IsaBus {
    fn channel_condition(&self) -> Result<ChannelConditionStatus, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                Bus::channel(self),
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

impl HardwareName for IsaBus {
    fn hardware_name(&self) -> Result<String, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_HARDWARE_NAME as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                Bus::channel(self),
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

impl ControllerNumber for IsaBus {
    fn controller_number(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                Bus::channel(self),
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

impl DevicePartNumber for IsaBus {
    fn device_part_number(&self) -> Result<String, PcanError> {
        let mut data = [0u8; 100];
        let code = unsafe {
            pcan::CAN_GetValue(
                Bus::channel(self),
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

/* Informational Parameters */

impl ChannelVersion for IsaBus {
    fn channel_version(&self) -> Result<Version, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_VERSION_STRING as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                Bus::channel(self),
                pcan::PCAN_CHANNEL_VERSION as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let newlines = s.lines().collect::<Vec<_>>();
                    if newlines.len() == 4 {
                        let newlines = newlines
                            .iter()
                            .map(|s| s.trim_matches(char::from(0)))
                            .collect::<Vec<_>>();

                        Ok(Version {
                            device_driver_name_and_version: String::from(newlines[0]),
                            architecture: String::from(newlines[1]),
                            year_of_copyright: String::from(newlines[2]),
                            company_name_and_city: String::from(newlines[4]),
                        })
                    } else {
                        Err(PcanError::Unknown)
                    }
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl ChannelFeatures for IsaBus {
    fn is_fd_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                Bus::channel(self),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_FD_CAPABLE == pcan::FEATURE_FD_CAPABLE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn is_delay_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                Bus::channel(self),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_DELAY_CAPABLE == pcan::FEATURE_DELAY_CAPABLE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn is_io_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                Bus::channel(self),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_IO_CAPABLE == pcan::FEATURE_IO_CAPABLE {
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

/* SPECIAL BEHAVIOR */
