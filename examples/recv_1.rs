use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::hw::SetChannelIdentifying;
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::Baudrate;
use pcan_basic::socket::{CanFrame, RecvCan};

fn main() {
    let usb_socket = match UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud500K) {
        Ok(socket) => socket,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    loop {
        let can_frame = usb_socket.recv();
        match can_frame {
            Ok((frame, timestamp)) => {
                println!("{:?}", frame);
                println!("{:?}", timestamp);
            }
            Err(_) => {}
        }
    }
}
