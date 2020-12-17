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

impl<T> Encoder<T> for RandomWhitespaceMethod
where
    T: Context,
{
    fn encode(
        &mut self,
        context: &mut T,
        data: &mut dyn Iterator<Item = Bit>,
    ) -> Result<EncoderResult, Box<dyn Error>> {
        Ok(match data.next() {
            Some(Bit(1)) => {
                let mut rng = thread_rng();
                let text = context.get_current_text_mut()?;
                let position_determinant = rng.gen_range(0, &text.len());
                let mut position = text.find(' ').unwrap_or_else(|| text.len());
                for (index, character) in text.char_indices() {
                    if index > position_determinant {
                        break;
                    }
                    if character.is_whitespace() {
                        position = index;
                    }
                }
                trace!("Putting space at position {}", position);
                text.insert_str(position, &String::from(ASCII_ENCODING_WHITESPACE));
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

impl<D> Decoder<D> for RandomWhitespaceMethod
where
    D: Context,
{
    fn decode(&self, context: &D) -> Result<Vec<Bit>, ContextError> {
        let mut seen_whitespace = false;
        for character in context.get_current_text()?.chars() {
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

impl<E, D> Method<E, D> for RandomWhitespaceMethod
where
    E: Context,
    D: Context,
{
}
