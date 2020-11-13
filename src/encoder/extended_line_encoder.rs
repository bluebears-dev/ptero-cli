use std::cell::RefMut;

use crate::{binary::Bit, text::WordIterator};

use super::{Encoder, line_extend_encoder, random_whitespace_encoder, trailing_whitespace_encoder};

pub struct ExtendedLineEncoder<'a> {
    encoders: Vec<Box<dyn Encoder + 'a>>,
}

impl<'a> ExtendedLineEncoder<'a> {
    pub fn new<T: WordIterator>(word_iter: RefMut<'a, T>) -> Self {
        ExtendedLineEncoder {
            encoders: vec![
                Box::new(random_whitespace_encoder::RandomWhitespaceEncoder::new()),
                Box::new(line_extend_encoder::LineExtendEncoder::new(word_iter)),
                Box::new(trailing_whitespace_encoder::TrailingWhitespaceEncoder::new()),
            ],
        }
    }
}

impl<'a> Encoder for ExtendedLineEncoder<'a> {
    fn encode(&mut self, data: &mut dyn Iterator<Item = Bit>, line: &mut String) -> bool {
        let mut is_data_still_available = true;
        for encoder in &mut self.encoders {
            if !encoder.encode(data, line) {
                is_data_still_available = false;
                break;
            }
        }
        is_data_still_available
    }
}
