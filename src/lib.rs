use pcan_basic_sys as pcan;
use std::os::raw::c_void;

pub mod error;
pub mod io;

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

pub const STANDARD_MASK: u32 = 0x07_FF;
pub const EXTENDED_MASK: u32 = 0x1F_FF_FF_FF;

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

/* TRAITS */

pub trait ToHandle {
    fn handle(&self) -> u16;
}

pub enum CanStatus {}

pub enum ChannelConditionStatus {
    Unavailable,
    Available,
    Occupied,
    PCANView,
}

pub trait HasChannelCondition {}

pub trait ChannelCondition {
    fn channel_condition(&self) -> Result<ChannelConditionStatus, ()>;
}

impl<T: HasChannelCondition + ToHandle> ChannelCondition for T {
    fn channel_condition(&self) -> Result<ChannelConditionStatus, ()> {
        let value: u32 = 0;
        let flag = unsafe {
            pcan::CAN_GetValue(
                self.handle(),
                pcan::PCAN_CHANNEL_CONDITION as u8,
                value as *mut c_void,
                4,
            )
        } == pcan::PCAN_ERROR_OK;

        if flag {
            if value & pcan::PCAN_CHANNEL_AVAILABLE == pcan::PCAN_CHANNEL_AVAILABLE {
                return Ok(ChannelConditionStatus::Available);
            }

            if value & pcan::PCAN_CHANNEL_UNAVAILABLE == pcan::PCAN_CHANNEL_UNAVAILABLE {
                return Ok(ChannelConditionStatus::Unavailable);
            }

            if value & pcan::PCAN_CHANNEL_OCCUPIED == pcan::PCAN_CHANNEL_OCCUPIED {
                return Ok(ChannelConditionStatus::Occupied);
            }

            if value & pcan::PCAN_CHANNEL_PCANVIEW == pcan::PCAN_CHANNEL_PCANVIEW {
                return Ok(ChannelConditionStatus::PCANView);
            }

            Err(())
        } else {
            Err(())
        }
    }
}

/* Channel Identifying */

pub trait HasChannelIdentifying {}

pub trait ChannelIdentifying {
    fn enable_identifying(&self) -> Result<(), ()>;
    fn disable_identifying(&self) -> Result<(), ()>;
    fn identifying(&self, value: bool) -> Result<(), ()> {
        match value {
            false => self.disable_identifying(),
            true => self.enable_identifying(),
        }
    }
}

impl<T: HasChannelIdentifying + ToHandle> ChannelIdentifying for T {
    fn enable_identifying(&self) -> Result<(), ()> {
        let activate = pcan::PCAN_PARAMETER_ON;
        let flag = unsafe {
            pcan::CAN_SetValue(
                self.handle(),
                pcan::PCAN_CHANNEL_IDENTIFYING as u8,
                activate as *mut c_void,
                4,
            )
        } == pcan::PCAN_ERROR_OK;

        match flag {
            true => Ok(()),
            false => Err(()),
        }
    }

    fn disable_identifying(&self) -> Result<(), ()> {
        let activate = pcan::PCAN_PARAMETER_OFF;
        let flag = unsafe {
            pcan::CAN_SetValue(
                self.handle(),
                pcan::PCAN_CHANNEL_IDENTIFYING as u8,
                activate as *mut c_void,
                4,
            )
        } == pcan::PCAN_ERROR_OK;

        match flag {
            true => Ok(()),
            false => Err(()),
        }
    }
}

/* Device Id */

pub trait HasDeviceId {}

pub trait DeviceId {
    fn device_id(&self) -> Result<u32, ()>;
}

impl<T: HasDeviceId + ToHandle> DeviceId for T {
    fn device_id(&self) -> Result<u32, ()> {
        let devive_id = 0u32;
        let flag = unsafe {
            pcan::CAN_GetValue(
                self.handle(),
                pcan::PCAN_DEVICE_ID as u8,
                devive_id as *mut c_void,
                4,
            )
        } == pcan::PCAN_ERROR_OK;

        match flag {
            true => Ok(devive_id),
            false => Err(()),
        }
    }
}

/* Hardware Name */

pub trait HasHardwareName {}

pub trait HardwareName {
    fn hardware_name(&self) -> Result<String, ()>;
}

impl<T: HasHardwareName + ToHandle> HardwareName for T {
    fn hardware_name(&self) -> Result<String, ()> {
        let mut hardware_name = [0u8; pcan::MAX_LENGTH_HARDWARE_NAME as usize];
        let flag = unsafe {
            pcan::CAN_GetValue(
                self.handle(),
                pcan::PCAN_HARDWARE_NAME as u8,
                hardware_name.as_mut_ptr() as *mut c_void,
                pcan::MAX_LENGTH_HARDWARE_NAME,
            )
        } == pcan::PCAN_ERROR_OK;

        match flag {
            true => {
                let s = std::str::from_utf8(&hardware_name);
                match s {
                    Ok(s) => Ok(String::from(s)),
                    Err(_) => Err(()),
                }
            }
            false => Err(()),
        }
    }
}

/* Controller Number */

pub trait HasControllerNumber {}

pub trait ControllerNumber {
    fn controller_number(&self) -> Result<u32, ()>;
}

impl<T: HasControllerNumber + ToHandle> ControllerNumber for T {
    fn controller_number(&self) -> Result<u32, ()> {
        let controller_number = 0u32;
        let flag = unsafe {
            pcan::CAN_GetValue(
                self.handle(),
                pcan::PCAN_CONTROLLER_NUMBER as u8,
                controller_number as *mut c_void,
                4,
            )
        } == pcan::PCAN_ERROR_OK;

        match flag {
            true => Ok(controller_number),
            false => Err(()),
        }
    }
}

/* Device Part Number */

pub trait HasDevicePartNumber {}

pub trait DevicePartNumber {
    fn device_part_number(&self) -> Result<String, ()>;
}

impl<T: HasDevicePartNumber + ToHandle> DevicePartNumber for T {
    fn device_part_number(&self) -> Result<String, ()> {
        let mut device_part_number = [0u8; 100];
        let flag = unsafe {
            pcan::CAN_GetValue(
                self.handle(),
                pcan::PCAN_DEVICE_PART_NUMBER as u8,
                device_part_number.as_mut_ptr() as *mut c_void,
                100,
            )
        } == pcan::PCAN_ERROR_OK;

        match flag {
            true => {
                let s = std::str::from_utf8(&device_part_number);
                match s {
                    Ok(s) => Ok(String::from(s)),
                    Err(_) => Err(()),
                }
            }
            false => Err(()),
        }
    }
}

/* BUS SECTION */

///
pub enum IsaBus {
    ///
    ISA1,
    ///
    ISA2,
    ///
    ISA3,
    ///
    ISA4,
    ///
    ISA5,
    ///
    ISA6,
    ///
    ISA7,
    ///
    ISA8,
}

impl ToHandle for IsaBus {
    fn handle(&self) -> u16 {
        match self {
            IsaBus::ISA1 => pcan::PCAN_ISABUS1 as u16,
            IsaBus::ISA2 => pcan::PCAN_ISABUS2 as u16,
            IsaBus::ISA3 => pcan::PCAN_ISABUS3 as u16,
            IsaBus::ISA4 => pcan::PCAN_ISABUS4 as u16,
            IsaBus::ISA5 => pcan::PCAN_ISABUS5 as u16,
            IsaBus::ISA6 => pcan::PCAN_ISABUS6 as u16,
            IsaBus::ISA7 => pcan::PCAN_ISABUS7 as u16,
            IsaBus::ISA8 => pcan::PCAN_ISABUS8 as u16,
        }
    }
}

impl HasChannelCondition for IsaBus {}
impl HasHardwareName for IsaBus {}
impl HasControllerNumber for IsaBus {}
impl HasDevicePartNumber for IsaBus {}

///
pub enum DngBus {
    ///
    DNG1,
}

impl ToHandle for DngBus {
    fn handle(&self) -> u16 {
        match self {
            DngBus::DNG1 => pcan::PCAN_DNGBUS1 as u16,
        }
    }
}

impl HasChannelCondition for DngBus {}
impl HasHardwareName for DngBus {}
impl HasControllerNumber for DngBus {}
impl HasDevicePartNumber for DngBus {}

///
pub enum PciBus {
    ///
    PCI1,
    ///
    PCI2,
    ///
    PCI3,
    ///
    PCI4,
    ///
    PCI5,
    ///
    PCI6,
    ///
    PCI7,
    ///
    PCI8,
    ///
    PCI9,
    ///
    PCI10,
    ///
    PCI11,
    ///
    PCI12,
    ///
    PCI13,
    ///
    PCI14,
    ///
    PCI15,
    ///
    PCI16,
}

impl ToHandle for PciBus {
    fn handle(&self) -> u16 {
        match self {
            PciBus::PCI1 => pcan::PCAN_PCIBUS1 as u16,
            PciBus::PCI2 => pcan::PCAN_PCIBUS2 as u16,
            PciBus::PCI3 => pcan::PCAN_PCIBUS3 as u16,
            PciBus::PCI4 => pcan::PCAN_PCIBUS4 as u16,
            PciBus::PCI5 => pcan::PCAN_PCIBUS5 as u16,
            PciBus::PCI6 => pcan::PCAN_PCIBUS6 as u16,
            PciBus::PCI7 => pcan::PCAN_PCIBUS7 as u16,
            PciBus::PCI8 => pcan::PCAN_PCIBUS8 as u16,
            PciBus::PCI9 => pcan::PCAN_PCIBUS9 as u16,
            PciBus::PCI10 => pcan::PCAN_PCIBUS10 as u16,
            PciBus::PCI11 => pcan::PCAN_PCIBUS11 as u16,
            PciBus::PCI12 => pcan::PCAN_PCIBUS12 as u16,
            PciBus::PCI13 => pcan::PCAN_PCIBUS13 as u16,
            PciBus::PCI14 => pcan::PCAN_PCIBUS14 as u16,
            PciBus::PCI15 => pcan::PCAN_PCIBUS15 as u16,
            PciBus::PCI16 => pcan::PCAN_PCIBUS16 as u16,
        }
    }
}

impl HasChannelCondition for PciBus {}
impl HasHardwareName for PciBus {}
impl HasControllerNumber for PciBus {}
impl HasDevicePartNumber for PciBus {}

///
pub enum UsbBus {
    ///
    USB1,
    ///
    USB2,
    ///
    USB3,
    ///
    USB4,
    ///
    USB5,
    ///
    USB6,
    ///
    USB7,
    ///
    USB8,
    ///
    USB9,
    ///
    USB10,
    ///
    USB11,
    ///
    USB12,
    ///
    USB13,
    ///
    USB14,
    ///
    USB15,
    ///
    USB16,
}

impl ToHandle for UsbBus {
    fn handle(&self) -> u16 {
        match self {
            UsbBus::USB1 => pcan::PCAN_USBBUS1 as u16,
            UsbBus::USB2 => pcan::PCAN_USBBUS2 as u16,
            UsbBus::USB3 => pcan::PCAN_USBBUS3 as u16,
            UsbBus::USB4 => pcan::PCAN_USBBUS4 as u16,
            UsbBus::USB5 => pcan::PCAN_USBBUS5 as u16,
            UsbBus::USB6 => pcan::PCAN_USBBUS6 as u16,
            UsbBus::USB7 => pcan::PCAN_USBBUS7 as u16,
            UsbBus::USB8 => pcan::PCAN_USBBUS8 as u16,
            UsbBus::USB9 => pcan::PCAN_USBBUS9 as u16,
            UsbBus::USB10 => pcan::PCAN_USBBUS10 as u16,
            UsbBus::USB11 => pcan::PCAN_USBBUS11 as u16,
            UsbBus::USB12 => pcan::PCAN_USBBUS12 as u16,
            UsbBus::USB13 => pcan::PCAN_USBBUS13 as u16,
            UsbBus::USB14 => pcan::PCAN_USBBUS14 as u16,
            UsbBus::USB15 => pcan::PCAN_USBBUS15 as u16,
            UsbBus::USB16 => pcan::PCAN_USBBUS16 as u16,
        }
    }
}

impl HasChannelCondition for UsbBus {}
impl HasChannelIdentifying for UsbBus {}
impl HasDeviceId for UsbBus {}
impl HasHardwareName for UsbBus {}
impl HasControllerNumber for UsbBus {}
impl HasDevicePartNumber for UsbBus {}

///
pub enum PccBus {
    ///
    PCC1,
    ///
    PCC2,
}

impl ToHandle for PccBus {
    fn handle(&self) -> u16 {
        match self {
            PccBus::PCC1 => pcan::PCAN_PCCBUS1 as u16,
            PccBus::PCC2 => pcan::PCAN_PCCBUS2 as u16,
        }
    }
}

impl HasChannelCondition for PccBus {}
impl HasHardwareName for PccBus {}
impl HasControllerNumber for PccBus {}
impl HasDevicePartNumber for PccBus {}

///
pub enum LanBus {
    ///
    LAN1,
    ///
    LAN2,
    ///
    LAN3,
    ///
    LAN4,
    ///
    LAN5,
    ///
    LAN6,
    ///
    LAN7,
    ///
    LAN8,
    ///
    LAN9,
    ///
    LAN10,
    ///
    LAN11,
    ///
    LAN12,
    ///
    LAN13,
    ///
    LAN14,
    ///
    LAN15,
    ///
    LAN16,
}

impl ToHandle for LanBus {
    fn handle(&self) -> u16 {
        match self {
            LanBus::LAN1 => pcan::PCAN_LANBUS1 as u16,
            LanBus::LAN2 => pcan::PCAN_LANBUS2 as u16,
            LanBus::LAN3 => pcan::PCAN_LANBUS3 as u16,
            LanBus::LAN4 => pcan::PCAN_LANBUS4 as u16,
            LanBus::LAN5 => pcan::PCAN_LANBUS5 as u16,
            LanBus::LAN6 => pcan::PCAN_LANBUS6 as u16,
            LanBus::LAN7 => pcan::PCAN_LANBUS7 as u16,
            LanBus::LAN8 => pcan::PCAN_LANBUS8 as u16,
            LanBus::LAN9 => pcan::PCAN_LANBUS9 as u16,
            LanBus::LAN10 => pcan::PCAN_LANBUS10 as u16,
            LanBus::LAN11 => pcan::PCAN_LANBUS11 as u16,
            LanBus::LAN12 => pcan::PCAN_LANBUS12 as u16,
            LanBus::LAN13 => pcan::PCAN_LANBUS13 as u16,
            LanBus::LAN14 => pcan::PCAN_LANBUS14 as u16,
            LanBus::LAN15 => pcan::PCAN_LANBUS15 as u16,
            LanBus::LAN16 => pcan::PCAN_LANBUS16 as u16,
        }
    }
}

impl HasChannelCondition for LanBus {}
impl HasHardwareName for LanBus {}
impl HasControllerNumber for LanBus {}
impl HasDevicePartNumber for LanBus {}

pub struct Channel {
    handle: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_frame_new_001() {
        let can_frame_1 =
            CanFrame::new(0x20, MessageType::Standard, &[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();

        let can_frame_2 =
            CanFrame::new(0x20, MessageType::Standard, &[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();

        assert_eq!(can_frame_1, can_frame_2);
    }

    #[test]
    fn can_frame_new_002() {
        let can_frame_1 =
            CanFrame::new(0x20, MessageType::Extended, &[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();

        let can_frame_2 =
            CanFrame::new(0x20, MessageType::Extended, &[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();

        assert_eq!(can_frame_1, can_frame_2);
    }

    #[test]
    #[should_panic]
    fn can_frame_new_003() {
        let _can_frame_1 =
            CanFrame::new(0x20, MessageType::Standard, &[0, 1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
    }

    #[test]
    #[should_panic]
    fn can_frame_new_004() {
        let _can_frame_1 =
            CanFrame::new(0x20, MessageType::Extended, &[0, 1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
    }

    /* CAN FD FRAME */

    #[test]
    fn can_fd_frame_new_001() {
        let can_frame_1 =
            CanFdFrame::new(0x20, MessageType::Standard, &(0..64u8).collect::<Vec<_>>()).unwrap();

        let can_frame_2 =
            CanFdFrame::new(0x20, MessageType::Standard, &(0..64u8).collect::<Vec<_>>()).unwrap();

        assert_eq!(can_frame_1, can_frame_2);
    }

    #[test]
    fn can_fd_frame_new_002() {
        let can_frame_1 =
            CanFdFrame::new(0x20, MessageType::Extended, &(0..64u8).collect::<Vec<_>>()).unwrap();

        let can_frame_2 =
            CanFdFrame::new(0x20, MessageType::Extended, &(0..64u8).collect::<Vec<_>>()).unwrap();

        assert_eq!(can_frame_1, can_frame_2);
    }

    #[test]
    #[should_panic]
    fn can_fd_frame_new_003() {
        let _can_frame_1 =
            CanFdFrame::new(0x20, MessageType::Standard, &(0..65u8).collect::<Vec<_>>()).unwrap();
    }

    #[test]
    #[should_panic]
    fn can_fd_frame_new_004() {
        let _can_frame_1 =
            CanFrame::new(0x20, MessageType::Extended, &&(0..65u8).collect::<Vec<_>>()).unwrap();
    }

    /* CHANNEL CONDITION */
    #[test]
    #[should_panic]
    fn usb_bus_channel_condition_001() {
        let usb = UsbBus::USB1;
        usb.channel_condition().unwrap();
    }
}
