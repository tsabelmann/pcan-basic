use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::hw::DeviceId;
use pcan_basic::info::ChannelFeatures;

fn main() {
    match UsbBus::USB1.is_fd_capable() {
        Ok(id) => println!("is_fd_capable={}", id),
        Err(err) => println!("{:?}", err)
    }

    match UsbBus::USB1.is_delay_capable() {
        Ok(id) => println!("is_delay_capable={}", id),
        Err(err) => println!("{:?}", err)
    }

    match UsbBus::USB1.is_io_capable() {
        Ok(id) => println!("is_io_capable={}", id),
        Err(err) => println!("{:?}", err)
    }
}