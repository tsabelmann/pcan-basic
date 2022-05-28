use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::Baudrate;
use pcan_basic::trace::{set_default_trace_location, SetTraceLocation, TraceLocation};

fn main() {
    let can_socket = match UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud500K) {
        Ok(can_socket) => can_socket,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    match can_socket.trace_location() {
        Ok(path) => println!("path={:?}", path),
        Err(err) => println!("{:?}", err),
    }

    match can_socket.set_trace_location("/home/tsa") {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.trace_location() {
        Ok(path) => println!("path={:?}", path),
        Err(err) => println!("{:?}", err),
    }

    match set_default_trace_location(&can_socket) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.trace_location() {
        Ok(path) => println!("path={:?}", path),
        Err(err) => println!("{:?}", err),
    }
}
