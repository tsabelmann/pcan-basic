use pcan_basic::info::api_version;

fn main() {
    let result = api_version();
    match result {
        Ok(api_version) => println!("{}", api_version),
        _ => println!("An error occurred!")
    }
}