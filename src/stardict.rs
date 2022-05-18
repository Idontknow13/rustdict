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
struct SDifo {
    // Required fields
    version: String,
    bookname: String,
    wordcount: u32,
    idxfilesize: usize,
    sametypesequence: String,
    // Optional fields
    synwordcount: Option<usize>,
    idxoffsetbits: Option<usize>,
    author: Option<String>,
    email: Option<String>,
    website: Option<String>,
    desc: Option<String>,
    date: Option<String>,
}

impl<'a> SDifo {
    pub fn new(dict_prefix: &'a str) -> Self {
        let mut ifo_object = Self::default();
        let filename = format!("{dict_prefix}.ifo");

        ifo_object
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
