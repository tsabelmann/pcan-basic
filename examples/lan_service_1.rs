use pcan_basic::info::{lan_service_is_running, lan_service_is_stopped};

fn main() {
    let result_1 = lan_service_is_running();
    match result_1 {
        Ok(is_running) => println!("is_running={}", is_running),
        Err(err) => println!("{:?}", err),
    }

    let result_2 = lan_service_is_stopped();
    match result_2 {
        Ok(is_stopped) => println!("is_stopped={}", is_stopped),
        Err(err) => println!("{:?}", err),
    }
}
