use crate::binary::Bit;

/// Decoder which decodes bits encoded by [RandomWhitespaceEncoder](../encoder/random_whitespace_encoder/struct.RandomWhitespaceEncoder.html).
pub mod random_whitespace_decoder;

/// Decoder which decodes bits encoded by [TrailingWhitespaceEncoder](../encoder/trailing_whitespace_encoder/struct.TrailingWhitespaceEncoder.html).
pub mod trailing_whitespace_decoder;

/// Decoder which decodes bits encoded by [LineExtendEncoder](../encoder/line_extend_encoder/struct.LineExtendEncoder.html).
pub mod line_extend_decoder;

/// Complex decoders - composed of smaller and simpler decoders
pub mod complex {
    /// Decoder which decodes bits encoded by [ExtendedLineEncoder](../../encoder/complex/extended_line_encoder/struct.ExtendedLineEncoder.html).
    pub mod extended_line_decoder;
}

/// Whitespace character used for decoding
pub const ASCII_DECODING_WHITESPACE: char = ' ';

/// Base trait for all data decoders
pub trait Decoder {
    /// Method which decodes bits possibly encoded in `line` string.
    ///
    /// # Arguments
    ///
    /// * `line` - line containing part of the stegotext
    /// * `pivot` - pivot used to encode data
    ///
    /// # Returns
    /// It returns data decoded from the provided `line`.
    ///
    fn decode(&self, line: &str) -> Vec<Bit>;
}
