//!
//!

use crate::error::{PcanError, PcanOkError};
use crate::hw_ident::{
    ChannelCondition, ChannelConditionStatus, ChannelIdentifying, ControllerNumber, DeviceId,
    DevicePartNumber, HardwareName,
};
use crate::info::{ChannelFeatures, ChannelVersion, Version};
use crate::pcan;
use std::os::raw::c_void;

pub trait Bus {
    fn channel(&self) -> u16;
}

///
#[derive(Debug, PartialEq, Copy, Clone)]
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

impl From<IsaBus> for u16 {
    fn from(value: IsaBus) -> Self {
        let ret = match value {
            IsaBus::ISA1 => pcan::PCAN_ISABUS1,
            IsaBus::ISA2 => pcan::PCAN_ISABUS2,
            IsaBus::ISA3 => pcan::PCAN_ISABUS3,
            IsaBus::ISA4 => pcan::PCAN_ISABUS4,
            IsaBus::ISA5 => pcan::PCAN_ISABUS5,
            IsaBus::ISA6 => pcan::PCAN_ISABUS6,
            IsaBus::ISA7 => pcan::PCAN_ISABUS7,
            IsaBus::ISA8 => pcan::PCAN_ISABUS8,
        } as u16;
        ret
    }
}

impl TryFrom<u16> for IsaBus {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value as u32 {
            pcan::PCAN_ISABUS1 => Ok(IsaBus::ISA1),
            pcan::PCAN_ISABUS2 => Ok(IsaBus::ISA2),
            pcan::PCAN_ISABUS3 => Ok(IsaBus::ISA3),
            pcan::PCAN_ISABUS4 => Ok(IsaBus::ISA4),
            pcan::PCAN_ISABUS5 => Ok(IsaBus::ISA5),
            pcan::PCAN_ISABUS6 => Ok(IsaBus::ISA6),
            pcan::PCAN_ISABUS7 => Ok(IsaBus::ISA7),
            pcan::PCAN_ISABUS8 => Ok(IsaBus::ISA8),
            _ => Err(()),
        }
    }
}

impl Bus for IsaBus {
    fn channel(&self) -> u16 {
        u16::from(*self)
    }
}

/* Hardware Identification */

impl ChannelCondition for IsaBus {
    fn channel_condition(&self) -> Result<ChannelConditionStatus, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_CONDITION as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        let value: u32 = u32::from_le_bytes(data);
        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match ChannelConditionStatus::try_from(value) {
                Ok(status) => Ok(status),
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl HardwareName for IsaBus {
    fn hardware_name(&self) -> Result<String, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_HARDWARE_NAME as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_HARDWARE_NAME as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let s = s.trim_matches(char::from(0));
                    Ok(String::from(s))
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl ControllerNumber for IsaBus {
    fn controller_number(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CONTROLLER_NUMBER as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(u32::from_le_bytes(data)),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl DevicePartNumber for IsaBus {
    fn device_part_number(&self) -> Result<String, PcanError> {
        let mut data = [0u8; 100];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_DEVICE_PART_NUMBER as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let s = s.trim_matches(char::from(0));
                    Ok(String::from(s))
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Informational Parameters */

impl ChannelVersion for IsaBus {
    fn channel_version(&self) -> Result<Version, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_VERSION_STRING as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_VERSION as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let newlines = s.lines().collect::<Vec<_>>();
                    if newlines.len() == 4 {
                        let newlines = newlines
                            .iter()
                            .map(|s| s.trim_matches(char::from(0)))
                            .collect::<Vec<_>>();

                        Ok(Version {
                            device_driver_name_and_version: String::from(newlines[0]),
                            architecture: String::from(newlines[1]),
                            year_of_copyright: String::from(newlines[2]),
                            company_name_and_city: String::from(newlines[4]),
                        })
                    } else {
                        Err(PcanError::Unknown)
                    }
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl ChannelFeatures for IsaBus {
    fn is_fd_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_FD_CAPABLE == pcan::FEATURE_FD_CAPABLE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn is_delay_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_DELAY_CAPABLE == pcan::FEATURE_DELAY_CAPABLE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn is_io_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_IO_CAPABLE == pcan::FEATURE_IO_CAPABLE {
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

///
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DngBus {
    ///
    DNG1,
}

impl From<DngBus> for u16 {
    fn from(value: DngBus) -> Self {
        let ret = match value {
            DngBus::DNG1 => pcan::PCAN_DNGBUS1,
        } as u16;
        ret
    }
}

impl TryFrom<u16> for DngBus {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value as u32 {
            pcan::PCAN_DNGBUS1 => Ok(DngBus::DNG1),
            _ => Err(()),
        }
    }
}

impl Bus for DngBus {
    fn channel(&self) -> u16 {
        u16::from(*self)
    }
}

/* Hardware Identification */

impl ChannelCondition for DngBus {
    fn channel_condition(&self) -> Result<ChannelConditionStatus, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_CONDITION as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        let value: u32 = u32::from_le_bytes(data);
        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match ChannelConditionStatus::try_from(value) {
                Ok(status) => Ok(status),
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl HardwareName for DngBus {
    fn hardware_name(&self) -> Result<String, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_HARDWARE_NAME as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_HARDWARE_NAME as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let s = s.trim_matches(char::from(0));
                    Ok(String::from(s))
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl ControllerNumber for DngBus {
    fn controller_number(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CONTROLLER_NUMBER as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(u32::from_le_bytes(data)),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl DevicePartNumber for DngBus {
    fn device_part_number(&self) -> Result<String, PcanError> {
        let mut data = [0u8; 100];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_DEVICE_PART_NUMBER as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let s = s.trim_matches(char::from(0));
                    Ok(String::from(s))
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Informational Parameters */

impl ChannelVersion for DngBus {
    fn channel_version(&self) -> Result<Version, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_VERSION_STRING as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_VERSION as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let newlines = s.lines().collect::<Vec<_>>();
                    if newlines.len() == 4 {
                        let newlines = newlines
                            .iter()
                            .map(|s| s.trim_matches(char::from(0)))
                            .collect::<Vec<_>>();

                        Ok(Version {
                            device_driver_name_and_version: String::from(newlines[0]),
                            architecture: String::from(newlines[1]),
                            year_of_copyright: String::from(newlines[2]),
                            company_name_and_city: String::from(newlines[4]),
                        })
                    } else {
                        Err(PcanError::Unknown)
                    }
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl ChannelFeatures for DngBus {
    fn is_fd_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_FD_CAPABLE == pcan::FEATURE_FD_CAPABLE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn is_delay_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_DELAY_CAPABLE == pcan::FEATURE_DELAY_CAPABLE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn is_io_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_IO_CAPABLE == pcan::FEATURE_IO_CAPABLE {
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

///
#[derive(Debug, PartialEq, Copy, Clone)]
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

impl From<PciBus> for u16 {
    fn from(value: PciBus) -> Self {
        let ret = match value {
            PciBus::PCI1 => pcan::PCAN_PCIBUS1,
            PciBus::PCI2 => pcan::PCAN_PCIBUS2,
            PciBus::PCI3 => pcan::PCAN_PCIBUS3,
            PciBus::PCI4 => pcan::PCAN_PCIBUS4,
            PciBus::PCI5 => pcan::PCAN_PCIBUS5,
            PciBus::PCI6 => pcan::PCAN_PCIBUS6,
            PciBus::PCI7 => pcan::PCAN_PCIBUS7,
            PciBus::PCI8 => pcan::PCAN_PCIBUS8,
            PciBus::PCI9 => pcan::PCAN_PCIBUS9,
            PciBus::PCI10 => pcan::PCAN_PCIBUS10,
            PciBus::PCI11 => pcan::PCAN_PCIBUS11,
            PciBus::PCI12 => pcan::PCAN_PCIBUS12,
            PciBus::PCI13 => pcan::PCAN_PCIBUS13,
            PciBus::PCI14 => pcan::PCAN_PCIBUS14,
            PciBus::PCI15 => pcan::PCAN_PCIBUS15,
            PciBus::PCI16 => pcan::PCAN_PCIBUS16,
        } as u16;
        ret
    }
}

impl TryFrom<u16> for PciBus {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value as u32 {
            pcan::PCAN_PCIBUS1 => Ok(PciBus::PCI1),
            pcan::PCAN_PCIBUS2 => Ok(PciBus::PCI2),
            pcan::PCAN_PCIBUS3 => Ok(PciBus::PCI3),
            pcan::PCAN_PCIBUS4 => Ok(PciBus::PCI4),
            pcan::PCAN_PCIBUS5 => Ok(PciBus::PCI5),
            pcan::PCAN_PCIBUS6 => Ok(PciBus::PCI6),
            pcan::PCAN_PCIBUS7 => Ok(PciBus::PCI7),
            pcan::PCAN_PCIBUS8 => Ok(PciBus::PCI8),
            pcan::PCAN_PCIBUS9 => Ok(PciBus::PCI1),
            pcan::PCAN_PCIBUS10 => Ok(PciBus::PCI10),
            pcan::PCAN_PCIBUS11 => Ok(PciBus::PCI11),
            pcan::PCAN_PCIBUS12 => Ok(PciBus::PCI12),
            pcan::PCAN_PCIBUS13 => Ok(PciBus::PCI13),
            pcan::PCAN_PCIBUS14 => Ok(PciBus::PCI14),
            pcan::PCAN_PCIBUS15 => Ok(PciBus::PCI15),
            pcan::PCAN_PCIBUS16 => Ok(PciBus::PCI16),
            _ => Err(()),
        }
    }
}

impl Bus for PciBus {
    fn channel(&self) -> u16 {
        u16::from(*self)
    }
}

/* Hardware Identification */

impl ChannelCondition for PciBus {
    fn channel_condition(&self) -> Result<ChannelConditionStatus, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_CONDITION as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        let value: u32 = u32::from_le_bytes(data);
        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match ChannelConditionStatus::try_from(value) {
                Ok(status) => Ok(status),
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl DeviceId for PciBus {
    fn device_id(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_DEVICE_ID as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(u32::from_le_bytes(data)),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl HardwareName for PciBus {
    fn hardware_name(&self) -> Result<String, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_HARDWARE_NAME as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_HARDWARE_NAME as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let s = s.trim_matches(char::from(0));
                    Ok(String::from(s))
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl ControllerNumber for PciBus {
    fn controller_number(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CONTROLLER_NUMBER as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(u32::from_le_bytes(data)),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl DevicePartNumber for PciBus {
    fn device_part_number(&self) -> Result<String, PcanError> {
        let mut data = [0u8; 100];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_DEVICE_PART_NUMBER as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let s = s.trim_matches(char::from(0));
                    Ok(String::from(s))
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Informational Parameters */

impl ChannelVersion for PciBus {
    fn channel_version(&self) -> Result<Version, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_VERSION_STRING as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_VERSION as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let newlines = s.lines().collect::<Vec<_>>();
                    if newlines.len() == 4 {
                        let newlines = newlines
                            .iter()
                            .map(|s| s.trim_matches(char::from(0)))
                            .collect::<Vec<_>>();

                        Ok(Version {
                            device_driver_name_and_version: String::from(newlines[0]),
                            architecture: String::from(newlines[1]),
                            year_of_copyright: String::from(newlines[2]),
                            company_name_and_city: String::from(newlines[4]),
                        })
                    } else {
                        Err(PcanError::Unknown)
                    }
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl ChannelFeatures for PciBus {
    fn is_fd_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_FD_CAPABLE == pcan::FEATURE_FD_CAPABLE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn is_delay_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_DELAY_CAPABLE == pcan::FEATURE_DELAY_CAPABLE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn is_io_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_IO_CAPABLE == pcan::FEATURE_IO_CAPABLE {
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

///
#[derive(Debug, PartialEq, Copy, Clone)]
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

impl From<UsbBus> for u16 {
    fn from(value: UsbBus) -> Self {
        let ret = match value {
            UsbBus::USB1 => pcan::PCAN_USBBUS1,
            UsbBus::USB2 => pcan::PCAN_USBBUS2,
            UsbBus::USB3 => pcan::PCAN_USBBUS3,
            UsbBus::USB4 => pcan::PCAN_USBBUS4,
            UsbBus::USB5 => pcan::PCAN_USBBUS5,
            UsbBus::USB6 => pcan::PCAN_USBBUS6,
            UsbBus::USB7 => pcan::PCAN_USBBUS7,
            UsbBus::USB8 => pcan::PCAN_USBBUS8,
            UsbBus::USB9 => pcan::PCAN_USBBUS9,
            UsbBus::USB10 => pcan::PCAN_USBBUS10,
            UsbBus::USB11 => pcan::PCAN_USBBUS11,
            UsbBus::USB12 => pcan::PCAN_USBBUS12,
            UsbBus::USB13 => pcan::PCAN_USBBUS13,
            UsbBus::USB14 => pcan::PCAN_USBBUS14,
            UsbBus::USB15 => pcan::PCAN_USBBUS15,
            UsbBus::USB16 => pcan::PCAN_USBBUS16,
        } as u16;
        ret
    }
}

impl TryFrom<u16> for UsbBus {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value as u32 {
            pcan::PCAN_USBBUS1 => Ok(UsbBus::USB1),
            pcan::PCAN_USBBUS2 => Ok(UsbBus::USB2),
            pcan::PCAN_USBBUS3 => Ok(UsbBus::USB3),
            pcan::PCAN_USBBUS4 => Ok(UsbBus::USB4),
            pcan::PCAN_USBBUS5 => Ok(UsbBus::USB5),
            pcan::PCAN_USBBUS6 => Ok(UsbBus::USB6),
            pcan::PCAN_USBBUS7 => Ok(UsbBus::USB7),
            pcan::PCAN_USBBUS8 => Ok(UsbBus::USB8),
            pcan::PCAN_USBBUS9 => Ok(UsbBus::USB9),
            pcan::PCAN_USBBUS10 => Ok(UsbBus::USB10),
            pcan::PCAN_USBBUS11 => Ok(UsbBus::USB11),
            pcan::PCAN_USBBUS12 => Ok(UsbBus::USB12),
            pcan::PCAN_USBBUS13 => Ok(UsbBus::USB13),
            pcan::PCAN_USBBUS14 => Ok(UsbBus::USB14),
            pcan::PCAN_USBBUS15 => Ok(UsbBus::USB15),
            pcan::PCAN_USBBUS16 => Ok(UsbBus::USB16),
            _ => Err(()),
        }
    }
}

impl Bus for UsbBus {
    fn channel(&self) -> u16 {
        u16::from(*self)
    }
}

/* Hardware Identification */

impl ChannelCondition for UsbBus {
    fn channel_condition(&self) -> Result<ChannelConditionStatus, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_CONDITION as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        let value: u32 = u32::from_le_bytes(data);
        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match ChannelConditionStatus::try_from(value) {
                Ok(status) => Ok(status),
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl ChannelIdentifying for UsbBus {
    fn enable_identifying(&self) -> Result<(), PcanError> {
        let mut data = u32::to_le_bytes(pcan::PCAN_PARAMETER_ON);
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_IDENTIFYING as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(()),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn disable_identifying(&self) -> Result<(), PcanError> {
        let mut data = u32::to_le_bytes(pcan::PCAN_PARAMETER_OFF);
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_IDENTIFYING as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(()),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn is_identifying(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_IDENTIFYING as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
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

impl DeviceId for UsbBus {
    fn device_id(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_DEVICE_ID as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(u32::from_le_bytes(data)),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl HardwareName for UsbBus {
    fn hardware_name(&self) -> Result<String, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_HARDWARE_NAME as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_HARDWARE_NAME as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let s = s.trim_matches(char::from(0));
                    Ok(String::from(s))
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl ControllerNumber for UsbBus {
    fn controller_number(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CONTROLLER_NUMBER as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(u32::from_le_bytes(data)),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl DevicePartNumber for UsbBus {
    fn device_part_number(&self) -> Result<String, PcanError> {
        let mut data = [0u8; 100];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_DEVICE_PART_NUMBER as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let s = s.trim_matches(char::from(0));
                    Ok(String::from(s))
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Informational Parameters */

impl ChannelVersion for UsbBus {
    fn channel_version(&self) -> Result<Version, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_VERSION_STRING as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_VERSION as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let newlines = s.lines().collect::<Vec<_>>();
                    if newlines.len() == 4 {
                        let newlines = newlines
                            .iter()
                            .map(|s| s.trim_matches(char::from(0)))
                            .collect::<Vec<_>>();

                        Ok(Version {
                            device_driver_name_and_version: String::from(newlines[0]),
                            architecture: String::from(newlines[1]),
                            year_of_copyright: String::from(newlines[2]),
                            company_name_and_city: String::from(newlines[4]),
                        })
                    } else {
                        Err(PcanError::Unknown)
                    }
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl ChannelFeatures for UsbBus {
    fn is_fd_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_FD_CAPABLE == pcan::FEATURE_FD_CAPABLE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn is_delay_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_DELAY_CAPABLE == pcan::FEATURE_DELAY_CAPABLE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn is_io_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_IO_CAPABLE == pcan::FEATURE_IO_CAPABLE {
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

///
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PccBus {
    ///
    PCC1,
    ///
    PCC2,
}

impl From<PccBus> for u16 {
    fn from(value: PccBus) -> Self {
        let ret = match value {
            PccBus::PCC1 => pcan::PCAN_PCCBUS1,
            PccBus::PCC2 => pcan::PCAN_PCCBUS2,
        } as u16;
        ret
    }
}

impl TryFrom<u16> for PccBus {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value as u32 {
            pcan::PCAN_PCCBUS1 => Ok(PccBus::PCC1),
            pcan::PCAN_PCCBUS2 => Ok(PccBus::PCC2),
            _ => Err(()),
        }
    }
}

impl Bus for PccBus {
    fn channel(&self) -> u16 {
        u16::from(*self)
    }
}

/* Hardware Identification */

impl ChannelCondition for PccBus {
    fn channel_condition(&self) -> Result<ChannelConditionStatus, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_CONDITION as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        let value: u32 = u32::from_le_bytes(data);
        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match ChannelConditionStatus::try_from(value) {
                Ok(status) => Ok(status),
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl HardwareName for PccBus {
    fn hardware_name(&self) -> Result<String, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_HARDWARE_NAME as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_HARDWARE_NAME as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let s = s.trim_matches(char::from(0));
                    Ok(String::from(s))
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl ControllerNumber for PccBus {
    fn controller_number(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CONTROLLER_NUMBER as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(u32::from_le_bytes(data)),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl DevicePartNumber for PccBus {
    fn device_part_number(&self) -> Result<String, PcanError> {
        let mut data = [0u8; 100];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_DEVICE_PART_NUMBER as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let s = s.trim_matches(char::from(0));
                    Ok(String::from(s))
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Informational Parameters */

impl ChannelVersion for PccBus {
    fn channel_version(&self) -> Result<Version, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_VERSION_STRING as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_VERSION as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let newlines = s.lines().collect::<Vec<_>>();
                    if newlines.len() == 4 {
                        let newlines = newlines
                            .iter()
                            .map(|s| s.trim_matches(char::from(0)))
                            .collect::<Vec<_>>();

                        Ok(Version {
                            device_driver_name_and_version: String::from(newlines[0]),
                            architecture: String::from(newlines[1]),
                            year_of_copyright: String::from(newlines[2]),
                            company_name_and_city: String::from(newlines[4]),
                        })
                    } else {
                        Err(PcanError::Unknown)
                    }
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl ChannelFeatures for PccBus {
    fn is_fd_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_FD_CAPABLE == pcan::FEATURE_FD_CAPABLE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn is_delay_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_DELAY_CAPABLE == pcan::FEATURE_DELAY_CAPABLE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn is_io_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_IO_CAPABLE == pcan::FEATURE_IO_CAPABLE {
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

///
#[derive(Debug, PartialEq, Copy, Clone)]
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

impl From<LanBus> for u16 {
    fn from(value: LanBus) -> Self {
        let ret = match value {
            LanBus::LAN1 => pcan::PCAN_LANBUS1,
            LanBus::LAN2 => pcan::PCAN_LANBUS2,
            LanBus::LAN3 => pcan::PCAN_LANBUS3,
            LanBus::LAN4 => pcan::PCAN_LANBUS4,
            LanBus::LAN5 => pcan::PCAN_LANBUS5,
            LanBus::LAN6 => pcan::PCAN_LANBUS6,
            LanBus::LAN7 => pcan::PCAN_LANBUS7,
            LanBus::LAN8 => pcan::PCAN_LANBUS8,
            LanBus::LAN9 => pcan::PCAN_LANBUS9,
            LanBus::LAN10 => pcan::PCAN_LANBUS10,
            LanBus::LAN11 => pcan::PCAN_LANBUS11,
            LanBus::LAN12 => pcan::PCAN_LANBUS12,
            LanBus::LAN13 => pcan::PCAN_LANBUS13,
            LanBus::LAN14 => pcan::PCAN_LANBUS14,
            LanBus::LAN15 => pcan::PCAN_LANBUS15,
            LanBus::LAN16 => pcan::PCAN_LANBUS16,
        } as u16;
        ret
    }
}

impl TryFrom<u16> for LanBus {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value as u32 {
            pcan::PCAN_LANBUS1 => Ok(LanBus::LAN1),
            pcan::PCAN_LANBUS2 => Ok(LanBus::LAN2),
            pcan::PCAN_LANBUS3 => Ok(LanBus::LAN3),
            pcan::PCAN_LANBUS4 => Ok(LanBus::LAN4),
            pcan::PCAN_LANBUS5 => Ok(LanBus::LAN5),
            pcan::PCAN_LANBUS6 => Ok(LanBus::LAN6),
            pcan::PCAN_LANBUS7 => Ok(LanBus::LAN7),
            pcan::PCAN_LANBUS8 => Ok(LanBus::LAN8),
            pcan::PCAN_LANBUS9 => Ok(LanBus::LAN9),
            pcan::PCAN_LANBUS10 => Ok(LanBus::LAN10),
            pcan::PCAN_LANBUS11 => Ok(LanBus::LAN11),
            pcan::PCAN_LANBUS12 => Ok(LanBus::LAN12),
            pcan::PCAN_LANBUS13 => Ok(LanBus::LAN13),
            pcan::PCAN_LANBUS14 => Ok(LanBus::LAN14),
            pcan::PCAN_LANBUS15 => Ok(LanBus::LAN15),
            pcan::PCAN_LANBUS16 => Ok(LanBus::LAN16),
            _ => Err(()),
        }
    }
}

impl Bus for LanBus {
    fn channel(&self) -> u16 {
        u16::from(*self)
    }
}

/* Hardware Identification */

impl ChannelCondition for LanBus {
    fn channel_condition(&self) -> Result<ChannelConditionStatus, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_CONDITION as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        let value: u32 = u32::from_le_bytes(data);
        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match ChannelConditionStatus::try_from(value) {
                Ok(status) => Ok(status),
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl DeviceId for LanBus {
    fn device_id(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_DEVICE_ID as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(u32::from_le_bytes(data)),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl HardwareName for LanBus {
    fn hardware_name(&self) -> Result<String, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_HARDWARE_NAME as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_HARDWARE_NAME as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let s = s.trim_matches(char::from(0));
                    Ok(String::from(s))
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl ControllerNumber for LanBus {
    fn controller_number(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CONTROLLER_NUMBER as u8,
                data.as_mut_ptr() as *mut c_void,
                4,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(u32::from_le_bytes(data)),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl DevicePartNumber for LanBus {
    fn device_part_number(&self) -> Result<String, PcanError> {
        let mut data = [0u8; 100];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_DEVICE_PART_NUMBER as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let s = s.trim_matches(char::from(0));
                    Ok(String::from(s))
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Informational Parameters */

impl ChannelVersion for LanBus {
    fn channel_version(&self) -> Result<Version, PcanError> {
        let mut data = [0u8; pcan::MAX_LENGTH_VERSION_STRING as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_VERSION as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(s) => {
                    let newlines = s.lines().collect::<Vec<_>>();
                    if newlines.len() == 4 {
                        let newlines = newlines
                            .iter()
                            .map(|s| s.trim_matches(char::from(0)))
                            .collect::<Vec<_>>();

                        Ok(Version {
                            device_driver_name_and_version: String::from(newlines[0]),
                            architecture: String::from(newlines[1]),
                            year_of_copyright: String::from(newlines[2]),
                            company_name_and_city: String::from(newlines[4]),
                        })
                    } else {
                        Err(PcanError::Unknown)
                    }
                }
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl ChannelFeatures for LanBus {
    fn is_fd_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_FD_CAPABLE == pcan::FEATURE_FD_CAPABLE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn is_delay_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_DELAY_CAPABLE == pcan::FEATURE_DELAY_CAPABLE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn is_io_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32,
            )
        };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => {
                let value = u32::from_le_bytes(data);
                if value & pcan::FEATURE_IO_CAPABLE == pcan::FEATURE_IO_CAPABLE {
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
