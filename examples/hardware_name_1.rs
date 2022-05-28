use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::hw::HardwareName;

fn main() {
    let result = UsbBus::USB1.hardware_name();

    match result {
        Ok(hardware_name) => println!("{}", hardware_name),
        _ => println!("An error occurred!"),
    }
}
