//! Generics in structs

// struct Point<T, E, F> {
//     x: T,
//     y: E,
//     z: F,
// }

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

pub fn run() {
    let int = Point { x: 12, y: 15 };

    println!(" the struct int and float is {:?}", int.x());
}
