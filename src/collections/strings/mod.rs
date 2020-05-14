//! String
//! - `to_string()` -> convert one data to other
//! - `push_str(" string value ")` - append data to string
//! - `push('C')` -> append character to string
//!
//! - Concatenation with plus operator and formate! macro
pub fn run() {
    let data = 1;
    let mut s = data.to_string();
    s.push_str(" hello number - ");
    s.push('C');
    println!(" the string value of data is : {:?}", s);

    // concatenating two strings
    let s1 = String::from("Hello ");
    let s2 = String::from("World!");

    let s3 = s1 + &s2; // add method of add trait
                       // s1 is no longer accessible here
    println!("the string value is {:?}", s3);

    // using format! to concatenate two strings
    let s4 = String::from("Hello S4");
    let s5 = String::from("World!");

    let s6 = format!("{} and {}", s4, s5); // it doesn't take ownership
                                           // s4 is accessible here
    println!(" s6 {:?}", s6);
}
