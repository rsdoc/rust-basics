// extern crate communicator; --> complete path
use communicator::api; // reduce path with use

// making use of enum
// use Colors::{Red, Yellow};
// use of glob
use Colors::*;

fn main() {
    // communicator::api::user();
    api::user();

    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
