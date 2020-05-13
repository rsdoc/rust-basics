// ! using enum predefined values
#[allow(dead_code)]
#[derive(Debug)]
enum Fruits {
    Mango = 10, // Variants values should only of type isize, floating point values are not allowed.
    Apple = 20,
    Melon = 30,
}

pub fn run() {
    // creating instance of enum
    let apple = Fruits::Apple;

    println!(" the enum variant apple is : {:?}", apple);
    println!(" the enum variant apple value is : {:?}", apple as i32); // we need to typecast it to get the value of the enum variant
}
