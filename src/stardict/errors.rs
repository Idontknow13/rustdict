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
    ParseIntErorr(std::num::ParseIntError),
}

impl Display for StardictError {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(fmt, "StardictError: {self:?}")
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
        Self::ParseIntErorr(err)
    }
}
