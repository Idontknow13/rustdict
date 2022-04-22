pub mod data;
use data::*;

use ansi_term::Colour;
use std::fmt::{Display, Formatter};

const YELLOW: Colour = Colour::RGB(255, 255, 128);
const VIOLET: Colour = Colour::RGB(102, 0, 255);

pub fn define(word: &str) -> Result<Vec<Word>, Box<dyn std::error::Error>> {
    let request_url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{word}");
    let response = reqwest::blocking::get(request_url)?.text()?;

    let data: Vec<Word> = serde_json::from_str(response.as_str())?;
    Ok(data)
}

pub fn define_urban(word: &str) -> Result<UrbanContainer, Box<dyn std::error::Error>> {
    let request_url = format!("https://api.urbandictionary.com/v0/define?term={word}");
    let response = reqwest::blocking::get(request_url)?.text()?;

    let data: UrbanContainer = serde_json::from_str(response.as_str())?;
    Ok(data)
}

pub fn print_colored(msg: &str) -> ansi_term::ANSIGenericString<'_, str> {
    YELLOW.bold().on(VIOLET).paint(format!(" {msg} "))
}

impl Display for Word {
    fn fmt(&self, fmtr: &mut Formatter) -> std::fmt::Result {
        writeln!(fmtr)?;

        write!(fmtr, "{}    ", print_colored(self.word.as_str()))?;

        for phonetic in self.phonetics.iter() {
            if let Some(phoneme) = &phonetic.text {
                writeln!(fmtr, "{}", phoneme)?;
                continue;
            }
        }
        if self.phonetics.is_empty() {
            writeln!(fmtr)?;
        }

        for meaning in self.meanings.iter() {
            writeln!(fmtr, "- {}", meaning.part_of_speech)?;

            for (num, definition) in meaning.definitions.iter().enumerate() {
                writeln!(fmtr, "  {}: {}", num + 1, definition.definition)?;
            }
            writeln!(fmtr, "\nSynonyms: {}", meaning.synonyms.join(", "))?;
            writeln!(fmtr, "Antonyms: {}", meaning.antonyms.join(", "))?;
        }

        Ok(())
    }
}

impl Display for UrbanContainer {
    fn fmt(&self, fmtr: &mut Formatter) -> std::fmt::Result {
        writeln!(fmtr)?;

        let main_word = &self.definitions[0].word;
        writeln!(fmtr, "{}", print_colored(main_word))?;

        for definition in self.definitions.iter() {
            let clean_definition = definition
                .definition
                .chars()
                .filter(|chr| !(chr == &'\r' || chr == &'\n' || chr == &'[' || chr == &']'))
                .collect::<String>();
            writeln!(
                fmtr,
                "  ー {} (by: {})",
                clean_definition, definition.author
            )?;
        }

        Ok(())
    }
}
