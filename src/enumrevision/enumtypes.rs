//! using different types to store values
#[allow(dead_code)]
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
    V8(u8, u8, u8, u8), // storing multiple types
}

pub fn run() {
    let four = IpAddr::V4(String::from("127.0.0.1"));
    let eight = IpAddr::V8(127, 0, 0, 0);

    println!(" the enum variant four value is {:?} {:?}", four, eight);
}
