//!
//!
//!

use crate::bus::PccBus;
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
use crate::special::{HasFiveVoltsPower, HasSetFiveVoltsPower};
use crate::trace::{
    HasSetTraceLocation, HasSetTraceSize, HasSetTraceStatus, HasTraceLocation, HasTraceSize,
    HasTraceStatus,
};

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

/* Channel trait implementation */

impl Channel for PccCanSocket {
    fn channel(&self) -> u16 {
        self.handle
    }
}

/* CAN trait implementations */

impl HasCanRead for PccCanSocket {}
impl HasCanReadFd for PccCanSocket {}
impl HasCanWrite for PccCanSocket {}
impl HasCanWriteFd for PccCanSocket {}

/* HARDWARE IDENTIFICATION */

impl HasHardwareName for PccCanSocket {}

impl HasControllerNumber for PccCanSocket {}
impl HasSetControllerNumber for PccCanSocket {}

impl HasDevicePartNumber for PccCanSocket {}

/* INFORMATIONAL PARAMETER */

impl HasChannelVersion for PccCanSocket {}

impl HasChannelFeatures for PccCanSocket {}

impl HasBitrateInfo for PccCanSocket {}

impl HasNominalBusSpeed for PccCanSocket {}

impl HasDataBusSpeed for PccCanSocket {}

impl HasFirmwareVersion for PccCanSocket {}

/* SPECIAL BEHAVIOR */

impl HasFiveVoltsPower for PccCanSocket {}
impl HasSetFiveVoltsPower for PccCanSocket {}

/* CONTROLLING DATA FLOW */

impl HasMessageFilter for PccCanSocket {}
impl HasSetMessageFilter for PccCanSocket {}

impl HasReceiveStatus for PccCanSocket {}
impl HasSetReceiveStatus for PccCanSocket {}

impl HasAllowStatusFrames for PccCanSocket {}
impl HasSetAllowStatusFrames for PccCanSocket {}

impl HasAllowRTRFrames for PccCanSocket {}
impl HasSetAllowRTRFrames for PccCanSocket {}

impl HasAllowErrorFrames for PccCanSocket {}
impl HasSetAllowErrorFrames for PccCanSocket {}

impl HasAcceptanceFilter11Bit for PccCanSocket {}
impl HasSetAcceptanceFilter11Bit for PccCanSocket {}

impl HasAcceptanceFilter29Bit for PccCanSocket {}
impl HasSetAcceptanceFilter29Bit for PccCanSocket {}

/* TRACING PARAMETERS */

impl HasTraceLocation for PccCanSocket {}
impl HasSetTraceLocation for PccCanSocket {}

impl HasTraceStatus for PccCanSocket {}
impl HasSetTraceStatus for PccCanSocket {}

impl HasTraceSize for PccCanSocket {}
impl HasSetTraceSize for PccCanSocket {}
