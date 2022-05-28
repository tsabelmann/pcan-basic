//!
//!
//!
//!

/* IO DIGITAL CONFIGURATION trait */

use crate::channel::Channel;
use crate::error::{PcanError, PcanOkError};
use crate::pcan;
use std::ffi::c_void;

#[derive(PartialEq, Debug)]
pub enum IOConfig {
    In,
    InOut,
}

impl TryFrom<u32> for IOConfig {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(IOConfig::In),
            1 => Ok(IOConfig::InOut),
            _ => Err(()),
        }
    }
}

impl From<IOConfig> for u32 {
    fn from(value: IOConfig) -> Self {
        match value {
            IOConfig::In => 0,
            IOConfig::InOut => 1,
        }
    }
}

pub(crate) trait HasDigitalConfiguration {}

pub trait DigitalConfiguration {
    fn mode(&self, pin: u8) -> Result<IOConfig, PcanError>;
    fn mode_word(&self) -> Result<u32, PcanError>;
}

impl<T: HasDigitalConfiguration + Channel> DigitalConfiguration for T {
    fn mode(&self, pin: u8) -> Result<IOConfig, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_IO_DIGITAL_CONFIGURATION as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let mode_word = u32::from_le_bytes(data);
                let pin_enabled = mode_word & (1 << pin);

                if pin_enabled == 0 {
                    Ok(IOConfig::In)
                } else {
                    Ok(IOConfig::InOut)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn mode_word(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_IO_DIGITAL_CONFIGURATION as u8,
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

pub(crate) trait HasSetDigitalConfiguration {}

pub trait SetDigitalConfiguration {
    fn set_mode(&self, pin: u8, mode: IOConfig) -> Result<(), PcanError>;
    fn set_mode_word(&self, mode_word: u32) -> Result<(), PcanError>;
}

impl<T: HasSetDigitalConfiguration + Channel> SetDigitalConfiguration for T {
    fn set_mode(&self, pin: u8, mode: IOConfig) -> Result<(), PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_IO_DIGITAL_CONFIGURATION as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        let mode_word = match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => u32::from_le_bytes(data),
            Ok(PcanOkError::Err(err)) => return Err(err),
            Err(_) => return Err(PcanError::Unknown),
        };

        let mode_word = match mode {
            IOConfig::In => mode_word | !(1 << pin),
            IOConfig::InOut => mode_word | (1 << pin),
        };
        let mut data = mode_word.to_le_bytes();

        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
                pcan::PCAN_IO_DIGITAL_CONFIGURATION as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        return match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(()),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        };
    }

    fn set_mode_word(&self, mode_word: u32) -> Result<(), PcanError> {
        let mut data = mode_word.to_le_bytes();
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
                pcan::PCAN_IO_DIGITAL_CONFIGURATION as u8,
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

/* IO DIGITAL VALUE */

#[derive(PartialEq, Debug)]
pub enum IOValue {
    Low,
    High,
}

impl TryFrom<u32> for IOValue {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(IOValue::Low),
            1 => Ok(IOValue::High),
            _ => Err(()),
        }
    }
}

impl From<IOValue> for u32 {
    fn from(value: IOValue) -> Self {
        match value {
            IOValue::Low => 0,
            IOValue::High => 1,
        }
    }
}

pub(crate) trait HasDigitalValue {}

pub trait DigitalValue {
    fn value(&self, pin: u8) -> Result<IOValue, PcanError>;
    fn value_word(&self) -> Result<u32, PcanError>;
}

impl<T: HasSetDigitalValue + Channel> DigitalValue for T {
    fn value(&self, pin: u8) -> Result<IOValue, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_IO_DIGITAL_VALUE as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let mode_word = u32::from_le_bytes(data);
                let pin_enabled = mode_word & (1 << pin);

                if pin_enabled == 0 {
                    Ok(IOValue::Low)
                } else {
                    Ok(IOValue::High)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn value_word(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_IO_DIGITAL_VALUE as u8,
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

pub(crate) trait HasSetDigitalValue {}

pub trait SetDigitalValue {
    fn set_value(&self, pin: u8, value: IOValue) -> Result<(), PcanError>;
    fn set_value_word(&self, value_word: u32) -> Result<(), PcanError>;
}

impl<T: HasSetDigitalValue + Channel> SetDigitalValue for T {
    fn set_value(&self, pin: u8, value: IOValue) -> Result<(), PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_IO_DIGITAL_CONFIGURATION as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        let mode_word = match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => u32::from_le_bytes(data),
            Ok(PcanOkError::Err(err)) => return Err(err),
            Err(_) => return Err(PcanError::Unknown),
        };

        let mode_word = match value {
            IOValue::Low => mode_word | !(1 << pin),
            IOValue::High => mode_word | (1 << pin),
        };
        let mut data = mode_word.to_le_bytes();

        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
                pcan::PCAN_IO_DIGITAL_VALUE as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        return match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(()),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        };
    }

    fn set_value_word(&self, value_word: u32) -> Result<(), PcanError> {
        let mut data = value_word.to_le_bytes();
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
                pcan::PCAN_IO_DIGITAL_VALUE as u8,
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
