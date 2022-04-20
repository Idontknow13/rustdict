// Contains the definitions of the data structures
// needed for the app's custom API.

pub struct Word {
    word: String,
    phonetics: Vec<PhoneticMetadata>,
    meanings: Vec<Definition>,
}

pub struct PhoneticMetadata {
    text: Option<String>,
    audio: String,
}

pub struct Definition {
    part_of_speech: String,
    definitions: Vec<DefinitionBody>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

pub struct DefinitionBody {
    definition: String,
}
