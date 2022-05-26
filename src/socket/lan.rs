//!
//!
//!

use crate::bus::LanBus;
use crate::socket::{Baudrate, HasCanRead, HasCanReadFd, HasCanWrite, HasCanWriteFd, Socket};
use crate::channel::Channel;
use crate::error::{PcanError, PcanOkError};
use crate::hw::{HasControllerNumber, HasDeviceId, HasHardwareName, HasIpAddress, HasSetControllerNumber, HasSetDeviceId};
use crate::pcan;

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

/* Channel trait implementation */

impl Channel for LanCanSocket {
    fn channel(&self) -> u16 {
        self.handle
    }
}

/* CAN trait implementations */

impl HasCanRead for LanCanSocket {}
impl HasCanReadFd for LanCanSocket {}
impl HasCanWrite for LanCanSocket {}
impl HasCanWriteFd for LanCanSocket {}

/* HARDWARE IDENTIFICATION */

impl HasDeviceId for LanCanSocket {}
impl HasSetDeviceId for LanCanSocket {}

impl HasHardwareName for LanCanSocket {}

impl HasControllerNumber for LanCanSocket {}
impl HasSetControllerNumber for LanCanSocket {}

impl HasIpAddress for LanCanSocket {}

/* SPECIAL BEHAVIOR */
