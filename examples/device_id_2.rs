use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::hw::DeviceId;
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::Baudrate;

fn main() {
    let can_socket = UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud500K);

    let can_socket = match can_socket {
        Ok(socket) => socket,
        Err(_) => {
            return;
        }
    };

    let result = can_socket.device_id();
    match result {
        Ok(id) => println!("{}", id),
        _ => println!("An error occurred!"),
    }
}
