//! Matching error kinds
//!  - using ErrorKind struct, helps us in finding error types.
use std::fs::File;
use std::io::ErrorKind;

pub fn run() {
    let file = File::open("hello1.txt");

    let f = match file {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello1.txt") {
            Ok(fc) => fc,
            Err(err) => {
                panic!("Not able to create file - {:?}", err);
            }
        },
        Err(error) => {
            panic!("unable to open file : {:?}", error);
        }
    };

    println!("file value is : {:?}", f);
}
