use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::hw::DevicePartNumber;
use pcan_basic::socket::Baudrate;
use pcan_basic::socket::usb::UsbCanSocket;

fn main() {
    let can_socket = match UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud500K) {
        Ok(can_socket) => { can_socket }
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    match can_socket.device_part_number() {
        Ok(device_part_number) => println!("device_part_number={}", device_part_number),
        Err(err) => println!("{:?}", err)
    }
}