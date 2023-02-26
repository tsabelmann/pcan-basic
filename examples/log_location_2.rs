use std::path::PathBuf;
use pcan_basic::log::{log_location, set_log_location};

fn main() {
    println!("First attempt");
    match log_location() {
        Ok(path) => println!("{:?}", path),
        Err(err) => println!("{:?}", err),
    }

    println!("Second attempt");
    match set_log_location("/home/tsa/") {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match log_location() {
        Ok(path) => println!("{:?}", path),
        Err(err) => println!("{:?}", err),
    }

    println!("Third attempt");
    match set_log_location("") {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match log_location() {
        Ok(path) => println!("{:?}", path),
        Err(err) => println!("{:?}", err),
    }
}
