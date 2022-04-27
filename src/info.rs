//!
//!

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
    pub architecture: String,
    pub year_of_copyright: String,
    pub company_name_and_city: String,
}

/* ChannelVersion trait */

pub(crate) trait HasChannelVersion {}

pub trait ChannelVersion {
    fn channel_version(&self) -> Result<Version, PcanError>;
}

/* ChannelFeatures trait */

pub(crate) trait HasChannelFeatures {}

pub trait ChannelFeatures {
    fn is_fd_capable(&self) -> Result<bool, PcanError>;
    fn is_delay_capable(&self) -> Result<bool, PcanError>;
    fn is_io_capable(&self) -> Result<bool, PcanError>;
}

/* BitrateInfo trait */

pub(crate) trait HasBitrateInfo {}

pub trait BitrateInfo {
    fn bitrate_info(&self) -> Result<u16, PcanError>;
}

/* BitrateFdInfo trait */

pub(crate) trait HasBitrateFdInfo {}

pub trait BitrateFdInfo {
    fn bitrate_fd_info(&self) -> Result<u16, PcanError>;
}

/* NominalBusSpeed trait */

pub(crate) trait HasNominalBusSpeed {}

pub trait NominalBusSpeed {
    fn nominal_bus_speed(&self) -> Result<u32, PcanError>;
}

/* DataBusSpeed trait */

pub(crate) trait HasDataBusSpeed {}

pub trait DataBusSpeed {
    fn data_bus_speed(&self) -> Result<u32, PcanError>;
}

// pub fn lan_service_running() -> Result<String, PcanError> {
//     let mut data = [0u8; pcan::MAX_LENGTH_VERSION_STRING as usize];
//     let code = unsafe {
//         pcan::CAN_GetValue(
//             pcan::PCAN_NONEBUS as u16,
//             pcan_basic_sys::PCAN_API_VERSION as u8,
//             data.as_mut_ptr() as *mut c_void,
//             data.len() as u32,
//         )
//     };
//
//     match PcanOkError::try_from(code) {
//         Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
//             Ok(s) => {
//                 let s = s.trim_matches(char::from(0));
//                 Ok(String::from(s))
//             }
//             Err(_) => Err(PcanError::Unknown),
//         },
//         Ok(PcanOkError::Err(err)) => Err(err),
//         Err(_) => Err(PcanError::Unknown),
//     }
// }

// pub fn lan_service_not_running() -> Result<bool, PcanError>

/* FirmwareVersion trait */

pub(crate) trait HasFirmwareVersion {}

pub trait FirmwareVersion {
    fn firmware_version(&self) -> Result<String, PcanError>;
}

// pub fn attached_channel_count() -> Result<u32, PcanError>
