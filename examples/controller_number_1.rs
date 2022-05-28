use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::hw::ControllerNumber;

fn main() {
    let result = UsbBus::USB1.controller_number();

    match result {
        Ok(controller_number) => println!("{}", controller_number),
        _ => println!("An error occurred!"),
    }
}
