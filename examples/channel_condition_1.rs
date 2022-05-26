use pcan_basic::bus::UsbBus;
use pcan_basic::hw::ChannelCondition;

fn main() {
    let bus = UsbBus::USB1;
    let channel_condition = bus.channel_condition();

    match channel_condition {
        Ok(status) => println!("{:?}", status),
        _ => println!("An error occurred!")
    }
}