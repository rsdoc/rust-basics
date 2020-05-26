//! Closures - anonymous functions that cann capture the environment
//! save in a variable and can be passed to another functions

use std::thread;
use std::time::Duration;

pub fn run() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!(" Calculating slowly ...");
    // sleeping thread for 2 sec
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            " Today do {} push ups",
            simulated_expensive_calculation(intensity)
        );

        println!(
            " Today do {} sit ups",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!(" Today is break");
        } else {
            println!(
                " Today run for {} mins",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
