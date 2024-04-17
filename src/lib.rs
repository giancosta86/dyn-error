//! This crate provides error-related utility macros.
//!
//! In particular, its core feature resides in the
//! automatic implementation of a wide range of
//! equality checks via the [impl_err_equality] macro,
//! which is especially useful in documentation examples
//! as well as in testing.
//!
mod dyn_result;
mod equality;
