use std::{borrow::BorrowMut, cell::RefMut};

use crate::binary::Bit;

use super::{ASCII_ENCODING_WHITESPACE, Encoder};

pub struct LineExtendEncoder<'a> {
    word_iter: RefMut<'a, dyn Iterator<Item = &'a str>>,
}

impl<'a> LineExtendEncoder<'a> {
    pub fn new(word_iter: RefMut<'a, dyn Iterator<Item = &'a str>>) -> Self {
        LineExtendEncoder { word_iter }
    }
}

impl<'a> Encoder for LineExtendEncoder<'a> {
    fn encode(&mut self, data: &mut dyn Iterator<Item = Bit>, line: &mut String) -> bool {
        match data.next() {
            Some(Bit(1)) => {
                let word = self
                    .word_iter
                    .borrow_mut()
                    .next()
                    .expect("No words left to extend a line");
                line.push(ASCII_ENCODING_WHITESPACE);
                line.push_str(word);
                true
            }
            None => false,
            _ => true,
        }
    }
}
