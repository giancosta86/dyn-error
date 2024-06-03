use crate::ErrBoxCheckFailure;
use std::error::Error;

/// Tests whether the given [Result] is actually
/// an [Err] with a [Box] containing an [Error] implementation
/// equal to the *expected* instance:
///
/// * If the equality check succeeds, just returns [Ok] with
/// a no-op `()` value;
///
/// * otherwise, returns [Err] - with a descriptive [ErrBoxCheckFailure].
///
/// ```
/// use dyn_error::*;
/// use std::fmt::Display;
/// use std::error::Error;
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct MyErr(pub u8);
///
/// impl Display for MyErr {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "Custom error: {}", self.0)
///     }
/// }
///
/// impl Error for MyErr {};
///
/// let result: Result<String, Box<dyn Error>> = Err(Box::new(
///     MyErr(90)
/// ));
///
/// assert_eq!(
///     check_err_box(
///         result,
///         MyErr(90)
///     ),
///     Ok(())
/// );
/// ```
///
/// In case of inequality, the function returns [Err]
/// with [ErrBoxCheckFailure::NotEqual], containing the
/// string representations of the two instances:
///
/// ```
/// use dyn_error::*;
/// use std::fmt::Display;
/// use std::error::Error;
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct MyErr(pub u8);
///
/// impl Display for MyErr {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "Custom error: {}", self.0)
///     }
/// }
///
/// impl Error for MyErr {};
///
/// let result: Result<String, Box<dyn Error>> = Err(Box::new(
///     MyErr(90)
/// ));
///
/// assert_eq!(
///     check_err_box(result, MyErr(7)),
///     Err(ErrBoxCheckFailure::NotEqual {
///         expected: "Custom error: 7".to_string(),
///         actual: "Custom error: 90".to_string()
///     })
/// );
/// ```
///
/// Of course, the check also fails if the boxed error and the expected error belong to unrelated types:
///
/// ```
/// use dyn_error::*;
/// use std::fmt::Display;
/// use std::error::Error;
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct AlphaErr(pub u8);
/// impl Display for AlphaErr {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "Alpha error: {}", self.0)
///     }
/// }
/// impl Error for AlphaErr {};
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct BetaErr(pub u8);
/// impl Display for BetaErr {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "Beta error: {}", self.0)
///     }
/// }
/// impl Error for BetaErr {};
///
/// let result: Result<String, Box<dyn Error>> = Err(Box::new(
///     BetaErr(90)
/// ));
///
/// assert_eq!(
///     check_err_box(result, AlphaErr(90)),
///     Err(ErrBoxCheckFailure::DowncastFailed)
/// );
/// ```
///
/// Finally, the function fails also if the result is just [Ok]:
///
/// ```
/// use dyn_error::*;
/// use std::fmt::Display;
/// use std::error::Error;
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct MyErr(pub u8);
///
/// impl Display for MyErr {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "Custom error: {}", self.0)
///     }
/// }
///
/// impl Error for MyErr {};
///
/// let result: Result<String, Box<dyn Error>> = Ok("Dodo".to_string());
///
/// check_err_box(result, MyErr(7));
/// ```
pub fn check_err_box<T, E: Error + PartialEq<E> + 'static>(
    result: Result<T, Box<dyn Error>>,
    expected_err: E,
) -> Result<(), ErrBoxCheckFailure> {
    match result {
        Ok(_) => Err(ErrBoxCheckFailure::ResultIsNotErr),

        Err(ref e) => match e.downcast_ref::<E>() {
            Some(boxed_err) => {
                if expected_err == *boxed_err {
                    Ok(())
                } else {
                    Err(ErrBoxCheckFailure::NotEqual {
                        expected: expected_err.to_string(),
                        actual: boxed_err.to_string(),
                    })
                }
            }

            None => Err(ErrBoxCheckFailure::DowncastFailed),
        },
    }
}
