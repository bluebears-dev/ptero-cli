//! # Description
//!
//! This decoder seeks trailing whitespace [ASCII whitespace](../constant.ASCII_DECODING_WHITESPACE.html) in the line.
//! If the whitespace is present the bit 1 is decoded, otherwise 0.
use crate::binary::Bit;

use super::{Decoder, ASCII_DECODING_WHITESPACE};
use log::trace;

pub struct TrailingWhitespaceDecoder {}

impl Default for TrailingWhitespaceDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl TrailingWhitespaceDecoder {
    pub fn new() -> Self {
        TrailingWhitespaceDecoder {}
    }
}

impl Decoder for TrailingWhitespaceDecoder {
    fn decode(&self, line: &str) -> Vec<Bit> {
        let bit = if line.ends_with(ASCII_DECODING_WHITESPACE) {
            trace!("Found trailing whitespace");
            Bit(1)
        } else {
            Bit(0)
        };
        vec![bit]
    }
}
