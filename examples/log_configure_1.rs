use pcan_basic::log::log_configuration;

fn main() {
    match log_configuration() {
        Ok(log_configuration) => println!("log_configuration={:?}", log_configuration),
        Err(err) => println!("{:?}", err),
    }
}
