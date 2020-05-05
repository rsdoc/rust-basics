/*
 * References -
 *  - refering the value without taking the ownership of that variable
 *  - & - infront of any variable --> it becomes references.
 */
pub fn run() {
    let s1 = String::from("Original String");

    let s2 = &s1; // passed the reference to s2

    // ! s2.push_str(" added value"); --> Error --> Cannot modify the borrowed value
    ref_wo_ownership(&s1);

    println!(" s1 is : {} and s2 is : {}", s1, s2);
}

fn ref_wo_ownership(s: &String) {
    println!("The received value is : {}", s);
}
