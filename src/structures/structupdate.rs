/*
 * - Struct update syntax
 *  - copy value of one instance to other, then we use struct update syntax
 */
#[derive(Debug)]
struct User {
    email: String,
    username: String,
    age: i32,
    hobbies: String,
}

pub fn run() {
    let rahul = User {
        email: String::from("rahul@test.com"),
        username: String::from("rahul1234"),
        age: 28,
        hobbies: String::from("Cricket, Chess, Football"),
    };

    // creating other user and taking some fields from user 1
    let raj = User {
        email: String::from("raj@test.com"),
        username: String::from("raj1234"),
        age: rahul.age,
        hobbies: rahul.hobbies.clone(), // we need to clone because we cann't use partially moved value
    };

    // what we should do incase of more fields
    // let sonu = User {
    //     email: String::from("sonu@test.com"),
    //     username: String::from("sonu1234"),
    //     ..rahul // taking rest of the value from user1 - shorthand for copying the value
    // };

    println!("Rahul - {:?}", rahul);
    println!("Raj - {:?}", raj);
    // ! todo -  println!("Sonu - {:?}", sonu);
}
