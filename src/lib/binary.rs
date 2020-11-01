use std::{fs::File, io::{self, BufReader, Read}};

#[derive(Debug)]
pub struct BitIterator<I> {
    iter: I,
    current_byte: u8,
    fetch_pattern: u8,
}

impl<I> BitIterator<I> {
    pub fn new(iter: I) -> Self {
        // At the first execution we'll fetch the first value and then process it
        BitIterator {
            iter,
            current_byte: 0,
            fetch_pattern: 0b00000000,
        }
    }
}

impl<I: Iterator<Item = u8>> Iterator for BitIterator<I> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_byte = if self.fetch_pattern == 0b00000000 {
            self.fetch_pattern = 0b10000000;
            self.iter.next()?
        } else {
            self.current_byte
        };
        let bit = match self.current_byte & self.fetch_pattern {
            0 => 0,
            _ => 1,
        };
        self.fetch_pattern >>= 1;
        Some(bit)
    }
}

pub fn read_file_binary(path: &str) -> io::Result<Vec<u8>> {
    let mut binary_data = Vec::<u8>::new();
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_end(&mut binary_data)?;
    Ok(binary_data)
}
