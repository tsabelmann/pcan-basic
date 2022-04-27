//!
//!
//!

use crate::bus::PciBus;
use crate::can::{Baudrate, HasCanRead, HasCanReadFd, HasCanWrite, HasCanWriteFd, Socket};
use crate::channel::Channel;
use crate::error::{PcanError, PcanOkError};
use crate::pcan;

#[derive(Debug, PartialEq)]
pub struct PciCanSocket {
    handle: u16,
}

impl PciCanSocket {
    pub fn open(bus: PciBus, baud: Baudrate) -> Result<PciCanSocket, PcanError> {
        let handle = bus.into();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(PciCanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Drop trait implementation */

impl Drop for PciCanSocket {
    fn drop(&mut self) {
        unsafe { pcan::CAN_Uninitialize(self.handle) };
    }
}

/* Socket trait implementation */

impl Socket for PciCanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

/* Channel trait implementation */

impl Channel for PciCanSocket {
    fn channel(&self) -> u16 {
        self.handle
    }
}

/* CAN trait implementations */

impl HasCanRead for PciCanSocket {}
impl HasCanReadFd for PciCanSocket {}
impl HasCanWrite for PciCanSocket {}
impl HasCanWriteFd for PciCanSocket {}

/* HARDWARE IDENTIFICATION */

/* SPECIAL BEHAVIOR */
