use crate::colored_display::print_colored;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

/// A deeply-nested struct containing the main word defined,
/// the list of phonetic metadata, the list of meanings,
/// and the source URLs for more information.
#[derive(Debug, Deserialize)]
pub struct Word {
    word: String,
    phonetics: Vec<Phonetics>,
    meanings: Vec<Definitions>,
    #[serde(rename = "sourceUrls")]
    read_more: Vec<String>,
}

/// A container struct holding the phonetic string
/// for the word.
#[derive(Debug, Deserialize)]
struct Phonetics {
    text: Option<String>,
}

/// A nested struct containing the specific definition of
/// the word. Contains the part of speech, the exact
/// definition of the word, its synonyms and its antonyms.
#[derive(Debug, Deserialize)]
struct Definitions {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,
    definitions: Vec<Meaning>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

/// A container struct holding the exact definition of the word.
#[derive(Debug, Deserialize)]
struct Meaning {
    #[serde(rename = "definition")]
    meaning: String,
}

const URL: &str = "https://api.dictionaryapi.dev/api/v2/entries/en/";
type Error = Box<dyn std::error::Error>;
pub fn define(word: &str) -> Result<Vec<Word>, Error> {
    let request_url = format!("{URL}{word}");
    let response = reqwest::blocking::get(request_url)?.text()?;

    Ok(serde_json::from_str(response.as_str())?)
}

impl Word {
    pub fn get_word(&self) -> String {
        self.word.clone()
    }

    pub fn get_phonetics(&self) -> Vec<String> {
        let mut phonetic_strings = vec![];
        for ph in self.phonetics.iter() {
            if let Some(p) = &ph.text {
                phonetic_strings.push(p.clone());
            }
        }
        phonetic_strings
    }

    pub fn get_synonyms(&self) -> Vec<String> {
        self.meanings
            .iter()
            .flat_map(|def| def.synonyms.clone())
            .collect()
    }

    pub fn get_antonyms(&self) -> Vec<String> {
        self.meanings
            .iter()
            .flat_map(|def| def.antonyms.clone())
            .collect()
    }

    pub fn get_read_more(&self) -> Vec<String> {
        self.read_more.clone()
    }
}

impl Definitions {
    fn get_meanings(&self) -> Vec<String> {
        let mut meaning_list = vec![];
        for meaning in self.definitions.iter() {
            meaning_list.push(meaning.meaning.clone());
        }
        meaning_list
    }
}

impl Display for Word {
    fn fmt(&self, fmtr: &mut Formatter) -> std::fmt::Result {
        writeln!(fmtr)?;

        for p in self.get_phonetics().iter() {
            writeln!(fmtr, "{}    {p}", print_colored(self.get_word().as_str()))?;
        }
        for d in self.meanings.iter() {
            writeln!(fmtr, ": {}", d.part_of_speech)?;
            for (index, meaning) in d.get_meanings().iter().enumerate() {
                writeln!(fmtr, "    {}) {meaning}", index + 1)?;
            }
        }
        writeln!(fmtr)?;
        writeln!(fmtr, "Read More: {:?}", self.get_read_more())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn define(response: &str) -> Vec<Word> {
        serde_json::from_str(response).expect("Test String failed to parse.")
    }

    #[test]
    fn parsed_data_should_be_accurate() {
        const FAKE_DATA: &str = r#"
        [
            {
                "word": "faux",
                "phonetics": [{ "text": "/fo:/" }],
                "meanings": [
                    {
                        "partOfSpeech": "adjective",
                        "definitions": [
                            {
                                "definition": "Fake."
                            }
                        ],
                        "synonyms": ["fake", "fraudulent"],
                        "antonyms": ["authentic", "real"]
                    }
                ],
                "sourceUrls": ["https://example.com"]
            }
        ]
        "#;

        let data = &define(FAKE_DATA)[0];

        assert_eq!("faux".to_string(), data.get_word());
        assert_eq!(vec!["/fo:/".to_string()], data.get_phonetics());
        assert_eq!(
            vec!["fake".to_string(), "fraudulent".to_string()],
            data.get_synonyms()
        );
        assert_eq!(
            vec!["authentic".to_string(), "real".to_string()],
            data.get_antonyms()
        );
        assert_eq!(
            vec!["https://example.com".to_string()],
            data.get_read_more()
        );
    }

    #[test]
    fn get_meaning_should_work() {
        const FAKE_DEFINITION: &str = r#"
        {
            "partOfSpeech": "verb",
            "definitions": [
                {
                    "definition": "lmao."
                }
            ],
            "synonyms": ["a"],
            "antonyms": ["b"]
        }
        "#;

        let data: Definitions =
            serde_json::from_str(FAKE_DEFINITION).expect("Test String failed to parse");

        assert_eq!(vec!["lmao.".to_string()], data.get_meanings());
    }
}
