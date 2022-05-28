use pcan_basic::bus::UsbBus;
use pcan_basic::df::{AllowStatusFrames, SetAllowStatusFrames};
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

    match can_socket.allow_status_frames(false) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.allows_status_frames() {
        Ok(allows_status_frames) => println!("allows_status_frames={}", allows_status_frames),
        Err(err) => println!("{:?}", err),
    }

    match can_socket.allow_status_frames(true) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.allows_status_frames() {
        Ok(allows_status_frames) => println!("allows_status_frames={}", allows_status_frames),
        Err(err) => println!("{:?}", err),
    }
}
