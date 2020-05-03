/**
 * loops
 *  - for
 *  - while
 *  - loop
 */
pub fn run() {
    // loop();
    // while_loop();
    for_loop();
}

fn _loop() {
    let mut n: u8 = 0;

    loop {
        if n <= 5 {
            println!(" the value of n is : {}", n);
            n = n + 1;
        } else {
            break;
        }
    }
}

fn _while_loop() {
    let mut n: u8 = 0;

    while n <= 5 {
        println!(" the value of n is : {}", n);
        n += 1;
    }
}

fn for_loop() {
    // iterate  over collection
    // includes  1 and excludes 5
    for n in 1..5 {
        println!("the value of n is : {}", n);
    }
}
