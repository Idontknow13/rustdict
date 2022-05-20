//! A container of the .dict parser for StarDict.
//!

use super::{Stardict, StardictResult};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

pub struct SDdict<'dict> {
    pub file: File,
    pub container: &'dict Stardict<'dict>,
    // pub in_memory: bool,
}

impl<'dict> SDdict<'dict> {
    /// Creates a new instance of SDdict.
    pub fn new(dict_prefix: &str, container: &'dict Stardict) -> StardictResult<Self> {
        let filename = format!("{dict_prefix}.dict");
        Ok(Self {
            file: File::open(filename)?,
            container,
        })
    }

    /// Grabs the specified word from inside the dict
    /// and returns a tuple containing a copy of Self
    /// and the dictionary entry of the word.
    pub fn get(mut self, word: &str) -> StardictResult<(Self, String)> {
        let cords = &self.container.idx[word];

        self.file.seek(SeekFrom::Start(cords.offset as u64))?;

        let mut buf = vec![];
        let mut handler = self.file.take(cords.data_size as u64);
        let _ = handler.read(&mut buf)?;

        Ok((
            Self {
                file: handler.into_inner(),
                container: self.container,
            },
            String::from_utf8(buf)?,
        ))
    }
}

impl Default for SDdict<'_> {
    fn default() -> Self {
        // TODO
        todo!()
    }
}
