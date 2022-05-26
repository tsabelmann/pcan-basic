use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::hw::DeviceId;
use pcan_basic::info::BitrateInfo;

fn main() {
    match UsbBus::USB1.bitrate_info() {
        Ok((btr0, btr1)) => {
            println!("btr0={}", btr0);
            println!("btr1={}", btr1);
        },
        Err(err) => println!("{:?}", err)
    }
}
