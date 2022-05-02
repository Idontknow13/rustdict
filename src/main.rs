//! # Rustdict
//!
//! A dictionary CLI tool inspired by BetaPictoris' [dict](https://github.com/BetaPictoris/dict)
//! CLI tool as well as Matthew Hartman's [Word Lookup](https://github.com/matthewhartman/word-lookup)
//! app.
//!

mod colored_display;
mod dictionary;
mod urban_dictionary;
use dictionary::Semantic;

const HELP: &str = r#"
RustDict - a dictionary lookup tool written in Rust.

USAGE:
    rdict [<OPTION>] [<PARAM>] [WORD]

OPTIONS:
    -h / --help                         Display this help message
    -u / --urban                        Grab the urban dictionary definition of the word
    -s / --syn                          Grab the definition of the word + its synonyms
    -a / --ant                          Grab the definition of the word + its antonyms

EXAMPLE USAGE:
    rdict yester                        Defines the word "yester"
    rdict -u "poison pill"              Defines "poison pill" from Urban Dictionary
    rdict -s coward                     Defines coward and grabs its synonyms
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
        "-s" | "--syn" => try_get_semantics(args[1].as_str(), Semantic::Synonym),
        "-a" | "--ant" => try_get_semantics(args[1].as_str(), Semantic::Antonym),
        _ => print!("{HELP}"),
    }
}

//* Definition Wrappers *//

fn try_define(word: &str) {
    match dictionary::define(word) {
        Ok(definitions) => {
            for definition in definitions.iter() {
                print!("{definition}");
            }
        }
        Err(_) => println!("Definition for {word} not found."),
    }
}

fn try_define_urban(word: &str) {
    match urban_dictionary::define(word) {
        Ok(dictionary_def) => print!("{dictionary_def}"),
        Err(_) => println!("Urban Dictionary failed to make a connection. Please try again later."),
    }
}

//* Semantic Wrappers *//

fn try_get_semantics(word: &str, semantic: Semantic) {
    let definitions = match dictionary::define(word) {
        Ok(definitions) => definitions,
        Err(_) => {
            println!("Definition for {word} not found.");
            return;
        }
    };

    for definition in definitions {
        print!("{definition}");
        match semantic {
            Semantic::Synonym => {
                let synonyms = definition.get_semantics(&semantic);
                println!("Synonyms: {synonyms:?} ");
            }
            Semantic::Antonym => {
                let antonyms = definition.get_semantics(&semantic);
                println!("Antonyms: {antonyms:?} ");
            }
        }
    }
}
