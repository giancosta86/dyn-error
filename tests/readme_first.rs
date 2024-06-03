//
// CODE FROM README
//

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

fn main() {
    let result: Result<String, Box<dyn Error>> = Err(Box::new(MyErr(90)));

    assert_err_box!(result, MyErr(90));
}

//
// TEST
//
use speculate2::*;

speculate! {
    describe "The first code snippet in the README file" {
        it "must compile and run" {
            main();
        }
    }
}
