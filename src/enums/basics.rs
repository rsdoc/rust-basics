/*
 * - Enums allows you to define a type by enumerating it's possible values
 * - Enum and it's variant should have CamelCase
 */

#[allow(dead_code)] // disabling unused variables warning
#[derive(Debug)]
enum IpAdrrKind {
    V4,
    V6,
}

pub fn run() {
    // instantiating enums
    let four = IpAdrrKind::V4;
    let six = IpAdrrKind::V6;

    println!(" Enums - {:?}", four);
    route(four);
    route(six);
}

// passing enums to a function
fn route(ip: IpAdrrKind) {
    println!(" route - {:?}", ip);
}
