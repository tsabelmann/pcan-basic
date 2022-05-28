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

    match can_socket.digital_value(0) {
        Ok(digital_value) => println!("digital_value={:?}", digital_value),
        Err(err) => println!("{:?}", err),
    }
}
