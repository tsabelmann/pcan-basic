use pcan_basic::bus::UsbBus;
use pcan_basic::io::{DigitalValue, IOConfig, IOValue, SetDigitalConfiguration, SetDigitalValue};
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::Baudrate;
use pcan_basic::trace::{TraceConfigure, TraceLocation, TraceStatus};

fn main() {
    let can_socket = match UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud500K) {
        Ok(can_socket) => can_socket,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    match can_socket.set_mode(0, IOConfig::InOut) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.set_value(0, IOValue::High) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.value(0) {
        Ok(value) => println!("value={:?}", value),
        Err(err) => println!("{:?}", err),
    }

    match can_socket.value_word() {
        Ok(value_word) => println!("value_word={:b}", value_word),
        Err(err) => println!("{:?}", err),
    }

    match can_socket.set_value(0, IOValue::Low) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.value(0) {
        Ok(value) => println!("value={:?}", value),
        Err(err) => println!("{:?}", err),
    }

    match can_socket.value_word() {
        Ok(value_word) => println!("value_word={:b}", value_word),
        Err(err) => println!("{:?}", err),
    }
}
