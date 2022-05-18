mod errors;

use errors::StardictError;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

type StardictResult<T> = std::result::Result<T, StardictError>;

/// A struct for manipulating StarDict dictionaries.
struct Dictionary {}

/// A struct parsing a StarDict .ifo file.
///
/// An .ifo file follows the following format:
///
/// StarDict's dict ifo file
/// version=2.4.2
/// [options]
///
/// [options] (!req is a required field)
/// bookname=!req
/// wordcount=!req
/// synwordcount=!req(if .syn exists)
/// idxfilesize=!req
/// idxoffsetbits=!req(if version is 3.0.0)
/// author=
/// email=
/// website=
/// description=
/// date=
/// sametypesequence=!req(important)
///
#[derive(Debug, PartialEq, Default)]
pub struct SDifo {
    // Required fields
    version: String,
    bookname: String,
    wordcount: u32,
    idxfilesize: usize,
    sametypesequence: char,
    // Optional fields
    synwordcount: Option<usize>,
    idxoffsetbits: Option<usize>,
    author: Option<String>,
    email: Option<String>,
    website: Option<String>,
    desc: Option<String>,
    date: Option<String>,
}

impl SDifo {
    /// Creates a new SDifo object.
    pub fn new(dict_prefix: &str) -> StardictResult<Self> {
        let mut _self = Self::default();
        let mut config = BTreeMap::new();
        let filename = format!("{dict_prefix}.ifo");

        let ifo = File::open(filename)?;
        let ifo_reader = BufReader::new(ifo);

        // Create config with fields
        for (index, line) in ifo_reader.lines().enumerate() {
            if index == 0 {
                continue; // Skips header
            }
            if let Ok(l) = line {
                let pair: Vec<&str> = l.split('=').map(|s| s.trim_end()).collect();
                config.insert(pair[0].to_string(), pair[1].to_string());
            }
        }

        // Required fields
        _self.version = config.get("version").expect("Version is required.").clone();
        _self.bookname = config
            .get("bookname")
            .expect("Bookname is required.")
            .clone();
        _self.wordcount = config
            .get("wordcount")
            .expect("Wordcount is required.")
            .parse()?;
        _self.idxfilesize = config
            .get("idxfilesize")
            .expect("Idxfilesize is required.")
            .parse()?;
        _self.sametypesequence = config
            .get("sametypesequence")
            .expect("Sametypesequence is highly required.")
            .chars()
            .next()
            .expect("Sametypesequence should yield a character");
        // Optional fields
        if _self.version == "3.0.0" {
            _self.synwordcount = Some(
                config
                    .get("synwordcount")
                    .expect("Synwordcount is required at 3.0.0")
                    .parse()?,
            );
            _self.idxoffsetbits = Some(
                config
                    .get("idxoffsetbits")
                    .unwrap_or(&"32".to_string())
                    .parse()?,
            );
        }
        _self.author = Some(config.get("author").unwrap_or(&"".to_string()).to_string());
        _self.email = Some(config.get("email").unwrap_or(&"".to_string()).to_string());
        _self.website = Some(config.get("website").unwrap_or(&"".to_string()).to_string());
        _self.desc = Some(config.get("desc").unwrap_or(&"".to_string()).to_string());
        _self.date = Some(config.get("date").unwrap_or(&"".to_string()).to_string());

        Ok(_self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // const TESTDIR: &str = "src/testdata";
    const FILEDIR: &str = "src/testdata/stardict-EnglishEtymology-2.4.2/EnglishEtymology";

    #[test]
    fn ifo_parser_test() {
        let ifo = SDifo::new(FILEDIR).expect("File should parse properly");
        assert_eq!(ifo.version, "2.4.2");
        assert_eq!(ifo.wordcount, 18380);
        assert_eq!(ifo.idxfilesize, 303020);
        assert_eq!(ifo.bookname, "English Etymology");
        assert_eq!(ifo.sametypesequence, 'm');
    }

    #[test]
    #[ignore]
    fn idx_parser_test() {}

    #[test]
    #[ignore]
    fn dict_parser_test() {}

    #[test]
    #[ignore]
    fn dictionary_test() {}
}
