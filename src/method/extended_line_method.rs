use std::iter::Peekable;
use std::ops::{Deref, DerefMut};
use std::sync::mpsc::Sender;
use log::{error, trace};
use rand::{Rng, RngCore};
use unicode_segmentation::{UnicodeSegmentation, UWordBounds};
use snafu::Snafu;
use crate::binary::Bit;
use crate::encoder::{EncoderResult, EncodingError};
use crate::method::config::{CommonMethodConfig, CommonMethodConfigBuilder, MethodProgressStatus};
use crate::method::SteganographyMethod;

const ASCII_DELIMITER: &str = " ";
const NEWLINE_STR: &str = "\n";

const DEFAULT_PIVOT: usize = 15;

#[derive(Debug)]
pub enum MethodActions {
    LineExtend,
    RandomASCIIWhitespace,
    TrailingASCIIWhitespace,
}

#[derive(Debug)]
pub enum Variant {
    V1,
    V2,
    V3,
}

pub struct ExtendedLineMethodBuilder<'a> {
    pivot: usize,
    variant: Variant,
    config_builder: CommonMethodConfigBuilder<'a>,
}

impl<'a> Default for ExtendedLineMethodBuilder<'a> {
    fn default() -> Self {
        ExtendedLineMethodBuilder {
            pivot: DEFAULT_PIVOT,
            variant: Variant::V1,
            config_builder: CommonMethodConfig::builder(),
        }
    }
}

impl<'a> ExtendedLineMethodBuilder<'a> {
    /// Set custom RNG for method.
    pub fn with_rng(mut self, rng: Box<dyn RngCore>) -> Self {
        self.config_builder = self.config_builder.with_rng(rng);
        self
    }

    /// Register progress status pipe
    pub fn register(mut self, observer: &'a Sender<MethodProgressStatus>) -> Self {
        self.config_builder = self.config_builder.register(observer);
        self
    }

    /// Set variant of the method
    pub fn with_variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_pivot(mut self, pivot: usize) -> Self {
        self.pivot = pivot;
        self
    }

    /// Constructs the method
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::sync::mpsc::channel;
    /// use rand::rngs::mock::StepRng;
    /// use ptero::method::extended_line_method::{ExtendedLineMethod, Variant};
    /// use ptero::method::config::MethodProgressStatus;
    ///
    /// let (tx, rx) = channel::<MethodProgressStatus>();
    ///
    /// let method = ExtendedLineMethod::builder()
    ///     .with_rng(Box::new(StepRng::new(0, 1)))
    ///     .with_variant(Variant::V2)
    ///     .with_pivot(20)
    ///     .register(&tx)
    ///     .build();
    /// ```
    pub fn build(self) -> ExtendedLineMethod<'a> {
        ExtendedLineMethod {
            pivot: self.pivot,
            variant: self.variant,
            config: self.config_builder.build().unwrap(),
        }
    }
}

pub(crate) fn get_variant_methods(variant: &Variant) -> &'static [MethodActions; 3] {
    match variant {
        Variant::V1 => &[MethodActions::LineExtend, MethodActions::RandomASCIIWhitespace, MethodActions::TrailingASCIIWhitespace],
        Variant::V2 => &[MethodActions::LineExtend, MethodActions::TrailingASCIIWhitespace, MethodActions::RandomASCIIWhitespace],
        Variant::V3 => &[MethodActions::RandomASCIIWhitespace, MethodActions::LineExtend, MethodActions::TrailingASCIIWhitespace]
    }
}

fn graphemes_length(text: &str) -> usize {
    text.graphemes(true).count()
}

pub struct ExtendedLineMethod<'a> {
    pivot: usize,
    variant: Variant,
    config: CommonMethodConfig<'a>,
}

#[derive(Debug, PartialEq, Snafu)]
pub enum MethodError {
    #[snafu(display("Pivot {} is smaller then the longest word in cover: {}", pivot, word))]
    PivotTooSmall {
        word: String,
        pivot: usize,
    },
    #[snafu(display("Cover too small - '{}' bytes left unprocessed", remaining_data_size))]
    CoverTextTooSmall {
        remaining_data_size: usize
    },
    #[snafu(display("Cannot extend line to be longer than {}", pivot))]
    CannotExtendLineAbovePivotLength {
        reason: PivotExtensionErrorReason,
        pivot: usize,
    },
}

#[cfg(not(tarpaulin_include))]
impl MethodError {
    fn pivot_too_small(word: String, pivot: usize) -> MethodError {
        MethodError::PivotTooSmall {
            word,
            pivot,
        }
    }

    fn cover_text_too_small(remaining_data_size: usize) -> MethodError {
        MethodError::CoverTextTooSmall {
            remaining_data_size
        }
    }

    fn no_cover_words(pivot: usize) -> MethodError {
        MethodError::CannotExtendLineAbovePivotLength {
            reason: PivotExtensionErrorReason::NoCoverWords,
            pivot,
        }
    }

    fn line_too_short(pivot: usize) -> MethodError {
        MethodError::CannotExtendLineAbovePivotLength {
            reason: PivotExtensionErrorReason::LineTooShort,
            pivot,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PivotExtensionErrorReason {
    NoCoverWords,
    LineTooShort,
}

type Result<Success> = std::result::Result<Success, MethodError>;
type VerificationResult = std::result::Result<(), MethodError>;

impl<'a> ExtendedLineMethod<'a> {
    pub fn builder() -> ExtendedLineMethodBuilder<'a> {
        ExtendedLineMethodBuilder::default()
    }

    fn verify_pivot(&self, cover: &str) -> VerificationResult {
        let words_longer_than_pivot = cover.split_whitespace()
            .filter(|word| word.len() > self.pivot)
            .collect::<Vec<&str>>();

        if !words_longer_than_pivot.is_empty() {
            let word = words_longer_than_pivot[0];
            Err(MethodError::pivot_too_small(word.to_string(), self.pivot))
        } else {
            Ok(())
        }
    }

    fn partial_conceal<'b, I>(
        &mut self,
        word_iterator: &mut Peekable<I>,
        data: &mut dyn Iterator<Item=Bit>,
        result: &mut String,
    ) -> Result<EncoderResult>
        where I: Iterator<Item=&'b str>
    {
        let pivot_line = self.construct_pivot_line(word_iterator);
        result.push_str(&pivot_line);

        if pivot_line.is_empty() {
            let remaining_data_size = data.count();
            return Err(MethodError::cover_text_too_small(remaining_data_size));
        }

        for action in get_variant_methods(&self.variant) {
            let method_result = match action {
                MethodActions::LineExtend => {
                    self.conceal_in_extend_line(graphemes_length(&pivot_line), word_iterator, data, result)
                }
                MethodActions::RandomASCIIWhitespace => {
                    self.conceal_in_random_ascii_whitespace(data, result)
                }
                MethodActions::TrailingASCIIWhitespace => {
                    self.conceal_in_trailing_ascii_whitespace(data, result)
                }
            };
            if let EncoderResult::NoDataLeft = method_result? {
                return Ok(EncoderResult::NoDataLeft);
            }
        };
        Ok(EncoderResult::Success)
    }

    fn conceal_in_extend_line<'b, I>(
        &self,
        pivot_line_length: usize,
        word_iter: &mut Peekable<I>,
        data: &mut dyn Iterator<Item=Bit>,
        result: &mut String,
    ) -> Result<EncoderResult> where I: Iterator<Item=&'b str> {
        Ok(match data.next() {
            Some(Bit(1)) => {
                let next_word = word_iter.next()
                    .ok_or_else(|| MethodError::no_cover_words(self.pivot))?;

                if pivot_line_length + graphemes_length(next_word) + graphemes_length(ASCII_DELIMITER) <= self.pivot {
                    return Err(MethodError::line_too_short(self.pivot));
                }

                println!("Extending line with '{}'", &next_word);
                result.push_str(ASCII_DELIMITER);
                result.push_str(next_word);
                EncoderResult::Success
            }
            None => EncoderResult::NoDataLeft,
            _ => {
                println!("Leaving line as-is");
                EncoderResult::Success
            }
        })
    }

    fn conceal_in_random_ascii_whitespace(
        &mut self,
        data: &mut dyn Iterator<Item=Bit>,
        cover: &mut String,
    ) -> Result<EncoderResult> {
        Ok(match data.next() {
            Some(Bit(1)) => {
                let last_newline_index = cover.rfind(NEWLINE_STR).unwrap_or(0);
                let position = self.find_approx_whitespace_position(cover, last_newline_index);

                println!("Putting space at position {}", position);
                cover.insert_str(position, &String::from(ASCII_DELIMITER));
                EncoderResult::Success
            }
            None => EncoderResult::NoDataLeft,
            _ => {
                println!("Skipping double whitespace");
                EncoderResult::Success
            }
        })
    }

    fn conceal_in_trailing_ascii_whitespace(
        &self,
        data: &mut dyn Iterator<Item=Bit>,
        cover: &mut String,
    ) -> Result<EncoderResult> {
        Ok(match data.next() {
            Some(Bit(1)) => {
                println!("Putting whitespace at the end of the line");
                cover.push_str(ASCII_DELIMITER);
                EncoderResult::Success
            }
            None => EncoderResult::NoDataLeft,
            _ => {
                println!("Skipping trailing whitespace");
                EncoderResult::Success
            }
        })
    }

    fn construct_pivot_line<'b, I>(&self, word_iter: &mut Peekable<I>) -> String where I: Iterator<Item=&'b str> {
        let mut current_line_length = 0;
        let mut result = String::new();

        while let Some(next_word) = word_iter.peek() {
            let line_appendix = if current_line_length > 0 {
                [ASCII_DELIMITER, next_word].join("")
            } else {
                next_word.to_string()
            };

            if current_line_length + graphemes_length(&line_appendix) > self.pivot {
                break;
            }

            current_line_length += graphemes_length(&line_appendix);
            result.push_str(&line_appendix);

            word_iter.next();
        }
        println!("Constructed line of length: '{}' while '{}' is the pivot", current_line_length, self.pivot);
        result
    }


    fn find_approx_whitespace_position(&mut self, cover: &mut String, last_newline_index: usize) -> usize {
        let rng = self.config.rng.deref_mut();
        let approx_position = rng.gen_range(last_newline_index, cover.len());

        let last_line = &cover[last_newline_index..];
        let mut position = last_line.find(' ')
            .unwrap_or_else(|| last_line.len()) + last_newline_index;

        for (index, character) in last_line.char_indices() {
            if index + last_newline_index > approx_position {
                break;
            }
            if character.is_whitespace() && !NEWLINE_STR.contains(character) {
                position = index + last_newline_index;
            }
        }
        position
    }
}

impl<'a> SteganographyMethod<&'a str, MethodError> for ExtendedLineMethod<'a> {
    type Output = String;
    type Input = &'a mut dyn Iterator<Item=Bit>;

    fn try_conceal(&mut self, cover: &str, data: Self::Input) -> Result<Self::Output> {
        self.verify_pivot(cover)?;

        let mut result = String::with_capacity(cover.len());

        let mut word_iterator = cover.split_whitespace()
            .filter(|word| !word.contains(char::is_whitespace))
            .peekable();

        while let EncoderResult::Success = self.partial_conceal(&mut word_iterator, data, &mut result)? {
            println!("PARTIAL: {}", result);
            result.push_str(NEWLINE_STR);
        }

        loop {
            let line = self.construct_pivot_line(&mut word_iterator);
            if line.is_empty() {
                break;
            }
            result.push_str(NEWLINE_STR);
            result.push_str(&line);
        }

        Ok(result)
    }

    fn try_reveal(&mut self, stego_text: &str) -> Result<Self::Input> {
        todo!()
    }
}

#[allow(unused_imports)]
mod test {
    use std::error::Error;
    use rand::Rng;
    use rand::rngs::mock::StepRng;
    use crate::binary::BitIterator;
    use crate::method::extended_line_method::{ExtendedLineMethod, MethodError};
    use crate::method::extended_line_method::Variant;
    use crate::method::SteganographyMethod;

    const SINGLE_CHAR_TEXT: &str = "a b ca b ca b ca b ca b c";
    const WITH_WORDS_TEXT: &str = "A little panda has fallen from a tree. The panda went rolling down the hill";
    const WITH_OTHER_WHITESPACE_TEXT: &str = "A\tlittle  panda \
        has fallen from a tree. \
        The panda went rolling \
        down the\t hill";
    const WITH_EMOJI_TEXT: &str = "A little 🐼 has (fallen) from a \\🌳/. The 🐼 went rolling down the hill.";
    const HTML_TEXT: &str = "<div> \
        <button style=\" background: red;\">Click me</button> \
        <div/> \
        <footer> This is the end \
        </footer>";
    const TINY_TEXT: &str = "TINY COVER";
    const EMPTY_TEXT: &str = "";

    fn get_method<'a>(pivot: usize, variant: Variant) -> ExtendedLineMethod<'a> {
        ExtendedLineMethod::builder()
            .with_pivot(pivot)
            .with_rng(Box::new(StepRng::new(1, 1)))
            .with_variant(variant)
            .build()
    }

    #[test]
    fn conceals_text_data() -> Result<(), Box<dyn Error>> {
        let data_input = "a";
        let mut data_iterator = BitIterator::new(data_input.as_bytes());
        let mut method = get_method(4, Variant::V1);

        let stego_text = method.try_conceal(SINGLE_CHAR_TEXT, &mut data_iterator)?;

        assert_eq!(stego_text, "a  b \nca b\nca  b\nca b\nca b\nc");
        Ok(())
    }

    #[test]
    fn conceals_binary_data() -> Result<(), Box<dyn Error>> {
        let data_input: Vec<u8> = vec![0b11111111];
        let mut data_iterator = BitIterator::new(&data_input);
        let mut method = get_method(3, Variant::V1);

        let stego_text = method.try_conceal(SINGLE_CHAR_TEXT, &mut data_iterator)?;

        assert_eq!(stego_text, "a  b ca \nb  ca \nb  ca\nb\nca\nb c");
        Ok(())
    }

    #[test]
    fn conceals_data_in_cover_with_words() -> Result<(), Box<dyn Error>> {
        let data_input: Vec<u8> = vec![0b11111111];
        let mut data_iterator = BitIterator::new(&data_input);
        let mut method = get_method(10, Variant::V1);

        let stego_text = method.try_conceal(WITH_WORDS_TEXT, &mut data_iterator)?;

        assert_eq!(stego_text, "A  little panda \nhas  fallen from \na  tree. The\npanda went\nrolling\ndown the\nhill");
        Ok(())
    }


    #[test]
    fn conceals_data_in_cover_with_other_whitespace() -> Result<(), Box<dyn Error>> {
        let data_input: Vec<u8> = vec![0b11111111];
        let mut data_iterator = BitIterator::new(&data_input);
        let mut method = get_method(10, Variant::V1);

        let stego_text = method.try_conceal(WITH_OTHER_WHITESPACE_TEXT, &mut data_iterator)?;

        assert_eq!(stego_text, "A  little panda \nhas  fallen from \na  tree. The\npanda went\nrolling\ndown the\nhill");
        Ok(())
    }

    #[test]
    fn conceals_data_in_cover_with_special_chars() -> Result<(), Box<dyn Error>> {
        let data_input: Vec<u8> = vec![0b11111111];
        let mut data_iterator = BitIterator::new(&data_input);
        let mut method = get_method(10, Variant::V1);

        let stego_text = method.try_conceal(WITH_EMOJI_TEXT, &mut data_iterator)?;

        assert_eq!(stego_text, "A  little 🐼 has \n(fallen)  from \na  \\🌳/. The 🐼\nwent\nrolling\ndown the\nhill.");
        Ok(())
    }


    #[test]
    fn conceals_data_in_html_cover() -> Result<(), Box<dyn Error>> {
        let data_input: Vec<u8> = vec![0b11111111];
        let mut data_iterator = BitIterator::new(&data_input);
        let mut method = get_method(15, Variant::V1);

        let stego_text = method.try_conceal(HTML_TEXT, &mut data_iterator)?;

        assert_eq!(stego_text, "<div>  <button style=\" \nbackground:  red;\">Click \nme</button>  <div/>\n<footer> This\nis the end\n</footer>");
        Ok(())
    }

    #[test]
    fn conceals_with_variant_v1() -> Result<(), Box<dyn Error>> {
        let data_input: Vec<u8> = vec![0b10101011];
        let mut data_iterator = BitIterator::new(&data_input);
        let mut method = get_method(8, Variant::V1);

        let stego_text = method.try_conceal(WITH_WORDS_TEXT, &mut data_iterator)?;

        assert_eq!(stego_text, "A little panda \nhas \nfallen  from\na tree.\nThe\npanda\nwent\nrolling\ndown the\nhill");
        Ok(())
    }

    #[test]
    fn conceals_with_variant_v2() -> Result<(), Box<dyn Error>> {
        let data_input: Vec<u8> = vec![0b10101011];
        let mut data_iterator = BitIterator::new(&data_input);
        let mut method = get_method(8, Variant::V2);

        let stego_text = method.try_conceal(WITH_WORDS_TEXT, &mut data_iterator)?;

        assert_eq!(stego_text, "A  little panda\nhas \nfallen from \na tree.\nThe\npanda\nwent\nrolling\ndown the\nhill");
        Ok(())
    }

    #[test]
    fn conceals_with_variant_v3() -> Result<(), Box<dyn Error>> {
        let data_input: Vec<u8> = vec![0b10101011];
        let mut data_iterator = BitIterator::new(&data_input);
        let mut method = get_method(8, Variant::V3);

        let stego_text = method.try_conceal(WITH_WORDS_TEXT, &mut data_iterator)?;

        assert_eq!(stego_text, "A  little \npanda has\nfallen  from\na tree.\nThe\npanda\nwent\nrolling\ndown the\nhill");
        Ok(())
    }

    #[test]
    fn works_with_empty_data() -> Result<(), Box<dyn Error>> {
        let data_input: Vec<u8> = vec![0b0];
        let mut data_iterator = BitIterator::new(&data_input);
        let mut method = get_method(8, Variant::V1);

        let stego_text = method.try_conceal(WITH_WORDS_TEXT, &mut data_iterator)?;

        assert_eq!(stego_text, "A little\npanda\nhas\nfallen\nfrom a\ntree.\nThe\npanda\nwent\nrolling\ndown the\nhill");
        Ok(())
    }

    #[test]
    fn errors_when_cover_contains_word_longer_than_pivot() -> Result<(), Box<dyn Error>> {
        let data_input: Vec<u8> = vec![0b11111111];
        let mut data_iterator = BitIterator::new(&data_input);
        let mut method = get_method(2, Variant::V1);

        let stego_text = method.try_conceal(WITH_WORDS_TEXT, &mut data_iterator);

        assert_eq!(stego_text, Err(MethodError::pivot_too_small("little".to_string(), 2)));
        Ok(())
    }

    #[test]
    fn errors_when_cover_is_too_small() -> Result<(), Box<dyn Error>> {
        let data_input: Vec<u8> = vec![0b11111111];
        let mut data_iterator = BitIterator::new(&data_input);
        let mut method = get_method(5, Variant::V3);

        let stego_text = method.try_conceal(TINY_TEXT, &mut data_iterator);

        assert_eq!(stego_text, Err(MethodError::cover_text_too_small(5)));
        Ok(())
    }

    #[test]
    fn errors_when_cover_is_empty() -> Result<(), Box<dyn Error>> {
        let data_input: Vec<u8> = vec![0b11111111];
        let mut data_iterator = BitIterator::new(&data_input);
        let mut method = get_method(5, Variant::V3);

        let stego_text = method.try_conceal(EMPTY_TEXT, &mut data_iterator);

        assert_eq!(stego_text, Err(MethodError::cover_text_too_small(8)));
        Ok(())
    }
}