use std::process;

use log::error;

use crate::encoder::ASCII_ENCODING_WHITESPACE;
#[derive(Debug)]
pub struct CoverTextWordIterator {
    words: Vec<String>,
    word_index: usize,
}

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

    pub fn peek_word(&self) -> Option<String> {
        self.words
            .get(self.word_index)
            .map(|string| string.to_owned())
    }

    pub fn construct_line_by_pivot(&mut self, pivot: usize) -> Option<String> {
        let mut word = self.words.get(self.word_index)?;

        if word.len() > pivot {
            error!("Pivot is to small! Stuck at word of length {}.", word.len());
            process::exit(1);
        }
        let mut line = String::new();
        while line.len() + word.len() <= pivot {
            line.push_str(word);
            line.push(ASCII_ENCODING_WHITESPACE);
            self.word_index += 1;

            if let Some(next_word) = self.words.get(self.word_index) {
                word = next_word;
            } else {
                return Some(line);
            }
        }
        Some(line.trim_end().to_string())
    }
}

impl Iterator for CoverTextWordIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let word = self.peek_word()?;
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
