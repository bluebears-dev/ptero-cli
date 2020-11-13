//! # Description
//!
//! This encoder implements the Extender Line steganography algorithm. It consist of three
//! simpler encoders:
//! * [RandomWhitespaceEncoder](../../random_whitespace_encoder/struct.RandomWhitespaceEncoder.html),
//! * [LineExtendEncoder](../../line_extend_encoder/struct.LineExtendEncoder.html),
//! * [TrailingWhitespaceEncoder](../../trailing_whitespace_encoder/struct.TrailingWhitespaceEncoder.html).
//!
//! For more info read docs on each one of the above encoders.
use std::cell::RefMut;

use crate::{
    binary::Bit,
    encoder::{
        line_extend_encoder, random_whitespace_encoder, trailing_whitespace_encoder, Encoder,
        EncoderResult, Result,
    },
    text::WordIterator,
};

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
    fn encode(
        &mut self,
        data: &mut dyn Iterator<Item = Bit>,
        line: &mut String,
    ) -> Result<EncoderResult> {
        let mut is_data_still_available = EncoderResult::Success;
        for encoder in &mut self.encoders {
            if let EncoderResult::NoDataLeft = encoder.encode(data, line)? {
                is_data_still_available = EncoderResult::NoDataLeft;
                break;
            }
        }
        Ok(is_data_still_available)
    }
}
