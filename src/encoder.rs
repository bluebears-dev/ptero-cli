use std::{error::Error, fmt, result};

use crate::binary::Bit;

/// Contains complex encoders - composition of simpler ones.
pub mod complex {
    /// Encoder realizing extended-line steganography algorithm
    pub mod extended_line_encoder;
}
/// Encoder which adds extra word when encoding bit
pub mod line_extend_encoder;
/// Encoder which puts extra ASCII space when encoding bit
pub mod random_whitespace_encoder;
/// Encoder which puts trailing ASCII space when encoding bit
pub mod trailing_whitespace_encoder;

/// Whitespace used to encode bits
pub const ASCII_ENCODING_WHITESPACE: char = ' ';

/// Possible results of data encoding
#[derive(Debug, Clone)]
pub enum EncoderResult {
    Success,
    NoDataLeft,
}

/// Result type for returning `EncodingError`
pub type Result<T> = result::Result<T, EncodingError>;

/// Base trait for all data encoders
pub trait Encoder {
    /// Method which encodes bits provided by `data` iterator into provided `line` string.
    /// It may change the line in process e.g. add some additional characters.
    ///
    /// # Arguments
    ///
    /// * `data` - data iterator which return bit with each iteration
    /// * `line` - line string holder
    ///
    /// # Returns
    /// It returns whether the encoding was successful. See [EncoderResult](enum.EncoderResult.html) and [EncodingError](struct.EncodingError.html).
    ///
    fn encode(
        &mut self,
        data: &mut dyn Iterator<Item = Bit>,
        line: &mut String,
    ) -> Result<EncoderResult>;
}
/// Enum for data encoding errors types
#[derive(Debug, Clone)]
pub enum EncodingErrorKind {
    CapacityTooLow,
    NoWordsLeft,
}

/// Represents every encoding error 
#[derive(Debug, Clone)]
pub struct EncodingError {
    kind: EncodingErrorKind,
}

impl EncodingError {
    /// Facade for creating [EncodingErrorKind::CapacityTooLow](enum.EncodingErrorKind.html)
    pub fn capacity_error() -> Self {
        EncodingError {
            kind: EncodingErrorKind::CapacityTooLow,
        }
    }
    /// Facade for creating [EncodingErrorKind::NoWordsLeft](enum.EncodingErrorKind.html)
    pub fn no_words_error() -> Self {
        EncodingError {
            kind: EncodingErrorKind::NoWordsLeft,
        }
    }
}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            EncodingErrorKind::CapacityTooLow => write!(f, "Exceeded cover text capacity"),
            EncodingErrorKind::NoWordsLeft => write!(
                f,
                "No extra words found in cover text when tried to encode a bit"
            ),
        }
    }
}

impl Error for EncodingError {}