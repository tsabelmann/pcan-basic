//!
//!

use crate::bus::IsaBus;
use crate::can::Baudrate;
use crate::error::{PcanError, PcanOkError};
use crate::pcan;

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
