//! Data flow specific CAN-bus configurations.
//!
//!

use crate::channel::Channel;
use crate::error::{PcanError, PcanOkError};
use crate::pcan;
use std::ffi::c_void;

/* MessageFilter traits */

pub(crate) trait HasMessageFilter {}

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
                if u32::from_le_bytes(data) == pcan::PCAN_FILTER_OPEN {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn is_closed_filter(&self) -> Result<bool, PcanError> {
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
                if u32::from_le_bytes(data) == pcan::PCAN_FILTER_CLOSE {
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

/* ReceiveStatus traits */

pub(crate) trait HasReceiveStatus {}

pub trait ReceiveStatus {
    fn is_receiving(&self) -> Result<bool, PcanError>;
}

impl<T: HasReceiveStatus + Channel> ReceiveStatus for T {
    fn is_receiving(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_RECEIVE_STATUS as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let code = u32::from_le_bytes(data);
                if code == pcan::PCAN_PARAMETER_ON {
                    Ok(true)
                } else if code == pcan::PCAN_PARAMETER_OFF {
                    Ok(false)
                } else {
                    Err(PcanError::Unknown)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub(crate) trait HasSetReceiveStatus {}

pub trait SetReceiveStatus {
    fn set_receiving(&self, status: bool) -> Result<(), PcanError>;
}

impl<T: HasSetReceiveStatus + Channel> SetReceiveStatus for T {
    fn set_receiving(&self, status: bool) -> Result<(), PcanError> {
        let mut data = match status {
            true => pcan::PCAN_PARAMETER_ON.to_le_bytes(),
            false => pcan::PCAN_PARAMETER_OFF.to_le_bytes(),
        };
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
                pcan::PCAN_RECEIVE_STATUS as u8,
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

/* ALLOW STATUS FRAMES traits*/

pub(crate) trait HasAllowStatusFrames {}

pub trait AllowStatusFrames {
    fn allows_status_frames(&self) -> Result<bool, PcanError>;
}

impl<T: HasAllowStatusFrames + Channel> AllowStatusFrames for T {
    fn allows_status_frames(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_ALLOW_STATUS_FRAMES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let code = u32::from_le_bytes(data);
                if code == pcan::PCAN_PARAMETER_ON {
                    Ok(true)
                } else if code == pcan::PCAN_PARAMETER_OFF {
                    Ok(false)
                } else {
                    Err(PcanError::Unknown)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub(crate) trait HasSetAllowStatusFrames {}

pub trait SetAllowStatusFrames {
    fn allow_status_frames(&self, enable: bool) -> Result<(), PcanError>;
}

impl<T: HasSetAllowStatusFrames + Channel> SetAllowStatusFrames for T {
    fn allow_status_frames(&self, enable: bool) -> Result<(), PcanError> {
        let mut data = match enable {
            true => pcan::PCAN_PARAMETER_ON.to_le_bytes(),
            false => pcan::PCAN_PARAMETER_OFF.to_le_bytes(),
        };
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
                pcan::PCAN_ALLOW_STATUS_FRAMES as u8,
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
