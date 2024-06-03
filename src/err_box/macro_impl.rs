/// Macro that internally calls [check_err_box](crate::check_err_box),
/// issuing [panic] in case of failure.
///
/// If the actual error is equal to the expected error, the macro has no effects:
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
/// assert_err_box!(
///     result,
///     MyErr(90)
/// );
/// ```
///
/// In case of inequality, the macro initiates a [panic]:
///
/// ```should_panic
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
/// assert_err_box!(result, MyErr(7));
/// ```
///
/// Of course, the macro panics if the boxed error and the expected error belong to incompatible types:
///
/// ```should_panic
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
/// assert_err_box!(result, AlphaErr(90));
/// ```
///
/// Finally, the macro panics also if the result is just [Ok]:
///
/// ```should_panic
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
/// assert_err_box!(result, MyErr(7));
/// ```
#[macro_export]
macro_rules! assert_err_box {
    ($result: expr, $expected_err: expr) => {{
        let result = check_err_box($result, $expected_err);

        if let Err(error) = result {
            panic!("{}() failed: {}", stringify!(check_err_box), error);
        }
    }};
}
