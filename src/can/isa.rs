//!
//!

use crate::bus::IsaBus;
use crate::can::{Baudrate, HasCanRead, HasCanReadFd, HasCanWrite, HasCanWriteFd, Socket};
use crate::error::{PcanError, PcanOkError};
use crate::pcan;
use crate::special::{BusOffAutoreset, SetBusOffAutoreset};
use std::ffi::c_void;

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

/* HasCanRead trait implementation */

impl HasCanRead for IsaCanSocket {}

/* HasCanReadFd trait implementation */

impl HasCanReadFd for IsaCanSocket {}

/* HasCanWrite trait implementation */

impl HasCanWrite for IsaCanSocket {}

/* HasCanWriteFd trait implementation */

impl HasCanWriteFd for IsaCanSocket {}

/* SPECIAL BEHAVIOR */

/* BusOffAutoreset trait implementation */

impl BusOffAutoreset for IsaCanSocket {
    fn bus_off_autoreset(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.handle,
                pcan::PCAN_BUSOFF_AUTORESET as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::PCAN_PARAMETER_ON == pcan::PCAN_PARAMETER_ON {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl SetBusOffAutoreset for IsaCanSocket {
    fn set_bus_off_autoreset(&self, value: bool) -> Result<(), PcanError> {
        let mut data = match value {
            true => pcan::PCAN_PARAMETER_ON.to_le_bytes(),
            false => pcan::PCAN_PARAMETER_OFF.to_le_bytes(),
        };
        let code = unsafe {
            pcan::CAN_SetValue(
                self.handle,
                pcan::PCAN_BUSOFF_AUTORESET as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(()),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}
