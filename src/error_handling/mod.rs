//! Error handling in rust
//! - Recoverable Errors eg. file not found error
//!     - Result enum
//! - Unrecoverable Errors eg. accessing location beyond array limit
//!     - panic()
// mod unrecoverable_errors;
// mod recoverable_errors;
// mod matching_error_kind;
// mod expect_unwrap;
mod error_propogation;

pub fn run() {
    // unrecoverable_errors::run();
    // recoverable_errors::run();
    // matching_error_kind::run();
    // expect_unwrap::run();
    error_propogation::run();
}
