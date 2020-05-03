use std::io;

/**
 * If else in rust
 */
pub fn run() {
    // let result = is_even(21);
    // decision_based_on_if_else();
    iflet();
}

fn _is_even(number: i32) -> bool {
    // check if number is odd or even
    if number % 2 == 0 {
        return true;
    }

    false
}

fn _decision_based_on_if_else() {
    let mut user_input = String::new(); // creating new empty string and assign user input to this variable

    println!(" Are you coming for movie ❓ [Hint ! yes or no]");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed at parsing user input");

    // let user_input = user_input.trim(); // trim the \n character at the end of the input, we can avoid shadowing here since we dont want to change the type of data

    user_input = user_input.trim().to_string();

    if user_input == "yes" {
        println!(" ok let go for movie and have fun!");
    } else {
        // ask user what is his age

        println!("What's your age buddy ?");

        // here we are getting old value stored in user_input, we need to clear it - because values are appended in the previous values
        user_input.clear();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed in else statement");

        let user_input: i32 = user_input.trim().parse().expect("failed at parsing age");

        if user_input > 18 && user_input < 30 {
            println!("You can do what ever you want");
        } else if user_input > 30 && user_input < 35 {
            println!("You can cook");
        } else {
            println!("You can read and learn to cook");
        }
    }
}

// using if let - storing value based on conditions
fn iflet() {
    // check if value is even - then allow users else block them
    let n: i32 = 21;

    let evn: bool = if n % 2 == 0 {
        println!("Hey you are even with us !");
        true
    } else {
        println!("Hey you are odd with us !");
        false
    };

    // when you do not want to return any thing
    let no_return = if n < 10 {
        println!(" Value is less than 10");
        // true
        ()
    }; // returing none type - issue while compile as above if block returns type is bool

    println!(" Result found is {}", evn);

    println!(" the no return value {:?}", no_return);

    // {} - default fomatter in println!
    // {:?} - printing compound types
}
