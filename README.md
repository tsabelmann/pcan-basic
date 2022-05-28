# pcan-basic

[![crates](https://img.shields.io/crates/v/pcan-basic.svg)](https://crates.io/crates/pcan-basic)
[![Documentation](https://img.shields.io/docsrs/pcan-basic.svg)](https://docs.rs/pcan-basic)
[![Crate License](https://img.shields.io/crates/l/pcan-basic.svg)](https://crates.io/crates/pcan-basic)
[![Dependency Status](https://deps.rs/repo/github/tsabelmann/pcan-basic/status.svg)](https://deps.rs/repo/github/tsabelmann/pcan-basic)

Safe Rust wrapper around the [pcan-basic-sys](https://github.com/tsabelmann/pcan-basic-sys) crate wrapping `V4.6.0.600` of the [PCAN-Basic API](https://www.peak-system.com/PCAN-Basic.239.0.html) provided by the [PEAK-System Technik GmbH](https://www.peak-system.com/).

**Disclaimer**: Since I am currently working on this crate, API changes may appear at any time. Please consider this if
you want to use this crate. However, most features should be stable so not much to worry about.

## Installation

### Windows

- Install the [PCAN-Basic API](https://www.peak-system.com/quick/DrvSetup) driver
- Use `pcan-basic` as a dependency

### Linux

- Install the [PCAN-Basic API](http://www.peak-system.com/fileadmin/media/linux/files/peak-linux-driver-8.14.0.tar.gz) driver
- Use `pcan-basic` as a dependency

## Features

- [x] Support for Windows and Linux
- [x] Sending and receiving of CAN-bus frames
- [x] Hardware identification features
- [x] Additional information features
- [x] Data flow features
- [x] Log features
- [x] Trace features
- [x] IO features
- [x] Many example files to choose from
- [ ] Proper documentation for each part of the API
- [ ] Implementation of the special API 
- [ ] Implementation of CanFd sockets
- [ ] Proper testing of features for which I do not have the hardware available 
- [ ] Trace file format implementation (will most likely go into [cantools](https://github.com/tsabelmann/cantools-rs))

## License / Terms of Usage

The source code of this project is licensed under the MIT/Apache-2.0 license. This implies that you are free to use, share, and adapt it. However, please give appropriate credit by citing the project.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the MIT/Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Contact

If you have problems using the software, find mistakes, or have general questions please use the [issue tracker](https://github.com/tsabelmann/pcan-basic/issues) to contact us.

## Contributors

* [Tim Lucas Sabelmann](https://github.com/tsabelmann)

