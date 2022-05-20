//! This module holds the errors for the stardict
//! parser.

use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// An enum holding the possible error types of
/// the Stardict parser.
#[derive(Debug)]
pub enum StardictError {
    IOError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    InvalidByteError(&'static str),
    WordCountError(&'static str),
}

impl Display for StardictError {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        match self {
            StardictError::IOError(e) => write!(fmt, "IOError: {e}"),
            StardictError::ParseIntError(e) => write!(fmt, "ParseIntError: {e}"),
            StardictError::InvalidByteError(e) => write!(fmt, "InvalidByteError: {e}"),
            StardictError::WordCountError(e) => write!(fmt, "WordCountError: {e}"),
        }
    }
}

impl Error for StardictError {}

impl From<std::io::Error> for StardictError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<std::num::ParseIntError> for StardictError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ParseIntError(err)
    }
}
