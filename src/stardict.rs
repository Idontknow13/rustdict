mod errors;
pub mod idx;
pub mod ifo;
pub mod syn;

use errors::StardictError;
use std::{collections::BTreeMap, fs::File, io::Read};

type StardictResult<T> = std::result::Result<T, StardictError>;

/// A struct for manipulating StarDict dictionaries.
struct Stardict {
    in_memory: bool,
    ifo: ifo::SDifo,
    idx: idx::SDidx,
    dict: SDdict,
    syn: Option<syn::SDSyn>,
    cache: BTreeMap<String, String>,
}

impl Stardict {}

struct SDdict {}

#[cfg(test)]
mod tests {
    // use super::*;

    // const TESTDIR: &str = "src/testdata";
    const FILEDIR: &str = "src/testdata/stardict-EnglishEtymology-2.4.2/EnglishEtymology";

    #[test]
    #[ignore]
    fn dict_parser_test() {}

    #[test]
    #[ignore]
    fn dictionary_test() {}
}
