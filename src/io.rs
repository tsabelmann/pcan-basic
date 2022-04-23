use crate::{CanFdFrame, CanFrame, Timestamp};
use pcan_basic_sys as pcan;

trait CanRead {
    fn read(&self) -> Result<(CanFrame, Timestamp), ()>;
    fn read_frame(&self) -> Result<CanFrame, ()>;
}

trait CanFdRead {
    fn read(&self) -> Result<(CanFdFrame, Timestamp), ()>;
    fn read_frame(&self) -> Result<CanFdFrame, ()>;
}

trait CanWrite {
    fn write(&self, frame: CanFrame) -> Result<(), ()>;
}

trait CanFdWrite {
    fn write(&self, frame: CanFdFrame) -> Result<(), ()>;
}

/* Bitrate */
pub enum Bitrate {}

/* CAN socket types */

pub struct IsaCanSocket {
    handle: u16,
}

pub struct DngCanSocket {
    handle: u16,
}

pub struct PciCanSocket {
    handle: u16,
}

pub struct UsbCanSocket {
    handle: u16,
}

pub struct PccCanSocket {
    handle: u16,
}

pub struct LanCanSocket {
    handle: u16,
}

/* Drop trait implementations */

impl Drop for IsaCanSocket {
    fn drop(&mut self) {
        unsafe { pcan::CAN_Uninitialize(self.handle) };
    }
}

impl Drop for DngCanSocket {
    fn drop(&mut self) {
        unsafe { pcan::CAN_Uninitialize(self.handle) };
    }
}

impl Drop for PciCanSocket {
    fn drop(&mut self) {
        unsafe { pcan::CAN_Uninitialize(self.handle) };
    }
}

impl Drop for UsbCanSocket {
    fn drop(&mut self) {
        unsafe { pcan::CAN_Uninitialize(self.handle) };
    }
}

impl Drop for PccCanSocket {
    fn drop(&mut self) {
        unsafe { pcan::CAN_Uninitialize(self.handle) };
    }
}

impl Drop for LanCanSocket {
    fn drop(&mut self) {
        unsafe { pcan::CAN_Uninitialize(self.handle) };
    }
}

/* CanRead trait implementations */

// impl<T> CanRead for T {
//     fn read(&self) -> Result<(CanFrame, Timestamp), ()> {
//         let frame = CanFrame::default();
//         let timestamp = Timestamp::default();
//
//         let flag = unsafe {
//             pcan::CAN_Read()
//         }
//
//
//     }
// }
