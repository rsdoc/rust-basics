/*
 * - Enums as a types for the field of struct
 * - Predefined values
 * - using different types inside enums
 */

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
    let ip_addr_v4 = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let ip_addr_v6 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!(" ipV4 - {:?}", ip_addr_v4);
    println!(" ipV4 - {:?}", ip_addr_v6);
}
