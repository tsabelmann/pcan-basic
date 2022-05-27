//! Data flow specific CAN-bus configurations.
//!
//!

use crate::channel::Channel;
use crate::error::{PcanError, PcanOkError};
use crate::pcan;
use std::ffi::c_void;
use std::ops::BitXor;

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

/* ALLOW STATUS FRAMES traits */

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

/* ALLOW RTS FRAMES traits */

pub(crate) trait HasAllowRTRFrames {}

pub trait AllowRTRFrames {
    fn allows_rtr_frames(&self) -> Result<bool, PcanError>;
}

impl<T: HasAllowRTRFrames + Channel> AllowRTRFrames for T {
    fn allows_rtr_frames(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_ALLOW_RTR_FRAMES as u8,
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

pub(crate) trait HasSetAllowRTRFrames {}

pub trait SetAllowRTRFrames {
    fn allow_rtr_frames(&self, enable: bool) -> Result<(), PcanError>;
}

impl<T: HasSetAllowRTRFrames + Channel> SetAllowRTRFrames for T {
    fn allow_rtr_frames(&self, enable: bool) -> Result<(), PcanError> {
        let mut data = match enable {
            true => pcan::PCAN_PARAMETER_ON.to_le_bytes(),
            false => pcan::PCAN_PARAMETER_OFF.to_le_bytes(),
        };
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
                pcan::PCAN_ALLOW_RTR_FRAMES as u8,
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

/* ALLOW ERROR FRAMES traits */

pub(crate) trait HasAllowErrorFrames {}

pub trait AllowErrorFrames {
    fn allows_error_frames(&self) -> Result<bool, PcanError>;
}

impl<T: HasAllowErrorFrames + Channel> AllowErrorFrames for T {
    fn allows_error_frames(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_ALLOW_ERROR_FRAMES as u8,
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

pub(crate) trait HasSetAllowErrorFrames {}

pub trait SetAllowErrorFrames {
    fn allow_error_frames(&self, enable: bool) -> Result<(), PcanError>;
}

impl<T: HasSetAllowErrorFrames + Channel> SetAllowErrorFrames for T {
    fn allow_error_frames(&self, enable: bool) -> Result<(), PcanError> {
        let mut data = match enable {
            true => pcan::PCAN_PARAMETER_ON.to_le_bytes(),
            false => pcan::PCAN_PARAMETER_OFF.to_le_bytes(),
        };
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
                pcan::PCAN_ALLOW_ERROR_FRAMES as u8,
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

/* ALLOW ECHO FRAMES traits */

pub(crate) trait HasAllowEchoFrames {}

pub trait AllowEchoFrames {
    fn allows_echo_frames(&self) -> Result<bool, PcanError>;
}

impl<T: HasAllowEchoFrames + Channel> AllowEchoFrames for T {
    fn allows_echo_frames(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_ALLOW_ECHO_FRAMES as u8,
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

pub(crate) trait HasSetAllowEchoFrames {}

pub trait SetAllowEchoFrames {
    fn allow_echo_frames(&self, enable: bool) -> Result<(), PcanError>;
}

impl<T: HasSetAllowEchoFrames + Channel> SetAllowEchoFrames for T {
    fn allow_echo_frames(&self, enable: bool) -> Result<(), PcanError> {
        let mut data = match enable {
            true => pcan::PCAN_PARAMETER_ON.to_le_bytes(),
            false => pcan::PCAN_PARAMETER_OFF.to_le_bytes(),
        };
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
                pcan::PCAN_ALLOW_ECHO_FRAMES as u8,
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

/* ACCEPTANCE FILTER 11Bit traits */

pub(crate) trait HasAcceptanceFilter11Bit {}

pub trait AcceptanceFilter11Bit {
    fn acceptance_filter_11bit(&self) -> Result<(u32, u32), PcanError>;
}

impl<T: HasAcceptanceFilter11Bit + Channel> AcceptanceFilter11Bit for T {
    fn acceptance_filter_11bit(&self) -> Result<(u32, u32), PcanError> {
        let mut data = [0u8; 8];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_ACCEPTANCE_FILTER_11BIT as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let acceptance_mask = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
                let acceptance_code = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
                Ok((acceptance_mask, acceptance_code))
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub(crate) trait HasSetAcceptanceFilter11Bit {}

pub trait SetAcceptanceFilter11Bit {
    fn set_acceptance_filter_11bit(&self, ids: &[u32]) -> Result<(), PcanError>;
}

impl<T: HasSetAcceptanceFilter11Bit + Channel> SetAcceptanceFilter11Bit for T {
    fn set_acceptance_filter_11bit(&self, ids: &[u32]) -> Result<(), PcanError> {
        let acceptance_code = ids
            .iter()
            .map(|x| *x & 0x7_FFu32)
            .fold(0xFF_FF_FF_FFu32, |x, y| x & y);
        let acceptance_code_data = acceptance_code.to_le_bytes();

        let acceptance_mask = ids.iter().map(|x| *x & 0x7_FFu32).fold(0u32, |x, y| x ^ y);
        let acceptance_mask_data = acceptance_mask.to_le_bytes();

        let mut data = [
            acceptance_mask_data[0],
            acceptance_mask_data[1],
            acceptance_mask_data[2],
            acceptance_mask_data[3],
            acceptance_code_data[0],
            acceptance_code_data[1],
            acceptance_code_data[2],
            acceptance_code_data[3],
        ];
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
                pcan::PCAN_ACCEPTANCE_FILTER_11BIT as u8,
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

/* ACCEPTANCE FILTER 29Bit traits */

pub(crate) trait HasAcceptanceFilter29Bit {}

pub trait AcceptanceFilter29Bit {
    fn acceptance_filter_29bit(&self) -> Result<(u32, u32), PcanError>;
}

impl<T: HasAcceptanceFilter29Bit + Channel> AcceptanceFilter29Bit for T {
    fn acceptance_filter_29bit(&self) -> Result<(u32, u32), PcanError> {
        let mut data = [0u8; 8];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_ACCEPTANCE_FILTER_29BIT as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let acceptance_mask = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
                let acceptance_code = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
                Ok((acceptance_mask, acceptance_code))
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub(crate) trait HasSetAcceptanceFilter29Bit {}

pub trait SetAcceptanceFilter29Bit {
    fn set_acceptance_filter_29bit(&self, ids: &[u32]) -> Result<(), PcanError>;
}

impl<T: HasSetAcceptanceFilter29Bit + Channel> SetAcceptanceFilter29Bit for T {
    fn set_acceptance_filter_29bit(&self, ids: &[u32]) -> Result<(), PcanError> {
        let acceptance_code = ids
            .iter()
            .map(|x| *x & 0x1F_FF_FF_FFu32)
            .fold(0xFF_FF_FF_FFu32, |x, y| x & y);
        let acceptance_code_data = acceptance_code.to_le_bytes();

        let acceptance_mask = ids
            .iter()
            .map(|x| *x & 0x1F_FF_FF_FFu32)
            .fold(0u32, |x, y| x ^ y);
        let acceptance_mask_data = acceptance_mask.to_le_bytes();

        let mut data = [
            acceptance_mask_data[0],
            acceptance_mask_data[1],
            acceptance_mask_data[2],
            acceptance_mask_data[3],
            acceptance_code_data[0],
            acceptance_code_data[1],
            acceptance_code_data[2],
            acceptance_code_data[3],
        ];
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
                pcan::PCAN_ACCEPTANCE_FILTER_29BIT as u8,
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
