use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::hw::ChannelIdentifying;
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::{Baudrate, MessageType, SendCan};
use pcan_basic::socket::{CanFrame, RecvCan};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let usb_socket = match UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud500K) {
        Ok(socket) => socket,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    let res1 = usb_socket.set_channel_identifying(true);
    println!("{:?}", res1);

    sleep(Duration::from_secs(5));

    let res2 = usb_socket.is_channel_identifying();
    println!("{:?}", res2);

    sleep(Duration::from_secs(5));

    let res3 = usb_socket.set_channel_identifying(false);
    println!("{:?}", res3);
}
