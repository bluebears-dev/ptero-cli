//! # Description
//!
//! This method implements the Extended Line Unicode Variant (ELUV) steganography algorithm. It consist of three
//! simpler methods:
//! * [RandomWhitespaceMethod](crate::method::random_whitespace::RandomWhitespaceMethod),
//! * [LineExtendMethod](crate::method::line_extend::LineExtendMethod),
//! * [TrailingUnicodeMethod](crate::method::trailing_unicode::TrailingUnicodeMethod).
//!
//! For more info read docs on each one of the above encoders.

use crate::{
    context::{PivotByLineContext, PivotByRawLineContext},
    impl_complex_decoder, impl_complex_encoder,
    method::{line_extend, random_whitespace, trailing_unicode, Method},
};

/// Structure representing the ELUV algorithm. 
/// Contains the vector of used methods. Uses macros to implement the required traits. 
pub struct ELUVMethod {
    methods: Vec<Box<dyn Method<PivotByLineContext, PivotByRawLineContext>>>,
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

impl_complex_encoder!(ELUVMethod, PivotByLineContext);
impl_complex_decoder!(ELUVMethod, PivotByRawLineContext);

impl Method<PivotByLineContext, PivotByRawLineContext> for ELUVMethod {}
