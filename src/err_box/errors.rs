use std::{error::Error, fmt::Display};

/// Unsuccessful outcome of [check_err_box](crate::check_err_box).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrBoxCheckFailure {
    /// If the tested [Result] is [Ok] instead of [Err].
    ResultIsNotErr,

    /// If the wrapped [Error] cannot be downcast
    /// to the expected error type.
    DowncastFailed,

    /// If the wrapped [Error] is not equal (via [PartialEq])
    /// to the expected error.
    NotEqual { expected: String, actual: String },
}

/// ```
/// use dyn_error::*;
///
/// assert_eq!(
///     ErrBoxCheckFailure::ResultIsNotErr.to_string(),
///     "The result is not Err."
/// );
///
/// assert_eq!(
///     ErrBoxCheckFailure::DowncastFailed.to_string(),
///     "The boxed error cannot be downcast to the expected type."
/// );
///
/// assert_eq!(
///     ErrBoxCheckFailure::NotEqual {
///         expected: "EXP".to_string(),
///         actual: "ACT".to_string()
///     }.to_string(),
///     "Expected error: 'EXP', actual error: 'ACT'."
/// );
/// ```
impl Display for ErrBoxCheckFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrBoxCheckFailure::ResultIsNotErr => write!(f, "The result is not Err."),

            ErrBoxCheckFailure::DowncastFailed => {
                write!(
                    f,
                    "The boxed error cannot be downcast to the expected type."
                )
            }

            ErrBoxCheckFailure::NotEqual { expected, actual } => {
                write!(
                    f,
                    "Expected error: '{}', actual error: '{}'.",
                    expected, actual
                )
            }
        }
    }
}

impl Error for ErrBoxCheckFailure {}
