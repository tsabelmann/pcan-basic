//!
//!
//!

pub mod dng;
pub mod isa;
pub mod lan;
pub mod pcc;
pub mod pci;
pub mod usb;

use crate::bus::Bus;
use crate::error::{PcanError, PcanOkError};
use crate::pcan;

pub const STANDARD_MASK: u32 = 0x07_FF;
pub const EXTENDED_MASK: u32 = 0x1F_FF_FF_FF;

#[derive(Debug, PartialEq)]
pub enum MessageType {
    Standard,
    Extended,
}

#[derive(Debug, PartialEq)]
pub enum FrameConstructionError {
    TooMuchData,
    CanIdMessageTypeMismatch,
}

#[derive(Debug, Copy, Clone)]
pub struct CanFrame {
    frame: pcan::TPCANMsg,
}

impl CanFrame {
    const MAX_DLC: usize = 8;

    pub fn new(
        can_id: u32,
        msg_type: MessageType,
        data: &[u8],
    ) -> Result<CanFrame, FrameConstructionError> {
        if data.len() > Self::MAX_DLC {
            Err(FrameConstructionError::TooMuchData)
        } else {
            let mut frame_data: [u8; 8] = [0; 8];
            for (i, v) in data.into_iter().enumerate() {
                frame_data[i] = *v;
            }

            match msg_type {
                MessageType::Standard => Ok(CanFrame {
                    frame: pcan::TPCANMsg {
                        ID: can_id & STANDARD_MASK,
                        MSGTYPE: pcan::PCAN_MESSAGE_STANDARD as u8,
                        LEN: data.len() as u8,
                        DATA: frame_data,
                    },
                }),
                MessageType::Extended => Ok(CanFrame {
                    frame: pcan::TPCANMsg {
                        ID: can_id & STANDARD_MASK,
                        MSGTYPE: pcan::PCAN_MESSAGE_STANDARD as u8,
                        LEN: data.len() as u8,
                        DATA: frame_data,
                    },
                }),
            }
        }
    }

    pub fn is_standard_frame(&self) -> bool {
        if self.frame.MSGTYPE & pcan::PCAN_MESSAGE_STANDARD as u8 != 0 {
            true
        } else {
            false
        }
    }

    pub fn is_extended_frame(&self) -> bool {
        if self.frame.MSGTYPE & pcan::PCAN_MESSAGE_EXTENDED as u8 != 0 {
            true
        } else {
            false
        }
    }

    pub fn can_id(&self) -> u32 {
        if self.is_standard_frame() {
            self.frame.ID & STANDARD_MASK
        } else {
            self.frame.ID & EXTENDED_MASK
        }
    }

    pub fn dlc(&self) -> u8 {
        self.frame.LEN
    }

    pub fn data(&self) -> &[u8] {
        &self.frame.DATA[0..self.dlc() as usize]
    }

    pub fn mut_data(&mut self) -> &mut [u8] {
        let dlc = self.dlc();
        &mut self.frame.DATA[0..dlc as usize]
    }
}

impl Default for CanFrame {
    fn default() -> Self {
        CanFrame::new(0, MessageType::Standard, &[]).unwrap()
    }
}

impl PartialEq for CanFrame {
    fn eq(&self, other: &Self) -> bool {
        if self.frame.ID != other.frame.ID {
            return false;
        }

        if self.frame.LEN != other.frame.LEN {
            return false;
        }

        if self.frame.MSGTYPE != other.frame.MSGTYPE {
            return false;
        }

        if self.data() != other.data() {
            return false;
        }

        true
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CanFdFrame {
    frame: pcan::TPCANMsgFD,
}

impl CanFdFrame {
    const MAX_DLC: usize = 64;

    pub fn new(
        can_id: u32,
        msg_type: MessageType,
        data: &[u8],
    ) -> Result<CanFdFrame, FrameConstructionError> {
        if data.len() > Self::MAX_DLC {
            Err(FrameConstructionError::TooMuchData)
        } else {
            let mut frame_data: [u8; 64] = [0; 64];
            for (i, v) in data.into_iter().enumerate() {
                frame_data[i] = *v;
            }

            match msg_type {
                MessageType::Standard => Ok(CanFdFrame {
                    frame: pcan::TPCANMsgFD {
                        ID: can_id & STANDARD_MASK,
                        MSGTYPE: pcan::PCAN_MESSAGE_STANDARD as u8,
                        DLC: data.len() as u8,
                        DATA: frame_data,
                    },
                }),
                MessageType::Extended => Ok(CanFdFrame {
                    frame: pcan::TPCANMsgFD {
                        ID: can_id & STANDARD_MASK,
                        MSGTYPE: pcan::PCAN_MESSAGE_STANDARD as u8,
                        DLC: data.len() as u8,
                        DATA: frame_data,
                    },
                }),
            }
        }
    }

    pub fn is_standard_frame(&self) -> bool {
        if self.frame.MSGTYPE & pcan::PCAN_MESSAGE_STANDARD as u8 != 0 {
            true
        } else {
            false
        }
    }

    pub fn is_extended_frame(&self) -> bool {
        if self.frame.MSGTYPE & pcan::PCAN_MESSAGE_EXTENDED as u8 != 0 {
            true
        } else {
            false
        }
    }

    pub fn can_id(&self) -> u32 {
        if self.is_standard_frame() {
            self.frame.ID & STANDARD_MASK
        } else {
            self.frame.ID & EXTENDED_MASK
        }
    }

    pub fn dlc(&self) -> u8 {
        self.frame.DLC
    }

    pub fn data(&self) -> &[u8] {
        &self.frame.DATA[0..self.dlc() as usize]
    }

    pub fn mut_data(&mut self) -> &mut [u8] {
        let dlc = self.dlc();
        &mut self.frame.DATA[0..dlc as usize]
    }
}

impl Default for CanFdFrame {
    fn default() -> Self {
        CanFdFrame::new(0, MessageType::Standard, &[]).unwrap()
    }
}

impl PartialEq for CanFdFrame {
    fn eq(&self, other: &Self) -> bool {
        if self.frame.ID != other.frame.ID {
            return false;
        }

        if self.frame.DLC != other.frame.DLC {
            return false;
        }

        if self.frame.MSGTYPE != other.frame.MSGTYPE {
            return false;
        }

        if self.data() != other.data() {
            return false;
        }

        true
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Timestamp {
    timestamp: pcan::TPCANTimestamp,
}

impl Default for Timestamp {
    fn default() -> Timestamp {
        Timestamp {
            timestamp: pcan::TPCANTimestamp {
                micros: 0,
                millis: 0,
                millis_overflow: 0,
            },
        }
    }
}

impl PartialEq for Timestamp {
    fn eq(&self, other: &Self) -> bool {
        if self.timestamp.micros != other.timestamp.micros {
            return false;
        }

        if self.timestamp.millis != other.timestamp.millis {
            return false;
        }

        if self.timestamp.millis_overflow != other.timestamp.millis_overflow {
            return false;
        }

        true
    }
}

#[derive(Debug, PartialEq)]
pub struct CanSocket {
    handle: u16,
}

impl CanSocket {
    pub fn open<T: Bus>(bus: T, baud: Baudrate) -> Result<CanSocket, PcanError> {
        let handle = bus.channel();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(CanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub trait CanRead {
    fn read(&self) -> Result<(CanFrame, Timestamp), PcanError>;
    fn read_frame(&self) -> Result<CanFrame, PcanError>;
}

pub trait CanReadFd {
    fn read(&self) -> Result<(CanFdFrame, u64), PcanError>;
    fn read_frame(&self) -> Result<CanFdFrame, PcanError>;
}

pub trait CanWrite {
    fn write(&self, frame: CanFrame) -> Result<(), PcanError>;
}

pub trait CanWriteFd {
    fn write(&self, frame: CanFdFrame) -> Result<(), PcanError>;
}

trait Socket {
    fn handle(&self) -> u16;
}

trait HasCanRead {}
trait HasCanReadFd {}
trait HasCanWrite {}
trait HasCanWriteFd {}

/* Baudrate */

#[derive(Debug, PartialEq)]
pub enum Baudrate {
    Baud1M,
    Baud800K,
    Baud500K,
    Baud250K,
    Baud125K,
    Baud100K,
    Baud95K,
    Baud83,
    Baud50K,
    Baud47K,
    Baud33K,
    Baud20K,
    Baud10K,
    Baud5K,
}

impl From<Baudrate> for u16 {
    fn from(value: Baudrate) -> Self {
        let ret = match value {
            Baudrate::Baud1M => pcan::PCAN_BAUD_1M,
            Baudrate::Baud800K => pcan::PCAN_BAUD_800K,
            Baudrate::Baud500K => pcan::PCAN_BAUD_500K,
            Baudrate::Baud250K => pcan::PCAN_BAUD_250K,
            Baudrate::Baud125K => pcan::PCAN_BAUD_125K,
            Baudrate::Baud100K => pcan::PCAN_BAUD_100K,
            Baudrate::Baud95K => pcan::PCAN_BAUD_95K,
            Baudrate::Baud83 => pcan::PCAN_BAUD_83K,
            Baudrate::Baud50K => pcan::PCAN_BAUD_50K,
            Baudrate::Baud47K => pcan::PCAN_BAUD_47K,
            Baudrate::Baud33K => pcan::PCAN_BAUD_33K,
            Baudrate::Baud20K => pcan::PCAN_BAUD_20K,
            Baudrate::Baud10K => pcan::PCAN_BAUD_10K,
            Baudrate::Baud5K => pcan::PCAN_BAUD_5K,
        } as u16;
        ret
    }
}