use pcan_basic::bus::UsbBus;
use pcan_basic::df::ReceiveStatus;
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

    match can_socket.is_receiving() {
        Ok(receiving_status) => println!("is_receiving={}", receiving_status),
        Err(err) => println!("{:?}", err),
    }
}
