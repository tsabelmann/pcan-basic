use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::hw::ControllerNumber;
use pcan_basic::socket::Baudrate;
use pcan_basic::socket::usb::UsbCanSocket;

fn main() {
    let can_socket = UsbCanSocket::open(UsbBus::USB1,
                                    Baudrate::Baud500K);

    let can_socket = match can_socket {
        Ok(socket) => { socket }
        Err(_) => { return; }
    };

    let result = can_socket.controller_number();
    match result {
        Ok(controller_number) => println!("{}", controller_number),
        _ => println!("An error occurred!")
    }
}