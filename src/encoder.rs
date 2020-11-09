use std::cell::RefMut;

use crate::binary::Bit;

pub mod line_extend_encoder;
pub mod random_whitespace_encoder;
pub mod trailing_whitespace_encoder;

const ASCII_ENCODING_WHITESPACE: char = ' ';

pub trait Encoder {
    fn encode(&mut self, data: &mut dyn Iterator<Item = Bit>, line: &mut String) -> bool;
}

pub struct ExtendedLineEncoders<'a> {
    encoders: Vec<Box<dyn Encoder + 'a>>,
}

impl<'a> ExtendedLineEncoders<'a> {
    pub fn new(word_iter: RefMut<'a, dyn Iterator<Item = &'a str>>) -> Self {
        ExtendedLineEncoders {
            encoders: vec![
                Box::new(random_whitespace_encoder::RandomWhitespaceEncoder::new()),
                Box::new(line_extend_encoder::LineExtendEncoder::new(word_iter)),
                Box::new(trailing_whitespace_encoder::TrailingWhitespaceEncoder::new()),
            ],
        }  
    }
}
 
impl<'a> Encoder for ExtendedLineEncoders<'a> {
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
