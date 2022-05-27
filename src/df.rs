//!
//!
//!

use crate::channel::Channel;
use crate::error::{PcanError, PcanOkError};
use crate::pcan;
use std::ffi::c_void;

/* MessageFilter traits */

pub(crate) HasMessageFilter {}

pub trait MessageFilter {
    fn is_open_filter(&self) -> Result<bool, PcanError>;
    fn is_closed_filter(&self) -> Result<bool, PcanError>;
}

impl<T: HasMessageFilter + Channel> MessageFilter for T {
    fn is_open_filter(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_MESSAGE_FILTER as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                if u32::from_le_bytes(data) & pcan::PCAN_FILTER_OPEN == pcan::PCAN_FILTER_OPEN {
                    Ok(true)
                } else {
                    Ok(false)
                }
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn is_closed_filter(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
                pcan::PCAN_MESSAGE_FILTER as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                if u32::from_le_bytes(data) & pcan::PCAN_FILTER_CLOSE == pcan::PCAN_FILTER_CLOSE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub(crate) trait HasSetMessageFilter {}

pub trait SetMessageFilter {
    fn set_open_filter(&self) -> Result<(), PcanError>;
    fn set_closed_filter(&self) -> Result<(), PcanError>;
}

impl<T: HasSetMessageFilter + Channel> SetMessageFilter for T {
    fn set_open_filter(&self) -> Result<(), PcanError> {
        let mut data = pcan::PCAN_FILTER_OPEN.to_le_bytes();
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
                pcan::PCAN_MESSAGE_FILTER as u8,
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

    fn set_closed_filter(&self) -> Result<(), PcanError> {
        let mut data = pcan::PCAN_FILTER_CLOSE.to_le_bytes();
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
                pcan::PCAN_MESSAGE_FILTER as u8,
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
