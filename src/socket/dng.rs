//!
//!
//!

use crate::bus::DngBus;
use crate::channel::Channel;
use crate::df::{
    HasAcceptanceFilter11Bit, HasAcceptanceFilter29Bit, HasAllowErrorFrames, HasAllowRTRFrames,
    HasAllowStatusFrames, HasMessageFilter, HasReceiveStatus, HasSetAcceptanceFilter11Bit,
    HasSetAcceptanceFilter29Bit, HasSetAllowErrorFrames, HasSetAllowRTRFrames,
    HasSetAllowStatusFrames, HasSetMessageFilter, HasSetReceiveStatus,
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
use crate::trace::{HasSetTraceLocation, HasSetTraceStatus, HasTraceLocation, HasTraceStatus};

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

impl HasHardwareName for DngCanSocket {}

impl HasControllerNumber for DngCanSocket {}
impl HasSetControllerNumber for DngCanSocket {}

impl HasDevicePartNumber for DngCanSocket {}

/* INFORMATIONAL PARAMETER */

impl HasChannelVersion for DngCanSocket {}

impl HasChannelFeatures for DngCanSocket {}

impl HasBitrateInfo for DngCanSocket {}

impl HasNominalBusSpeed for DngCanSocket {}

impl HasDataBusSpeed for DngCanSocket {}

impl HasFirmwareVersion for DngCanSocket {}

/* SPECIAL BEHAVIOR */

/* CONTROLLING DATA FLOW */

impl HasMessageFilter for DngCanSocket {}
impl HasSetMessageFilter for DngCanSocket {}

impl HasReceiveStatus for DngCanSocket {}
impl HasSetReceiveStatus for DngCanSocket {}

impl HasAllowStatusFrames for DngCanSocket {}
impl HasSetAllowStatusFrames for DngCanSocket {}

impl HasAllowRTRFrames for DngCanSocket {}
impl HasSetAllowRTRFrames for DngCanSocket {}

impl HasAllowErrorFrames for DngCanSocket {}
impl HasSetAllowErrorFrames for DngCanSocket {}

impl HasAcceptanceFilter11Bit for DngCanSocket {}
impl HasSetAcceptanceFilter11Bit for DngCanSocket {}

impl HasAcceptanceFilter29Bit for DngCanSocket {}
impl HasSetAcceptanceFilter29Bit for DngCanSocket {}

/* TRACING PARAMETERS */

impl HasTraceLocation for DngCanSocket {}
impl HasSetTraceLocation for DngCanSocket {}

impl HasTraceStatus for DngCanSocket {}
impl HasSetTraceStatus for DngCanSocket {}
