use pcan_basic::bus::UsbBus;
use pcan_basic::df::SetMessageFilter;
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::Baudrate;
use pcan_basic::socket::CanRead;

fn main() {
    let can_socket = match UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud500K) {
        Ok(can_socket) => can_socket,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    match can_socket.set_closed_filter() {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    loop {
        let can_frame = can_socket.read();
        match can_frame {
            Ok((frame, timestamp)) => {
                println!("{:?}", frame);
                println!("{:?}", timestamp);
            }
            Err(_) => {}
        }
    }
}
