//! Option Enum
//! - Option enum is defined in standard library
//! - Option Enum type
//!     - Some()
//!     - None
//! - Rust doest not have `null` feature
/// ```
///     enum Option<T> {
///        Some(T),
///        None
///     }
/// ```
/// - Variants of Option enum are included in prelude(we don't need to include)
/// - `Option::None` or we can use directly `None`
pub fn run() {
    let num = Option::Some(5);
    let text = Some("Hello from Option::Some() variant"); // we can use directly as it is in prelude

    // when we use None variant of Option, we need to give type of value
    let future: Option<i32> = None;

    println!(" the num is {:?}", num);
    println!(" the text is {:?}", text);
    println!(" the future is {:?}", future);
}
