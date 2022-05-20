//! A module containing the .syn parser for StarDict
//!

use std::fs::File;

/// A struct parsing a StarDict .syn file.
///
/// .syn files are optional, and thus the new()
/// method has a chance to return the `None`
/// variant.
pub struct SDSyn {
    syn: File,
}

impl SDSyn {
    pub fn new(dict_prefix: &str) -> Option<Self> {
        let filename = format!("{dict_prefix}.syn");
        if let Ok(f) = File::open(filename) {
            Some(Self { syn: f })
        } else {
            // .syn files are optional
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const FILEDIR: &str = "src/testdata/stardict-EnglishEtymology-2.4.2/EnglishEtymology";

    #[test]
    fn syn_parser_test() {
        let syn = SDSyn::new(FILEDIR);
        assert!(syn.is_none());
    }
}
