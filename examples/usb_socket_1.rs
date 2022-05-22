use pcan_basic::bus::UsbBus;
use pcan_basic::can::Baudrate;
use pcan_basic::can::{CanFrame, CanRead};
use pcan_basic::can::usb::UsbCanSocket;
use pcan_basic::error::PcanError;
use pcan_basic::hw_ident::SetChannelIdentifying;


fn main() {
    let usb_socket = match UsbCanSocket::open(
        UsbBus::USB1,
        Baudrate::Baud500K
    ) {
        Ok(socket) => { socket }
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    loop {
        let can_frame = usb_socket.read_frame();
        match can_frame{
            Ok(can_frame) => {
                println!("{:?}", can_frame);
            }
            Err(_) => {}
        }
    }

}