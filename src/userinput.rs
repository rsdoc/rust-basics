// importing the modules
use std::io;

pub fn run() {
    let mut user_input = String::new();

    // asking user to input data
    println!("Enter the input data");

    // taking input
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed at processing user input");

    // taking input in other types
    let user_input: i32 = user_input
        .trim() // removing new line from the string while user input from command
        .parse()
        .expect("Failed at parsing to interger");

    println!("user entered input, {} new line", user_input);
}
