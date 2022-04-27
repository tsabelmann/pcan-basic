//!
//!
//!

use crate::bus::LanBus;
use crate::can::{Baudrate, HasCanRead, HasCanReadFd, HasCanWrite, HasCanWriteFd, Socket};
use crate::error::{PcanError, PcanOkError};
use crate::hw_ident::IpAddress;
use crate::pcan;
use crate::special::{BusOffAutoreset, ListenOnly, SetBusOffAutoreset};
use std::ffi::c_void;
use std::net::Ipv4Addr;

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

/* HasCanRead trait implementation */

impl HasCanRead for LanCanSocket {}

/* HasCanReadFd trait implementation */

impl HasCanReadFd for LanCanSocket {}

/* HasCanWrite trait implementation */

impl HasCanWrite for LanCanSocket {}

/* HasCanWriteFd trait implementation */

impl HasCanWriteFd for LanCanSocket {}

/* IpAddress trait implementation */

impl IpAddress for LanCanSocket {
    fn ip_address(&self) -> Result<Ipv4Addr, PcanError> {
        let mut data = [0u8; 20];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.handle,
                pcan::PCAN_IP_ADDRESS as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let s = s.trim_matches(char::from(0));
                    match s.parse() {
                        Ok(ip) => Ok(ip),
                        Err(_) => Err(PcanError::Unknown),
                    }
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            _ => Err(PcanError::Unknown),
        }
    }
}

/* SPECIAL BEHAVIOR */

/* BusOfAutoreset trait implementation */

impl BusOffAutoreset for LanCanSocket {
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

impl SetBusOffAutoreset for LanCanSocket {
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

/* ListenOnly trait implementation */

impl ListenOnly for LanCanSocket {
    fn listen_only(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.handle,
                pcan::PCAN_LISTEN_ONLY as u8,
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
