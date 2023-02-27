//! Module providing a convenient interface mentioned in the 'Using Logging Parameters' chapter.

use crate::error::{PcanError, PcanOkError};
use crate::pcan;
use std::ffi::c_void;
use std::path::{Path, PathBuf};
use std::ptr::null_mut;

/* LOG LOCATION functions */

/// Retrieves the log location path.
///
/// Tries to retrieve the log location path. If the function succeeds,
/// a [PathBuf](PathBuf) is returned. Otherwise, it returns a
/// [PcanError](PcanError).
///
/// # Example
/// ```
/// use pcan_basic::log::log_location;
/// match log_location() {
///     Ok(path) => println!("The path: {:?}", path),
///     Err(error) => println!("The error: {:?}", error)
/// }
/// ```
pub fn log_location() -> Result<PathBuf, PcanError> {
    let mut data = [0u8; pcan::MAX_LENGTH_VERSION_STRING as usize];
    let code = unsafe {
        pcan::CAN_GetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan::PCAN_LOG_LOCATION as u8,
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

/// Sets the log location path.
///
/// Tries to set the log location path. If the function succeeds, it returns the empty type.
/// Otherwise, it returns a [PcanError](PcanError).
///
/// # Example
/// ```
/// use pcan_basic::log::set_log_location;
/// match set_log_location("/home/pi") {
///     Ok(_) => println!("Successful!"),
///     Err(error) => println!("The error: {:?}", error)
/// }
/// ```
pub fn set_log_location<P: AsRef<Path>>(path: P) -> Result<(), PcanError> {
    let mut data = match path.as_ref().to_str() {
        None => {
            return Err(PcanError::Unknown);
        }
        Some(s) => String::from(s),
    };

    let code = if data.len() == 0 {
        unsafe {
            pcan::CAN_SetValue(
                pcan::PCAN_NONEBUS as u16,
                pcan::PCAN_LOG_LOCATION as u8,
                null_mut() as *mut c_void,
                0 as u32,
            )
        }
    } else {
        unsafe {
            pcan::CAN_SetValue(
                pcan::PCAN_NONEBUS as u16,
                pcan::PCAN_LOG_LOCATION as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        }
    };

    match PcanOkError::try_from(code) {
        Ok(PcanOkError::Ok) => Ok(()),
        Ok(PcanOkError::Err(err)) => Err(err),
        Err(_) => Err(PcanError::Unknown),
    }
}

/// Resets the log location path to the default.
///
/// Tries to reset the log location path to the default. If the function succeeds, it returns the
/// empty type. Otherwise, it returns a [PcanError](PcanError).
///
/// # Example
/// ```
/// use pcan_basic::log::set_default_log_location;
/// match set_default_log_location() {
///     Ok(_) => println!("Successful!"),
///     Err(error) => println!("The error: {:?}", error)
/// }
/// ```
pub fn set_default_log_location() -> Result<(), PcanError> {
    set_log_location("")
}

/* LOG STATUS functions */

/// Retrieves the current logging status.
///
/// If the function succeeds, it returns either `true` or `false`. Otherwise, it returns a
/// [PcanError](PcanError).
///
/// # Example
/// ```
/// use pcan_basic::log::is_logging;
/// match is_logging() {
///     Ok(status) => println!("Is logging: {}", status),
///     Err(error) => println!("The error: {:?}", error)
/// }
/// ```
pub fn is_logging() -> Result<bool, PcanError> {
    let mut data = [0u8; 4];
    let code = unsafe {
        pcan::CAN_GetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan::PCAN_LOG_STATUS as u8,
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

/// Sets the logging active or inactive.
///
/// Tries to activate or deactivate logging. If the functions succeeds, the logging is either active
/// or inactive. Otherwise, it returns a [PcanError](PcanError).
///
/// # Example
/// ```
/// use pcan_basic::log::set_logging;
/// match set_logging(true) {
///     Ok(_) => println!("Logging is now active!"),
///     Err(error) => println!("The error: {:?}", error)
/// }
/// ```
pub fn set_logging(enable: bool) -> Result<(), PcanError> {
    let mut data = match enable {
        true => pcan::PCAN_PARAMETER_ON.to_le_bytes(),
        false => pcan::PCAN_PARAMETER_OFF.to_le_bytes(),
    };
    let code = unsafe {
        pcan::CAN_SetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan::PCAN_LOG_STATUS as u8,
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

/* LOG CONFIGURE functions */

/// Holds the log function variants.
#[derive(PartialEq, Debug)]
pub enum LogFunction {
    /// This value is always active.
    Default,
    /// Logs when a function is entered.
    Entry,
    /// Logs the parameters passed to a function.
    Parameters,
    /// Logs when a function is leaved and its return value.
    Leave,
    /// Logs the parameters and CAN data passed to the CAN_Write function.
    Write,
    /// Logs the parameters and CAN data received through the CAN_Read function.
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

/// Retrieves the current log function.
///
/// Tries to retrieve the current log function. If the function succeeds, it returns a
/// [LogFunction](LogFunction). Otherwise, it returns a [PcanError](PcanError).
///
/// # Example
/// ```
/// use pcan_basic::log::log_configuration;
/// match log_configuration() { ///
///     Ok(config) => println!("Log configuration: {:?}", config),
///     Err(err) => println!("The error: {:?}", err),
/// }
/// ```
pub fn log_configuration() -> Result<LogFunction, PcanError> {
    let mut data = [0u8; 4];
    let code = unsafe {
        pcan::CAN_GetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan::PCAN_LOG_CONFIGURE as u8,
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

/// Sets the log function.
///
/// Tries to set the current log function. If the function succeeds, it returns the empty type.
/// Otherwise, it returns a [PcanError](PcanError).
///
/// # Example
/// ```
/// use pcan_basic::log::{configure_log, LogFunction};
/// match configure_log(LogFunction::Parameters) {
///     Ok(_) => println!("Successful!"),
///     Err(err) => println!("The error: {:?}", err),
/// }
/// ```
pub fn configure_log(config: LogFunction) -> Result<(), PcanError> {
    let mut data = u32::from(config).to_le_bytes();
    let code = unsafe {
        pcan::CAN_SetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan::PCAN_LOG_CONFIGURE as u8,
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

/* LOG TEXT function */

/// Inserts a text into the log file.
///
/// Tries to insert the text into the current log file. If the function succeeds, the text is
/// inserted into the current log file. Otherwise, it returns a
/// [PcanError](PcanError).
///
/// # Example
/// ```
/// use pcan_basic::log::{insert_log_text, is_logging, set_logging};
/// println!("Set logging active!");
/// match set_logging(true) {
///     Ok(_) => println!("Successful!"),
///     Err(err) => println!("{:?}", err),
/// }
///
/// println!("Get current logging status");
/// match is_logging() {
///     Ok(is_logging) => println!("Is logging: {}", is_logging),
/// Err(err) => println!("{:?}", err),
/// }
///
/// println!("Set text");
/// match insert_log_text("Hello World") {
///     Ok(_) => println!("Successful!"),
///     Err(err) => println!("{:?}", err),
/// }
///
/// println!("Set logging inactive!");
/// match set_logging(false) {
///     Ok(_) => println!("Successful!"),
///     Err(err) => println!("{:?}", err),
/// }
///
/// println!("Get current logging status");
/// match is_logging() {
/// Ok(is_logging) => println!("Is logging: {}", is_logging),
///     Err(err) => println!("{:?}", err),
/// }
/// ```
pub fn insert_log_text<S: AsRef<str>>(text: S) -> Result<(), PcanError> {
    let mut data = String::from(text.as_ref());
    let code = unsafe {
        pcan::CAN_SetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan::PCAN_LOG_TEXT as u8,
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
