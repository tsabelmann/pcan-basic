use pcan_basic::bus::UsbBus;
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::Baudrate;
use pcan_basic::trace::{TraceConfigure, TraceLocation, TraceStatus};

fn main() {
    let can_socket = match UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud500K) {
        Ok(can_socket) => can_socket,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    match can_socket.trace_configuration() {
        Ok(trace_configuration) => println!("trace_configuration={:?}", trace_configuration),
        Err(err) => println!("{:?}", err),
    }
}
