//!
//!
//!

use crate::bus::LanBus;
use crate::channel::Channel;
use crate::df::{HasMessageFilter, HasSetMessageFilter};
use crate::error::{PcanError, PcanOkError};
use crate::hw::{
    HasControllerNumber, HasDeviceId, HasDevicePartNumber, HasHardwareName, HasIpAddress,
    HasSetControllerNumber, HasSetDeviceId,
};
use crate::info::{
    HasBitrateInfo, HasChannelFeatures, HasChannelVersion, HasDataBusSpeed, HasFirmwareVersion,
    HasNominalBusSpeed,
};
use crate::pcan;
use crate::socket::{Baudrate, HasCanRead, HasCanReadFd, HasCanWrite, HasCanWriteFd, Socket};

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

impl HasDevicePartNumber for LanCanSocket {}

/* INFORMATIONAL PARAMETER */

impl HasChannelVersion for LanCanSocket {}

impl HasChannelFeatures for LanCanSocket {}

impl HasBitrateInfo for LanCanSocket {}

impl HasNominalBusSpeed for LanCanSocket {}

impl HasDataBusSpeed for LanCanSocket {}

impl HasFirmwareVersion for LanCanSocket {}

/* SPECIAL BEHAVIOR */

/* CONTROLLING DATA FLOW */

impl HasMessageFilter for LanCanSocket {}
impl HasSetMessageFilter for LanCanSocket {}
