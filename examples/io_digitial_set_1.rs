use pcan_basic::bus::UsbBus;
use pcan_basic::io::{SetDigitalConfiguration, SetDigitalSet};
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

    match can_socket.set_digital_mode_word(0xFF_FF_FF_FF) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.digital_set(0xFF_00_00_FF) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }
}
