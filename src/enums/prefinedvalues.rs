/*
 * - Predefined values in enum
 *  - variants default value should be of integer type
 *  - typecast to get value, f as i32
 */
#[allow(dead_code)]
#[derive(Debug)]
enum Fruits {
    Apple = 10,
    Mango = 20,
    Watermelon = 30,
}

pub fn run() {
    let f = Fruits::Apple;

    println!(" the f is - {:?}", f);

    // to get the value of f -> we have to typecast it
    println!(" the fruit f value is - {:?}", f as i32);
}
