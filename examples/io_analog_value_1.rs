use pcan_basic::bus::UsbBus;
use pcan_basic::io::AnalogValue;
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

    match can_socket.analog_value() {
        Ok(analog_value) => println!("analog_value={:?}", analog_value),
        Err(err) => println!("{:?}", err),
    }
}
