use crate::error::{PcanError, PcanOkError};
use crate::pcan;
use std::ffi::c_void;
use std::path::{Path, PathBuf};

pub fn log_location() -> Result<PathBuf, PcanError> {
    let mut data = [0u8; pcan::MAX_LENGTH_VERSION_STRING as usize];
    let code = unsafe {
        pcan::CAN_GetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan_basic_sys::PCAN_LOG_LOCATION as u8,
            data.as_mut_ptr() as *mut c_void,
            data.len() as u32,
        )
    };

    match PcanOkError::try_from(code) {
        Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
            Ok(s) => {
                let s = s.trim_matches(char::from(0));
                Ok(PathBuf::from(s))
            }
            Err(_) => Err(PcanError::Unknown),
        },
        Ok(PcanOkError::Err(err)) => Err(err),
        Err(_) => Err(PcanError::Unknown),
    }
}

pub fn set_log_location<P: AsRef<Path>>(path: P) -> Result<(), PcanError> {
    let mut data = match path.as_ref().to_str() {
        None => {
            return Err(PcanError::Unknown);
        }
        Some(s) => String::from(s),
    };
    let code = unsafe {
        pcan::CAN_SetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan_basic_sys::PCAN_LOG_LOCATION as u8,
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

pub fn set_default_log_location() -> Result<(), PcanError> {
    set_log_location("")
}

pub fn is_logging() -> Result<bool, PcanError> {
    let mut data = [0u8; 4];
    let code = unsafe {
        pcan::CAN_GetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan_basic_sys::PCAN_LOG_STATUS as u8,
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

pub fn set_logging(enable: bool) -> Result<(), PcanError> {
    let mut data = match enable {
        true => pcan::PCAN_PARAMETER_ON.to_le_bytes(),
        false => pcan::PCAN_PARAMETER_OFF.to_le_bytes(),
    };
    let code = unsafe {
        pcan::CAN_SetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan_basic_sys::PCAN_LOG_STATUS as u8,
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

///
///
#[derive(PartialEq, Debug)]
pub enum LogFunction {
    /// Default
    Default,
    ///
    Entry,
    ///
    Parameters,
    ///
    Leave,
    ///
    Write,
    ///
    Read,
}

impl TryFrom<u32> for LogFunction {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            pcan::LOG_FUNCTION_DEFAULT => Ok(LogFunction::Default),
            pcan::LOG_FUNCTION_ENTRY => Ok(LogFunction::Entry),
            pcan::LOG_FUNCTION_PARAMETERS => Ok(LogFunction::Parameters),
            pcan::LOG_FUNCTION_LEAVE => Ok(LogFunction::Leave),
            pcan::LOG_FUNCTION_WRITE => Ok(LogFunction::Write),
            pcan::LOG_FUNCTION_READ => Ok(LogFunction::Read),
            _ => Err(()),
        }
    }
}

impl From<LogFunction> for u32 {
    fn from(value: LogFunction) -> Self {
        match value {
            LogFunction::Default => pcan::LOG_FUNCTION_DEFAULT,
            LogFunction::Entry => pcan::LOG_FUNCTION_ENTRY,
            LogFunction::Parameters => pcan::LOG_FUNCTION_PARAMETERS,
            LogFunction::Leave => pcan::LOG_FUNCTION_LEAVE,
            LogFunction::Write => pcan::LOG_FUNCTION_WRITE,
            LogFunction::Read => pcan::LOG_FUNCTION_READ,
        }
    }
}

pub fn log_configuration() -> Result<LogFunction, PcanError> {
    let mut data = [0u8; 4];
    let code = unsafe {
        pcan::CAN_GetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan_basic_sys::PCAN_LOG_CONFIGURE as u8,
            data.as_mut_ptr() as *mut c_void,
            data.len() as u32,
        )
    };

    match PcanOkError::try_from(code) {
        Ok(PcanOkError::Ok) => {
            let code = u32::from_le_bytes(data);
            match LogFunction::try_from(code) {
                Ok(log_config) => Ok(log_config),
                Err(_) => Err(PcanError::Unknown),
            }
        }
        Ok(PcanOkError::Err(err)) => Err(err),
        Err(_) => Err(PcanError::Unknown),
    }
}

pub fn configure_log(config: LogFunction) -> Result<(), PcanError> {
    let mut data = u32::from(config).to_le_bytes();
    let code = unsafe {
        pcan::CAN_SetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan_basic_sys::PCAN_LOG_CONFIGURE as u8,
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

pub fn log_text<S: AsRef<str>>(text: S) -> Result<(), PcanError> {
    let mut data = String::from(text.as_ref());
    let code = unsafe {
        pcan::CAN_SetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan_basic_sys::PCAN_LOG_TEXT as u8,
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
