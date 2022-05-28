use pcan_basic::bus::UsbBus;
use pcan_basic::df::{ReceiveStatus, SetReceiveStatus};
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::Baudrate;
use pcan_basic::socket::RecvCan;
use std::time::{Duration, Instant};

fn main() {
    let can_socket = match UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud500K) {
        Ok(can_socket) => can_socket,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    match can_socket.set_receiving(false) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.is_receiving() {
        Ok(receiving_status) => println!("is_receiving={}", receiving_status),
        Err(err) => println!("{:?}", err),
    }

    let now = Instant::now();
    while now.elapsed() <= Duration::from_secs(20) {
        let can_frame = can_socket.recv();
        match can_frame {
            Ok((frame, timestamp)) => {
                println!("{:?}", frame);
                println!("{:?}", timestamp);
            }
            Err(_) => {}
        }
    }

    println!("20s are over...");

    match can_socket.set_receiving(true) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.is_receiving() {
        Ok(receiving_status) => println!("is_receiving={}", receiving_status),
        Err(err) => println!("{:?}", err),
    }

    loop {
        let can_frame = can_socket.recv();
        match can_frame {
            Ok((frame, timestamp)) => {
                println!("{:?}", frame);
                println!("{:?}", timestamp);
            }
            Err(_) => {}
        }
    }
}
