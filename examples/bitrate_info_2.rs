use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::info::BitrateInfo;
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

    match can_socket.bitrate_info() {
        Ok((btr0, btr1)) => {
            println!("btr0={}", btr0);
            println!("btr1={}", btr1);
        }
        Err(err) => println!("{:?}", err),
    }
}
