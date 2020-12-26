//! # Description
//!
//! This method uses the trailing [ASCII_WHITESPACE] to encode bits.
//! If the whitespace is present the bit 1 is encoded, otherwise 0.
//!
//! This method provides both encoding and decoding algorithm.

use std::error::Error;

use log::{trace};

use crate::{binary::{Bit}, context::{Context, ContextError}, decoder::Decoder, encoder::{Encoder, EncoderResult}};

/// Character used as the trailing whitespace in the method.
pub const ASCII_WHITESPACE: char = ' ';

use super::Method;
// Unit structure used to define the method.
// Implements both [Encoder](crate::encoder::Encode) and [Decoder](crate::decoder::Decoder) traits.
// Accepts any [Context](crate::context::Context).
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
    fn partial_encode(
        &self,
        context: &mut E,
        data: &mut dyn Iterator<Item = Bit>,
    ) -> Result<EncoderResult, Box<dyn Error>> {
        Ok(match data.next() {
            Some(Bit(1)) => {
                trace!("Putting whitespace at the end of the line");
                context.get_current_text_mut()?.push(ASCII_WHITESPACE);
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
    fn partial_decode(&self, context: &D) -> Result<Vec<Bit>, ContextError> {
        let bit = if context.get_current_text()?.ends_with(ASCII_WHITESPACE) {
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
