use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::info::ChannelVersion;

fn main() {
    let result = UsbBus::USB1.channel_version();

    match result {
        Ok(version) => println!("{:?}", version),
        Err(err) => println!("{:?}", err),
    }
}
