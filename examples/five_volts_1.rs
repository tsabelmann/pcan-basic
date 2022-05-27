use pcan_basic::bus::UsbBus;
use pcan_basic::special::FiveVoltsPower;

fn main() {
    match UsbBus::USB1.five_volts() {
        Ok(five_volts) => {
            println!("five_volts={}", five_volts);
        }
        Err(err) => println!("{:?}", err),
    }
}
