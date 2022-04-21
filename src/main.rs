//! # Rustdict
//!
//! A dictionary CLI tool inspired by BetaPictoris' [dict](https://github.com/BetaPictoris/dict)
//! CLI tool as well as Matthew Hartman's [Word Lookup](https://github.com/matthewhartman/word-lookup)
//! app.
//!

mod api;

const HELP: &str = r#"
RustDict - a dictionary lookup tool written in Rust.

USAGE:
    rd [<OPTION>] [<PARAM>] [WORD]

OPTIONS:
    -h / --help                         Display this help message
    -s / --synonym                      Grab the synonyms of the word
    -a / --antonym                      Grab the antonyms of the word
    -u / --urban                        Grab the urban dictionary definition of the word
    -t / --top                          Grab the top definition of the word
    -c / --category [<PART_OF_SPEECH>]  Grab the definition of a word with a specific part of speech.
"#;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    cli(&args[1..]);
}

fn cli(args: &[String]) {
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
        "-s" | "--synonym" => todo!(),
        "-a" | "--antonym" => todo!(),
        "-u" | "--urban" => try_define_urban(args[1].as_str()),
        "-t" | "--top" => todo!(),
        "-c" | "--category" if args.len() > 3 => todo!(),
        "-h" | "--help" => print!("{HELP}"),
        _ => todo!(),
    }
}

fn try_define(word: &str) {
    if let Some(definitions) = api::define(word).ok() {
        definitions
            .iter()
            .for_each(|definition| print!("{definition}"));
        return;
    }
    print!("{HELP}");
}

fn try_define_urban(word: &str) {
    if let Some(definitions) = api::define_urban(word).ok() {
        print!("{definitions}");
        return;
    } else {
        print!("{HELP}");
    }
}
