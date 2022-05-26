use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::hw::{HardwareName};
use pcan_basic::info::ChannelFeatures;
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

    match can_socket.is_fd_capable() {
        Ok(id) => println!("is_fd_capable={}", id),
        Err(err) => println!("{:?}", err)
    }

    match can_socket.is_delay_capable() {
        Ok(id) => println!("is_delay_capable={}", id),
        Err(err) => println!("{:?}", err)
    }

    match can_socket.is_io_capable() {
        Ok(id) => println!("is_io_capable={}", id),
        Err(err) => println!("{:?}", err)
    }
}