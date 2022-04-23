use crate::error::{PcanError, PcanOkError};
use crate::{CanFdFrame, CanFrame, DngBus, IsaBus, LanBus, PciBus, Timestamp, ToHandle, UsbBus};
use pcan_basic_sys as pcan;

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

/* CAN socket types */

pub struct IsaCanSocket {
    handle: u16,
}

impl IsaCanSocket {
    pub fn new(bus: IsaBus, baud: Baudrate) -> Result<IsaCanSocket, PcanError> {
        let handle = bus.handle();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(IsaCanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub struct DngCanSocket {
    handle: u16,
}

impl DngCanSocket {
    pub fn new(bus: DngBus, baud: Baudrate) -> Result<DngCanSocket, PcanError> {
        let handle = bus.handle();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(DngCanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub struct PciCanSocket {
    handle: u16,
}

impl PciCanSocket {
    pub fn new(bus: PciBus, baud: Baudrate) -> Result<PciCanSocket, PcanError> {
        let handle = bus.handle();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(PciCanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub struct PccCanSocket {
    handle: u16,
}

impl PccCanSocket {
    pub fn new(bus: PciBus, baud: Baudrate) -> Result<PccCanSocket, PcanError> {
        let handle = bus.handle();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(PccCanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub struct UsbCanSocket {
    handle: u16,
}

impl UsbCanSocket {
    pub fn new(bus: UsbBus, baud: Baudrate) -> Result<UsbCanSocket, PcanError> {
        let handle = bus.handle();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(UsbCanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub struct LanCanSocket {
    handle: u16,
}

impl LanCanSocket {
    pub fn new(bus: LanBus, baud: Baudrate) -> Result<LanCanSocket, PcanError> {
        let handle = bus.handle();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(LanCanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub struct CanSocket {
    handle: u16,
}

impl CanSocket {
    pub fn new<T: ToHandle>(bus: T, baud: Baudrate) -> Result<CanSocket, PcanError> {
        let handle = bus.handle();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(CanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Socket trait implementations */

impl Socket for IsaCanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

impl Socket for DngCanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

impl Socket for PciCanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

impl Socket for PccCanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

impl Socket for UsbCanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

impl Socket for LanCanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

impl Socket for CanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

/* HasCanRead trait implementations */

impl HasCanRead for IsaCanSocket {}
impl HasCanRead for DngCanSocket {}
impl HasCanRead for PciCanSocket {}
impl HasCanRead for PccCanSocket {}
impl HasCanRead for UsbCanSocket {}
impl HasCanRead for LanCanSocket {}
impl HasCanRead for CanSocket {}

/* HasCanReadFd trait implementations */

impl HasCanReadFd for IsaCanSocket {}
impl HasCanReadFd for DngCanSocket {}
impl HasCanReadFd for PciCanSocket {}
impl HasCanReadFd for PccCanSocket {}
impl HasCanReadFd for UsbCanSocket {}
impl HasCanReadFd for LanCanSocket {}
impl HasCanReadFd for CanSocket {}

/* HasCanWrite trait implementations */

impl HasCanWrite for IsaCanSocket {}
impl HasCanWrite for DngCanSocket {}
impl HasCanWrite for PciCanSocket {}
impl HasCanWrite for PccCanSocket {}
impl HasCanWrite for UsbCanSocket {}
impl HasCanWrite for LanCanSocket {}
impl HasCanWrite for CanSocket {}

/* HasCanWriteFd trait implementations */

impl HasCanWriteFd for IsaCanSocket {}
impl HasCanWriteFd for DngCanSocket {}
impl HasCanWriteFd for PciCanSocket {}
impl HasCanWriteFd for PccCanSocket {}
impl HasCanWriteFd for UsbCanSocket {}
impl HasCanWriteFd for LanCanSocket {}
impl HasCanWriteFd for CanSocket {}

/* Drop trait implementations */

struct SocketDropWrapper<T: Socket> {
    socket: T,
}

impl<T: Socket> Drop for SocketDropWrapper<T> {
    fn drop(&mut self) {
        unsafe { pcan::CAN_Uninitialize(self.socket.handle()) };
    }
}

/* CanRead trait implementations */

impl<T: Socket + HasCanRead> CanRead for T {
    fn read(&self) -> Result<(CanFrame, Timestamp), PcanError> {
        let mut frame = CanFrame::default();
        let mut timestamp = Timestamp::default();

        let error_code = unsafe {
            pcan::CAN_Read(
                self.handle(),
                &mut frame.frame as *mut pcan::TPCANMsg,
                &mut timestamp.timestamp as *mut pcan::TPCANTimestamp,
            )
        };

        match PcanOkError::try_from(error_code) {
            Ok(PcanOkError::Ok) => Ok((frame, timestamp)),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn read_frame(&self) -> Result<CanFrame, PcanError> {
        let mut frame = CanFrame::default();

        let error_code = unsafe {
            pcan::CAN_Read(
                self.handle(),
                &mut frame.frame as *mut pcan::TPCANMsg,
                0 as *mut pcan::TPCANTimestamp,
            )
        };

        match PcanOkError::try_from(error_code) {
            Ok(PcanOkError::Ok) => Ok(frame),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* CanFdRead trait implementation */

impl<T: Socket + HasCanReadFd> CanReadFd for T {
    fn read(&self) -> Result<(CanFdFrame, u64), PcanError> {
        let mut frame = CanFdFrame::default();
        let mut timestamp = 0u64;

        let error_code = unsafe {
            pcan::CAN_ReadFD(
                self.handle(),
                &mut frame.frame as *mut pcan::TPCANMsgFD,
                &mut timestamp as *mut u64,
            )
        };

        match PcanOkError::try_from(error_code) {
            Ok(PcanOkError::Ok) => Ok((frame, timestamp)),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn read_frame(&self) -> Result<CanFdFrame, PcanError> {
        let mut frame = CanFdFrame::default();

        let error_code = unsafe {
            pcan::CAN_ReadFD(
                self.handle(),
                &mut frame.frame as *mut pcan::TPCANMsgFD,
                0 as *mut u64,
            )
        };

        match PcanOkError::try_from(error_code) {
            Ok(PcanOkError::Ok) => Ok(frame),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* CanWrite trait implementations */

impl<T: Socket + HasCanWrite> CanWrite for T {
    fn write(&self, frame: CanFrame) -> Result<(), PcanError> {
        let mut frame = frame;
        let error_code =
            unsafe { pcan::CAN_Write(self.handle(), &mut frame.frame as *mut pcan::TPCANMsg) };

        match PcanOkError::try_from(error_code) {
            Ok(PcanOkError::Ok) => Ok(()),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* CanWriteFd trait implementation */

impl<T: Socket + HasCanWriteFd> CanWriteFd for T {
    fn write(&self, frame: CanFdFrame) -> Result<(), PcanError> {
        let mut frame = frame;
        let error_code =
            unsafe { pcan::CAN_WriteFD(self.handle(), &mut frame.frame as *mut pcan::TPCANMsgFD) };

        match PcanOkError::try_from(error_code) {
            Ok(PcanOkError::Ok) => Ok(()),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}
