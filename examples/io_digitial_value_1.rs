use pcan_basic::bus::UsbBus;
use pcan_basic::io::DigitalValue;
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

    match can_socket.value(0) {
        Ok(value) => println!("value={:?}", value),
        Err(err) => println!("{:?}", err),
    }
}
