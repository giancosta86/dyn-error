/// Given an existing type `E` implementing the [Error](std::error::Error) and [PartialEq] traits,
/// automatically provides an implementation for the following equality checks *and for the symmetric ones*:
///
/// * `E == Box<E>`, returning `true` if the boxed `E` equals the given one.
///
/// * `E == Box<dyn Error>`, returning `true` if the boxed error is of type `E` and actually equals the given one.
///
/// * `E == Result<_, E>`, returning `true` if the [Result] is [Err] and its wrapped error equals the given one.
///
/// * `E == Result<_, Box<E>>`, returning `true` if the [Result] is [Err] and the boxed error equals the given one.
///
/// * `E == Result<_, Box<dyn Error>>`, returning `true` if the [Result] is [Err], while the boxed error is of type `E` and equals the given one.
///
/// ```
/// use dyn_error::impl_err_equality;
/// use std::error::Error;
///
/// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// enum MyError {
///     First,
///     Second
/// }
///
/// impl std::fmt::Display for MyError {
///    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
///        write!(
///            f,
///            "{}",
///            match self {
///                Self::First => "First error".to_string(),
///                Self::Second => "Second error".to_string(),
///            }
///       )
///    }
/// }
///
/// impl Error for MyError {}
///
/// impl_err_equality!(MyError);
///
///
/// // Ensuring E == E
/// assert_eq!(MyError::First, MyError::First);
///
/// // Checking E == Box<E> (and vice versa)
/// assert_eq!(MyError::First, Box::new(MyError::First));
/// assert_eq!(Box::new(MyError::First), MyError::First);
///
/// // Checking E == Box<dyn Error> (and vice versa)
/// let dyn_box: Box<dyn Error> = Box::new(MyError::First);
/// assert_eq!(MyError::First, dyn_box);
/// assert_eq!(dyn_box, MyError::First);
///
/// // Checking E == Result<_, E> (and vice versa)
/// let result_with_err: Result<u8, MyError> = Err(MyError::First);
/// assert_eq!(MyError::First, result_with_err);
/// assert_eq!(result_with_err, MyError::First);
///
/// // Checking E == Result<_, Boxed<E>> (and vice versa)
/// let result_with_boxed_err: Result<u8, Box<MyError>> = Err(Box::new(MyError::First));
/// assert_eq!(MyError::First, result_with_boxed_err);
/// assert_eq!(result_with_boxed_err, MyError::First);   
///     
/// // Checking E == Result<_, Boxed<dyn Error>> (and vice versa)
/// let result_with_boxed_dyn_err: Result<u8, Box<dyn Error>> = Err(Box::new(MyError::First));
/// assert_eq!(MyError::First, result_with_boxed_dyn_err);
/// assert_eq!(result_with_boxed_dyn_err, MyError::First);
/// ```
#[macro_export]
macro_rules! impl_err_equality {
    (
        // The existing type for which the equality checks must be defined
        $type: tt
    ) => {
        impl PartialEq<Box<$type>> for $type {
            fn eq(&self, other: &Box<$type>) -> bool {
                *self == *other.as_ref()
            }
        }

        impl PartialEq<$type> for Box<$type> {
            fn eq(&self, other: &$type) -> bool {
                *self.as_ref() == *other
            }
        }

        impl PartialEq<Box<dyn std::error::Error>> for $type {
            fn eq(&self, other: &Box<dyn std::error::Error>) -> bool {
                match other.downcast_ref::<$type>() {
                    Some(cast_box) => *cast_box == *self,
                    None => false,
                }
            }
        }

        impl PartialEq<$type> for Box<dyn std::error::Error> {
            fn eq(&self, other: &$type) -> bool {
                match self.downcast_ref::<$type>() {
                    Some(cast_box) => *cast_box == *other,
                    None => false,
                }
            }
        }

        impl<T> PartialEq<Result<T, $type>> for $type {
            fn eq(&self, other: &Result<T, $type>) -> bool {
                match other {
                    Ok(_) => false,
                    Err(error) => *error == *self,
                }
            }
        }

        impl<T> PartialEq<$type> for Result<T, $type> {
            fn eq(&self, other: &$type) -> bool {
                match self {
                    Ok(_) => false,
                    Err(error) => *error == *other,
                }
            }
        }

        impl<T> PartialEq<Result<T, Box<$type>>> for $type {
            fn eq(&self, other: &Result<T, Box<$type>>) -> bool {
                match other {
                    Ok(_) => false,
                    Err(error_box) => *error_box == *self,
                }
            }
        }

        impl<T> PartialEq<$type> for Result<T, Box<$type>> {
            fn eq(&self, other: &$type) -> bool {
                match self {
                    Ok(_) => false,
                    Err(error_box) => *error_box == *other,
                }
            }
        }

        impl<T> PartialEq<Result<T, Box<dyn std::error::Error>>> for $type {
            fn eq(&self, other: &Result<T, Box<dyn std::error::Error>>) -> bool {
                match other {
                    Ok(_) => false,
                    Err(error_box) => *error_box == *self,
                }
            }
        }

        impl<T> PartialEq<$type> for Result<T, Box<dyn std::error::Error>> {
            fn eq(&self, other: &$type) -> bool {
                match self {
                    Ok(_) => false,
                    Err(error_box) => *error_box == *other,
                }
            }
        }
    };
}
