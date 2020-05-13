//! Pattern that binds to a value
//! - Another feature of match arm is that they can bind to the parts of the values that match the pattern.
//! - This is how we can extract values out of enum variants.

#[allow(dead_code)]
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alaska,
    Arizona,
    // etc ..
}

pub fn run() {
    let quater = Coin::Quarter(UsState::Alaska);

    println!(" the res : {:?}", value_in_cents(quater));
}

fn value_in_cents(c: Coin) -> i32 {
    match c {
        Coin::Penny => {
            println!(" penny ");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // here state will bind to the value provided by user
            println!(" quarter {:?}", state);
            match state {
                UsState::Alaska => {
                    println!("state is {:?}", state);
                    25
                }
                UsState::Arizona => {
                    println!("state is {:?}", state);
                    25
                }
            }
        }
    }
}
