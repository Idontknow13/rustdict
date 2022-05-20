//! A module containing the .ifo parser for StarDict
//!

use super::StardictResult;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

/// A struct parsing a StarDict .ifo file.
///
/// An .ifo file follows the following format:
///
/// StarDict's dict ifo file
/// version=2.4.2
/// [options]
///
/// [options] <!req is a required field>
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
    pub version: String,
    pub bookname: String,
    pub wordcount: usize,
    pub idxfilesize: usize,
    pub idxoffsetbits: usize,
    pub sametypesequence: char,
    // Optional fields
    pub synwordcount: Option<usize>,
    pub author: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub desc: Option<String>,
    pub date: Option<String>,
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
                let pair: Vec<String> = l.split('=').map(|s| s.trim_end().to_string()).collect();
                config.insert(pair[0].clone(), pair[1].clone());
            }
        }

        // Required fields
        _self.version = config.remove("version").expect("Version is required.");
        _self.bookname = config.remove("bookname").expect("Bookname is required.");
        _self.wordcount = config
            .remove("wordcount")
            .expect("Wordcount is required.")
            .parse()?;
        _self.idxfilesize = config
            .remove("idxfilesize")
            .expect("Idxfilesize is required.")
            .parse()?;
        _self.sametypesequence = config
            .remove("sametypesequence")
            .expect("Sametypesequence is highly required.")
            .chars()
            .next()
            .expect("Sametypesequence should yield a character");
        _self.idxoffsetbits = config
            .remove("idxoffsetbits")
            .unwrap_or_else(|| String::from("32"))
            .parse()?;
        // Optional fields
        if _self.version == "3.0.0" {
            _self.synwordcount = Some(
                config
                    .remove("synwordcount")
                    .expect("Synwordcount is required at 3.0.0")
                    .parse()?,
            );
        }
        _self.author = Some(config.remove("author").unwrap_or_default());
        _self.email = Some(config.remove("email").unwrap_or_default());
        _self.website = Some(config.remove("website").unwrap_or_default());
        _self.desc = Some(config.remove("description").unwrap_or_default());
        _self.date = Some(config.remove("date").unwrap_or_default());

        Ok(_self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const FILEDIR: &str = "src/testdata/stardict-EnglishEtymology-2.4.2/EnglishEtymology";

    #[test]
    fn ifo_parser_test() {
        let ifo = SDifo::new(FILEDIR).expect("File should parse properly");
        assert_eq!("2.4.2", ifo.version);
        assert_eq!(18380, ifo.wordcount);
        assert_eq!(303020, ifo.idxfilesize);
        assert_eq!("English Etymology", ifo.bookname);
        assert_eq!('m', ifo.sametypesequence);
    }
}
