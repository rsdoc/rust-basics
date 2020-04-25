/**
 * Scalar data types
 *  - single value
 *      - interger, floating-point, Boolean, character
 */
const MAX_POINTS: u16 = 1000;
pub fn run() {
    let a: u32 = 0;

    println!("The value is {}", a);

    // integer overflow
    let x: u8 = 255; // u8 - max value is 0 -255, if 256 - it is overflow

    println!("The value is {}", x);

    println!("The value is const is {}", MAX_POINTS);
}
