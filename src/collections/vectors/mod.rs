//! Vector (vec<T>)
//! - vector can take value of any type.
//! - stores more than one value in single data structure.
//! - keeps all values next to each other in memory.
//! - it must store value of same type only.
//! ```
//!     let mut v: vec<i32> = Vec::new(); // creating instance of vector
//!     v.push(20); // adding value to vector
//! ```
//! Reading values from vector
//! - let value = v[0];
//! - let value = v.get(0);
//!
//! - Iterate over the vector values
//! ```
//!     for i in &v {
//!         // get value from vector -> i
//!     }
//! ```
//! we can make use of enum to store multiple types of values in a array
//! ```
//!     enum Spreassheet {
//!         Integer(i32),
//!         Float(f64),
//!         Text(String),
//!     }
//! ```
pub fn run() {
    // creating instance of vector
    let mut v: Vec<i32> = Vec::new();

    // adding data to vector
    v.push(10);
    v.push(20);
    v.push(30);

    // using vec! macro to create the instance of vector and add data to that.
    // we skip writing type of value in macro
    let vm = vec![20, 30, 40, 50, 60];

    let value = v[1]; // it's runtime error - cannot be catched at compile time.
    let value1 = v.get(10); // return type is Option<&T>; -> this way is not going to panic -> returns Option::None;

    println!(" vec values are {:?}", v);
    println!(" vec macro values are {:?}", vm);
    println!(" value v is {:?}", value);
    println!(" value value1 is {:?}", value1);
    // for loop
    for i in &mut v {
        // modify the value
        // * is a dereference operator
        *i = *i * 2; // first we need to make vector mutable and its ref to
        println!(" the vec value is {:?}", i);
    }

    println!(" vec values are {:?}", v);

    /// making use of enum to store multiple types in a vector
    #[derive(Debug)]
    enum Spreassheet {
        Integer(i32),
        Float(f32),
        Text(String),
    }

    let row = vec![
        Spreassheet::Integer(10),
        Spreassheet::Float(12.11),
        Spreassheet::Text(String::from("Hello users")),
    ];

    println!("the row : {:?}", row);
}
