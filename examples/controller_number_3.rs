use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::hw::{ControllerNumber, SetControllerNumber};
use pcan_basic::socket::Baudrate;
use pcan_basic::socket::usb::UsbCanSocket;

fn main() {
    let can_socket = UsbCanSocket::open(UsbBus::USB2,
                                    Baudrate::Baud500K);

    let can_socket = match can_socket {
        Ok(socket) => { socket }
        Err(_) => { return; }
    };

    let result = can_socket.set_controller_number(0);
    match result {
        Ok(_) => println!("Ok"),
        Err(_) => println!("An error occurred!")
    }


    let result = can_socket.controller_number();
    match result {
        Ok(controller_number) => println!("{}", controller_number),
        _ => println!("An error occurred!")
    }
}