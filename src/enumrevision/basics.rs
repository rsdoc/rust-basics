// creating an enum
#[allow(dead_code)] // so that we don't have to use all the variants
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

pub fn run() {
    // creating an instance of an enum IpAddrKind
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!(" the value of V4 variant is {:?}", four);
    route(six);
    route(four);
    // enum doest not implements Copy trait
    // println!(" the value of V6 variant is {:?}", six);
}

// passing an instances to a functions
fn route(ip: IpAddrKind) {
    println!(" the value of ip is dynamic - {:?}", ip);
}
