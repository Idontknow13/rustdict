//! # Rustdict
//!
//! A dictionary CLI tool inspired by BetaPictoris' [dict](https://github.com/BetaPictoris/dict)
//! CLI tool as well as Matthew Hartman's [Word Lookup](https://github.com/matthewhartman/word-lookup)
//! app.
//!

mod colored_display;
mod dictionary;
mod stardict;
mod urban_dictionary;

use clap::Parser;
use dictionary::Semantic;

/// A dictionary CLI tool written in Rust
/// insired by BetaPictoris' dict as well as
/// Matthew Hartman's Word Lookup tool.
#[derive(Parser)]
struct Cli {
    /// The word you want to search the definition of
    word: String,

    /// Search in Urban Dictionary
    #[clap(short, long)]
    urban: bool,

    /// Enable synonyms (will be prioritized over antonyms)
    #[clap(short, long = "syn")]
    synonyms: bool,

    /// Enable antonyms
    #[clap(short, long = "ant")]
    antonyms: bool,
}

fn main() {
    let args = Cli::parse();
    if args.urban {
        try_define_urban(&args.word);
    } else {
        try_define(&args.word);
    }

    let semantic = match (args.synonyms, args.antonyms) {
        (true, _) => Semantic::Synonym,
        (_, true) => Semantic::Antonym,
        (false, false) => return,
    };
    try_get_semantics(&args.word, semantic);
}

//* Definition Wrappers *//

fn try_define(word: &str) {
    if let Ok(definitions) = dictionary::define(word) {
        for definition in definitions.iter() {
            print!("{definition}");
        }
        return;
    }
    println!("Definition for {word} not found.");
}

fn try_define_urban(word: &str) {
    if let Ok(dictionary_def) = urban_dictionary::define(word) {
        print!("{dictionary_def}");
        return;
    }
    println!("Urban Dictionary failed to make a connection. Please try again later.");
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

    for definition in definitions.iter() {
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
