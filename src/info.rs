//!
//!

use crate::channel::Channel;
use crate::error::{PcanError, PcanOkError};
use crate::pcan;
use std::ffi::c_void;

pub fn api_version() -> Result<String, PcanError> {
    let mut data = [0u8; pcan::MAX_LENGTH_VERSION_STRING as usize];
    let code = unsafe {
        pcan::CAN_GetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan_basic_sys::PCAN_API_VERSION as u8,
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

#[derive(Debug, PartialEq, Clone)]
pub struct Version {
    pub device_driver_name_and_version: String,
    pub year_of_copyright: String,
    pub company_name_and_city: String,
}

/* ChannelVersion trait */

pub(crate) trait HasChannelVersion {}

pub trait ChannelVersion {
    fn channel_version(&self) -> Result<Version, PcanError>;
}

impl<T: HasChannelVersion + Channel> ChannelVersion for T {
    fn channel_version(&self) -> Result<Version, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_VERSION_STRING as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_VERSION as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let newlines = s.lines().collect::<Vec<_>>();

                    if newlines.len() == 3 {
                        let newlines = newlines
                            .iter()
                            .map(|s| s.trim_matches(char::from(0)))
                            .collect::<Vec<_>>();

                        Ok(Version {
                            device_driver_name_and_version: String::from(newlines[0]),
                            year_of_copyright: String::from(newlines[1]),
                            company_name_and_city: String::from(newlines[2]),
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

/* ChannelFeatures trait */

pub(crate) trait HasChannelFeatures {}

pub trait ChannelFeatures {
    fn is_fd_capable(&self) -> Result<bool, PcanError>;
    fn is_delay_capable(&self) -> Result<bool, PcanError>;
    fn is_io_capable(&self) -> Result<bool, PcanError>;
}

impl<T: HasChannelFeatures + Channel> ChannelFeatures for T {
    fn is_fd_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
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
                self.channel(),
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
                self.channel(),
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

/* BitrateInfo trait */

pub(crate) trait HasBitrateInfo {}

pub trait BitrateInfo {
    fn bitrate_info(&self) -> Result<(u16, u16), PcanError>;
}

impl<T: HasBitrateInfo + Channel> BitrateInfo for T {
    fn bitrate_info(&self) -> Result<(u16, u16), PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_BITRATE_INFO as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let btr0 = u16::from_le_bytes([data[0], data[1]]);
                let btr1 = u16::from_le_bytes([data[2], data[3]]);
                Ok((btr0, btr1))
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* BitrateFdInfo trait */

pub(crate) trait HasBitrateInfoFd {}

pub trait BitrateInfoFd {
    fn bitrate_info_fd(&self) -> Result<String, PcanError>;
}

impl<T: HasBitrateInfoFd + Channel> BitrateInfoFd for T {
    fn bitrate_info_fd(&self) -> Result<String, PcanError> {
        let mut data = [0u8; 256];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_BITRATE_INFO_FD as u8,
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

/* NominalBusSpeed trait */

pub(crate) trait HasNominalBusSpeed {}

pub trait NominalBusSpeed {
    fn nominal_bus_speed(&self) -> Result<u32, PcanError>;
}

impl<T: HasNominalBusSpeed + Channel> NominalBusSpeed for T {
    fn nominal_bus_speed(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_BUSSPEED_NOMINAL as u8,
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

/* DataBusSpeed trait */

pub(crate) trait HasDataBusSpeed {}

pub trait DataBusSpeed {
    fn data_bus_speed(&self) -> Result<u32, PcanError>;
}

impl<T: HasDataBusSpeed + Channel> DataBusSpeed for T {
    fn data_bus_speed(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_BUSSPEED_DATA as u8,
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

pub fn lan_service_is_running() -> Result<bool, PcanError> {
    let mut data = [0u8; 4];
    let code = unsafe {
        pcan::CAN_GetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan::PCAN_LAN_SERVICE_STATUS as u8,
            data.as_mut_ptr() as *mut c_void,
            data.len() as u32,
        )
    };

    match PcanOkError::try_from(code) {
        Ok(PcanOkError::Ok) => {
            let code = u32::from_le_bytes(data);
            if code & pcan::SERVICE_STATUS_RUNNING == pcan::SERVICE_STATUS_RUNNING {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Ok(PcanOkError::Err(err)) => Err(err),
        Err(_) => Err(PcanError::Unknown),
    }
}

pub fn lan_service_is_stopped() -> Result<bool, PcanError> {
    let mut data = [0u8; 4];
    let code = unsafe {
        pcan::CAN_GetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan::PCAN_LAN_SERVICE_STATUS as u8,
            data.as_mut_ptr() as *mut c_void,
            data.len() as u32,
        )
    };

    match PcanOkError::try_from(code) {
        Ok(PcanOkError::Ok) => {
            let code = u32::from_le_bytes(data);
            if code & pcan::SERVICE_STATUS_STOPPED == pcan::SERVICE_STATUS_STOPPED {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Ok(PcanOkError::Err(err)) => Err(err),
        Err(_) => Err(PcanError::Unknown),
    }
}

/* FirmwareVersion trait */

pub(crate) trait HasFirmwareVersion {}

pub trait FirmwareVersion {
    fn firmware_version(&self) -> Result<String, PcanError>;
}

// pub fn attached_channel_count() -> Result<u32, PcanError>
