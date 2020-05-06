/*
 * Rust does not have null, but it does have an enum that encode the concept of value being present or absent.
 *  Option Enum
 *  - defined in standard library
 *  - Value could be something or nothing.
 * eg.
 *
 * enum Option<T> {
 *    Some(T), // value provided will be replaced by T
 *    None,
 * }
 *
 * T is generic type parameter
 *
 * - Option enum is included in prelude, so are it's variants, we can use Some and None without the Option::prefix
 */
pub fn run() {
    // using Some
    let num = Some(5); // integer type
    let text = Some("Hello"); // string literal
    let length: Option<i32> = None; // None type - we have to give type of None
    let email: Option<&str> = None;

    println!(" {:?} {:?} {:?} {:?}", num, text, length, email);
}
