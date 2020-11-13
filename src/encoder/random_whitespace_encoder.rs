//! # Description
//! 
//! This encoder puts [ASCII whitespace](../constant.ASCII_ENCODING_WHITESPACE.html) between randomly selected two words.
//! If the duplicate whitespace is present the bit 1 is encoded, otherwise 0.
use crate::binary::Bit;

use super::{ASCII_ENCODING_WHITESPACE, Encoder, EncoderResult, Result};
use log::trace;
use rand::{thread_rng, Rng};

pub struct RandomWhitespaceEncoder {}

impl Default for RandomWhitespaceEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomWhitespaceEncoder {
    pub fn new() -> Self {
        RandomWhitespaceEncoder {}
    }
}

impl Encoder for RandomWhitespaceEncoder {
    fn encode(&mut self, data: &mut dyn Iterator<Item = Bit>, line: &mut String) -> Result<EncoderResult> {
        Ok(match data.next() {
            Some(Bit(1)) => {
                let mut rng = thread_rng();
                let position_determinant = rng.gen_range(0, &line.len());
                let mut position = line.find(' ').unwrap_or(line.len() - 1);
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
}
