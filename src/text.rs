use std::{io::{self, BufRead}, str::SplitWhitespace};

/// Word iterator.
/// It is a proxy for the [SplitWhitespace](https://doc.rust-lang.org/std/str/struct.SplitWhitespace.html) iterator enhanced
/// by ability to read from [BufRead](https://doc.rust-lang.org/std/io/trait.BufRead.html).
#[derive(Debug)]
pub struct WordIterator {
    raw_text: String,
}

impl WordIterator {
    /// Creates new `WordIterator` from given string slice.
    /// It converts it to the owned `String` internally.
    pub fn new(text: &str) -> WordIterator {
        WordIterator {
            raw_text: text.to_owned(),
        }
    }

    /// Proxy method for returning [SplitWhitespace](https://doc.rust-lang.org/std/str/struct.SplitWhitespace.html) iterator.
    pub fn iter(&self) -> SplitWhitespace {
        self.raw_text.split_whitespace()
    }

    /// Reads whole content from the specified [BufRead](https://doc.rust-lang.org/std/io/trait.BufRead.html)
    /// and returns it as the instance of the `WordIterator`.
    pub fn from_reader<R: BufRead>(reader: &mut R) -> io::Result<WordIterator> {
        let mut str = String::new();
        reader.read_to_string(&mut str)?;
        Ok(WordIterator::new(&str))
    }
}