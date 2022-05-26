use pcan_basic::error::PcanError;
use pcan_basic::hw::attached_channels_count;

fn main() {
    let result = attached_channels_count();
    match result {
        Ok(channel_count) => println!("{}", channel_count),
        Err(_) => println!("An error occurred!")
    }
}