/*
 * Redeclaring the variable with the same name as previous variable
 */
pub fn run() {
    let a = "Hello World";

    println!("The original value of a is {}", a);

    // shadowing the above variable
    let a = 20;

    println!("The shadowed value of a is {}", a)
}
