/*
 * Custom data types, which lets you name and package together multiple data types.
 * - struct uses CamelCase naming convension
 * - instantiate structure - provide value for each fields
 * - get specific value from struct - use dot notation eg. rahul.email(instantiated struct)
 * - instantance must be mutable - to change the value using dot notation
 * - all properties of an instantance will be mutable, not specific properties
 */

#[derive(Debug)]
struct User {
    email: String,
    age: i32,
}

pub fn run() {
    let mut rahul = User {
        email: String::from("rscodedocs@gmail.com"),
        age: 28,
    };

    println!(" User is {:?}", rahul);
    println!(" Age is : {:?}", rahul.age);

    // updating rahul(instance properties) - make instance mutable
    rahul.age = 27;

    println!(" Age is : {:?}", rahul.age);
}
