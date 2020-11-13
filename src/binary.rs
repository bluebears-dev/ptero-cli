use std::{
    fmt,
    fs::File,
    io::{self, BufReader, Read},
};

/// Type for representing a bit.
#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Bit(pub u8);

#[derive(Debug)]
struct BinaryPattern(u8);

/// Bit sequence iterator.
/// It enables user to read [Bits](struct.Bit.html) from any iterator that provides bytes as `u8`.
#[derive(Debug)]
pub struct BitIterator<'a> {
    bytes: &'a [u8],
    index: usize,
    fetch_pattern: BinaryPattern,
}

/// Reads content from the file into binary data and returns it.
/// Returns either `Vec<u8>` with the file content or `std::io::Error` when it fails.
///
/// # Arguments
///
/// * `path` - A string slice that hold path to file from which the data will be read
///
/// # Examples
///
/// ```no_run
/// use ptero::binary::read_file_binary;
///
/// let result_data = read_file_binary("Cargo.toml");
/// ```
///
pub fn read_file_binary(path: &str) -> io::Result<Vec<u8>> {
    let mut binary_data = Vec::<u8>::new();
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_end(&mut binary_data)?;
    Ok(binary_data)
}

const MOST_SIGNIFICANT_BIT_PATTERN: u8 = 0b10000000;
const CLEARED_PATTERN: u8 = 0b00000000;

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

impl<'a> BitIterator<'a> {
    /// Creates a bit iterator for specified bytes
    ///
    /// **Please note that it begins iteration from the MSB.**
    ///
    /// # Arguments
    ///
    /// * `iter` - Iterator of type `Iterator<Item = u8>`
    ///
    /// # Examples
    ///
    /// ## Returns optional value
    /// ```
    /// use ptero::binary::{Bit, BitIterator};
    ///
    /// let array: Vec<Byte> = vec!(1, 0, 2, 3);
    /// let mut iterator = BitIterator::new(array);
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
    /// let array: Vec<Byte> = vec!(0);
    /// let mut iterator = BitIterator::new(array);
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
    pub fn new(vector: &'a [u8]) -> Self {
        // At the first execution we'll fetch the first value and then process it
        BitIterator {
            bytes: vector,
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

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", self.0)
    }
}
