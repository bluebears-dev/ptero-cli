//! # Description
//!
//! This encoder puts [ASCII whitespace](../constant.ASCII_ENCODING_WHITESPACE.html) between randomly selected two words.
//! If the duplicate whitespace is present the bit 1 is encoded, otherwise 0.
use std::error::Error;

use crate::{
    binary::Bit,
    context::{Context, ContextError},
    decoder::{Decoder, ASCII_DECODING_WHITESPACE},
    encoder::{Encoder, EncoderResult, ASCII_ENCODING_WHITESPACE},
};

use log::trace;
use rand::{thread_rng, Rng};

use super::Method;

pub struct RandomWhitespaceMethod;

impl Default for RandomWhitespaceMethod {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomWhitespaceMethod {
    pub fn new() -> Self {
        RandomWhitespaceMethod {}
    }
}

impl Encoder for RandomWhitespaceMethod {
    fn encode(
        &mut self,
        context: &mut Context,
        data: &mut dyn Iterator<Item = Bit>,
        line: &mut String,
    ) -> Result<EncoderResult, Box<dyn Error>> {
        Ok(match data.next() {
            Some(Bit(1)) => {
                let mut rng = thread_rng();
                let position_determinant = rng.gen_range(0, &line.len());
                let mut position = line.find(' ').unwrap_or_else(|| line.len());
                for (index, character) in line.char_indices() {
                    if index > position_determinant {
                        break;
                    }
                    if character.is_whitespace() {
                        position = index;
                    }
                }
                trace!("Putting space at position {}", position);
                line.insert_str(position, &String::from(ASCII_ENCODING_WHITESPACE));
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

impl Decoder for RandomWhitespaceMethod {
    fn decode(&self, context: &Context, line: &str) -> Result<Vec<Bit>, ContextError> {
        let mut seen_whitespace = false;
        for character in line.chars() {
            let is_whitespace = character == ASCII_DECODING_WHITESPACE;
            if seen_whitespace && is_whitespace {
                trace!(
                    "Found two consecutive '{}' between words",
                    ASCII_DECODING_WHITESPACE
                );
                return Ok(vec![Bit(1)]);
            }
            seen_whitespace = is_whitespace;
        }
        Ok(vec![Bit(0)])
    }
}

impl Method for RandomWhitespaceMethod {}