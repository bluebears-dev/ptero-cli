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
use std::error::Error;

use log::{trace};
use regex::Regex;

use crate::{binary::Bit, context::{Context, ContextError, PivotByLineContext, PivotByRawLineContext}, decoder::Decoder, encoder::{Encoder, EncoderResult, EncodingError}};

use super::Method;

/// Character used as the word delimiter.
pub const ASCII_DELIMITER: char = ' ';

/// Unit structure representing the line extension method.
///
/// Accepts only following contexts: [PivotByLineContext](crate::context::PivotByLineContext) for [Encoder](crate::encoder::Encoder) trait and
// [PivotByRawLineContext](crate::context::PivotByRawLineContext) for [Decoder](crate::decoder::Decoder) trait.
// *Decoder needs to consume raw lines to be able to decode information using pivot.*
pub struct LineExtendMethod;

impl LineExtendMethod {
    pub fn new() -> Self {
        LineExtendMethod {}
    }
}

impl Default for LineExtendMethod {
    fn default() -> Self {
        Self::new()
    }
}

impl Encoder<PivotByLineContext> for LineExtendMethod {
    fn partial_encode(
        &self,
        context: &mut PivotByLineContext,
        data: &mut dyn Iterator<Item = Bit>,
    ) -> Result<EncoderResult, Box<dyn Error>> {
        Ok(match data.next() {
            Some(Bit(1)) => {
                // TODO: Provide mapping for ContextError -> EncodingError
                let word = context
                    .next_word()
                    .ok_or_else(EncodingError::no_words_error)?;
                trace!("Extending line with '{}'", &word);
                let text = context.get_current_text_mut()?;
                text.push(ASCII_DELIMITER);
                text.push_str(word.as_str());
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

impl Decoder<PivotByRawLineContext> for LineExtendMethod {
    fn partial_decode(&self, context: &PivotByRawLineContext) -> Result<Vec<Bit>, ContextError> {
        let pattern = Regex::new(r"\s+").unwrap();
        let cleaned_line = pattern.replace_all(context.get_current_text()?, " ");
        let bit = if cleaned_line.trim_end().len() > context.get_pivot() {
            trace!("Line is extended over the {} length", context.get_pivot());
            Bit(1)
        } else {
            Bit(0)
        };
        Ok(vec![bit])
    }
}

impl Method<PivotByLineContext, PivotByRawLineContext> for LineExtendMethod {}
