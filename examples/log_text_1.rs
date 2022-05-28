use pcan_basic::log::log_text;

fn main() {
    match log_text("Hello World") {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }
}
