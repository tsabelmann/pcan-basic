use crate::bus::Bus;
use crate::error::{PcanError, PcanOkError};
use crate::hw_ident::{
    ChannelCondition, ChannelConditionStatus, ChannelIdentifying, ControllerNumber, DeviceId,
    DevicePartNumber, HardwareName,
};
use crate::info::{ChannelFeatures, ChannelVersion, Version};
use crate::pcan;
use std::os::raw::c_void;

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
