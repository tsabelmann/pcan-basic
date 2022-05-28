use pcan_basic::bus::UsbBus;
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::Baudrate;
use pcan_basic::trace::TraceLocation;

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
}
