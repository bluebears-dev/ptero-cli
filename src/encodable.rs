use std::{cell::RefCell, rc::Rc};

use log::{debug, trace};

use crate::{
    binary::BitIterator,
    encoder::{
        complex::extended_line_encoder::ExtendedLineEncoder, Encoder, EncoderResult, EncodingError,
        Result,
    },
    text::LineByPivotIterator,
};

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

            debug!(
                "Trying to encode the data to line of length {}",
                &line.len()
            );
            trace!("Constructed line: {}", &line);

            if !no_data_left {
                let mut encoder = ExtendedLineEncoder::new(line_iterator.borrow_mut());
                if let EncoderResult::NoDataLeft = encoder.encode(&mut bits, &mut line)? {
                    debug!("No data left to encode, setting flag to true");
                    no_data_left = true;
                }
            }

            stego_text.push_str(&format!("{}\n", &line));
        }
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
