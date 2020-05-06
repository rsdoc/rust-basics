/*
 *  - Methods are similar to functions, but they are defined in context of structs
 *  - Methods are called by instantiated structs
 *  - first parameter is always self - which refers to instance of struct
 *  - more than one parameter in methods
 *  - Parameters can be of any type even struct type too.
 */

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

// to add method to this struct - add implementation block for that
impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn run() {
    let plot = Rectangle {
        width: 300.50,
        height: 200.98,
    };

    let other_plot = Rectangle {
        width: 200.0,
        height: 300.0,
    };

    println!("Area of plot is {} m", plot.area());

    println!(" The can hold function : {:?}", plot.can_hold(&other_plot))
}
