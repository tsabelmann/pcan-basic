//!

use crate::bus::DngBus;
use crate::can::{Baudrate, HasCanRead, HasCanReadFd, HasCanWrite, HasCanWriteFd, Socket};
use crate::channel::Channel;
use crate::error::{PcanError, PcanOkError};
use crate::pcan;
use crate::special;
use crate::special::{HasBusOffAutoreset, HasListenOnly, HasSetBusOffAutoreset, HasSetListenOnly};
use std::ffi::c_void;

#[derive(Debug, PartialEq)]
pub struct DngCanSocket {
    handle: u16,
}

impl DngCanSocket {
    pub fn open(bus: DngBus, baud: Baudrate) -> Result<DngCanSocket, PcanError> {
        let handle = bus.into();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(DngCanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Drop trait implementations */

impl Drop for DngCanSocket {
    fn drop(&mut self) {
        unsafe { pcan::CAN_Uninitialize(self.handle) };
    }
}

/* Socket trait implementation */

impl Socket for DngCanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

/* Channel trait implementation */

impl Channel for DngCanSocket {
    fn channel(&self) -> u16 {
        self.handle
    }
}

/* CAN trait implementations */

impl HasCanRead for DngCanSocket {}
impl HasCanReadFd for DngCanSocket {}
impl HasCanWrite for DngCanSocket {}
impl HasCanWriteFd for DngCanSocket {}

/* HARDWARE IDENTIFICATION */

/* SPECIAL BEHAVIOR */
