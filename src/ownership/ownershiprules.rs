/*
 * Ownership Rules
 *  - each value in rust has variable - it's owner of the value
 *  - there can only one owner at a time, two variables cannot point to the same memory location.
 *  - when owner goes out of scope, value will be dropped
 *
 * Functions -
 *  - Passing the variable to a function - will move or copy the data as variable assignment works.
 */
pub fn run() {
    // creating a String - pointer to this value will be stored in stack and data will be stored in heap memory.
    let mut greet = String::from("Hello, ");

    greet.push_str("world");

    let s2 = greet; // here we are transferring memory location pointer to s2 variable, below this, greet variable is not accessible - As ownership of greet is transferred to variable s2

    // ! Q. what if you want to get the data into some other variables ? --> use clone()
    let s3 = s2.clone();

    let mut s4 = s2.clone(); // created variable to test the return ownership of the value

    println!("s2 : {}\ns3: {}", s2, s3);

    // passing variable to a function --> this will pass the ownership of current variable to the function
    take_ownership(s3);

    // we can not use s3 variable now
    println!(" s3 is not valid :");

    // to take the ownership back again - we return the variable from called function
    println!(" the value before s4 : {}", s4);
    s4 = return_ownership(s4); // shadowing variable

    println!(" s4 will be valid here : {}", s4);
} // drop method will be called automatically by rust - to clean memory - gc work

fn take_ownership(s: String) {
    println!(" [ take_ownership ] - {}", s);
}

fn return_ownership(mut s: String) -> String {
    s.push_str("  - new world !");
    println!(" [ return_ownership ] - {}", s);
    s
}
