//!
//!
//!

use crate::bus::IsaBus;
use crate::channel::Channel;
use crate::df::{
    HasAcceptanceFilter11Bit, HasAllowErrorFrames, HasAllowRTRFrames, HasAllowStatusFrames,
    HasMessageFilter, HasReceiveStatus, HasSetAcceptanceFilter11Bit, HasSetAllowErrorFrames,
    HasSetAllowRTRFrames, HasSetAllowStatusFrames, HasSetMessageFilter, HasSetReceiveStatus,
};
use crate::error::{PcanError, PcanOkError};
use crate::hw::{
    HasControllerNumber, HasDevicePartNumber, HasHardwareName, HasSetControllerNumber,
};
use crate::info::{
    HasBitrateInfo, HasChannelFeatures, HasChannelVersion, HasDataBusSpeed, HasFirmwareVersion,
    HasNominalBusSpeed,
};
use crate::pcan;
use crate::socket::{Baudrate, HasCanRead, HasCanReadFd, HasCanWrite, HasCanWriteFd, Socket};

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

/* CAN trait implementations */

impl HasCanRead for IsaCanSocket {}
impl HasCanReadFd for IsaCanSocket {}
impl HasCanWrite for IsaCanSocket {}
impl HasCanWriteFd for IsaCanSocket {}

/* HARDWARE IDENTIFICATION */

impl HasHardwareName for IsaCanSocket {}

impl HasControllerNumber for IsaCanSocket {}
impl HasSetControllerNumber for IsaCanSocket {}

impl HasDevicePartNumber for IsaCanSocket {}

/* INFORMATIONAL PARAMETER */

impl HasChannelVersion for IsaCanSocket {}

impl HasChannelFeatures for IsaCanSocket {}

impl HasBitrateInfo for IsaCanSocket {}

impl HasNominalBusSpeed for IsaCanSocket {}

impl HasDataBusSpeed for IsaCanSocket {}

impl HasFirmwareVersion for IsaCanSocket {}

/* SPECIAL BEHAVIOR */

/* CONTROLLING DATA FLOW */

impl HasMessageFilter for IsaCanSocket {}
impl HasSetMessageFilter for IsaCanSocket {}

impl HasReceiveStatus for IsaCanSocket {}
impl HasSetReceiveStatus for IsaCanSocket {}

impl HasAllowStatusFrames for IsaCanSocket {}
impl HasSetAllowStatusFrames for IsaCanSocket {}

impl HasAllowRTRFrames for IsaCanSocket {}
impl HasSetAllowRTRFrames for IsaCanSocket {}

impl HasAllowErrorFrames for IsaCanSocket {}
impl HasSetAllowErrorFrames for IsaCanSocket {}

impl HasAcceptanceFilter11Bit for IsaCanSocket {}
impl HasSetAcceptanceFilter11Bit for IsaCanSocket {}
