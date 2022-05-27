use pcan_basic::bus::UsbBus;
use pcan_basic::df::{ReceiveStatus, SetReceiveStatus};
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::Baudrate;

fn main() {
    match UsbBus::USB1.set_receiving(false) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match UsbBus::USB1.is_receiving() {
        Ok(receiving_status) => println!("is_receiving={}", receiving_status),
        Err(err) => println!("{:?}", err),
    }

    match UsbBus::USB1.set_receiving(true) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match UsbBus::USB1.is_receiving() {
        Ok(receiving_status) => println!("is_receiving={}", receiving_status),
        Err(err) => println!("{:?}", err),
    }
}
