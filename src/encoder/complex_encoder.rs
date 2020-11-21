use crate::binary::Bit;

use super::{Encoder, EncoderResult, Result};

/// Encoder realizing extended-line steganography algorithm
pub mod extended_line_encoder;

/// Encoder realizing Extended Line Unicode Variant steganography algorithm
pub mod eluv_encoder;

pub type EncoderArray<'a> = Vec<Box<dyn Encoder + 'a>>;

pub struct ComplexEncoder<'a> {
    encoders: Vec<Box<dyn Encoder + 'a>>,
}

impl<'a> ComplexEncoder<'a> {
    pub fn new(encoders: EncoderArray<'a>) -> Self {
        ComplexEncoder { encoders }
    }
}

impl<'a> Encoder for ComplexEncoder<'a> {
    fn encode(
        &mut self,
        data: &mut dyn Iterator<Item = Bit>,
        line: &mut String,
    ) -> Result<EncoderResult> {
        let mut is_data_still_available = EncoderResult::Success;
        for encoder in &mut self.encoders {
            if let EncoderResult::NoDataLeft = encoder.encode(data, line)? {
                is_data_still_available = EncoderResult::NoDataLeft;
                break;
            }
        }
        Ok(is_data_still_available)
    }
}