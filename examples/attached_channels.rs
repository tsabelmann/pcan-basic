use pcan_basic::error::PcanError;
use pcan_basic::hw::attached_channels;

fn main() {
    let result = attached_channels();
    match result {
        Ok(attached_channels) => {
            for channel in attached_channels {
                println!("{:?}", channel.device_name());
            }
        },
        Err(_) => println!("An error occurred!")
    }
}