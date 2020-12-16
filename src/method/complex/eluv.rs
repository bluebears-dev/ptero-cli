// //! # Description
// //!
// //! This encoder implements the Extender Line Unicode Variant (ELUV) steganography algorithm. It consist of three
// //! simpler encoders:
// //! * [RandomWhitespaceEncoder](../../random_whitespace_encoder/struct.RandomWhitespaceEncoder.html),
// //! * [LineExtendEncoder](../../line_extend_encoder/struct.LineExtendEncoder.html),
// //! * [TrailingUnicodeEncoder](../../trailing_unicode_encoder/struct.TrailingUnicodeEncoder.html).
// //!
// //! For more info read docs on each one of the above encoders.
// //!
// //! See also [ComplexEncoder](../struct.ComplexEncoder.html) for more info about how to use this encoder.

use crate::{
    impl_complex_decoder, impl_complex_encoder,
    method::{line_extend, random_whitespace, trailing_unicode, Method},
};
pub struct ELUVMethod {
    methods: Vec<Box<dyn Method>>,
}

impl ELUVMethod {
    fn new() -> Self {
        ELUVMethod {
            methods: vec![
                Box::new(random_whitespace::RandomWhitespaceMethod::default()),
                Box::new(line_extend::LineExtendMethod::default()),
                Box::new(trailing_unicode::TrailingUnicodeMethod::default()),
            ],
        }
    }
}

impl Default for ELUVMethod {
    fn default() -> Self {
        Self::new()
    }
}

impl_complex_encoder!(ELUVMethod);
impl_complex_decoder!(ELUVMethod);

impl Method for ELUVMethod {}
