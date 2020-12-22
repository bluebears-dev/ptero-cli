//! # Description
//!
//! This method implements the *Extended Line* steganography algorithm. It consist of three
//! simpler methods:
//! * [RandomWhitespaceMethod](crate::method::random_whitespace::RandomWhitespaceMethod),
//! * [LineExtendMethod](crate::method::line_extend::LineExtendMethod),
//! * [TrailingWhitespaceMethod](crate::trailing_whitespace::TrailingWhitespaceMethod).
//!
//! For more info read docs on each one of the above encoders.

use crate::{context::{PivotByRawLineContext, PivotByLineContext}, impl_complex_decoder, impl_complex_encoder, method::{line_extend, random_whitespace, trailing_whitespace, Method}};

/// Structure representing the Extended Line algorithm. 
/// Contains the vector of used methods. Uses macros to implement the required traits. 
pub struct ExtendedLineMethod {
    methods: Vec<Box<dyn Method<PivotByLineContext, PivotByRawLineContext>>>,
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

impl_complex_encoder!(ExtendedLineMethod, PivotByLineContext);
impl_complex_decoder!(ExtendedLineMethod, PivotByRawLineContext);

impl Method<PivotByLineContext, PivotByRawLineContext> for ExtendedLineMethod {}
