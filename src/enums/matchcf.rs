/*
 * - Match
 *  - compare values against pattern
 */

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn run() {
    let matched_val = value_in_cents(Coin::Penny);

    println!(" the matche value is : {}", matched_val);
}

fn value_in_cents(c: Coin) -> u32 {
    match c {
        Coin::Penny => {
            // code to execute after matching expression
            println!(" the value is Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
