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

use log::trace;
use regex::Regex;

use crate::{binary::Bit, context::{Context, ContextError, PivotDecoderContext, PivotEncoderContext}, decoder::Decoder, encoder::{Encoder, EncoderResult, EncodingError, ASCII_ENCODING_WHITESPACE}};

use super::Method;

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

impl Encoder<PivotEncoderContext> for LineExtendMethod {
    fn encode(
        &mut self,
        context: &mut PivotEncoderContext,
        data: &mut dyn Iterator<Item = Bit>,
    ) -> Result<EncoderResult, Box<dyn Error>> {
        Ok(match data.next() {
            Some(Bit(1)) => {
                let word = context
                    .get_cover_text_iter_mut()
                    .next()
                    .ok_or_else(EncodingError::no_words_error)?;
                trace!("Extending line with '{}'", &word);
                let text = context.get_current_text_mut()?;
                text.push(ASCII_ENCODING_WHITESPACE);
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

impl Decoder<PivotDecoderContext> for LineExtendMethod {
    fn decode(&self, context: &PivotDecoderContext) -> Result<Vec<Bit>, ContextError> {
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

impl Method<PivotEncoderContext, PivotDecoderContext> for LineExtendMethod {}
