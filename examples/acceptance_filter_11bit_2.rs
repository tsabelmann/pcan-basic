use pcan_basic::bus::UsbBus;
use pcan_basic::df::{AcceptanceFilter11Bit, SetAcceptanceFilter11Bit};
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::Baudrate;
use pcan_basic::socket::RecvCan;

fn main() {
    let usb_socket = match UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud500K) {
        Ok(socket) => socket,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    match usb_socket.acceptance_filter_11bit() {
        Ok((mask, code)) => {
            println!("mask={:X}", mask);
            println!("code={:X}", code);
        }
        Err(_) => return,
    }
}
