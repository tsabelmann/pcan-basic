use pcan_basic::log::{log_location, set_log_location, set_default_log_location};

fn main() {
    println!("Get current path");
    match log_location() {
        Ok(path) => println!("{:?}", path),
        Err(err) => println!("{:?}", err),
    }

    println!("Set path - not the default");
    match set_log_location("/home/") {
        Ok(_) => println!("Successful!"),
        Err(err) => println!("{:?}", err),
    }

    println!("Get current path");
    match log_location() {
        Ok(path) => println!("{:?}", path),
        Err(err) => println!("{:?}", err),
    }

    println!("Reset the path to the default");
    match set_default_log_location() {
        Ok(_) => println!("Successful!"),
        Err(err) => println!("{:?}", err),
    }

    println!("Get current path");
    match log_location() {
        Ok(path) => println!("{:?}", path),
        Err(err) => println!("{:?}", err),
    }
}
