use crate::{
    binary::Bit,
    context::{ContextError},
};

// TODO: Provide DecoderError and mapping from ContextError
/// Base trait for all data decoders.
/// The generic type should contain data need by the decoder implementation.
pub trait Decoder<D> {
    /// Decodes bits from the cover text.
    /// The access to the cover text is bound by the [Context].
    ///
    /// # Arguments
    ///
    /// * `context` - context of the steganography method, can contain various needed info like pivot etc.
    ///
    /// # Returns
    /// It returns `Result`, either decoded data as as vector of [Bits] or error.
    ///
    /// [Context]: crate::context::Context
    /// [Bit]: crate::binary::Bit
    fn decode(&self, context: &D) -> Result<Vec<Bit>, ContextError>;
}
