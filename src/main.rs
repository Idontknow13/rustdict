//! # Rustdict
//!
//! A dictionary CLI tool inspired by BetaPictoris' [dict](https://github.com/BetaPictoris/dict)
//! CLI tool as well as Matthew Hartman's [Word Lookup](https://github.com/matthewhartman/word-lookup)
//! app.
//!

use clap::Parser;

mod colored_display;
mod dictionary;
mod urban_dictionary;
use dictionary::Semantic;

#[derive(Parser)]
struct Cli {
    /// The word you want to search the definition of
    word: String,

    /// Search in Urban Dictionary
    #[clap(short, long)]
    urban: bool,

    /// Enable synonyms
    #[clap(short, long)]
    synonym: bool,

    /// Enable antonyms
    #[clap(short, long)]
    antonym: bool,
}

fn main() {
    let args = Cli::parse();
    if args.urban {
        try_define_urban(&args.word);
    } else {
        try_define(&args.word);
    }

    let semantic = match (args.synonym, args.antonym) {
        (true, _) => Semantic::Synonym,
        (_, true) => Semantic::Antonym,
        (false, false) => return,
    };
    try_get_semantics(&args.word, semantic);
}

// fn argparse(args: &[String]) {
//     match args[0].as_str() {
//         "-u" | "--urban" => try_define_urban(args[1].as_str()),
//         "-s" | "--syn" => try_get_semantics(args[1].as_str(), Semantic::Synonym),
//         "-a" | "--ant" => try_get_semantics(args[1].as_str(), Semantic::Antonym),
//         _ => print!("{HELP}"),
//     }
// }

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
