use pcan_basic::bus::UsbBus;
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::Baudrate;
use pcan_basic::trace::{
    set_default_trace_size, SetTraceSize, SetTraceStatus, TraceSize, TraceStatus,
};

fn main() {
    let can_socket = match UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud500K) {
        Ok(can_socket) => can_socket,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    match can_socket.set_trace_size(42) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.trace_size() {
        Ok(trace_size) => println!("trace_size={:?}", trace_size),
        Err(err) => println!("{:?}", err),
    }

    match can_socket.set_trace_size(37) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.trace_size() {
        Ok(trace_size) => println!("trace_size={:?}", trace_size),
        Err(err) => println!("{:?}", err),
    }

    match set_default_trace_size(&can_socket) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.trace_size() {
        Ok(trace_size) => println!("trace_size={:?}", trace_size),
        Err(err) => println!("{:?}", err),
    }
}
