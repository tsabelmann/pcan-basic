use pcan_basic::bus::UsbBus;
use pcan_basic::io::{SetDigitalClear, SetDigitalConfiguration};
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

    match can_socket.set_mode_word(0xFF_FF_FF_FF) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.clear(0xFF_00_00_FF) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }
}
