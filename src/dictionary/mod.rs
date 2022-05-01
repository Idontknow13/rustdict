pub mod data;
use data::*;

use ansi_term::Colour;
use std::fmt::{Display, Formatter};

pub fn define(word: &str) -> Result<Vec<Word>, Box<dyn std::error::Error>> {
    let request_url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{word}");
    let response = reqwest::blocking::get(request_url)?.text()?;

    let data: Vec<Word> = serde_json::from_str(response.as_str())?;
    Ok(data)
}

const YELLOW: Colour = Colour::RGB(255, 255, 128);
const VIOLET: Colour = Colour::RGB(102, 0, 255);

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
            writeln!(fmtr, "{meaning}")?;
        }

        Ok(())
    }
}

impl Display for Definition {
    fn fmt(&self, fmtr: &mut Formatter) -> std::fmt::Result {
        writeln!(fmtr, "- {}", self.part_of_speech)?;

        for (num, definition) in self.definitions.iter().enumerate() {
            writeln!(fmtr, "  {}: {}", num + 1, definition.definition)?;
        }
        writeln!(fmtr, "\nSynonyms: {}", self.synonyms.join(", "))?;
        writeln!(fmtr, "Antonyms: {}", self.antonyms.join(", "))?;
        Ok(())
    }
}
