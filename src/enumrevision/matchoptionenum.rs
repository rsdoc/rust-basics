pub fn run() {
    println!(" the res is {:?}", plus_one(Some(20)));
    println!(" the res is {:?}", plus_one(None));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => {
            println!(" some is matched");
            Some(i + 1)
        } // None => {
        //     println!(" none is matched");
        //     None
        // }
        // what to do incase of missing enum variants in match
        _ => {
            println!(" wildcard match");
            None
        }
    }
}
