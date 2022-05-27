//!
//!
//!

use crate::bus::UsbBus;
use crate::socket::{Baudrate, HasCanRead, HasCanReadFd, HasCanWrite, HasCanWriteFd, Socket};
use crate::channel::Channel;
use crate::error::{PcanError, PcanOkError};
use crate::hw::{HasChannelIdentifying, HasControllerNumber, HasDeviceId, HasDevicePartNumber, HasHardwareName, HasSetControllerNumber, HasSetDeviceId};
use crate::info::{HasBitrateInfo, HasChannelFeatures, HasChannelVersion, HasDataBusSpeed, HasNominalBusSpeed};
use crate::pcan;
use crate::special::{
    HasBusOffAutoreset, HasFiveVoltsPower, HasInterframeDelay, HasListenOnly,
    HasSetBusOffAutoreset, HasSetFiveVoltsPower, HasSetInterframeDelay, HasSetListenOnly,
};

#[derive(Debug, PartialEq)]
pub struct UsbCanSocket {
    handle: u16,
}

impl UsbCanSocket {
    pub fn open(bus: UsbBus, baud: Baudrate) -> Result<UsbCanSocket, PcanError> {
        let handle = bus.into();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(UsbCanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Drop trait implementation */

impl Drop for UsbCanSocket {
    fn drop(&mut self) {
        unsafe { pcan::CAN_Uninitialize(self.handle) };
    }
}

/* Socket trait implementation */

impl Socket for UsbCanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

/* Channel trait implementation */

impl Channel for UsbCanSocket {
    fn channel(&self) -> u16 {
        self.handle
    }
}

/* CAN trait implementations */

impl HasCanRead for UsbCanSocket {}
impl HasCanReadFd for UsbCanSocket {}
impl HasCanWrite for UsbCanSocket {}
impl HasCanWriteFd for UsbCanSocket {}

/* HARDWARE IDENTIFICATION */

impl HasChannelIdentifying for UsbCanSocket {}

impl HasDeviceId for UsbCanSocket {}
impl HasSetDeviceId for UsbCanSocket {}

impl HasHardwareName for UsbCanSocket {}

impl HasControllerNumber for UsbCanSocket {}
impl HasSetControllerNumber for UsbCanSocket {}

impl HasDevicePartNumber for UsbCanSocket {}

/* INFORMATIONAL PARAMETER */

impl HasChannelVersion for UsbCanSocket {}

impl HasChannelFeatures for UsbCanSocket {}

impl HasBitrateInfo for UsbCanSocket {}

impl HasNominalBusSpeed for UsbCanSocket {}

impl HasDataBusSpeed for UsbCanSocket {}

/* SPECIAL BEHAVIOR */

impl HasFiveVoltsPower for UsbCanSocket {}
impl HasSetFiveVoltsPower for UsbCanSocket {}

impl HasBusOffAutoreset for UsbCanSocket {}
impl HasSetBusOffAutoreset for UsbCanSocket {}

impl HasListenOnly for UsbCanSocket {}
impl HasSetListenOnly for UsbCanSocket {}

impl HasInterframeDelay for UsbCanSocket {}
impl HasSetInterframeDelay for UsbCanSocket {}
