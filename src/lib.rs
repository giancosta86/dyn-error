//! This crate provides error-related utilities.
//!
//! In particular, the [assert_err_box] macro is a minimalist way
//! to test the inner value of an `Err<Box<dyn Error>>` from a [Result]:
//!
//! ```
//! use dyn_error::*;
//! use std::fmt::Display;
//! use std::error::Error;
//!
//! #[derive(Debug, PartialEq, Eq)]
//! struct MyErr(pub u8);
//!
//! impl Display for MyErr {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         write!(f, "Custom error: {}", self.0)
//!     }
//! }
//!
//! impl Error for MyErr {}
//!
//! let result: Result<String, Box<dyn Error>> = Err(Box::new(MyErr(90)));
//!
//! assert_err_box!(result, MyErr(90));
//! ```
//!
//! For more fine-grained control, the [check_err_box] function
//! performs the same test but returns a [Result] in lieu of calling [panic]:
//!
//! ```
//! use dyn_error::*;
//! use std::fmt::Display;
//! use std::error::Error;
//!
//! //
//! // Declaring a custom Error type
//! //
//! #[derive(Debug, PartialEq, Eq)]
//! struct MyErr(pub u8);
//!
//! impl Display for MyErr {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         write!(f, "Custom error: {}", self.0)
//!     }
//! }
//!
//! impl Error for MyErr {}
//!
//! //
//! // Test scenarios
//! //
//! let result: Result<String, Box<dyn Error>> = Err(Box::new(MyErr(90)));
//! assert_eq!(
//!     check_err_box(result, MyErr(90)),
//!     Ok(())
//! );
//!
//! let result: Result<String, Box<dyn Error>> = Err(Box::new(MyErr(90)));
//! assert_eq!(
//!     check_err_box(result, MyErr(7)),
//!     Err(ErrBoxCheckFailure::NotEqual {
//!         expected: "Custom error: 7".to_string(),
//!         actual: "Custom error: 90".to_string()
//!     })
//! );
//! ```

mod err_box;

pub use err_box::*;
