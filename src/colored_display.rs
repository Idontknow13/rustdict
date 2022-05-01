use ansi_term::Colour;

// TODO: Make these read from environment variables
const YELLOW: Colour = Colour::RGB(255, 255, 128);
const VIOLET: Colour = Colour::RGB(102, 0, 255);

pub fn print_colored(msg: &str) -> ansi_term::ANSIGenericString<'_, str> {
    YELLOW.bold().on(VIOLET).paint(format!(" {msg} "))
}
