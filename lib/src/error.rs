use crate::NotSupportedText;
use std::num::ParseIntError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    /// Parsing from `Binary` struct to _ASCII_ `String`
    /// error.
    ///
    /// The error behind this could be either a `FromUtf8Error`
    /// or a `ParseIntError`.
    ParseBinary(String),
}

impl From<FromUtf8Error> for Error {
    fn from(utf8_error: FromUtf8Error) -> Self {
        Self::ParseBinary(utf8_error.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(parse_int_error: ParseIntError) -> Self {
        Self::ParseBinary(parse_int_error.to_string())
    }
}

impl From<NotSupportedText> for Error {
    fn from(_: NotSupportedText) -> Self {
        Self::ParseBinary(String::from("Provided Text is not Supported"))
    }
}