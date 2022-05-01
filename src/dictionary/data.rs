// Contains the definitions of the data structures
// needed for the app's custom API.
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Word {
    pub word: String,
    pub phonetics: Vec<PhoneticMetadata>,
    pub meanings: Vec<Definition>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct PhoneticMetadata {
    pub text: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Definition {
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: String,
    pub definitions: Vec<DefinitionBody>,
    pub synonyms: Vec<String>,
    pub antonyms: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct DefinitionBody {
    pub definition: String,
}
