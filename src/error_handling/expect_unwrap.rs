//! Since match becomes too verbose, we can  make use of unwrap("default panic message will be thrown") and expect("In case of panic - pass custom error message")
use std::fs::File;

pub fn run() {
    // let f = File::open("hello2.txt").unwrap();

    // println!(" the file value is - {:?}", f);

    // now using expect() -> to pass custom error message
    let f2 = File::open("hello2.txt").expect(" [ Custom ] File Not found");

    println!(" the f2 value - {:?}", f2);
}
