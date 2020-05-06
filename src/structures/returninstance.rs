/*
 * - Why return instance from function ?
 *  - When we want to create instance for large groups
 *
 * - construct new instance of struct as last expression of the function body
 *
 * - Struct properties follows snake_case naming convension.
 */

#[derive(Debug)]
struct User {
    first_name: String,
    last_name: String,
    email: String,
    mobile: String,
    age: i32,
}

pub fn run() {
    let rahul = user(
        String::from("Rahul"),
        String::from("Singh"),
        String::from("rscodedocs@gmail.com"),
        String::from("+91-88607788990001000"),
        28,
    );

    println!("{:?}", rahul);
}

fn user(first_name: String, last_name: String, email: String, mobile: String, age: i32) -> User {
    // field init shorthand - when variables have the same name, only write the value
    User {
        first_name,
        last_name,
        email,
        mobile,
        age,
    }
}
