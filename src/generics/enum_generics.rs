//! Generic in enums

#[derive(Debug)]
enum Option<T> {
    Ok(T),
    None,
}

#[derive(Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub fn run() {}
