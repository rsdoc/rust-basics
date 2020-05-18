use std::fs::File;
use std::io;
use std::io::Read;

pub fn run() {
    let output = read();

    match output {
        Ok(f) => println!(" the file contennt is - {:?}", f),
        Err(e) => println!(" [ Main ] found error - {:?}", e),
    }
    let output = error_propogation_question_mark();

    match output {
        Ok(f) => println!(" the file contennt is - {:?}", f),
        Err(e) => println!(" [ Main ] found error - {:?}", e),
    };
}

fn read() -> Result<String, io::Error> {
    // read file
    let f = File::open("hello.txt");

    // incase of error return this to callee function
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // read data from file
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn error_propogation_question_mark() -> Result<String, io::Error> {
    let mut f = File::open("hello3.txt")?;

    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}
