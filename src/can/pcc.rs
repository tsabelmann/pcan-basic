//!
//!
//!

use crate::bus::PccBus;
use crate::can::{Baudrate, HasCanRead, HasCanReadFd, HasCanWrite, HasCanWriteFd, Socket};
use crate::error::{PcanError, PcanOkError};
use crate::pcan;

#[derive(Debug, PartialEq)]
pub struct PccCanSocket {
    handle: u16,
}

impl PccCanSocket {
    pub fn open(bus: PccBus, baud: Baudrate) -> Result<PccCanSocket, PcanError> {
        let handle = bus.into();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(PccCanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Drop trait implementation */

impl Drop for PccCanSocket {
    fn drop(&mut self) {
        unsafe { pcan::CAN_Uninitialize(self.handle) };
    }
}

/* Socket trait implementation */

impl Socket for PccCanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

/* HasCanRead trait implementation */

impl HasCanRead for PccCanSocket {}

/* HasCanReadFd trait implementation */

impl HasCanReadFd for PccCanSocket {}

/* HasCanWrite trait implementation */

impl HasCanWrite for PccCanSocket {}

/* HasCanWriteFd trait implementation */

impl HasCanWriteFd for PccCanSocket {}
