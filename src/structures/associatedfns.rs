/*
 * Associated functions -
 *  - functions inside impl block with no &self as a parameter
 *  - they still are functions not methods because they do not have the instance of the struct to work with
 *   - eg. String::from() -> from() is associated function and String is struct
 *  - Associated functions are often used for constructors that will return a new instance of the struct
 *  - can have multiple impl blocks
 */

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // creating associated function -> i.e how --> String::from() --> returns  the instance of the String struct
    fn square(size: f64) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn build(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
}

pub fn run() {
    let sq = Rectangle::square(20.0);
    let st = String::from("Hello");

    let b = Rectangle::build(20.10, 30.11);

    println!(" the sq value is - {:?} {:?} {:?}", sq, st, b);
}
