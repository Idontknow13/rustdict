use super::*;
use ansi_term::Colour;
use std::fmt::{Display, Formatter};

// TODO: Make these read from environment variables
const YELLOW: Colour = Colour::RGB(255, 255, 128);
const VIOLET: Colour = Colour::RGB(102, 0, 255);

pub fn print_colored(msg: &str) -> ansi_term::ANSIGenericString<'_, str> {
    YELLOW.bold().on(VIOLET).paint(format!(" {msg} "))
}

impl Display for UrbanDictionary {
    fn fmt(&self, fmtr: &mut Formatter) -> std::fmt::Result {
        writeln!(fmtr)?;

        writeln!(fmtr, "{}", print_colored(self.get_word().as_str()))?;
        for (author, definition) in self.get_definitions().iter() {
            writeln!(fmtr, "    - {definition} (by: {author})")?;
        }

        Ok(())
    }
}
