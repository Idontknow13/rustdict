// Here is where the custom errors are defined

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct CLINoArgsError {}

impl Error for CLINoArgsError {}

impl Display for CLINoArgsError {
    fn fmt(&self, fmtr: &mut Formatter) -> std::fmt::Result {
        write!(fmtr, "CLI Runtime Error: No Arguments Specified")
    }
}
