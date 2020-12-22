/// Cover text iterator which traverses the text word by word.
/// It also enables the user to peek the word without forwarding the iterator.
#[derive(Debug)]
pub struct CoverTextWordIterator {
    words: Vec<String>,
    word_index: usize,
}

/// Cover text iterator which traverses the text line by line.
#[derive(Debug)]
pub struct CoverTextLineIterator {
    lines: Vec<String>,
    line_index: usize,
}

impl CoverTextWordIterator {
    pub fn new(cover_text: &str) -> Self {
        CoverTextWordIterator {
            words: cover_text
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|v| v.to_string())
                .collect(),
            word_index: 0,
        }
    }

    pub fn peek(&self) -> Option<String> {
        self.words
            .get(self.word_index)
            .map(|string| string.to_owned())
    }
}

impl Iterator for CoverTextWordIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let word = self.peek()?;
        self.word_index += 1;
        Some(word)
    }
}

impl CoverTextLineIterator {
    pub fn new(cover_text: &str) -> Self {
        CoverTextLineIterator {
            lines: cover_text
                .lines()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            line_index: 0,
        }
    }
}

impl Iterator for CoverTextLineIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.get(self.line_index).map(|x| x.to_owned())?;
        self.line_index += 1;
        Some(line)
    }
}
