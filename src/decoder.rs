use crate::{
    binary::Bit,
    context::{Context, ContextError},
};

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
    fn decode(&self, context: &Context, line: &str) -> Result<Vec<Bit>, ContextError>;
}
