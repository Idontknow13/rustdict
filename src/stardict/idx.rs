//! A module containing the .idx parser for StarDict.
//!

use super::{Stardict, StardictResult};
use std::{collections::BTreeMap, fs::File, io::Read};

type Word = Vec<u8>;
type Cords = Vec<u8>;
/// A struct parsing a StarDict .idx file.
///
/// An .idx is a sorted list of word entries,
/// all of which contains three consecutive fields:
///     - word_str ;; A string terminated by a null byte
///     - word_data_offset ;; word data's offset in .dict, and
///     - word_data_size ;; word data's total size in .dict
#[derive(Default)]
pub struct SDidx {
    pub idx: Vec<u8>,
    pub idx_content: BTreeMap<Word, Cords>,
}

impl SDidx {
    /// Finds all bytes until null + cords amount of bytes; returns
    /// total bytes read for iteration.
    fn find_until_cords(buf: &mut Vec<u8>, byte_arr: &[u8], cords: usize) -> usize {
        let byte_arr = if byte_arr[0] == 0 {
            &byte_arr[1..] // skips first byte if first byte is null
        } else {
            byte_arr
        };

        let mut byte_iter = byte_arr.iter();

        let i = byte_iter
            .position(|&byte| byte == 0)
            .expect("Null byte should exist as separator.");
        buf.extend_from_slice(&byte_arr[..=i + cords]);

        // return total bytes read
        i + cords + 1
    }
    /// Splits a byte arr at the first instance of a null byte.
    fn split_at_null(byte_arr: &[u8]) -> (&[u8], &[u8]) {
        let mut byte_split = byte_arr.split(|&byte| byte == 0);

        let left = byte_split.next().expect("Left side should be present.");
        let right = byte_split.next().expect("Right side should be present.");

        (left, right)
    }
    /// Creates a new .idx container.
    pub fn new(dict_prefix: &str, container: &Stardict) -> StardictResult<Self> {
        let idx_filename = format!("{dict_prefix}.idx");
        // idx_filename_gz = format!("{dict_prefix}.gz");
        // open_file(idx_filename, idx_filename_gz);

        let mut idx = Vec::new();
        let mut file = File::open(idx_filename)?;
        // Make sure that file size matches ifo
        assert_eq!(container.ifo.idxfilesize, file.metadata()?.len() as usize);
        file.read_to_end(&mut idx)?;

        let mut idx_content = BTreeMap::new();
        let idx_offset_bytes_size = container.ifo.idxoffsetbits / 8;
        let idx_cords_bytes_size = idx_offset_bytes_size + 4;

        // Parse data with byte functions
        let mut words: Vec<Vec<u8>> = vec![];
        let mut byte_counter = 0;
        while byte_counter < idx.len() {
            let mut byte_chunk = vec![];
            byte_counter +=
                Self::find_until_cords(&mut byte_chunk, &idx[byte_counter..], idx_cords_bytes_size);
            words.push(byte_chunk);
        }

        // Make sure wordcount matches
        assert_eq!(container.ifo.wordcount, words.len());

        // Parse each record
        for word in words.iter() {
            let (word, cords) = Self::split_at_null(word);
            idx_content.insert(word.to_vec(), cords.to_vec());
        }

        Ok(Self { idx, idx_content })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const FILEDIR: &str = "src/testdata/stardict-EnglishEtymology-2.4.2/EnglishEtymology";

    #[test]
    #[ignore = "Expensive test -- reads file twice."]
    fn idx_parser_test() {
        let dummy_dict = Stardict {
            in_memory: false,
            ifo: crate::stardict::ifo::SDifo::new(FILEDIR).expect("File should parse properly"),
            idx: SDidx::default(),
            dict: crate::stardict::SDdict {},
            syn: None,
            cache: BTreeMap::new(),
        };
        let idx = SDidx::new(FILEDIR, &dummy_dict);
        assert!(idx.is_ok());
    }
}
