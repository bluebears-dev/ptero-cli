//! # Description
//!
//! This method encodes n-bits using available set of unicode whitespace. Larger sets provide better capacity
//! but might not work for every channel - it depends which characters get sanitized or are actually used.
//!
//! The whitespace are added at the end of the line as some of them are actually visible and might
//! lower the imperceptibility of this method.
//!
//! The amount of bits encoded by this implementation depends on used Unicode character set.
//! It it at most 5 bits per line with [FULL_UNICODE_CHARACTER_SET].
//!
//! Encoder does not inform if there is not data left!
pub mod character_sets;

use std::error::Error;

use crate::{
    binary::BitVec,
    context::{Context, ContextError},
    decoder::Decoder,
    encoder::{Capacity, Encoder, EncoderResult},
};
use log::trace;

use crate::binary::Bit;

use self::character_sets::{CharacterSetType, GetCharacterSet};

use super::Method;

/// Trailing unicode encoder for generic Unicode character sets.
/// It uses the [UnicodeSet] to get the character given the n-bits
/// (where n is the binary logarithm of the set size).
///
/// Accepts any [Context](crate::context::Context).
pub struct TrailingUnicodeMethod {
    character_set: CharacterSetType,
}

impl Default for TrailingUnicodeMethod {
    fn default() -> Self {
        Self::new(CharacterSetType::FullUnicodeSet)
    }
}

impl TrailingUnicodeMethod {
    pub fn new(unicode_set: CharacterSetType) -> Self {
        TrailingUnicodeMethod {
            character_set: unicode_set,
        }
    }
}

impl Capacity for TrailingUnicodeMethod {
    fn bitrate(&self) -> usize {
        let amount_of_bits = std::mem::size_of::<usize>() * 8;
        amount_of_bits - self.character_set.size().leading_zeros() as usize
    }
}

impl<E> Encoder<E> for TrailingUnicodeMethod
where
    E: Context,
{
    fn partial_encode(
        &self,
        context: &mut E,
        data: &mut dyn Iterator<Item = Bit>,
    ) -> Result<EncoderResult, Box<dyn Error>> {
        let set_capacity = self.bitrate();
        let next_n_bits = data.take(set_capacity).collect::<Vec<Bit>>();
        // We might not take exactly 5 bits, lets ensure we properly pad with 0 bits
        let amount_bits_taken = next_n_bits.len();
        let mut number: u32 = BitVec::from(next_n_bits).into();
        number <<= set_capacity - amount_bits_taken;

        trace!(
            "Took {} bits and assembled a number: {}",
            set_capacity,
            number
        );
        if let Some(character) = self.character_set.get_character(number) {
            trace!(
                "Putting unicode character {:?} at the end of the line",
                character
            );
            context.get_current_text_mut()?.push(*character);
        }

        Ok(EncoderResult::Success)
    }
}

impl<D> Decoder<D> for TrailingUnicodeMethod
where
    D: Context,
{
    fn partial_decode(&self, context: &D) -> Result<Vec<Bit>, ContextError> {
        if let Some(character) = context.get_current_text()?.chars().last() {
            let decoded_number = self.character_set.character_to_bits(&character);
            trace!(
                "Found {:?} at the end of the line, decoded into {}",
                &character,
                decoded_number
            );
            let data: Vec<Bit> = BitVec::from(decoded_number).into();
            let data_length = data.len();
            // Skip the unnecessary zeroes from the beginning
            let data_iter = data.into_iter().skip(data_length - self.bitrate());
            let decoded_data = data_iter.collect::<Vec<Bit>>();
            return Ok(decoded_data);
        }

        Ok(BitVec::filled_with(0, self.bitrate()).into())
    }
}

impl<E, D> Method<E, D> for TrailingUnicodeMethod
where
    E: Context,
    D: Context,
{
}
