/*
 * Arrays -
 *  - collection of values
 *  - all values must be of same types
 *  - length is fixed
 */
pub fn run() {
    let a = [1, 2, 3, 4, 5];

    // defining array explicitly with type and length
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // filling same values
    let mut c: [i32; 5] = [10; 5]; // same number 5 length times

    // updating the values
    c[0] = 30;

    println!("{:?}", a[0]);
    println!(" Array b {:?}", b);
    print_array(c);
    println!(" Array c {:?}", c);
}

fn print_array(x: [i32; 5]) {
    for index in 0..5 {
        println!(" value at el {} is {}", index, x[index]);
    }

    // using iter() - el is actual element
    // length of array = x.len()
    for el in x.iter() {
        println!(" iter n {}", el);
    }
}
