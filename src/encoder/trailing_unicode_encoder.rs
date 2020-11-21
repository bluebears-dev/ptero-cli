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
use crate::binary::BitVec;
use log::trace;

use crate::binary::Bit;

use super::{Encoder, EncoderResult, Result};
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
pub struct TrailingUnicodeEncoder<T: UnicodeSet> {
    unicode_set: T,
}

impl Default for TrailingUnicodeEncoder<FullUnicodeSet> {
    fn default() -> Self {
        Self::new(FullUnicodeSet {})
    }
}

impl<T> TrailingUnicodeEncoder<T>
where
    T: UnicodeSet,
{
    pub fn new(unicode_set: T) -> Self {
        TrailingUnicodeEncoder { unicode_set }
    }
}

impl<T> Encoder for TrailingUnicodeEncoder<T>
where
    T: UnicodeSet,
{
    fn encode(
        &mut self,
        data: &mut dyn Iterator<Item = Bit>,
        line: &mut String,
    ) -> Result<EncoderResult> {
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
            line.push(*character);
        }
        // Take doesn't advance the iterator so we have to do it byy ourselves
        for _ in 0..set_capacity {
            data.next();
        }
        Ok(EncoderResult::Success)
    }
}
