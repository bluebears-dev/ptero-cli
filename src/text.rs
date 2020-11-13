use crate::encoder::ASCII_ENCODING_WHITESPACE;

pub trait WordIterator {
    fn next_word(&mut self) -> Option<String>;
}

#[derive(Debug)]
pub struct LineByPivotIterator {
    words: Vec<String>,
    index: usize,
    pivot: usize,
}

impl LineByPivotIterator {
    pub fn new(text: &str, pivot: usize) -> Self {
        LineByPivotIterator {
            words: text
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|v| v.to_string())
                .collect(),
            pivot,
            index: 0,
        }
    }

    pub fn peek_word(&self) -> Option<String> {
        self.words.get(self.index).map(|string| string.to_owned())
    }
}

impl WordIterator for LineByPivotIterator {
    fn next_word(&mut self) -> Option<String> {
        let next_word = self.peek_word();
        if next_word.is_some() {
            self.index += 1;
        }
        next_word
    }
}

impl Iterator for LineByPivotIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut word = self.words.get(self.index)?;

        let mut line = String::new();
        while line.len() + word.len() <= self.pivot {
            line.push_str(word);
            line.push(ASCII_ENCODING_WHITESPACE);
            self.index += 1;
            word = self.words.get(self.index)?;
        }
        Some(line.trim_end().to_string())
    }
}
