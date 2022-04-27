//!
//!

#[warn(dead_code)]
pub mod bus;
pub mod can;
pub mod error;
pub mod hw_ident;
pub mod info;

use pcan_basic_sys as pcan;

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn can_frame_new_001() {
//         let can_frame_1 =
//             CanFrame::new(0x20, MessageType::Standard, &[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();
//
//         let can_frame_2 =
//             CanFrame::new(0x20, MessageType::Standard, &[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();
//
//         assert_eq!(can_frame_1, can_frame_2);
//     }
//
//     #[test]
//     fn can_frame_new_002() {
//         let can_frame_1 =
//             CanFrame::new(0x20, MessageType::Extended, &[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();
//
//         let can_frame_2 =
//             CanFrame::new(0x20, MessageType::Extended, &[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();
//
//         assert_eq!(can_frame_1, can_frame_2);
//     }
//
//     #[test]
//     #[should_panic]
//     fn can_frame_new_003() {
//         let _can_frame_1 =
//             CanFrame::new(0x20, MessageType::Standard, &[0, 1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
//     }
//
//     #[test]
//     #[should_panic]
//     fn can_frame_new_004() {
//         let _can_frame_1 =
//             CanFrame::new(0x20, MessageType::Extended, &[0, 1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
//     }
//
//     /* CAN FD FRAME */
//
//     #[test]
//     fn can_fd_frame_new_001() {
//         let can_frame_1 =
//             CanFdFrame::new(0x20, MessageType::Standard, &(0..64u8).collect::<Vec<_>>()).unwrap();
//
//         let can_frame_2 =
//             CanFdFrame::new(0x20, MessageType::Standard, &(0..64u8).collect::<Vec<_>>()).unwrap();
//
//         assert_eq!(can_frame_1, can_frame_2);
//     }
//
//     #[test]
//     fn can_fd_frame_new_002() {
//         let can_frame_1 =
//             CanFdFrame::new(0x20, MessageType::Extended, &(0..64u8).collect::<Vec<_>>()).unwrap();
//
//         let can_frame_2 =
//             CanFdFrame::new(0x20, MessageType::Extended, &(0..64u8).collect::<Vec<_>>()).unwrap();
//
//         assert_eq!(can_frame_1, can_frame_2);
//     }
//
//     #[test]
//     #[should_panic]
//     fn can_fd_frame_new_003() {
//         let _can_frame_1 =
//             CanFdFrame::new(0x20, MessageType::Standard, &(0..65u8).collect::<Vec<_>>()).unwrap();
//     }
//
//     #[test]
//     #[should_panic]
//     fn can_fd_frame_new_004() {
//         let _can_frame_1 =
//             CanFrame::new(0x20, MessageType::Extended, &(0..65u8).collect::<Vec<_>>()).unwrap();
//     }
//
//     /* USB CAN SOCKET */
//
//     #[test]
//     fn usb_can_socket_001() {
//         let usb_socket = UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud250K);
//         match usb_socket {
//             Ok(v) => println!("{:?}", v),
//             Err(err) => println!("{:?}", err),
//         }
//     }
// }
