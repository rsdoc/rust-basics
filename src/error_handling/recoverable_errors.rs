//! Recoverable errors
//! - Recoverable errors with Result enums
//! ```
//!     enum Result<T, E> {
//!         Ok(T),
//!         Err(E),
//!     }
//!
//!     eg. Open file -> if not there - create file
//! ```
use std::fs::File;

pub fn run() {
    let file = File::open("hello1.txt");

    // if file not found -> Error - Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
    let f = match file {
        Ok(file) => file,
        Err(error) => panic!(" file not found create file 🔥 {:?}", error)
    };

    println!(" the f value is {:?}", f);
}