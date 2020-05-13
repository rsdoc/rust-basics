//! Match Control flow
//! - Match allows you to compare the value againts series of patterns and the execute code, matching patters
//! ```
//!     enum Coin {
//!         Penny,
//!         Nickle,
//!         Dime,
//!         Quarter
//!     }
//!
//!     match c {
//!         Coin::Penny => 1,
//!         Coin::Nickle => 5,
//!         Coin::Dime => 10,
//!         Coin::Quater => 15,
//!     }
//!
//!     `match c` - Expression
//!     `Coin::Penny => 1` - Match arms
//!     `1 (code after => )` - Code to Execute
//!     `Coin::Penny` - Pattern to match
//!     ``
//! ```

#[allow(dead_code)]
#[derive(Debug)]
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quater,
}

pub fn run() {
    let penny = Coin::Penny;
    let quater = Coin::Quater;

    // let result = match quater {
    //     Coin::Penny => 1,
    //     Coin::Nickle => 5,
    //     Coin::Dime => 10,
    //     Coin::Quater => 15,
    // };

    let p_result = value_in_cents(penny);
    let q_result = value_in_cents(quater);

    println!(" the penny result is {:?}", p_result);
    println!(" the quater result is {:?}", q_result);
}

fn value_in_cents(c: Coin) -> u32 {
    match c {
        Coin::Penny => {
            println!("Value matched is penny");
            1
        }
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quater => 15,
    }
}
