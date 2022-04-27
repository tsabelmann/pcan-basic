//!
//!
//!

use crate::bus::PccBus;
use crate::can::{Baudrate, HasCanRead, HasCanReadFd, HasCanWrite, HasCanWriteFd, Socket};
use crate::error::{PcanError, PcanOkError};
use crate::pcan;
use crate::special::{BusOffAutoreset, FiveVoltsPower, SetBusOffAutoreset, SetFiveVoltsPower};
use std::ffi::c_void;

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

/* HasCanRead trait implementation */

impl HasCanRead for PccCanSocket {}

/* HasCanReadFd trait implementation */

impl HasCanReadFd for PccCanSocket {}

/* HasCanWrite trait implementation */

impl HasCanWrite for PccCanSocket {}

/* HasCanWriteFd trait implementation */

impl HasCanWriteFd for PccCanSocket {}

/* SPECIAL BEHAVIOR */

/* FiveVoltsPower trait implementation */

impl FiveVoltsPower for PccCanSocket {
    fn five_volts(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.handle,
                pcan::PCAN_5VOLTS_POWER as u8,
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

impl SetFiveVoltsPower for PccCanSocket {
    fn set_five_volts(&self, value: bool) -> Result<(), PcanError> {
        let mut data = match value {
            true => pcan::PCAN_PARAMETER_ON.to_le_bytes(),
            false => pcan::PCAN_PARAMETER_OFF.to_le_bytes(),
        };
        let code = unsafe {
            pcan::CAN_SetValue(
                self.handle,
                pcan::PCAN_5VOLTS_POWER as u8,
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

/* BusOfAutoreset trait implementation */

impl BusOffAutoreset for PccCanSocket {
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

impl SetBusOffAutoreset for PccCanSocket {
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
