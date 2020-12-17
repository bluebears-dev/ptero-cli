//! # Description
//!
//! This encoder implements the Extender Line steganography algorithm. It consist of three
//! simpler encoders:
//! * [RandomWhitespaceEncoder](../../random_whitespace_encoder/struct.RandomWhitespaceEncoder.html),
//! * [LineExtendEncoder](../../line_extend_encoder/struct.LineExtendEncoder.html),
//! * [TrailingWhitespaceEncoder](../../trailing_whitespace_encoder/struct.TrailingWhitespaceEncoder.html).
//!
//! For more info read docs on each one of the above encoders.
//!
//! See also [ComplexEncoder](../struct.ComplexEncoder.html) for more info about how to use this encoder.

use crate::{context::{PivotDecoderContext, PivotEncoderContext}, impl_complex_decoder, impl_complex_encoder, method::{line_extend, random_whitespace, trailing_whitespace, Method}};

pub struct ExtendedLineMethod {
    methods: Vec<Box<dyn Method<PivotEncoderContext, PivotDecoderContext>>>,
}

impl ExtendedLineMethod {
    pub fn new() -> Self {
        ExtendedLineMethod {
            methods: vec![
                Box::new(random_whitespace::RandomWhitespaceMethod::default()),
                Box::new(line_extend::LineExtendMethod::new()),
                Box::new(trailing_whitespace::TrailingWhitespaceMethod::default()),
            ],
        }
    }
}

impl Default for ExtendedLineMethod {
    fn default() -> Self {
        Self::new()
    }
}

impl_complex_encoder!(ExtendedLineMethod, PivotEncoderContext);
impl_complex_decoder!(ExtendedLineMethod, PivotDecoderContext);

impl Method<PivotEncoderContext, PivotDecoderContext> for ExtendedLineMethod {}
