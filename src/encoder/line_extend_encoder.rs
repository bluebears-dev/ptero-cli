use std::{borrow::BorrowMut, cell::RefMut};

use crate::binary::Bit;

use super::Encoder;

pub struct LineExtendEncoder<'a, T> {
    next: T,
    word_iter: RefMut<'a, dyn Iterator<Item = &'a str>>,
}

impl<'a, T> LineExtendEncoder<'a, T>
where
    T: Encoder,
{
    pub fn new(encoder: T, word_iter: RefMut<'a, dyn Iterator<Item = &'a str>>) -> Self {
        LineExtendEncoder {
            next: encoder,
            word_iter,
        }
    }
}

impl<'a, T> Encoder for LineExtendEncoder<'a, T>
where
    T: Encoder,
{
    fn encode(&mut self, data: &mut dyn Iterator<Item = Bit>, line: &mut String) {
        match data.next() {
            Some(Bit(1)) => {
                println!("Extending line: {}", &line);
                let word = self
                    .word_iter
                    .borrow_mut()
                    .next()
                    .expect("No words left to extend a line");
                line.push(' ');
                line.push_str(word);
                Encoder::encode(&mut self.next, data, line);
            }
            Some(Bit(0)) => Encoder::encode(&mut self.next, data, line),
            _ => (),
        }
    }
}
