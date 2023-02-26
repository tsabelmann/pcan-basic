use pcan_basic::log::{is_logging, set_logging};

fn main() {
    println!("Get current logging status");
    match is_logging() {
        Ok(is_logging) => println!("Is logging: {}", is_logging),
        Err(err) => println!("{:?}", err),
    }

    println!("Set logging active!");
    match set_logging(true) {
        Ok(_) => println!("Successful!"),
        Err(err) => println!("{:?}", err),
    }

    println!("Get current logging status");
    match is_logging() {
        Ok(is_logging) => println!("Is logging: {}", is_logging),
        Err(err) => println!("{:?}", err),
    }

    println!("Set logging inactive!");
    match set_logging(false) {
        Ok(_) => println!("Successful!"),
        Err(err) => println!("{:?}", err),
    }

    println!("Get current logging status");
    match is_logging() {
        Ok(is_logging) => println!("Is logging: {}", is_logging),
        Err(err) => println!("{:?}", err),
    }
}
