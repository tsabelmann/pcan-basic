use pcan_basic::bus::UsbBus;
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::Baudrate;
use pcan_basic::special::{FiveVoltsPower, SetFiveVoltsPower};

fn main() {
    let can_socket = match UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud500K) {
        Ok(can_socket) => can_socket,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    match can_socket.set_five_volts(true) {
        Ok(_) => {
            println!("set_five_volts=true");
        }
        Err(err) => println!("{:?}", err),
    }
}
