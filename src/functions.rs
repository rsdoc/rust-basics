// functions follow snake_case naming style
pub fn run() {
    log_error();
    add(10, 20);
    println!("The multiply of 10 and 20 : {}", multiply(10, 20));
    println!("The swapped value of 10 and 20 : {:?}", swap(10, 20));
    calculate();
}

// type 1 - no args no return type
fn log_error() {
    println!(" [ log_error ] - logs error");
}

// type 2 - args with no return type
fn add(a: i32, b: i32) {
    println!(" the sum is {}", a + b);
}

// type 3 - args with return type
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// type 4 - args with multiple return types
fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

// type 5 - function inside the function
fn calculate() {
    fn add() {
        // we cannot call this function from other
        println!("I am adding values of 10 and 20 : {}", 10 + 20);
    }

    add();
}
