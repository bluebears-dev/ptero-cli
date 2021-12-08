/// This trait is used for reading unicode set data.
///
/// New sets should implement `get_set` which provides the array with
/// unicode characters used by the method.
pub trait GetCharacterSet {
    /// Returns the array of characters representing the Unicode characters that should be used by the method.
    /// The size of the array should be a power od 2. This is a requirement to be able to encode integer amount of bits.
    fn get_set(&self) -> &[char];

    fn size(&self) -> usize {
        self.get_set().len()
    }

    /// Maps index (in other words, the value) to the character in the set.
    ///
    /// # Arguments
    ///
    /// * `index` - the value which will be mapped (i.e. index of the character in the set)
    ///
    ///
    /// # Examples
    /// ## Gets character which is in the set
    /// ```
    /// use ptero_text::extended_line_method::character_sets::{CharacterSetType, GetCharacterSet};
    ///
    /// let set = CharacterSetType::Full;
    ///
    /// assert_eq!(set.get_character(1), Some(&'\u{0020}'));
    /// assert_eq!(set.get_character(2), Some(&'\u{2000}'));
    /// assert_eq!(set.get_character(31), Some(&'\u{FEFF}'));
    /// ```
    /// ## Returns None if value cannot be mapped
    /// ```
    /// use ptero_text::extended_line_method::character_sets::{CharacterSetType, GetCharacterSet};
    ///
    /// let set = CharacterSetType::Full;
    ///
    /// assert_eq!(set.get_character(0), None);
    /// ```
    /// # Panics
    /// The method panics if the provided value is larger than the set size.
    /// ## Panics if index exceeds the size of the set
    /// ```should_panic
    /// use ptero_text::extended_line_method::character_sets::{CharacterSetType, GetCharacterSet};
    ///
    /// let set = CharacterSetType::Full;
    ///
    /// set.get_character(100);
    /// ```
    fn get_character(&self, index: usize) -> Option<&char> {
        if index == 0 {
            None
        } else if index > self.size() {
            panic!("Too large number for given unicode set - cannot encode this amount of bits");
        } else {
            self.get_set().get(index - 1)
        }
    }

    /// Returns the number represented by the character.
    /// The number is the bit representation of the character - or in other words the index.
    /// If the character is not recognized it returns 0 by default.
    ///
    /// # Arguments
    ///
    /// * `chr` - character which will be converted
    ///
    /// # Examples
    /// ## Converts recognized character
    /// ```
    /// use ptero_text::extended_line_method::character_sets::{CharacterSetType, GetCharacterSet};
    ///
    /// let set = CharacterSetType::Full;
    /// let value = set.character_to_bits(&'\u{200A}');
    ///
    /// assert_eq!(value, 11);
    /// ```
    /// ## Converts unrecognized character to 0
    /// ```
    /// use ptero_text::extended_line_method::character_sets::{CharacterSetType, GetCharacterSet};
    ///
    /// let set = CharacterSetType::Full;
    /// let value = set.character_to_bits(&'A');
    ///
    /// assert_eq!(value, 0);
    /// ```
    fn character_to_bits(&self, chr: &char) -> usize {
        if let Some(pos) = self.get_set().iter().position(|x| x == chr) {
            pos + 1
        } else {
            0
        }
    }
}

/// Full set of used Unicode whitespace and invisible special chars - from different width spaces
/// to formatting chars and zero-width spaces.
pub const FULL_UNICODE_CHARACTER_SET: [char; 31] = [
    '\u{0020}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}',
    '\u{2007}', '\u{2009}', '\u{200A}', '\u{200B}', '\u{200C}', '\u{200D}', '\u{200E}', '\u{2028}',
    '\u{202A}', '\u{202C}', '\u{202D}', '\u{202F}', '\u{205F}', '\u{2060}', '\u{2061}', '\u{2062}',
    '\u{2063}', '\u{2064}', '\u{2066}', '\u{2068}', '\u{2069}', '\u{3000}', '\u{FEFF}',
];

/// Set of characters used to encode messages on Twitter
pub const TWITTER_UNICODE_CHARACTER_SET: [char; 15] = [
    '\u{0020}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}',
    '\u{2007}', '\u{2009}', '\u{200A}', '\u{200B}', '\u{200C}', '\u{200D}', '\u{FEFF}',
];

/// Set providing pre-defined characters for 4-bit encoding capacity.
pub const FOUR_BIT_CHARACTER_SET: [char; 15] = [
    '\u{0020}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}',
    '\u{2007}', '\u{2009}', '\u{200A}', '\u{200B}', '\u{200C}', '\u{200D}', '\u{200E}',
];

/// Set providing pre-defined characters for 3-bit encoding capacity.
pub const THREE_BIT_CHARACTER_SET: [char; 7] = [
    '\u{0020}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}',
];

/// Set providing pre-defined characters for 2-bit encoding capacity.
pub const TWO_BIT_CHARACTER_SET: [char; 3] = ['\u{0020}', '\u{2000}', '\u{2001}'];

/// Set providing pre-defined characters for 1-bit encoding capacity.
///
/// This is the base Extended Line algorithm behaviour.
pub const ONE_BIT_CHARACTER_SET: [char; 1] = ['\u{0020}'];

/// Enum representing possible character sets e.g. [FULL_UNICODE_CHARACTER_SET].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CharacterSetType {
    Full,
    FourBit,
    ThreeBit,
    TwoBit,
    OneBit,
    Twitter,
}

impl GetCharacterSet for CharacterSetType {
    /// Returns pre-defined character sets based on enum value.
    ///
    /// # Examples
    /// ## Get every character set
    /// ```
    /// use ptero_text::extended_line_method::character_sets::*;
    ///
    ///
    /// assert_eq!(CharacterSetType::Full.get_set(), &FULL_UNICODE_CHARACTER_SET);
    /// assert_eq!(CharacterSetType::FourBit.get_set(), &FOUR_BIT_CHARACTER_SET);
    /// assert_eq!(CharacterSetType::ThreeBit.get_set(), &THREE_BIT_CHARACTER_SET);
    /// assert_eq!(CharacterSetType::TwoBit.get_set(), &TWO_BIT_CHARACTER_SET);
    /// assert_eq!(CharacterSetType::OneBit.get_set(), &ONE_BIT_CHARACTER_SET);
    /// assert_eq!(CharacterSetType::Twitter.get_set(), &TWITTER_UNICODE_CHARACTER_SET);
    /// ```
    fn get_set(&self) -> &[char] {
        match *self {
            CharacterSetType::Full => &FULL_UNICODE_CHARACTER_SET,
            CharacterSetType::FourBit => &FOUR_BIT_CHARACTER_SET,
            CharacterSetType::ThreeBit => &THREE_BIT_CHARACTER_SET,
            CharacterSetType::TwoBit => &TWO_BIT_CHARACTER_SET,
            CharacterSetType::OneBit => &ONE_BIT_CHARACTER_SET,
            CharacterSetType::Twitter => &TWITTER_UNICODE_CHARACTER_SET,
        }
    }
}
