use crate::binary::Bit;

use super::{ASCII_ENCODING_WHITESPACE, Encoder};

pub struct TrailingWhitespaceEncoder {}

impl Default for TrailingWhitespaceEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl TrailingWhitespaceEncoder {
    pub fn new() -> Self {
        TrailingWhitespaceEncoder {}
    }
}

impl Encoder for TrailingWhitespaceEncoder {
    fn encode(&mut self, data: &mut dyn Iterator<Item = Bit>, line: &mut String) -> bool {
        match data.next() {
            Some(Bit(1)) => {
                line.push( ASCII_ENCODING_WHITESPACE);
                true
            }
            None => false,
            _ => true,
        }
    }
}
