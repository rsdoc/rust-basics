pub fn run() {
    let some_u8 = Some(10);

    let res = if let Some(10) = some_u8 {
        println!(" matched condition ");
        true
    } else if let Some(4) = some_u8 {
        println!("something else if matched {:?}", some_u8);
        true
    } else {
        println!("something else matched");
        false
    };

    println!(" the final verdict ... {:?}", res);
}
