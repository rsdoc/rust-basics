/**
 * two types -
 * 1. Rust has only one string type - string slice or &str -  In core
 * 2. String type - provided by std library
 *  1. string is growable, mutable, UTF-8 encoded
 */
pub fn run() {
    // string literal,or  &str or referece to str - in core language
    let a = "Hello"; // the memory location where a  is  stored is - immutable

    println!("The &str or string slice value a is {}", a);

    // creating the String
    let mut first_name = String::new(); // creates empty string

    // adding value to the String
    first_name = String::from("Rahul"); // overriding above empty String

    println!("The String value is : {}", first_name);

    // reccommended way of writing String
    let mut last_name = String::from("Singh"); // creating and adding data to the String

    // Strings are growable
    last_name.push_str(" Singh");

    println!("The last_name String value is : {}", last_name);
}
