//! - Q. What is Enum ?
//!     - Enum allows you to define the type by enumerating its possible values.
//! - Q. How to define Enum ?
//! ```
//!     enum IpAddrKind { // Name must be Capitalized
//!         V4, // All it's variants should have CamelCase
//!         V6
//!     }
//! ```
//! - Q. How to use Enum ?
//! ```
//!    // create instance of enum
//!     let four = IpAddrKind::V4;
//! ```
//! - enum doest not implements Copy trait
//! - Q. How to store values inside enum ?
//! ```
//!     1. using enum as a type for fields of the struct.
//!     2. Predefined values
//!     3. using different types inside enum
//! ```

// mod basics;
// mod enumvalues;
// mod predefinedvalues;
// mod enumtypes;
// mod optionsenum;
// mod matchoperator;
mod patternvalues;

pub fn run() {
    // basics::run();
    // enumvalues::run();
    // predefinedvalues::run();
    // enumtypes::run();
    // optionsenum::run();
    // matchoperator::run();
    patternvalues::run();
}
