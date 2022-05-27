use pcan_basic::bus::UsbBus;
use pcan_basic::df::{AllowErrorFrames, SetAllowErrorFrames};
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::Baudrate;

fn main() {
    let can_socket = match UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud500K) {
        Ok(can_socket) => can_socket,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    match can_socket.allow_error_frames(false) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.allows_error_frames() {
        Ok(allows_error_frames) => println!("allows_error_frames={}", allows_error_frames),
        Err(err) => println!("{:?}", err),
    }

    match can_socket.allow_error_frames(true) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.allows_error_frames() {
        Ok(allows_error_frames) => println!("allows_error_frames={}", allows_error_frames),
        Err(err) => println!("{:?}", err),
    }
}
