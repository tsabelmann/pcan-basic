use pcan_basic::log::{configure_log, log_configuration, LogFunction};

fn main() {
    match configure_log(LogFunction::Write) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match log_configuration() {
        Ok(log_configuration) => println!("log_configuration={:?}", log_configuration),
        Err(err) => println!("{:?}", err),
    }

    match configure_log(LogFunction::Parameters) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match log_configuration() {
        Ok(log_configuration) => println!("log_configuration={:?}", log_configuration),
        Err(err) => println!("{:?}", err),
    }
}
