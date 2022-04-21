pub mod data;
use data::*;

use std::fmt::{Display, Formatter};

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

impl Display for Word {
    fn fmt(&self, fmtr: &mut Formatter) -> std::fmt::Result {
        writeln!(fmtr, "")?;

        for phonetic in self.phonetics.iter() {
            if let Some(phoneme) = &phonetic.text {
                writeln!(fmtr, "{}    {}", self.word, phoneme)?;
            }
        }

        for meaning in self.meanings.iter() {
            writeln!(fmtr, "- {}", meaning.part_of_speech)?;

            for (num, definition) in meaning.definitions.iter().enumerate() {
                writeln!(fmtr, "  {}: {}", num + 1, definition.definition)?;
            }
        }

        Ok(())
    }
}

impl Display for UrbanContainer {
    fn fmt(&self, fmtr: &mut Formatter) -> std::fmt::Result {
        writeln!(fmtr, "")?;

        let main_word = &self.definitions[0].word;
        writeln!(fmtr, "{main_word}")?;

        for definition in self.definitions.iter() {
            let clean_definition = definition
                .definition
                .chars()
                .filter(|chr| !(chr == &'\r' || chr == &'\n' || chr == &'[' || chr == &']'))
                .collect::<String>();
            writeln!(fmtr, "  - {} (by: {})", clean_definition, definition.author)?;
        }

        Ok(())
    }
}
