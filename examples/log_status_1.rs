use pcan_basic::log::is_logging;

fn main() {
    println!("Get current logging status");
    match is_logging() {
        Ok(is_logging) => println!("Is logging: {}", is_logging),
        Err(err) => println!("{:?}", err),
    }
}
