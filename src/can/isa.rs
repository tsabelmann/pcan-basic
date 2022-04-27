//!
//!

use crate::bus::IsaBus;
use crate::can::{Baudrate, HasCanRead, HasCanReadFd, HasCanWrite, HasCanWriteFd, Socket};
use crate::channel::Channel;
use crate::error::{PcanError, PcanOkError};
use crate::pcan;
use crate::special::{BusOffAutoreset, ListenOnly, SetBusOffAutoreset, SetListenOnly};
use std::ffi::c_void;

#[derive(Debug, PartialEq)]
pub struct IsaCanSocket {
    handle: u16,
}

impl IsaCanSocket {
    pub fn open(bus: IsaBus, baud: Baudrate) -> Result<IsaCanSocket, PcanError> {
        let code = unsafe { pcan::CAN_Initialize(bus.into(), baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(IsaCanSocket { handle: bus.into() }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Drop trait implementation */

impl Drop for IsaCanSocket {
    fn drop(&mut self) {
        unsafe { pcan::CAN_Uninitialize(self.handle) };
    }
}

/* Socket trait implementation */

impl Socket for IsaCanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

/* Channel trait implementation */

impl Channel for IsaCanSocket {
    fn channel(&self) -> u16 {
        self.handle
    }
}

/* HasCanRead trait implementation */

impl HasCanRead for IsaCanSocket {}

/* HasCanReadFd trait implementation */

impl HasCanReadFd for IsaCanSocket {}

/* HasCanWrite trait implementation */

impl HasCanWrite for IsaCanSocket {}

/* HasCanWriteFd trait implementation */

impl HasCanWriteFd for IsaCanSocket {}

/* SPECIAL BEHAVIOR */
