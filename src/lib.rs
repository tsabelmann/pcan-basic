use pcan_basic_sys as pcan;

struct CanFrame {
    frame: pcan::TPCANMsg
}



struct CanFdFrame {
    frame: pcan::TPCANMsgFD
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
