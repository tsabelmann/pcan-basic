use pcan_basic::bus::UsbBus;
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::CanFrame;
use pcan_basic::socket::{Baudrate, MessageType, SendCan};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let usb_socket = match UsbCanSocket::open(UsbBus::USB2, Baudrate::Baud500K) {
        Ok(socket) => socket,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    let mut counter = 0u8;

    loop {
        let can_frame = CanFrame::new(0x2_FF, MessageType::Extended, &[counter]).unwrap();
        counter += 1;

        let result = usb_socket.send(can_frame);
        if result.is_ok() {
            println!("Is OK!");
        }
        sleep(Duration::from_secs(1));
    }
}
