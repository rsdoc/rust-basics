pub fn run() {
    let mut s = String::from("Hello Borrowed ");

    borrowed_function(&mut s); // borrowing the value of s

    println!(" changed value is - {}", s);
}

fn borrowed_function(s: &mut String) {
    println!(" the borrowed value is  - {}", s);

    // since we got the borrowed value
    s.push_str(" add content !");
}
