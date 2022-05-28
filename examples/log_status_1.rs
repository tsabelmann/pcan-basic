use pcan_basic::log::is_logging;

fn main() {
    match is_logging() {
        Ok(is_logging) => println!("is_logging={}", is_logging),
        Err(err) => println!("{:?}", err),
    }
}
