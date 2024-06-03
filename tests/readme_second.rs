//
// CODE FROM README
//

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
fn main() {
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
}

//
// Test
//
use speculate2::*;

speculate! {
    describe "The second code snippet in the README file" {
        it "must compile and run" {
            main();
        }
    }
}
