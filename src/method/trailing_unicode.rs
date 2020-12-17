//! # Description
//!
//! This encoder encodes n-bits using available set of unicode whitespace. Larger sets provide better capacity
//! but might not work for every channel - it depends which characters get sanitized or are actually used.
//!
//! The whitespace are added at the end of the line as some of them are actually visible and might
//! lower the imperceptibility of this method.
//!
//! The amount of bits encoded by this implementation depends on used Unicode character set.
//! It it at most 5 bits per line with [FULL_UNICODE_CHARACTER_SET](constant.FULL_UNICODE_CHARACTER_SET.html).
//!
//! Encoder does not inform if there is not data left!
use std::error::Error;

use crate::{binary::{BitIterator, BitVec}, context::{Context, ContextError}, decoder::Decoder, encoder::{Encoder, EncoderResult}};
use log::trace;

use crate::binary::Bit;

use super::Method;

/// This trait is used for reading unicode set data
///
/// New sets should implement `get_set` which provides the array with unicode characters used.
pub trait UnicodeSet {
    fn get_set(&self) -> &[char];

    fn size(&self) -> usize {
        self.get_set().len()
    }
    fn capacity(&self) -> usize {
        let amount_of_bits = std::mem::size_of::<usize>() * 8;
        amount_of_bits - self.size().leading_zeros() as usize
    }

    fn get_character(&self, number: u32) -> Option<&char> {
        let index = number as usize;
        if number == 0 {
            None
        } else if index > self.size() {
            panic!("Too large number for given unicode set - cannot encode this amount of bits");
        } else {
            self.get_set().get(index - 1)
        }
    }

    fn character_to_bits(&self, chr: &char) -> u32 {
        if let Some(pos) = self.get_set().iter().position(|x| x == chr) {
            (pos + 1) as u32
        } else {
            0
        }
    }
}

/// Full set of used Unicode whitespace and invisible special chars - from different width spaces
/// to formatting chars and zero-width spaces
pub const FULL_UNICODE_CHARACTER_SET: [char; 31] = [
    '\u{0020}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}',
    '\u{2007}', '\u{2009}', '\u{200A}', '\u{200B}', '\u{200C}', '\u{200D}', '\u{200E}', '\u{2028}',
    '\u{202A}', '\u{202C}', '\u{202D}', '\u{202F}', '\u{205F}', '\u{2060}', '\u{2061}', '\u{2062}',
    '\u{2063}', '\u{2064}', '\u{2066}', '\u{2068}', '\u{2069}', '\u{3000}', '\u{FEFF}',
];
/// Struct representing the [FULL_UNICODE_CHARACTER_SET](constant.FULL_UNICODE_CHARACTER_SET.html).
pub struct FullUnicodeSet;

impl UnicodeSet for FullUnicodeSet {
    fn get_set(&self) -> &[char] {
        &FULL_UNICODE_CHARACTER_SET
    }
}

/// Trailing unicode encoder for generic Unicode character sets.
pub struct TrailingUnicodeMethod<T: UnicodeSet> {
    unicode_set: T,
}

impl Default for TrailingUnicodeMethod<FullUnicodeSet> {
    fn default() -> Self {
        Self::new(FullUnicodeSet {})
    }
}

impl<T> TrailingUnicodeMethod<T>
where
    T: UnicodeSet,
{
    pub fn new(unicode_set: T) -> Self {
        TrailingUnicodeMethod { unicode_set }
    }
}

impl<T, E> Encoder<E> for TrailingUnicodeMethod<T>
where
    T: UnicodeSet,
    E: Context,
{
    fn encode(
        &mut self,
        context: &mut E,
        data: &mut dyn Iterator<Item = Bit>,
    ) -> Result<EncoderResult, Box<dyn Error>> {
        let set_capacity = self.unicode_set.capacity();
        let next_n_bits: BitVec = data.take(set_capacity).collect::<Vec<Bit>>().into();
        let number: u32 = next_n_bits.into();
        trace!(
            "Took {} bits and assembled a number: {}",
            set_capacity,
            number
        );
        if let Some(character) = self.unicode_set.get_character(number) {
            trace!(
                "Putting unicode character {:?} at the end of the line",
                character
            );
            context.get_current_text_mut()?.push(*character);
        }
        // Take doesn't advance the iterator so we have to do it by ourselves
        for _ in 0..set_capacity {
            data.next();
        }
        Ok(EncoderResult::Success)
    }

    fn rate(&self) -> u32 {
        self.unicode_set.capacity() as u32
    }
}

impl<T, D> Decoder<D> for TrailingUnicodeMethod<T>
where
    T: UnicodeSet,
    D: Context,
{
    fn decode(&self, context: &D) -> Result<Vec<Bit>, ContextError> {
       if let Some(character) = context.get_current_text()?.chars().last() {
            Ok(BitVec::from(self.unicode_set.character_to_bits(&character)).into())
        } else {
            Ok(BitVec::from(0 as u32).into())
        }
    }
}

impl<T, E, D> Method<E, D> for TrailingUnicodeMethod<T>
where
    T: UnicodeSet,
    E: Context,
    D: Context,
{
}
