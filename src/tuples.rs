/*
 * tuples are compound data types
 *  - can store same or multiple types of values
 *  - tuples have fixed length, once declared they cannot grow or shrink in size
 *  - index starts from zero
 *  - destructuring with dot notation eg. price_tuple.2
 */
pub fn run() {
    // declaring and assigning tuples
    let price_tuple: (i32, f64, bool) = (10, 56.00, true); // different data types

    println!("{:?}", price_tuple.1); // accessing tuple values with dot notation

    tuple_as_args(price_tuple);
}

fn tuple_as_args(x: (i32, f64, bool)) {
    let (a, b, c) = x; // destructuring tuple
    println!("{} - {} - {}", a, b, c);
}
