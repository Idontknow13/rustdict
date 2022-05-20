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
    IO(std::io::Error),
    ParseInt(std::num::ParseIntError),
    InvalidByte(&'static str),
    WordCount(&'static str),
    FromUtf8(std::string::FromUtf8Error),
    Unpacking(packed_struct::PackingError),
}

impl Display for StardictError {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        match self {
            StardictError::IO(e) => write!(fmt, "IOError: {e}"),
            StardictError::ParseInt(e) => write!(fmt, "ParseIntError: {e}"),
            StardictError::InvalidByte(e) => write!(fmt, "InvalidByteError: {e}"),
            StardictError::WordCount(e) => write!(fmt, "WordCountError: {e}"),
            StardictError::FromUtf8(e) => write!(fmt, "FromUTF8Error: {e}"),
            StardictError::Unpacking(e) => write!(fmt, "UnpackingError: {e}"),
        }
    }
}

impl Error for StardictError {}

impl From<std::io::Error> for StardictError {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<std::num::ParseIntError> for StardictError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

impl From<std::string::FromUtf8Error> for StardictError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8(err)
    }
}

impl From<packed_struct::PackingError> for StardictError {
    fn from(err: packed_struct::PackingError) -> Self {
        Self::Unpacking(err)
    }
}
