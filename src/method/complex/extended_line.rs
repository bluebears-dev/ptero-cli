//! # Description
//!
//! This method implements the *Extended Line* steganography algorithm. It consist of three
//! simpler methods:
//! * [RandomWhitespaceMethod](crate::method::random_whitespace::RandomWhitespaceMethod),
//! * [LineExtendMethod](crate::method::line_extend::LineExtendMethod),
//! * [TrailingWhitespaceMethod](crate::trailing_whitespace::TrailingWhitespaceMethod).
//!
//! For more info read docs on each one of the above encoders.

use crate::{
    context::{PivotByLineContext, PivotByRawLineContext},
    impl_complex_decoder, impl_complex_encoder,
    method::{line_extend, random_whitespace, trailing_whitespace, Method},
};

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

#[allow(unused_imports)]
mod test {
    use std::error::Error;

    use crate::{
        binary::BitIterator,
        context::{PivotByLineContext, PivotByRawLineContext},
        decoder::Decoder,
        encoder::Encoder,
    };

    use super::ExtendedLineMethod;

    #[test]
    fn encodes_text_data() -> Result<(), Box<dyn Error>> {
        let cover_input = "a b c".repeat(5);
        let data_input = "a";
        let pivot: usize = 4;

        let mut data_iterator = BitIterator::new(&data_input.as_bytes());
        let method = ExtendedLineMethod::default();
        let mut context = PivotByLineContext::new(&cover_input, pivot);
        let stego_text = method.encode(&mut context, &mut data_iterator, None)?;

        assert_eq!(&stego_text, "a b ca \nb ca\nb ca b\nca b\nc \n");
        Ok(())
    }

    #[test]
    fn encodes_binary_data() -> Result<(), Box<dyn Error>> {
        let cover_input = "a b c ".repeat(5);
        let data_input: Vec<u8> = vec![0b11111111];
        let pivot: usize = 3;

        let mut data_iterator = BitIterator::new(&data_input);
        let method = ExtendedLineMethod::default();
        let mut context = PivotByLineContext::new(&cover_input, pivot);
        let stego_text = method.encode(&mut context, &mut data_iterator, None)?;

        assert_eq!(&stego_text, "a  b c \na  b c \na  b c\na b\nc a\nb c \n");
        Ok(())
    }

    #[test]
    fn decodes_binary_data() -> Result<(), Box<dyn Error>> {
        let stego_text = "a  bc\na bcd\na  b d \n";
        let pivot: usize = 4;

        let method = ExtendedLineMethod::default();
        let mut context = PivotByRawLineContext::new(&stego_text, pivot);
        let secret_data = method.decode(&mut context, None)?;

        assert_eq!(&secret_data, &[0b100_010_11, 0b100_000_00]);
        Ok(())
    }

    #[test]
    fn decodes_zeroes_if_no_data_encoded() -> Result<(), Box<dyn Error>> {
        let stego_text = "a\n".repeat(5);
        let pivot: usize = 4;

        let method = ExtendedLineMethod::default();
        let mut context = PivotByRawLineContext::new(&stego_text, pivot);
        let secret_data = method.decode(&mut context, None)?;

        assert_eq!(&secret_data, &[0, 0]);
        Ok(())
    }
}
