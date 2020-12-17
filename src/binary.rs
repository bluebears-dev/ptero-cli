use std::{error::Error, convert::TryFrom};
use std::{fmt, vec::Vec};

const MOST_SIGNIFICANT_BIT_PATTERN: u8 = 0b10000000;
const CLEARED_PATTERN: u8 = 0b00000000;
/// Type for representing a bit.
#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Bit(pub u8);

/// Wrapper for `Vec<Bit>` used for implementing `From` trait.
pub struct BitVec(Vec<Bit>);

impl From<BitVec> for Vec<Bit> {
    fn from(bit_vec: BitVec) -> Self {
        bit_vec.0
    }
}

impl From<Vec<Bit>> for BitVec {
    fn from(bit_vec: Vec<Bit>) -> Self {
        BitVec(bit_vec)
    }
}

impl From<BitVec> for u32 {
    /// Conversion implementation for `u32`.
    /// Converts array of bits to the corresponding number. The function
    /// expects that the first element is the most significant bit.
    ///
    /// # Examples
    ///
    /// ## Convert 5 bits to number
    /// ```
    /// use ptero::binary::{Bit, BitVec};
    ///
    /// let array: BitVec = vec![1, 0, 1, 1, 1]
    ///                         .iter()
    ///                         .map(|v| Bit(*v))
    ///                         .collect::<Vec<Bit>>()
    ///                         .into();
    /// let number: u32 = array.into();
    /// assert_eq!(number, 23);
    /// ```   
    fn from(bit_vec: BitVec) -> Self {
        let mut number: u32 = 0;
        for bit in bit_vec.0.into_iter() {
            number <<= 1;
            number += u32::from(bit.0);
        }
        number 
    }
}

impl From<u32> for BitVec {
    /// Conversion implementation for `u32`.
    /// Converts `u32` number to the vector of bits. 
    /// The result vector has the most significant bit at the beginning.
    ///
    /// # Examples
    ///
    /// ## Convert the 65 number
    /// ```
    /// use ptero::binary::{Bit, BitVec};
    ///
    /// let array: BitVec = vec![1, 0, 1, 1, 1]
    ///                         .iter()
    ///                         .map(|v| Bit(*v))
    ///                         .collect::<Vec<Bit>>()
    ///                         .into();
    /// let number: u32 = array.into();
    /// assert_eq!(number, 23);
    /// ```   
    fn from(number: u32) -> Self {
        let byte_array = [
            ((number >> 24) & 0xff) as u8,
            ((number >> 16) & 0xff) as u8,
            ((number >> 8) & 0xff) as u8,
            (number & 0xff) as u8
        ];
        BitIterator::new(&byte_array).collect::<Vec<Bit>>().into()
    }
}

impl TryFrom<BitVec> for Vec<u8> {
    type Error = BinaryConversionError;

    /// Tries to convert array of bits to the array of bytes. The function
    /// expects that each left most bit in byte-size boundary is the 
    /// most significant bit.
    ///
    /// # Arguments
    ///
    /// * `bits` - reference to array of bits `&[Bit]`
    /// 
    /// # Behavior
    /// 
    /// Function return [BinaryConversionError](struct.BinaryConversionError.html) when
    /// array is not padded to byte-size boundary i.e. length to divisible by 8. 
    ///
    /// # Examples
    ///
    /// ## Convert 16 bits to 2 bytes
    /// ```
    /// use ptero::binary::{Bit, BitVec, BinaryConversionError};
    /// use std::convert::TryFrom;
    /// 
    /// let array: BitVec = vec![0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1]
    ///                             .iter()
    ///                             .map(|v| Bit(*v))
    ///                             .collect::<Vec<Bit>>()
    ///                             .into();
    /// let result: Result<Vec<u8>, BinaryConversionError> = TryFrom::try_from(array);
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap(), vec![42, 129]);
    /// ```      
    /// 
    /// ## Return error if array is not in byte-size boundary
    /// ```
    /// use ptero::binary::{Bit, BinaryConversionError, BitVec};
    /// use std::convert::TryFrom;
    ///
    /// let array: BitVec = vec![0, 0, 1]
    ///                             .iter()
    ///                             .map(|v| Bit(*v))
    ///                             .collect::<Vec<Bit>>()
    ///                             .into();
    /// let result: Result<Vec<u8>, BinaryConversionError> = TryFrom::try_from(array);
    /// assert!(!result.is_ok());
    /// ``` 
    fn try_from(bit_vec: BitVec) -> Result<Vec<u8>, Self::Error> {
        let mut bytes = Vec::<u8>::default();
        let mut index = 0;
        if bit_vec.0.len() % 8 != 0 {
            return Err(BinaryConversionError::new(
                "Bit array length is not divisible by 8".to_string(),
            ));
        }
        while index < bit_vec.0.len() {
            let mut byte = 0;
            for _ in 0..8 {
                byte *= 2;
                byte += bit_vec.0.get(index).unwrap().0;
                index += 1;
            }
            bytes.push(byte);
        }
        Ok(bytes)
    }
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
/// Error signaling binary conversion issues. Used in `From` trait implementation.
#[derive(Debug, Clone)]
pub struct BinaryConversionError {
    message: String,
}

impl BinaryConversionError {
    fn new(message: String) -> Self {
        BinaryConversionError { message }
    }
}

impl fmt::Display for BinaryConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Binary conversion error")
    }
}

impl Error for BinaryConversionError {}

#[derive(Debug)]
struct BinaryPattern(u8);

impl BinaryPattern {
    fn new() -> BinaryPattern {
        BinaryPattern(MOST_SIGNIFICANT_BIT_PATTERN)
    }

    fn start(&mut self) {
        self.0 = MOST_SIGNIFICANT_BIT_PATTERN;
    }

    fn is_cleared(&self) -> bool {
        self.0 == CLEARED_PATTERN
    }

    fn shift(&mut self) {
        self.0 >>= 1;
    }

    fn get(&self, byte: u8) -> Bit {
        match byte & self.0 {
            0 => Bit(0),
            _ => Bit(1),
        }
    }
}

/// Bit sequence iterator.
/// It enables user to read [Bits](struct.Bit.html) from any iterator that provides bytes as `u8`.
#[derive(Debug)]
pub struct BitIterator<'a> {
    bytes: &'a [u8],
    index: usize,
    fetch_pattern: BinaryPattern,
}

impl<'a> BitIterator<'a> {
    /// Creates a bit iterator for specified byte array.
    ///
    /// **Please note that it begins iteration from the MSB.**
    ///
    /// # Arguments
    ///
    /// * `array` - reference to array of bytes `&[u8]`
    ///
    /// # Examples
    ///
    /// ## Returns optional value
    /// ```
    /// use ptero::binary::{Bit, BitIterator};
    ///
    /// let array: Vec<u8> = vec!(1, 0, 2, 3);
    /// let mut iterator = BitIterator::new(&array);
    ///
    /// let bit = iterator.next().unwrap();
    /// let Bit(value) = bit;
    /// assert_eq!(value, 0);
    /// ```    
    ///
    /// ## Repeats itself after reaching the end
    /// ```
    /// use ptero::binary::{Bit, BitIterator};
    ///
    /// let array: Vec<u8> = vec!(0);
    /// let mut iterator = BitIterator::new(&array);
    ///
    /// for v in &mut iterator {
    ///     assert_eq!(v, Bit(0));
    /// }
    /// iterator.next();
    /// for v in &mut iterator {
    ///     assert_eq!(v, Bit(0));
    /// }
    /// ```    
    ///
    ///
    pub fn new(array: &'a [u8]) -> Self {
        // At the first execution we'll fetch the first value and then process it
        BitIterator {
            bytes: array,
            index: 0,
            fetch_pattern: BinaryPattern::new(),
        }
    }
}

impl<'a> Iterator for BitIterator<'a> {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.fetch_pattern.is_cleared() {
            self.fetch_pattern.start();
            self.index += 1;
        };
        let byte = *self.bytes.get(self.index)?;
        let bit = self.fetch_pattern.get(byte);
        self.fetch_pattern.shift();
        Some(bit)
    }
}
