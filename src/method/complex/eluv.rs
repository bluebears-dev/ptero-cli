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

#[allow(unused_imports)]
mod test {
    use std::error::Error;

    use crate::{binary::BitIterator, cli::encoder::EncodeSubCommand, context::{PivotByLineContext, PivotByRawLineContext}, decoder::Decoder, encoder::Encoder};

    use super::ELUVMethod;

    #[test]
    fn encodes_text_data() -> Result<(), Box<dyn Error>> {
        let cover_input = "a b c ".repeat(5);
        let data_input = "abc";
        let pivot: usize = 3;

        let mut data_iterator = BitIterator::new(&data_input.as_bytes());
        let method = ELUVMethod::default();
        let mut context = PivotByLineContext::new(&cover_input, pivot);
        let stego_text = method.encode(&mut context, &mut data_iterator)?;

        assert_eq!(
            &stego_text,
            "a b c\u{2028}\na  b\u{2062}\nc  a\u{200b}\nb c a \nb c\na b\nc \n"
        );
        Ok(())
    }

    #[test]
    fn encodes_binary_data() -> Result<(), Box<dyn Error>> {
        let cover_input = "a b c ".repeat(6);
        let data_input: Vec<u8> = vec![0b11101010, 0b10000011, 0b01011110];
        let pivot: usize = 3;

        let mut data_iterator = BitIterator::new(&data_input);
        let method = ELUVMethod::default();
        let mut context = PivotByLineContext::new(&cover_input, pivot);
        let stego_text = method.encode(&mut context, &mut data_iterator)?;

        assert_eq!(
            &stego_text,
            "a  b c\u{205f}\na b c\na  b c\u{200a}\na  b c\na b\nc a\nb c \n"
        );
        Ok(())
    }

    #[test]
    fn decodes_binary_data() ->  Result<(), Box<dyn Error>> {
        let stego_text = "a  bc\na bcd \na  b d\u{205f}\n";
        let pivot: usize = 4;

        let method = ELUVMethod::default();
        let mut context = PivotByRawLineContext::new(&stego_text, pivot);
        let secret_data = method.decode(&mut context)?;

        assert_eq!(&secret_data, &[0b10_00000_0, 0b1_00001_11, 0b10101_000]);
        Ok(())
    }

    #[test]
    fn decodes_zeroes_if_no_data_encoded() ->  Result<(), Box<dyn Error>> {
        let stego_text = "a\n".repeat(5);
        let pivot: usize = 4;

        let method = ELUVMethod::default();
        let mut context = PivotByRawLineContext::new(&stego_text, pivot);
        let secret_data = method.decode(&mut context)?;

        assert_eq!(&secret_data, &[0, 0, 0, 0, 0]);
        Ok(())
    }
}
