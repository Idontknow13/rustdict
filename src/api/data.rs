// Contains the definitions of the data structures
// needed for the app's custom API.
#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Word {
    word: String,
    phonetics: Vec<PhoneticMetadata>,
    meanings: Vec<Definition>,
}

#[derive(Debug, Deserialize)]
pub struct PhoneticMetadata {
    text: Option<String>,
    audio: String,
}

#[derive(Debug, Deserialize)]
pub struct Definition {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,
    definitions: Vec<DefinitionBody>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DefinitionBody {
    definition: String,
}