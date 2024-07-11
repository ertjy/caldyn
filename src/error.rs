use alloc::string::String;
use core::fmt;
use core::fmt::{Display, Formatter};

/// Error type for the caldyn crate
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// Error while parsing an expression
    ParseError(String),
    /// Unknown variable during evaluation
    NameError(String),
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match *self {
            Error::ParseError(ref message) => write!(fmt, "ParseError: {}", message),
            Error::NameError(ref message) => write!(fmt, "NameError: {}", message),
        }
    }
}