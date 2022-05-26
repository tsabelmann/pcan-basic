use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::hw::DeviceId;

fn main() {
    let result = UsbBus::USB1.device_id();

    match result {
        Ok(id) => println!("{}", id),
        _ => println!("An error occurred!")
    }
}