/*
 * Converting from one type to another
 */
pub fn run() {
    // using as keyword
    let a: i32 = 20;
    let b: i64 = a as i64;
    let c: i64 = a as i64 + 10;
    let d: i64 = a.into(); // ! error while manipulating data - let d:i64 = a.into() + 10;

    // converted size type should be greated than assigned type else it throws an error

    println!("The value of a, b, c, d is {}, {}, {}, {}", a, b, c, d);
}
