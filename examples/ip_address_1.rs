use pcan_basic::bus::{LanBus, UsbBus};
use pcan_basic::error::PcanError;
use pcan_basic::hw::IpAddress;

fn main() {
    let result = LanBus::LAN1.ip_address();

    match result {
        Ok(ip_address) => println!("{}", ip_address),
        _ => println!("An error occurred!"),
    }
}
