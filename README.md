# dyn-error

_Error-related utilites for Rust_

[![CI](https://github.com/giancosta86/dyn-error/actions/workflows/publish-to-crates.yml/badge.svg)](https://github.com/giancosta86/dyn-error/actions/workflows/publish-to-crates.yml)
![Crates.io Version](https://img.shields.io/crates/v/dyn-error?style=flat&logo=rust)

This crate provides error-related utilities.

In particular, the `assert_err_box` macro is a minimalist way to test the inner value of an `Err<Box<dyn Error>>` from a `Result`:

```rust
use dyn_error::*;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
struct MyErr(pub u8);

impl Display for MyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Custom error: {}", self.0)
    }
}

impl Error for MyErr {}

fn main() -> Result<(), Box<dyn Error>> {
    let result: Result<String, Box<dyn Error>> = Err(Box::new(MyErr(90)));

    assert_err_box!(result, MyErr(90));
    Ok(())
}
```

For more fine-grained control, the `check_err_box` function performs the same test but returns a `Result` in lieu of calling `panic`:

```rust
use dyn_error::*;
use std::error::Error;
use std::fmt::Display;

//
// Declaring a custom Error type
//
#[derive(Debug, PartialEq, Eq)]
struct MyErr(pub u8);

impl Display for MyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Custom error: {}", self.0)
    }
}

impl Error for MyErr {}

//
// Test scenarios
//
fn main() -> Result<(), Box<dyn Error>> {
    let result: Result<String, Box<dyn Error>> = Err(Box::new(MyErr(90)));
    assert_eq!(check_err_box(result, MyErr(90)), Ok(()));

    let result: Result<String, Box<dyn Error>> = Err(Box::new(MyErr(90)));
    assert_eq!(
        check_err_box(result, MyErr(7)),
        Err(ErrBoxCheckFailure::NotEqual {
            expected: "Custom error: 7".to_string(),
            actual: "Custom error: 90".to_string()
        })
    );

    Ok(())
}
```

## Crates.io

https://crates.io/crates/dyn-error

## Documentation

https://docs.rs/dyn-error

## License

[MIT](LICENSE)
