pub mod data;
pub mod errors;
use data::*;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

const URL: &'static str = "https://api.dictionaryapi.dev/api/v2/entries/en/";

pub fn define(word: &str) -> Result<Vec<Word>> {
    let request_url = format!("{URL}{word}");
    let response = reqwest::blocking::get(request_url)?.text()?;
    Ok(serde_json::from_str(response.as_str()).expect("Word Parsing Error"))
}
