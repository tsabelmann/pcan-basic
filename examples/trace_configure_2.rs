use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::Baudrate;
use pcan_basic::trace::{
    set_default_trace_location, SetTraceConfigure, SetTraceLocation, TraceConfigure, TraceFile,
    TraceLocation,
};

fn main() {
    let can_socket = match UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud500K) {
        Ok(can_socket) => can_socket,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    match can_socket.configure_trace(TraceFile::Segmented) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.trace_configuration() {
        Ok(trace_configuration) => println!("trace_configuration={:?}", trace_configuration),
        Err(err) => println!("{:?}", err),
    }

    match can_socket.configure_trace(TraceFile::Overwrite) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.trace_configuration() {
        Ok(trace_configuration) => println!("trace_configuration={:?}", trace_configuration),
        Err(err) => println!("{:?}", err),
    }
}
