use pcan_basic::bus::UsbBus;
use pcan_basic::info::DataBusSpeed;
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

    match can_socket.data_bus_speed() {
        Ok(data_bus_speed) => {
            println!("data_bus_speed={}", data_bus_speed);
        }
        Err(err) => println!("{:?}", err),
    }
}
