use std::path::PathBuf;
use pcan_basic::log::{log_location, set_log_location};

fn main() {
    match log_location() {
        Ok(path) => println!("{:?}", path),
        Err(err) => println!("{:?}", err),
    }

    match set_log_location(r"/home") {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match log_location() {
        Ok(path) => println!("{:?}", path),
        Err(err) => println!("{:?}", err),
    }
}
