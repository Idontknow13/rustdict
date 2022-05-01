//! # Rustdict
//!
//! A dictionary CLI tool inspired by BetaPictoris' [dict](https://github.com/BetaPictoris/dict)
//! CLI tool as well as Matthew Hartman's [Word Lookup](https://github.com/matthewhartman/word-lookup)
//! app.
//!

mod colored_display;
mod dictionary;
mod urban_dictionary;

const HELP: &str = r#"
RustDict - a dictionary lookup tool written in Rust.

USAGE:
    rdict [<OPTION>] [<PARAM>] [WORD]

OPTIONS:
    -h / --help                         Display this help message
    -u / --urban                        Grab the urban dictionary definition of the word

EXAMPLE USAGE:
    rdict -h                             Prints this help message
    rdict yester                         Defines the word "yester"
    rdict -u ligma                       Defines "ligma" from Urban Dictionary
    rdict -u "poison pill"               Defines "poison pill" from Urban Dictionary
"#;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    cli(&args[1..]);
}

fn cli(args: &[String]) {
    if args.is_empty() {
        print!("{HELP}");
        return;
    }
    if args.len() == 1 {
        match args[0].as_str() {
            "-h" | "--help" => print!("{HELP}"),
            _ => try_define(args[0].as_str()),
        }
        return;
    }
    argparse(args)
}

fn argparse(args: &[String]) {
    match args[0].as_str() {
        "-u" | "--urban" => try_define_urban(args[1].as_str()),
        _ => print!("{HELP}"),
    }
}

fn try_define(word: &str) {
    if let Ok(definitions) = dictionary::define(word) {
        definitions
            .iter()
            .for_each(|definition| print!("{definition}"));
    } else {
        println!("Definition for {word} not found.");
    }
}

fn try_define_urban(word: &str) {
    if let Ok(definitions) = urban_dictionary::define(word) {
        print!("{definitions}");
    } else {
        println!("Urban Dictionary failed to make a connection. Please try again later.");
    }
}
