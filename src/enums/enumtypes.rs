/*
 * - Storing types with enum variants
 */

#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(u8, u8, u8, u8), // other type
}

pub fn run() {
    let ip_addr_v4 = IpAddrKind::V4(String::from("127.0.0.1"));
    let ip_addr_v6 = IpAddrKind::V6(127, 0, 0, 1);

    println!(" ip v4 - {:?}", ip_addr_v4);
    println!(" ip v6 - {:?}", ip_addr_v6);
}
