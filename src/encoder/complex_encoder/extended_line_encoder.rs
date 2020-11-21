//! # Description
//!
//! This encoder implements the Extender Line steganography algorithm. It consist of three
//! simpler encoders:
//! * [RandomWhitespaceEncoder](../../random_whitespace_encoder/struct.RandomWhitespaceEncoder.html),
//! * [LineExtendEncoder](../../line_extend_encoder/struct.LineExtendEncoder.html),
//! * [TrailingWhitespaceEncoder](../../trailing_whitespace_encoder/struct.TrailingWhitespaceEncoder.html).
//!
//! For more info read docs on each one of the above encoders.
//!
//! See also [ComplexEncoder](../struct.ComplexEncoder.html) for more info about how to use this encoder.
use std::cell::RefMut;

use crate::{
    encoder::{line_extend_encoder, random_whitespace_encoder, trailing_whitespace_encoder},
    text::WordIterator,
};

use super::{ComplexEncoder};

pub struct ExtendedLineEncoderFactory;

impl ExtendedLineEncoderFactory {
    pub fn build<T: WordIterator>(word_iter: RefMut<T>) -> ComplexEncoder {
        ComplexEncoder::new(vec![
            Box::new(random_whitespace_encoder::RandomWhitespaceEncoder::new()),
            Box::new(line_extend_encoder::LineExtendEncoder::new(word_iter)),
            Box::new(trailing_whitespace_encoder::TrailingWhitespaceEncoder::new()),
        ])
    }
}
