use pcan_basic::bus::UsbBus;
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::Baudrate;
use pcan_basic::trace::{SetTraceStatus, TraceStatus};

fn main() {
    let can_socket = match UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud500K) {
        Ok(can_socket) => can_socket,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    match can_socket.set_tracing(false) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.is_tracing() {
        Ok(is_tracing) => println!("is_tracing={:?}", is_tracing),
        Err(err) => println!("{:?}", err),
    }

    match can_socket.set_tracing(true) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.is_tracing() {
        Ok(is_tracing) => println!("is_tracing={:?}", is_tracing),
        Err(err) => println!("{:?}", err),
    }
}
