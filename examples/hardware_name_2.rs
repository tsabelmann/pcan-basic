use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::hw::{HardwareName};
use pcan_basic::socket::Baudrate;
use pcan_basic::socket::usb::UsbCanSocket;

fn main() {
    let can_socket = UsbCanSocket::open(UsbBus::USB1,
                                    Baudrate::Baud500K);

    let can_socket = match can_socket {
        Ok(socket) => { socket }
        Err(_) => { return; }
    };

    let result = can_socket.hardware_name();
    match result {
        Ok(hardware_name) => println!("{}", hardware_name),
        _ => println!("An error occurred!")
    }
}