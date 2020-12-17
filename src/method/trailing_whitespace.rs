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

impl<E> Encoder<E> for TrailingWhitespaceMethod
where
    E: Context,
{
    fn encode(
        &mut self,
        context: &mut E,
        data: &mut dyn Iterator<Item = Bit>,
    ) -> Result<EncoderResult, Box<dyn Error>> {
        Ok(match data.next() {
            Some(Bit(1)) => {
                trace!("Putting whitespace at the end of the line");
                context
                    .get_current_text_mut()?
                    .push(ASCII_ENCODING_WHITESPACE);
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

impl<D> Decoder<D> for TrailingWhitespaceMethod
where
    D: Context,
{
    fn decode(&self, context: &D) -> Result<Vec<Bit>, ContextError> {
        let bit = if context
            .get_current_text()?
            .ends_with(ASCII_DECODING_WHITESPACE)
        {
            trace!("Found trailing whitespace");
            Bit(1)
        } else {
            Bit(0)
        };
        Ok(vec![bit])
    }
}

impl<E, D> Method<E, D> for TrailingWhitespaceMethod
where
    E: Context,
    D: Context,
{
}
