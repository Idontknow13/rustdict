// Here is where the custom errors are defined

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct CLIArgsError {}

impl Error for CLIArgsError {}

impl Display for CLIArgsError {
    fn fmt(&self, fmtr: &mut Formatter) -> std::fmt::Result {
        write!(fmtr, "Insufficient Arguments Specified")
    }
}
