use pcan_basic::log::log_location;

fn main() {
    match log_location() {
        Ok(path) => println!("{:?}", path),
        Err(err) => println!("{:?}", err),
    }
}
