use std::cell::RefMut;

use log::trace;

use crate::{binary::Bit, text::WordIterator};

use super::{Encoder, ASCII_ENCODING_WHITESPACE};

pub struct LineExtendEncoder<'a, T>
where
    T: WordIterator,
{
    word_iter: RefMut<'a, T>,
}

impl<'a, T> LineExtendEncoder<'a, T>
where
    T: WordIterator,
{
    pub fn new(word_iter: RefMut<'a, T>) -> Self {
        LineExtendEncoder { word_iter }
    }
}

impl<'a, T> Encoder for LineExtendEncoder<'a, T>
where
    T: WordIterator,
{
    fn encode(&mut self, data: &mut dyn Iterator<Item = Bit>, line: &mut String) -> bool {
        match data.next() {
            Some(Bit(1)) => {
                let word = self
                    .word_iter
                    .next_word()
                    .expect("No words left to extend a line");
                trace!("Extending line");
                line.push(ASCII_ENCODING_WHITESPACE);
                line.push_str(word.as_str());
                true
            }
            None => false,
            _ => true,
        }
    }
}
