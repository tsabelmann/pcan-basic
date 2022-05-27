use pcan_basic::bus::UsbBus;
use pcan_basic::df::{
    AllowEchoFrames, AllowRTRFrames, AllowStatusFrames, SetAllowEchoFrames, SetAllowRTRFrames,
    SetAllowStatusFrames,
};
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

    match can_socket.allow_echo_frames(false) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.allows_echo_frames() {
        Ok(allows_echo_frames) => println!("allows_echo_frames={}", allows_echo_frames),
        Err(err) => println!("{:?}", err),
    }

    match can_socket.allow_echo_frames(true) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.allows_echo_frames() {
        Ok(allows_echo_frames) => println!("allows_echo_frames={}", allows_echo_frames),
        Err(err) => println!("{:?}", err),
    }
}
