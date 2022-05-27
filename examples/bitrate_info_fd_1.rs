use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::hw::DeviceId;
use pcan_basic::info::{BitrateInfoFd};

fn main() {
    match UsbBus::USB1.bitrate_info_fd() {
        Ok(bitrate_info_fd) => {
            println!("bitrate_info_fd={}", bitrate_info_fd);
        },
        Err(err) => println!("{:?}", err)
    }
}
