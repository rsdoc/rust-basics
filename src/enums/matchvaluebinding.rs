/*
 * Patterns that bind to a value
 *  - Another useful feature of match arm is that they can bind to the parts of the value that match the pattern. This is how we extract values out of enum variants.
 */

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarter variant is holding data from other enum
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Arizona,
}

pub fn run() {
    let matched_value = match_value(Coin::Quarter(UsState::Alaska));

    println!("Returned value is : {:?}", matched_value);
}

fn match_value(c: Coin) -> u32 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 10,
        Coin::Dime => 15,
        Coin::Quarter(state) => {
            println!("The matched state value is {:?}", state);
            25
        }
    }
}
