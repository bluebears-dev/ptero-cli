use std::{
    fmt,
    fs::File,
    io::{self, BufReader, Read},
};

/// Type for representing a byte.
#[derive(Debug)]
pub struct Byte(pub u8);
/// Type for representing a bit.
#[derive(Debug)]
pub struct Bit(pub u8);

#[derive(Debug)]
struct BinaryPattern(u8);

/// Bit sequence iterator.
/// It enables user to read [Bits](struct.Bit.html) from any iterator that provides [Bytes](struct.Byte.html).
#[derive(Debug)]
pub struct BitIterator<I> {
    iter: I,
    current_byte: Byte,
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
pub fn read_file_binary(path: &str) -> io::Result<Vec<Byte>> {
    let mut binary_data = Vec::<u8>::new();
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_end(&mut binary_data)?;
    Ok(binary_data.iter().map(|&v| Byte(v)).collect())
}

impl BinaryPattern {
    fn new() -> BinaryPattern {
        BinaryPattern(0)
    }

    fn start(&mut self) {
        self.0 = 0b10000000;
    }

    fn is_cleared(&self) -> bool {
        self.0 == 0
    }

    fn shift(&mut self) {
        self.0 >>= 1;
    }

    fn get(&self, byte: &Byte) -> Bit {
        match byte.0 & self.0 {
            0 => Bit(0),
            _ => Bit(1),
        }
    }
}

impl<I> BitIterator<I> {
    /// Creates a new iterator for the specified iterator. 
    /// Currently only supports [Byte](struct.Byte.html) type iterator.
    /// 
    /// **Please note that it begins iteration from the MSB.**
    ///
    /// # Arguments
    ///
    /// * `iter` - Iterator of type `Iterator<Item = Byte>`
    ///
    /// # Examples
    /// 
    /// ```
    /// use ptero::binary::{Bit, Byte, BitIterator};
    ///
    /// let array: Vec<Byte> = vec!(1, 0, 2, 3).iter().map(|&v| Byte(v)).collect();
    /// let mut iterator = BitIterator::new(array.into_iter());
    /// 
    /// let bit = iterator.next().unwrap();
    /// let Bit(value) = bit; 
    /// assert_eq!(value, 0);
    /// ```
    ///
    pub fn new(iter: I) -> Self {
        // At the first execution we'll fetch the first value and then process it
        BitIterator {
            iter,
            current_byte: Byte(0),
            fetch_pattern: BinaryPattern::new(),
        }
    }
}

impl<I: Iterator<Item = Byte>> Iterator for BitIterator<I> {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.fetch_pattern.is_cleared() {
            self.fetch_pattern.start();
            self.current_byte = self.iter.next()?;
        };

        let bit = self.fetch_pattern.get(&self.current_byte);
        self.fetch_pattern.shift();
        Some(bit)
    }
}

impl fmt::Display for Byte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", self.0)
    }
}
