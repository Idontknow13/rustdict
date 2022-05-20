pub mod dict;
mod errors;
pub mod idx;
pub mod ifo;
pub mod syn;

use errors::StardictError;
use std::{collections::BTreeMap, fs::File, io::Read};

type StardictResult<T> = std::result::Result<T, StardictError>;

/// A struct for manipulating StarDict dictionaries.
pub struct Stardict<'dict> {
    pub in_memory: bool,
    pub ifo: ifo::SDifo,
    pub idx: idx::SDidx,
    pub dict: dict::SDdict<'dict>,
    pub syn: Option<syn::SDSyn>,
    pub cache: BTreeMap<String, String>,
}

impl Stardict<'_> {}

impl Default for Stardict<'_> {
    fn default() -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // const TESTDIR: &str = "src/testdata";
    const FILEDIR: &str = "src/testdata/stardict-EnglishEtymology-2.4.2/EnglishEtymology";

    #[test]
    #[ignore = "Unimplemented"]
    fn dict_parser_test() {}

    #[test]
    #[ignore = "Unimplemented"]
    fn dictionary_test() {}
}
