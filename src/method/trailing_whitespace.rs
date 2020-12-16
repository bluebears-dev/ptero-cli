//! # Description
//!
//! This encoder uses the trailing [ASCII whitespace](./constant.ASCII_ENCODING_WHITESPACE.html) to encode bits.
//! If the whitespace is present the bit 1 is encoded, otherwise 0.

use std::error::Error;

use log::trace;

use crate::{
    binary::Bit,
    context::{Context, ContextError},
    decoder::{Decoder, ASCII_DECODING_WHITESPACE},
    encoder::{Encoder, EncoderResult, ASCII_ENCODING_WHITESPACE},
};

use super::Method;

pub struct TrailingWhitespaceMethod;

impl Default for TrailingWhitespaceMethod {
    fn default() -> Self {
        Self::new()
    }
}

impl TrailingWhitespaceMethod {
    pub fn new() -> Self {
        TrailingWhitespaceMethod {}
    }
}

impl Encoder for TrailingWhitespaceMethod {
    fn encode(
        &mut self,
        context: &mut Context,
        data: &mut dyn Iterator<Item = Bit>,
        line: &mut String,
    ) -> Result<EncoderResult, Box<dyn Error>> {
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

impl Decoder for TrailingWhitespaceMethod {
    fn decode(&self, context: &Context, line: &str) -> Result<Vec<Bit>, ContextError> {
        let bit = if line.ends_with(ASCII_DECODING_WHITESPACE) {
            trace!("Found trailing whitespace");
            Bit(1)
        } else {
            Bit(0)
        };
        Ok(vec![bit])
    }
}

impl Method for TrailingWhitespaceMethod {}
