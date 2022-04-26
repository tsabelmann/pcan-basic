use crate::error::PcanError;
use crate::pcan;
use std::net::Ipv4Addr;

#[derive(Debug, PartialEq)]
pub enum ChannelConditionStatus {
    Unavailable,
    Available,
    Occupied,
    PcanView,
}

impl From<ChannelConditionStatus> for u32 {
    fn from(value: ChannelConditionStatus) -> Self {
        match value {
            ChannelConditionStatus::Unavailable => pcan::PCAN_CHANNEL_UNAVAILABLE,
            ChannelConditionStatus::Available => pcan::PCAN_CHANNEL_AVAILABLE,
            ChannelConditionStatus::Occupied => pcan::PCAN_CHANNEL_OCCUPIED,
            ChannelConditionStatus::PcanView => pcan::PCAN_CHANNEL_PCANVIEW,
        }
    }
}

impl TryFrom<u32> for ChannelConditionStatus {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            pcan::PCAN_CHANNEL_AVAILABLE => Ok(ChannelConditionStatus::Available),
            pcan::PCAN_CHANNEL_UNAVAILABLE => Ok(ChannelConditionStatus::Unavailable),
            pcan::PCAN_CHANNEL_OCCUPIED => Ok(ChannelConditionStatus::Occupied),
            pcan::PCAN_CHANNEL_PCANVIEW => Ok(ChannelConditionStatus::PcanView),
            _ => Err(()),
        }
    }
}

pub trait ChannelCondition {
    fn channel_condition(&self) -> Result<ChannelConditionStatus, PcanError>;
}

pub trait ChannelIdentifying {
    fn enable_identifying(&self) -> Result<(), PcanError>;
    fn disable_identifying(&self) -> Result<(), PcanError>;
    fn identifying(&self, value: bool) -> Result<(), PcanError> {
        match value {
            false => self.disable_identifying(),
            true => self.enable_identifying(),
        }
    }
    fn is_identifying(&self) -> Result<bool, PcanError>;
}

/* Device Id */

pub trait DeviceId {
    fn device_id(&self) -> Result<u32, PcanError>;
}

pub trait SetDeviceId {
    type Item;
    fn set_device_id(&self, value: Self::Item) -> Result<(), PcanError>;
}

// impl<T: HasSetDeviceId + InitializedChannel> SetDeviceId for T {
//     type Item = u32;
//     fn set_device_id(&self, value: Self::Item) -> Result<(), PcanError> {
//         let mut data = value.to_le_bytes();
//         let code = unsafe {
//             pcan::CAN_SetValue(
//                 self.initialized_channel(),
//                 pcan::PCAN_DEVICE_ID as u8,
//                 data.as_mut_ptr() as *mut c_void,
//                 data.len() as u32,
//             )
//         };
//
//         match PcanOkError::try_from(code) {
//             Ok(PcanOkError::Ok) => Ok(()),
//             Ok(PcanOkError::Err(err)) => Err(err),
//             Err(_) => Err(PcanError::Unknown),
//         }
//     }
// }

/* Hardware Name */

pub trait HardwareName {
    fn hardware_name(&self) -> Result<String, PcanError>;
}

/* Controller Number */

pub trait ControllerNumber {
    fn controller_number(&self) -> Result<u32, PcanError>;
}

pub trait SetControllerNumber {
    type Item;
    fn set_controller_number(&self, value: Self::Item) -> Result<(), PcanError>;
}

// impl<T: HasSetControllerNumber + InitializedChannel> SetControllerNumber for T {
//     type Item = u32;
//     fn set_controller_number(&self, value: Self::Item) -> Result<(), PcanError> {
//         let mut data = value.to_le_bytes();
//         let code = unsafe {
//             pcan::CAN_SetValue(
//                 self.initialized_chanwnel(),
//                 pcan::PCAN_CONTROLLER_NUMBER as u8,
//                 data.as_mut_ptr() as *mut c_void,
//                 data.len() as u32,
//             )
//         };
//
//         match PcanOkError::try_from(code) {
//             Ok(PcanOkError::Ok) => Ok(()),
//             Ok(PcanOkError::Err(err)) => Err(err),
//             Err(_) => Err(PcanError::Unknown),
//         }
//     }
// }

trait IpAddress {
    fn ip_address(&self) -> Result<Ipv4Addr, PcanError>;
}

// impl<T: HasIpAddress + InitializedChannel> IpAddress for T {
//     fn ip_address(&self) -> Result<Ipv4Addr, PcanError> {
//         let mut data = [0u8; 20];
//         let code = unsafe {
//             pcan::CAN_GetValue(
//                 self.handle,
//                 pcan::PCAN_IP_ADDRESS as u8,
//                 data.as_mut_ptr() as *mut c_void,
//                 data.len() as u32,
//             )
//         };
//
//         match PcanOkError::try_from(code) {
//             Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
//                 Ok(s) => {
//                     let s = s.trim_matches(char::from(0));
//                     match s.parse() {
//                         Ok(ip) => Ok(ip),
//                         Err(_) => Err(PcanError::Unknown),
//                     }
//                 }
//                 Err(_) => Err(PcanError::Unknown),
//             },
//             Ok(PcanOkError::Err(err)) => Err(err),
//             _ => Err(PcanError::Unknown),
//         }
//     }
// }

pub trait DevicePartNumber {
    fn device_part_number(&self) -> Result<String, PcanError>;
}
