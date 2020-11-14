//! # Description
//! 
//! This decoder tries to find [ASCII whitespace](../constant.ASCII_DECODING_WHITESPACE.html) between any two words in the line.
//! If the duplicate whitespace is present the bit 1 is decoded, otherwise 0.
use crate::binary::Bit;

use super::{ASCII_DECODING_WHITESPACE, Decoder};
use log::trace;

pub struct RandomWhitespaceDecoder {}

impl Default for RandomWhitespaceDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomWhitespaceDecoder {
    pub fn new() -> Self {
        RandomWhitespaceDecoder {}
    }
}

impl Decoder for RandomWhitespaceDecoder {
    fn decode(&self, line: &str) -> Vec<Bit> {
        let mut seen_whitespace = false;
        for character in line.chars() {
            let is_whitespace = character == ASCII_DECODING_WHITESPACE;
            if seen_whitespace && is_whitespace {
                trace!("Found two consecutive '{}' between words", ASCII_DECODING_WHITESPACE);
                return vec![Bit(1)];
            }
            seen_whitespace = is_whitespace;
        }
        vec![Bit(0)]
    }
}
