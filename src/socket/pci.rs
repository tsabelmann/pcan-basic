//!
//!
//!

use crate::bus::PciBus;
use crate::channel::Channel;
use crate::df::{
    HasAcceptanceFilter11Bit, HasAcceptanceFilter29Bit, HasAllowEchoFrames, HasAllowErrorFrames,
    HasAllowRTRFrames, HasAllowStatusFrames, HasMessageFilter, HasReceiveStatus,
    HasSetAcceptanceFilter11Bit, HasSetAcceptanceFilter29Bit, HasSetAllowEchoFrames,
    HasSetAllowErrorFrames, HasSetAllowRTRFrames, HasSetAllowStatusFrames, HasSetMessageFilter,
    HasSetReceiveStatus,
};
use crate::error::{PcanError, PcanOkError};
use crate::hw::{
    HasControllerNumber, HasDeviceId, HasDevicePartNumber, HasHardwareName, HasSetControllerNumber,
    HasSetDeviceId,
};
use crate::info::{
    HasBitrateInfo, HasChannelFeatures, HasChannelVersion, HasDataBusSpeed, HasFirmwareVersion,
    HasNominalBusSpeed,
};
use crate::pcan;
use crate::socket::{Baudrate, HasCanRead, HasCanReadFd, HasCanWrite, HasCanWriteFd, Socket};
use crate::trace::{
    HasSetTraceLocation, HasSetTraceSize, HasSetTraceStatus, HasTraceLocation, HasTraceSize,
    HasTraceStatus,
};

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

impl HasDeviceId for PciCanSocket {}
impl HasSetDeviceId for PciCanSocket {}

impl HasHardwareName for PciCanSocket {}

impl HasControllerNumber for PciCanSocket {}
impl HasSetControllerNumber for PciCanSocket {}

impl HasDevicePartNumber for PciCanSocket {}

/* INFORMATIONAL PARAMETER */

impl HasChannelVersion for PciCanSocket {}

impl HasChannelFeatures for PciCanSocket {}

impl HasBitrateInfo for PciCanSocket {}

impl HasNominalBusSpeed for PciCanSocket {}

impl HasDataBusSpeed for PciCanSocket {}

impl HasFirmwareVersion for PciCanSocket {}

/* SPECIAL BEHAVIOR */

/* CONTROLLING DATA FLOW */

impl HasMessageFilter for PciCanSocket {}
impl HasSetMessageFilter for PciCanSocket {}

impl HasReceiveStatus for PciCanSocket {}
impl HasSetReceiveStatus for PciCanSocket {}

impl HasAllowStatusFrames for PciCanSocket {}
impl HasSetAllowStatusFrames for PciCanSocket {}

impl HasAllowRTRFrames for PciCanSocket {}
impl HasSetAllowRTRFrames for PciCanSocket {}

impl HasAllowErrorFrames for PciCanSocket {}
impl HasSetAllowErrorFrames for PciCanSocket {}

impl HasAllowEchoFrames for PciCanSocket {}
impl HasSetAllowEchoFrames for PciCanSocket {}

impl HasAcceptanceFilter11Bit for PciCanSocket {}
impl HasSetAcceptanceFilter11Bit for PciCanSocket {}

impl HasAcceptanceFilter29Bit for PciCanSocket {}
impl HasSetAcceptanceFilter29Bit for PciCanSocket {}

/* TRACING PARAMETERS */

impl HasTraceLocation for PciCanSocket {}
impl HasSetTraceLocation for PciCanSocket {}

impl HasTraceStatus for PciCanSocket {}
impl HasSetTraceStatus for PciCanSocket {}

impl HasTraceSize for PciCanSocket {}
impl HasSetTraceSize for PciCanSocket {}
