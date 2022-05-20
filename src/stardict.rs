mod errors;
pub mod idx;
pub mod ifo;
pub mod syn;

use errors::StardictError;
use std::{collections::BTreeMap, fs::File, io::Read};

type StardictResult<T> = std::result::Result<T, StardictError>;

/// A struct for manipulating StarDict dictionaries.
pub struct Stardict {
    pub in_memory: bool,
    pub ifo: ifo::SDifo,
    pub idx: idx::SDidx,
    pub dict: SDdict,
    pub syn: Option<syn::SDSyn>,
    pub cache: BTreeMap<String, String>,
}

impl Stardict {}

pub struct SDdict {}

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
