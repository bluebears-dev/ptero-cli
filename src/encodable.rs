use std::{error::Error};

use log::{debug};

use crate::{binary::BitIterator, context::{Context, PivotByLineContext}, encoder::{Encoder, EncoderResult, EncodingError}, method::complex::extended_line::ExtendedLineMethod};

/// Trait describing data types which can be encoded into cover text.
/// Contains base implementation for `&[u8]` which can be used as the starting point.
pub trait Encodable {
    fn encode(&self, cover_text: &str, pivot: usize) -> Result<String, Box<dyn Error>>;
}

impl Encodable for &[u8] {
    fn encode(&self, cover_text: &str, pivot: usize) -> Result<String, Box<dyn Error>> {
        let mut context = PivotByLineContext::new(cover_text, pivot);
        let mut bits = BitIterator::new(self);
        let mut stego_text = String::new();

        let mut no_data_left = false;
        while !no_data_left {
            let mut encoder = ExtendedLineMethod::default();
            context.load_text()?;
            if let EncoderResult::NoDataLeft = encoder.encode(&mut context, &mut bits)? {
                debug!("No data left to encode, setting flag to true");
                no_data_left = true;
            }
            let line = context.get_current_text()?;
            stego_text.push_str(&format!("{}\n", &line));
        }
        // Append the rest of possible missing cover text
        let mut appended_line_count = 0;
        while let Ok(line) = context.load_text() {
            appended_line_count += 1;
            stego_text.push_str(&format!("{}\n", &line));
        }
        debug!("Appended the {} of left lines", appended_line_count);

        if !no_data_left {
            debug!("Capacity exceeded by {} bits", bits.count());
            Err(EncodingError::capacity_error())?
        } else {
            Ok(stego_text)
        }
    }
}

impl Encodable for &str {
    fn encode(&self, cover_text: &str, pivot: usize) -> Result<String, Box<dyn Error>> {
        self.as_bytes().encode(cover_text, pivot)
    }
}

impl Encodable for Vec<u8> {
    fn encode(&self, cover_text: &str, pivot: usize) -> Result<String, Box<dyn Error>> {
        self.as_slice().encode(cover_text, pivot)
    }
}
