use pcan_basic::log::{is_logging, set_logging};

fn main() {
    match set_logging(false) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match is_logging() {
        Ok(is_logging) => println!("is_logging={}", is_logging),
        Err(err) => println!("{:?}", err),
    }

    match set_logging(true) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match is_logging() {
        Ok(is_logging) => println!("is_logging={}", is_logging),
        Err(err) => println!("{:?}", err),
    }
}
