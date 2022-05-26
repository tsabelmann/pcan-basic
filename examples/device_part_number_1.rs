use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::hw::DevicePartNumber;

fn main() {
    match UsbBus::USB1.device_part_number() {
        Ok(device_part_number) => println!("device_part_number={}", device_part_number),
        Err(err) => println!("{:?}", err)
    }
}