//! # Description
//!
//! This method implements the Extended Line Unicode Variant (ELUV) steganography algorithm. It consist of three
//! simpler methods:
//! * [RandomWhitespaceMethod](crate::method::random_whitespace::RandomWhitespaceMethod),
//! * [LineExtendMethod](crate::method::line_extend::LineExtendMethod),
//! * [TrailingUnicodeMethod](crate::method::trailing_unicode::TrailingUnicodeMethod).
//!
//! For more info read docs on each one of the above encoders.

use trailing_unicode::character_sets::CharacterSetType;

use crate::{
    context::{PivotByLineContext, PivotByRawLineContext},
    impl_complex_decoder, impl_complex_encoder,
    method::{line_extend, random_whitespace, trailing_unicode, Method},
};

type ELUVSubmethod = Box<dyn Method<PivotByLineContext, PivotByRawLineContext>>;
/// Structure representing the ELUV algorithm.
/// Contains the vector of used methods. Uses macros to implement the required traits.
pub struct ELUVMethod {
    methods: Vec<ELUVSubmethod>,
}

#[derive(Debug, PartialEq)]
pub enum ELUVMethodVariant {
    Variant1,
    Variant2,
    Variant3,
}

pub struct ELUVMethodBuilder {
    character_set: CharacterSetType,
    variant: ELUVMethodVariant,
}

impl ELUVMethodBuilder {
    pub fn new() -> Self {
        ELUVMethodBuilder {
            character_set: CharacterSetType::FullUnicodeSet,
            variant: ELUVMethodVariant::Variant1,
        }
    }

    pub fn character_set(mut self, set: CharacterSetType) -> Self {
        self.character_set = set;
        self
    }

    pub fn variant(mut self, variant: ELUVMethodVariant) -> Self {
        self.variant = variant;
        self
    }

    fn select_methods(&self) -> Vec<ELUVSubmethod> {
        let indices = match self.variant {
            ELUVMethodVariant::Variant1 => &[0, 1, 2],
            ELUVMethodVariant::Variant2 => &[1, 0, 2],
            ELUVMethodVariant::Variant3 => &[1, 2, 0],
        };

        indices
            .iter()
            .map(|i| {
                let method: ELUVSubmethod = match i {
                    0 => Box::new(random_whitespace::RandomWhitespaceMethod::default()),
                    1 => Box::new(line_extend::LineExtendMethod::default()),
                    _ => Box::new(trailing_unicode::TrailingUnicodeMethod::new(
                        self.character_set,
                    )),
                };
                method
            })
            .collect()
    }

    pub fn build(&self) -> ELUVMethod {
        ELUVMethod {
            methods: self.select_methods(),
        }
    }
}

impl Default for ELUVMethod {
    fn default() -> Self {
        ELUVMethodBuilder::new().build()
    }
}

impl_complex_encoder!(ELUVMethod, PivotByLineContext);
impl_complex_decoder!(ELUVMethod, PivotByRawLineContext);

impl Method<PivotByLineContext, PivotByRawLineContext> for ELUVMethod {
    fn method_name(&self) -> String {
        format!(
            "ELUVMethod({},{},{})",
            self.methods[0].method_name(),
            self.methods[1].method_name(),
            self.methods[2].method_name(),
        )
    }
}

#[allow(unused_imports)]
mod test {
    use std::error::Error;

    use crate::{
        binary::BitIterator,
        cli::encoder::EncodeSubCommand,
        context::{PivotByLineContext, PivotByRawLineContext},
        decoder::Decoder,
        encoder::Encoder,
        method::{random_whitespace::RandomWhitespaceMethod, Method},
    };

    use super::{ELUVMethod, ELUVMethodBuilder, ELUVMethodVariant};

    #[test]
    fn encodes_text_data() -> Result<(), Box<dyn Error>> {
        let cover_input = "a b c ".repeat(5);
        let data_input = "abc";
        let pivot: usize = 3;

        let mut data_iterator = BitIterator::new(&data_input.as_bytes());
        let method = ELUVMethod::default();
        let mut context = PivotByLineContext::new(&cover_input, pivot);
        let stego_text = method.encode(&mut context, &mut data_iterator, None)?;

        assert_eq!(
            &stego_text,
            "a b c\u{2028}\na  b\u{2062}\nc  a\u{200b}\nb c a\u{2028}\nb c\na b\nc \n"
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
        let stego_text = method.encode(&mut context, &mut data_iterator, None)?;

        assert_eq!(
            &stego_text,
            "a  b c\u{205f}\na b c\na  b c\u{200a}\na  b c\na b\nc a\nb c \n"
        );
        Ok(())
    }

    #[test]
    fn decodes_binary_data() -> Result<(), Box<dyn Error>> {
        let stego_text = "a  bc\na bcd \na  b d\u{205f}\n";
        let pivot: usize = 4;

        let method = ELUVMethod::default();
        let mut context = PivotByRawLineContext::new(&stego_text, pivot);
        let secret_data = method.decode(&mut context, None)?;

        assert_eq!(&secret_data, &[0b10_00000_0, 0b1_00001_11, 0b10101_000]);
        Ok(())
    }

    #[test]
    fn decodes_zeroes_if_no_data_encoded() -> Result<(), Box<dyn Error>> {
        let stego_text = "a\n".repeat(5);
        let pivot: usize = 4;

        let method = ELUVMethod::default();
        let mut context = PivotByRawLineContext::new(&stego_text, pivot);
        let secret_data = method.decode(&mut context, None)?;

        assert_eq!(&secret_data, &[0, 0, 0, 0, 0]);
        Ok(())
    }

    #[test]
    fn default_method_is_variant_1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            ELUVMethod::default().method_name(),
            "ELUVMethod(RandomWhitespaceMethod,LineExtendMethod,TrailingUnicodeMethod)"
        );
        Ok(())
    }

    #[test]
    fn builder_properly_constructs_variants() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            ELUVMethodBuilder::new()
                .variant(ELUVMethodVariant::Variant1)
                .build()
                .method_name(),
            "ELUVMethod(RandomWhitespaceMethod,LineExtendMethod,TrailingUnicodeMethod)"
        );
        assert_eq!(
            ELUVMethodBuilder::new()
                .variant(ELUVMethodVariant::Variant2)
                .build()
                .method_name(),
            "ELUVMethod(LineExtendMethod,RandomWhitespaceMethod,TrailingUnicodeMethod)"
        );
        assert_eq!(
            ELUVMethodBuilder::new()
                .variant(ELUVMethodVariant::Variant3)
                .build()
                .method_name(),
            "ELUVMethod(LineExtendMethod,TrailingUnicodeMethod,RandomWhitespaceMethod)"
        );
        Ok(())
    }
}
