//! # Rustdict
//!
//! A dictionary CLI tool inspired by BetaPictoris' [dict](https://github.com/BetaPictoris/dict)
//! CLI tool as well as Matthew Hartman's [Word Lookup](https://github.com/matthewhartman/word-lookup)
//! app.
//!

mod api;
use api::data::*;

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
        "-s" | "--synonym" => get_semantics(args[1].as_str(), Semantics::Synonyms),
        "-a" | "--antonym" => get_semantics(args[1].as_str(), Semantics::Antonyms),
        "-u" | "--urban" => try_define_urban(args[1].as_str()),
        "-c" | "--category" if args.len() > 3 => todo!(),
        "-h" | "--help" | _ => print!("{HELP}"),
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

fn get_semantics(word: &str, semantic: Semantics) {
    let word_def = api::define(word).expect("No definitions found");
    let semantics = get_semantics_helper(&word_def, &semantic);
    println!("{word}");
    println!("  {semantic:?}: {semantics:?}");
}

fn get_semantics_helper(definitions: &[Word], semantic: &Semantics) -> Vec<String> {
    let mut semantics = vec![];
    for word in definitions {
        word.meanings.iter().for_each(|def| match semantic {
            api::data::Semantics::Synonyms => semantics.push(def.synonyms.clone()),
            api::data::Semantics::Antonyms => semantics.push(def.antonyms.clone()),
        });
    }
    semantics.into_iter().flatten().collect()
}
