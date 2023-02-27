use pcan_basic::log::{configure_log, is_logging, log_configuration, LogFunction, set_logging};

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

    match is_logging() {
        Ok(logging) => println!("Is logging: {}", logging),
        Err(err) => println!("The error: {:?}", err)
    }

    match set_logging(true) {
        Ok(_) => println!("Successful!"),
        Err(err) => println!("The error: {:?}", err)
    }

    match log_configuration() {
        Ok(log_configuration) => println!("log_configuration={:?}", log_configuration),
        Err(err) => println!("{:?}", err),
    }
}
