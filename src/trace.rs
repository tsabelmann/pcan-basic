use crate::channel::Channel;
use crate::error::{PcanError, PcanOkError};
use crate::pcan;
use std::ffi::c_void;
use std::path::{Path, PathBuf};

pub(crate) trait HasTraceLocation {}

pub trait TraceLocation {
    fn trace_location(&self) -> Result<PathBuf, PcanError>;
}

impl<T: HasTraceLocation + Channel> TraceLocation for T {
    fn trace_location(&self) -> Result<PathBuf, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_VERSION_STRING as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan_basic_sys::PCAN_TRACE_LOCATION as u8,
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
}

pub(crate) trait HasSetTraceLocation {}

pub trait SetTraceLocation {
    fn set_trace_location<P: AsRef<Path>>(&self, path: P) -> Result<(), PcanError>;
}

impl<T: HasSetTraceLocation + Channel> SetTraceLocation for T {
    fn set_trace_location<P: AsRef<Path>>(&self, path: P) -> Result<(), PcanError> {
        let mut data = match path.as_ref().to_str() {
            None => {
                return Err(PcanError::Unknown);
            }
            Some(s) => String::from(s),
        };
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
                pcan::PCAN_TRACE_LOCATION as u8,
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

pub fn set_default_trace_location<T: SetTraceLocation>(value: &T) -> Result<(), PcanError> {
    value.set_trace_location(" ")
}
