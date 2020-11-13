//! # Description
//! 
//! This encoder extends line with extra word (to be greater than pivot) to encode bit.
//! If the line length is greater than the pivot the bit 1 is encoded, otherwise 0.
//!  
//! For more info about pivot see [LineByPivotIterator](../../text/struct.LineByPivotIterator.html).
//! 
//! # Behavior
//! 
//! This encoder can return [EncodingError](../struct.EncodingError.html) when no extra words are found
//! and the bit 1 occurs.
use std::cell::RefMut;

use log::trace;

use crate::{binary::Bit, text::WordIterator};

use super::{Encoder, EncoderResult, EncodingError, Result, ASCII_ENCODING_WHITESPACE};

pub struct LineExtendEncoder<'a, T>
where
    T: WordIterator,
{
    word_iter: RefMut<'a, T>,
}

impl<'a, T> LineExtendEncoder<'a, T>
where
    T: WordIterator,
{
    pub fn new(word_iter: RefMut<'a, T>) -> Self {
        LineExtendEncoder { word_iter }
    }
}

impl<'a, T> Encoder for LineExtendEncoder<'a, T>
where
    T: WordIterator,
{
    fn encode(
        &mut self,
        data: &mut dyn Iterator<Item = Bit>,
        line: &mut String,
    ) -> Result<EncoderResult> {
        Ok(match data.next() {
            Some(Bit(1)) => {
                let word = self
                    .word_iter
                    .next_word()
                    .ok_or_else(EncodingError::no_words_error)?;
                trace!("Extending line");
                line.push(ASCII_ENCODING_WHITESPACE);
                line.push_str(word.as_str());
                EncoderResult::Success
            }
            None => EncoderResult::NoDataLeft,
            _ => EncoderResult::Success,
        })
    }
}
