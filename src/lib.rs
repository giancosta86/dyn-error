//! This crate provides error-related utilities.
//!
//! In particular, the [check_err_box] function
//! can test an [Err] result wrapping a [Box] with an [Error](std::error::Error) implementation,
//! using equality test via [PartialEq]:
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
//! impl Error for MyErr {};
//!
//! // Declaring a Result that can hold any Error implementation
//! let mut result: Result<String, Box<dyn Error>>;
//!
//! //
//! // Test scenarios
//! //
//! result = Err(Box::new(MyErr(90)));
//! assert_eq!(
//!     check_err_box(result, MyErr(90)),
//!     Ok(())
//! );
//!
//! result = Err(Box::new(MyErr(90)));
//! assert_eq!(
//!     check_err_box(result, MyErr(7)),
//!     Err(ErrBoxCheckFailure::NotEqual {
//!         expected: "Custom error: 7".to_string(),
//!         actual: "Custom error: 90".to_string()
//!     })
//! );
//! ```
//!
//! For more details - including different [Error](std::error::Error)
//! taxonomies, please refer to [check_err_box].
//!
//! You can simplify the check even more, by using the panic-based
//! [assert_err_box] macro:
//!
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
//! impl Error for MyErr {};
//!
//! let result: Result<String, Box<dyn Error>> = Err(Box::new(MyErr(90)));
//!
//! assert_err_box!(result, MyErr(90));
//! ```
//!

mod err_box;

pub use err_box::*;
