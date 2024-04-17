use dyn_error::*;
use pretty_assertions::{assert_eq as eq, assert_ne as ne};
use speculate2::*;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum MyError {
    First,
    Second,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::First => "First error".to_string(),
                Self::Second => "Second error".to_string(),
            }
        )
    }
}

impl Error for MyError {}

impl_err_equality!(MyError);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum AnotherError {
    SomeOtherValue,
}

impl std::fmt::Display for AnotherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SomeOtherValue => write!(f, "Some other value"),
        }
    }
}

impl Error for AnotherError {}

impl_err_equality!(AnotherError);

speculate! {
    describe "Equality of Box<MyError>" {
        describe "with MyError" {
            describe "when the instances are equal" {
                it "should return true" {
                    let boxed: Box<MyError> = Box::new(MyError::First);
                    eq!(boxed, MyError::First);
                }
            }

            describe "when the instances are different" {
                it "should return false" {
                    let boxed: Box<MyError> = Box::new(MyError::First);
                    ne!(boxed, MyError::Second);
                }
            }
        }
    }

    describe "Equality of Box<dyn Error> wrapping MyError" {
        describe "with another instance of MyError" {
            describe "when the instances are equal" {
                it "should return true" {
                    let dyn_boxed: Box<dyn Error> = Box::new(MyError::First);
                    eq!(dyn_boxed, MyError::First);
                }
            }

            describe "when the instances are different" {
                it "should return false" {
                    let dyn_boxed: Box<dyn Error> = Box::new(MyError::First);
                    ne!(dyn_boxed, MyError::Second);
                }
            }
        }

        describe "with an instance of another error type" {
            it "should return false" {
                let dyn_boxed: Box<dyn Error> = Box::new(MyError::First);
                ne!(dyn_boxed, AnotherError::SomeOtherValue);
            }
        }
    }

    describe "Equality of Result<_, MyError>" {
        describe "with an instance of MyError" {
            describe "when the result is Ok" {
                it "should return false" {
                    let ok_result: Result<u8, MyError> = Ok(90);
                    ne!(ok_result, MyError::First);
                }
            }

            describe "when the result is Err" {
                describe "when it wraps an equal instance" {
                    it "should return true" {
                        let err_result: Result<u8, MyError> = Err(MyError::First);
                        eq!(err_result, MyError::First);
                    }
                }

                describe "when it wraps a different instance" {
                    it "should return false" {
                        let err_result: Result<u8, MyError> = Err(MyError::First);
                        ne!(err_result, MyError::Second);
                    }
                }
            }
        }
    }

    describe "Equality of Result<_, Box<MyError>>" {
        describe "with an instance of MyError" {
            describe "when the result is Ok" {
                it "should return false" {
                    let ok_result: Result<u8, Box<MyError>> = Ok(90);
                    ne!(ok_result, MyError::First);
                }
            }

            describe "when the result is Err" {
                describe "when the Box contains an equal instance" {
                    it "should return true" {
                        let err_result: Result<u8, Box<MyError>> = Err(Box::new(MyError::First));
                        eq!(err_result, MyError::First);
                    }
                }

                describe "when the Box contains a different instance" {
                    it "should return false" {
                        let err_result: Result<u8, Box<MyError>> = Err(Box::new(MyError::First));
                        ne!(err_result, MyError::Second);
                    }
                }
            }
        }
    }

    describe "Equality of Result<_, Box<dyn Error>>" {
        describe "with an instance of MyError" {
            describe "when the result is Ok" {
                it "should return false" {
                    let ok_result: Result<u8, Box<dyn Error>> = Ok(90);
                    ne!(ok_result, MyError::First);
                }
            }

            describe "when the result is Err" {
                describe "when the Box contains an equal instance" {
                    it "should return true" {
                        let err_result: Result<u8, Box<dyn Error>> = Err(Box::new(MyError::First));
                        eq!(err_result, MyError::First);
                    }
                }

                describe "when the Box contains a different instance" {
                    it "should return false" {
                        let err_result: Result<u8, Box<dyn Error>> = Err(Box::new(MyError::First));
                        ne!(err_result, MyError::Second);
                    }
                }

                describe "when the Box contains a different type of Error" {
                    it "should return false" {
                        let err_result: Result<u8, Box<dyn Error>> = Err(Box::new(MyError::First));
                        ne!(err_result, AnotherError::SomeOtherValue);
                    }
                }
            }
        }
    }

    describe "Equality of MyError" {
        describe "with Box<MyError>" {
            describe "when the instances are equal" {
                it "should return true" {
                    let boxed: Box<MyError> = Box::new(MyError::First);
                    eq!(MyError::First, boxed);
                }
            }

            describe "when the instances are different" {
                it "should return false" {
                    let boxed: Box<MyError> = Box::new(MyError::First);
                    ne!(MyError::Second, boxed);
                }
            }
        }

        describe "with Box<dyn Error>" {
            describe "when the instances are equal" {
                it "should return true" {
                    let dyn_boxed: Box<dyn Error> = Box::new(MyError::First);
                    eq!(MyError::First, dyn_boxed);
                }
            }

            describe "when the instances are different" {
                it "should return false" {
                    let dyn_boxed: Box<dyn Error> = Box::new(MyError::First);
                    ne!(MyError::Second, dyn_boxed);
                }
            }

            describe "when the instances are of different types" {
                it "should return false" {
                    let dyn_boxed: Box<dyn Error> = Box::new(MyError::First);
                    ne!(AnotherError::SomeOtherValue, dyn_boxed);
                }
            }
        }

        describe "with Result<_, MyError>" {
            describe "when the result is Ok" {
                it "should return false" {
                    let ok_result: Result<u8, MyError> = Ok(90);
                    ne!(MyError::First, ok_result);
                }
            }

            describe "when the result is Err" {
                describe "when it wraps an equal instance" {
                    it "should return true" {
                        let err_result: Result<u8, MyError> = Err(MyError::First);
                        eq!(MyError::First, err_result);
                    }
                }

                describe "when it wraps a different instance" {
                    it "should return false" {
                        let err_result: Result<u8, MyError> = Err(MyError::First);
                        ne!(MyError::Second, err_result);
                    }
                }
            }
        }

        describe "with Result<_, Box<MyError>>" {
            describe "when the result is Ok" {
                it "should return false" {
                    let ok_result: Result<u8, Box<MyError>> = Ok(90);
                    ne!(MyError::First, ok_result);
                }
            }

            describe "when the result is Err" {
                describe "when the Box contains an equal instance" {
                    it "should return true" {
                        let err_result: Result<u8, Box<MyError>> = Err(Box::new(MyError::First));
                        eq!(MyError::First, err_result);
                    }
                }

                describe "when the Box contains a different instance" {
                    it "should return false" {
                        let err_result: Result<u8, Box<MyError>> = Err(Box::new(MyError::First));
                        ne!(MyError::Second, err_result);
                    }
                }
            }
        }

        describe "with Result<_, Box<dyn Error>>" {
            describe "when the result is Ok" {
                it "should return false" {
                    let ok_result: Result<u8, Box<dyn Error>> = Ok(90);
                    ne!(MyError::First, ok_result);
                }
            }

            describe "when the result is Err" {
                describe "when the Box contains an equal instance" {
                    it "should return true" {
                        let err_result: Result<u8, Box<dyn Error>> = Err(Box::new(MyError::First));
                        eq!(MyError::First, err_result);
                    }
                }

                describe "when the Box contains a different instance" {
                    it "should return false" {
                        let err_result: Result<u8, Box<dyn Error>> = Err(Box::new(MyError::First));
                        ne!(MyError::Second, err_result);
                    }
                }

                describe "when the Box contains a different type of Error" {
                    it "should return false" {
                        let err_result: Result<u8, Box<dyn Error>> = Err(Box::new(MyError::First));
                        ne!(AnotherError::SomeOtherValue, err_result);
                    }
                }
            }
        }
    }
}
