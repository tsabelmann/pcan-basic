use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::info::NominalBusSpeed;
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

    match can_socket.nominal_bus_speed() {
        Ok(nominal_bus_speed) => {
            println!("nominal_bus_speed={}", nominal_bus_speed);
        }
        Err(err) => println!("{:?}", err),
    }
}
