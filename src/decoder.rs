use std::{convert::TryFrom, error::Error};

use log::{debug, trace};

use crate::{binary::{Bit, BitVec}, context::{Context, ContextError}};

// TODO: Provide DecoderError and mapping from ContextError
/// Base trait for all data decoders.
/// The generic type should contain data need by the decoder implementation.
pub trait Decoder<D> where D: Context {
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
    fn partial_decode(&self, context: &D) -> Result<Vec<Bit>, ContextError>;

    fn decode(&self, context: &mut D) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut secret = Vec::default();
        debug!("Decoding secret from the text");
        while let Ok(line) = context.load_text() {
            trace!("Line {}", line);
            let mut data = self.partial_decode(&context)?;
            secret.append(&mut data);
        }
        debug!("Padding bits to byte size boundary");
        while &secret.len() % 8 != 0 {
            secret.push(Bit(0));
        }
    
        debug!("Converting bits to bytes");
        let bit_vec: BitVec = secret.into();
        let bytes: Vec<u8> = TryFrom::try_from(bit_vec)?;
        Ok(bytes)
    }
}
