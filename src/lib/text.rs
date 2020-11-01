use std::{io::{self, BufRead}, str::SplitWhitespace};

#[derive(Debug)]
pub struct WordIterator {
    raw_text: String,
}

impl WordIterator {
    pub fn new(text: &str) -> WordIterator {
        WordIterator {
            raw_text: text.to_owned(),
        }
    }

    pub fn iter(&self) -> SplitWhitespace {
        self.raw_text.split_whitespace()
    }

    pub fn from_reader<R: BufRead>(reader: &mut R) -> io::Result<WordIterator> {
        let mut str = String::new();
        reader.read_to_string(&mut str)?;
        Ok(WordIterator::new(&str))
    }
}