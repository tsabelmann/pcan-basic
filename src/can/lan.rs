//!
//!
//!

use crate::bus::LanBus;
use crate::can::{Baudrate, HasCanRead, HasCanReadFd, HasCanWrite, HasCanWriteFd, Socket};
use crate::error::{PcanError, PcanOkError};
use crate::pcan;
use std::net::Ipv4Addr;

#[derive(Debug, PartialEq)]
pub struct LanCanSocket {
    handle: u16,
}

impl LanCanSocket {
    pub fn open(bus: LanBus, baud: Baudrate) -> Result<LanCanSocket, PcanError> {
        let handle = bus.into();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(LanCanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Drop trait implementation */

impl Drop for LanCanSocket {
    fn drop(&mut self) {
        unsafe { pcan::CAN_Uninitialize(self.handle) };
    }
}

/* Socket trait implementation */

impl Socket for LanCanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

/* HasCanRead trait implementation */

impl HasCanRead for LanCanSocket {}

/* HasCanReadFd trait implementation */

impl HasCanReadFd for LanCanSocket {}

/* HasCanWrite trait implementation */

impl HasCanWrite for LanCanSocket {}

/* HasCanWriteFd trait implementation */

impl HasCanWriteFd for LanCanSocket {}
