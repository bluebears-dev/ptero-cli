//! # Description
//!
//! This encoder uses the trailing [ASCII whitespace](./constant.ASCII_ENCODING_WHITESPACE.html) to encode bits.
//! If the whitespace is present the bit 1 is encoded, otherwise 0.

use log::trace;

use crate::binary::Bit;

use super::{Encoder, EncoderResult, Result, ASCII_ENCODING_WHITESPACE};

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
    fn encode(
        &mut self,
        data: &mut dyn Iterator<Item = Bit>,
        line: &mut String,
    ) -> Result<EncoderResult> {
        Ok(match data.next() {
            Some(Bit(1)) => {
                trace!("Putting whitespace at the end of the line");
                line.push(ASCII_ENCODING_WHITESPACE);
                EncoderResult::Success
            }
            None => EncoderResult::NoDataLeft,
            _ => EncoderResult::Success,
        })
    }

    fn rate(&self) -> u32 {
        1
    }
}
