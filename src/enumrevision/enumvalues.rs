//! 1. using enum as a type for fields of struct
#[allow(dead_code)]
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

pub fn run() {
    // instantiate struct
    let ip_add = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!(" the sturct - {:?}", ip_add);
}
