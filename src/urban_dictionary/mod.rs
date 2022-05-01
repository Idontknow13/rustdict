mod display;

use serde::Deserialize;
use std::collections::HashMap;

//* Urban Dictionary Fields *//
const URL: &str = "https://api.urbandictionary.com/v0/define?term=";
type Error = Box<dyn std::error::Error>;

/// A container for grabbing the list of definitions
/// inside the JSON response from Urban Dictionary.
#[derive(Clone, Debug, Deserialize)]
pub struct UrbanDictionary {
    #[serde(skip)]
    pub word: String,

    #[serde(rename = "list")]
    pub definitions: Vec<UrbanDefinition>,
}

/// A struct containing the word, its definition, as well as who
/// wrote the definition as obtained from Urban Dictionary.
#[derive(Clone, Debug, Deserialize)]
pub struct UrbanDefinition {
    word: String,
    definition: String,
    author: String,
}

/// A function which takes a word and returns a list of
/// definitions obtained from Urban Dictionary.
pub fn define(word: &str) -> Result<UrbanDictionary, Error> {
    let request_url = format!("{URL}{word}");
    let response = reqwest::blocking::get(request_url)?.text()?;

    let mut parsed_resp: UrbanDictionary = serde_json::from_str(response.as_str())?;
    parsed_resp.word = parsed_resp.definitions[0].get_word();
    Ok(parsed_resp)
}

impl UrbanDictionary {
    pub fn get_word(&self) -> String {
        self.word.clone()
    }

    pub fn get_definitions(&self) -> HashMap<String, String> {
        let mut definition_map = HashMap::new();
        for def_object in self.definitions.iter() {
            definition_map.insert(def_object.get_author(), def_object.get_definition());
        }
        definition_map
    }
}

impl UrbanDefinition {
    /// Gets the word defined by the definition
    fn get_word(&self) -> String {
        self.word.clone()
    }

    fn get_definition(&self) -> String {
        self.definition
            .chars()
            .filter(|chr| chr != &'\r' && chr != &'\n' && chr != &'[' && chr != &']')
            .collect()
    }

    fn get_author(&self) -> String {
        self.author.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static FAKE_DATA: &str = r#"
    {
        "list": [
            {
                "definition": "Fake definition.",
                "author": "fake1",
                "word": "faux"
            },
            {
                "definition": "Fake definition #2.",
                "author": "fake2",
                "word": "faux"
            }
        ]
    }"#;

    fn define_urban(response: &str) -> UrbanDictionary {
        let mut parsed_data: UrbanDictionary =
            serde_json::from_str(response).expect("Test String failed to parse");
        parsed_data.word = parsed_data.definitions[0].get_word();
        parsed_data
    }

    #[test]
    fn get_word_should_work() {
        let data = define_urban(FAKE_DATA);
        assert_eq!("faux".to_string(), data.get_word());
    }

    #[test]
    fn get_definition_by_author() {
        let data = define_urban(FAKE_DATA);
        assert_eq!(
            "Fake definition.".to_string(),
            data.get_definitions()["fake1"]
        );
        assert_eq!(
            "Fake definition #2.".to_string(),
            data.get_definitions()["fake2"]
        );
    }

    #[test]
    fn correct_definition_count() {
        let mut counter = 0u8;
        let data = define_urban(FAKE_DATA);
        for _ in data.get_definitions().keys() {
            counter += 1;
        }
        assert_eq!(2, counter);
    }
}
