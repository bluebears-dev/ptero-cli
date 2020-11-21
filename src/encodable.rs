use std::{cell::RefCell, rc::Rc};

use log::{debug, trace};

use crate::{binary::BitIterator, encoder::{Encoder, EncoderResult, EncodingError, Result, complex_encoder::extended_line_encoder::ExtendedLineEncoderFactory}, text::LineByPivotIterator};

/// Trait describing data types which can be encoded into cover text.
/// Contains base implementation for `&[u8]` which can be used as the starting point.
pub trait Encodable {
    fn encode(&self, cover_text: &str, pivot: usize) -> Result<String>;
}

impl Encodable for &[u8] {
    fn encode(&self, cover_text: &str, pivot: usize) -> Result<String> {
        let line_iterator = Rc::new(RefCell::new(LineByPivotIterator::new(&cover_text, pivot)));
        let mut bits = BitIterator::new(self);
        let mut stego_text = String::new();

        let mut no_data_left = false;
        while !no_data_left {
            let mut line: String;
            if let Some(next_line) = line_iterator.borrow_mut().next() {
                line = next_line;
            } else {
                debug!("No words left, stopping...");
                break;
            }

            if line_iterator.borrow().peek_word().is_some() {
                debug!(
                    "Trying to encode the data to line of length {}",
                    &line.len()
                );
                trace!("Constructed line: {}", &line);
                if !no_data_left {
                    let mut encoder = ExtendedLineEncoderFactory::build(line_iterator.borrow_mut());
                    if let EncoderResult::NoDataLeft = encoder.encode(&mut bits, &mut line)? {
                        debug!("No data left to encode, setting flag to true");
                        no_data_left = true;
                    }
                }
            } else {
                debug!("Last line occurred, skipping the encoding");
            }

            stego_text.push_str(&format!("{}\n", &line));
        }
        // Append the rest of possible missing cover text
        let mut appended_line_count = 0;
        while let Some(next_line) = line_iterator.borrow_mut().next() {
            appended_line_count += 1;
            stego_text.push_str(&format!("{}\n", &next_line));
        }
        debug!("Appended the {} of left lines", appended_line_count);

        if !no_data_left {
            debug!("Capacity exceeded by {} bits", bits.count());
            Err(EncodingError::capacity_error())
        } else {
            Ok(stego_text)
        }
    }
}

impl Encodable for &str {
    fn encode(&self, cover_text: &str, pivot: usize) -> Result<String> {
        self.as_bytes().encode(cover_text, pivot)
    }
}

impl Encodable for Vec<u8> {
    fn encode(&self, cover_text: &str, pivot: usize) -> Result<String> {
        self.as_slice().encode(cover_text, pivot)
    }
}
