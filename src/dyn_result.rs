/// Convenience macro declaring a type alias for `Result<T, Box<dyn std::error::Error>>`.
///
/// ```
/// use dyn_error::declare_dyn_result;
///
/// //This macro call creates the type alias.
/// declare_dyn_result!(pub, GenericResult);
///
/// fn f(x: i8) -> GenericResult<u8> {
///     if x >= 0 {
///         Ok(x as u8)
///     } else {
///         // This is a somewhat contrived example, for the sake of brevity:
///         // you'll certainly want to use a custom error type instead.
///         Err(Box::new(std::io::Error::other(format!("{} is < 0!", x))))
///     }
/// }
///
/// fn main() -> GenericResult<()> {
///     assert_eq!(f(9)?, 9);
///
///     assert_eq!(f(-8).unwrap_err().to_string(), "-8 is < 0!");
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! declare_dyn_result {
    (
        //The type visibility, such as "pub" or "pub(self)".
        $visibility: vis,

        //The identifier of the type to create.
        $type: ident
    ) => {
        /// The most generic [Error](std::error::Error)-based [Result](std::result::Result).
        $visibility type $type<T> = Result<T, Box<dyn std::error::Error>>;
    };
}
