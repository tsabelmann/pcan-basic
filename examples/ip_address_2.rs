use pcan_basic::bus::LanBus;
use pcan_basic::error::PcanError;
use pcan_basic::hw::{HardwareName, IpAddress};
use pcan_basic::socket::lan::LanCanSocket;
use pcan_basic::socket::Baudrate;

fn main() {
    let can_socket = LanCanSocket::open(LanBus::LAN1, Baudrate::Baud500K);

    let can_socket = match can_socket {
        Ok(socket) => socket,
        Err(_) => {
            return;
        }
    };

    let result = can_socket.ip_address();
    match result {
        Ok(ip_address) => println!("{}", ip_address),
        _ => println!("An error occurred!"),
    }
}
