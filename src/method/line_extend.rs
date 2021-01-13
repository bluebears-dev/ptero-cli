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

use crate::{
    binary::Bit,
    context::{Context, ContextError, PivotByLineContext, PivotByRawLineContext},
    decoder::Decoder,
    encoder::{Capacity, Encoder, EncoderResult, EncodingError},
};

use super::Method;

/// Character used as the word delimiter.
pub const ASCII_DELIMITER: char = ' ';

/// Set of possible line endings, the set is different from one used [crate::method::trailing_unicode] as it
/// includes all possible characters, not the curated set used in encoding.
pub const POSSIBLE_LINE_ENDINGS_SET: [char; 32] = [
    '\u{0020}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}',
    '\u{2007}', '\u{2009}', '\u{200A}', '\u{200B}', '\u{200C}', '\u{200D}', '\u{200E}', '\u{2028}',
    '\u{202A}', '\u{202C}', '\u{202D}', '\u{202F}', '\u{205F}', '\u{2060}', '\u{2061}', '\u{2062}',
    '\u{2063}', '\u{2064}', '\u{2066}', '\u{2068}', '\u{2069}', '\u{3000}', '\u{FEFF}', '\u{00A0}',
];

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

impl Capacity for LineExtendMethod {
    fn bitrate(&self) -> usize {
        1
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
            _ => {
                trace!("Leaving line as-is");
                EncoderResult::Success
            }
        })
    }
}

impl Decoder<PivotByRawLineContext> for LineExtendMethod {
    fn partial_decode(&self, context: &PivotByRawLineContext) -> Result<Vec<Bit>, ContextError> {
        let repeated_whitespace_pattern = Regex::new(r"\s+").unwrap();
        let cleaned_line = repeated_whitespace_pattern
            .replace_all(context.get_current_text()?, " ");
        let bit = if cleaned_line.trim_end_matches(&POSSIBLE_LINE_ENDINGS_SET[..]).len() > context.get_pivot() {
            trace!("Line is extended over the {} length", context.get_pivot());
            Bit(1)
        } else {
            trace!("Line not extended");
            Bit(0)
        };
        Ok(vec![bit])
    }
}

impl Method<PivotByLineContext, PivotByRawLineContext> for LineExtendMethod {
    fn method_name(&self) -> String {
        "LineExtendMethod".to_string()
    }
}
