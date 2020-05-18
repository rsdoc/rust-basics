//! panic occurs are runtime
pub fn run() {
    // accessing element out of bound in array
    // let a = [1.0, 2.3, 4.2];

    // println!(" the array value is {:?}", a[6]); // Error-  index out of bounds: the len is 3 but the index is 6

    // manually creating panic
    panic!(" this is manual panic 🔥"); // here system will stop and clean all memory and exit

    // Eg. close application after 30 days of trail period
}