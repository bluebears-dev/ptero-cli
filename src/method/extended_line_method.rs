use std::error::Error;
use std::iter::Peekable;
use std::ops::{Deref, DerefMut};
use std::sync::mpsc::Sender;
use log::{error, trace};
use rand::{Rng, RngCore};
use unicode_segmentation::{UnicodeSegmentation, UWordBounds};
use crate::binary::Bit;
use crate::encoder::{EncoderResult, EncodingError};
use crate::method::config::{CommonMethodConfig, CommonMethodConfigBuilder, MethodProgressStatus};
use crate::method::StegoMethod;

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

pub(crate) fn get_variant_methods(variant: &Variant) -> &'static [MethodActions; 3] {
    &[MethodActions::LineExtend, MethodActions::RandomASCIIWhitespace, MethodActions::TrailingASCIIWhitespace]
}

pub struct ExtendedLineMethod<'a> {
    pivot: usize,
    variant: Variant,
    config: CommonMethodConfig<'a>,
}

impl<'a> ExtendedLineMethod<'a> {
    pub fn builder() -> ExtendedLineMethodBuilder<'a> {
        ExtendedLineMethodBuilder::default()
    }
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

impl<'a> ExtendedLineMethod<'a> {
    pub(crate) fn hide_in_extend_line<I: Iterator<Item=&'a str>>(
        &self,
        word_iter: &mut Peekable<I>,
        data: &mut dyn Iterator<Item=Bit>,
        result: &mut String,
    ) -> Result<EncoderResult, Box<dyn Error>> {
        let mut current_line_length = 0;

        loop {
            let next_word = *word_iter.peek().ok_or_else(EncodingError::no_words_error)?;
            let word_size = graphemes_length(next_word);

            if current_line_length + word_size > self.pivot {
                trace!("Constructed line of length: '{}'", current_line_length);
                break;
            }

            if current_line_length > 0 {
                result.push_str(ASCII_DELIMITER);
                current_line_length += graphemes_length(ASCII_DELIMITER);
            }

            current_line_length += word_size;
            result.push_str(next_word);

            word_iter.next();
        }

        Ok(match data.next() {
            Some(Bit(1)) => {
                let next_word = word_iter.next().ok_or_else(EncodingError::no_words_error)?;
                trace!("Extending line with '{}'", &next_word);

                result.push_str(ASCII_DELIMITER);
                result.push_str(next_word);
                EncoderResult::Success
            }
            None => EncoderResult::NoDataLeft,
            _ => {
                trace!("Leaving line as-is");
                EncoderResult::Success
            }
        })
    }

    pub(crate) fn hide_in_random_ascii_whitespace(
        &mut self,
        data: &mut dyn Iterator<Item=Bit>,
        cover: &mut String,
    ) -> Result<EncoderResult, Box<dyn Error>> {
        Ok(match data.next() {
            Some(Bit(1)) => {
                let last_newline_index = cover.rfind(NEWLINE_STR).unwrap_or(0);

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
                println!("Putting space at position {} with approx. {}", position, approx_position);
                cover.insert_str(position, &String::from(ASCII_DELIMITER));
                EncoderResult::Success
            }
            None => EncoderResult::NoDataLeft,
            _ => {
                trace!("Skipping double whitespace");
                EncoderResult::Success
            }
        })
    }

    pub(crate) fn hide_in_trailing_ascii_whitespace(
        &self,
        data: &mut dyn Iterator<Item=Bit>,
        cover: &mut String,
    ) -> Result<EncoderResult, Box<dyn Error>> {
        Ok(match data.next() {
            Some(Bit(1)) => {
                trace!("Putting whitespace at the end of the line");
                cover.push_str(ASCII_DELIMITER);
                EncoderResult::Success
            }
            None => EncoderResult::NoDataLeft,
            _ => {
                trace!("Skipping trailing whitespace");
                EncoderResult::Success
            }
        })
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

impl<'a> StegoMethod<ExtendedLineMethod<'a>> for String {
    fn hide(&self, data: &mut dyn Iterator<Item=Bit>, method: &mut ExtendedLineMethod<'a>) -> Result<String, Box<dyn Error>> {
        let mut result = String::with_capacity(self.len());
        let actions = get_variant_methods(&method.variant);

        let mut word_iterator = self.split_whitespace()
            .filter(|word| !word.contains(char::is_whitespace))
            .peekable();

        'outer: loop {
            for action in actions {
                let method_result = match action {
                    MethodActions::LineExtend => {
                        method.hide_in_extend_line(&mut word_iterator, data, &mut result)
                    }
                    MethodActions::RandomASCIIWhitespace => {
                        method.hide_in_random_ascii_whitespace(data, &mut result)
                    }
                    MethodActions::TrailingASCIIWhitespace => {
                        method.hide_in_trailing_ascii_whitespace(data, &mut result)
                    }
                };
                if let Ok(EncoderResult::NoDataLeft) = method_result {
                    break 'outer;
                }
            };
            result.push_str(NEWLINE_STR);
        }

        while word_iterator.peek().is_some() {
            result.push_str(NEWLINE_STR);
            method.hide_in_extend_line(&mut word_iterator, data, &mut result).ok();
        }

        Ok(result)
    }

    fn reveal(&self, method: ExtendedLineMethod<'a>) -> Result<Vec<Bit>, Box<dyn Error>> {
        todo!()
    }
}

fn graphemes_length(text: &str) -> usize {
    text.graphemes(true).count()
}

#[allow(unused_imports)]
mod test {
    use std::error::Error;
    use rand::Rng;
    use rand::rngs::mock::StepRng;
    use crate::binary::BitIterator;
    use crate::method::extended_line_method::ExtendedLineMethod;
    use crate::method::extended_line_method::Variant;
    use crate::method::StegoMethod;

    #[test]
    fn encodes_text_data() -> Result<(), Box<dyn Error>> {
        let cover = "a b c".repeat(5);
        let data_input = "a";
        let pivot: usize = 4;

        let mut data_iterator = BitIterator::new(data_input.as_bytes());
        let mut method = ExtendedLineMethod::builder()
            .with_pivot(pivot)
            .with_rng(Box::new(StepRng::new(1, 1)))
            .with_variant(Variant::V1)
            .build();

        let stego_text = cover.hide(&mut data_iterator, &mut method)?;

        assert_eq!(stego_text, "a  b \nca b\nca  b\nca b\nca b\nc");
        Ok(())
    }

    #[test]
    fn encodes_binary_data() -> Result<(), Box<dyn Error>> {
        let cover = "a b c ".repeat(5);
        let data_input: Vec<u8> = vec![0b11111111];
        let pivot: usize = 3;

        let mut data_iterator = BitIterator::new(&data_input);
        let mut method = ExtendedLineMethod::builder()
            .with_pivot(pivot)
            .with_rng(Box::new(StepRng::new(1, 1)))
            .with_variant(Variant::V1)
            .build();

        let stego_text = cover.hide(&mut data_iterator, &mut method)?;

        assert_eq!(stego_text, "a  b c \na  b c \na  b c\na b\nc a\nb c");
        Ok(())
    }

    #[test]
    fn encodes_data_in_cover_with_words() -> Result<(), Box<dyn Error>> {
        let cover = "A little panda has fallen from a tree".to_string();
        let data_input: Vec<u8> = vec![0b11111111];
        let pivot: usize = 10;

        let mut data_iterator = BitIterator::new(&data_input);
        let mut method = ExtendedLineMethod::builder()
            .with_pivot(pivot)
            .with_rng(Box::new(StepRng::new(1, 1)))
            .with_variant(Variant::V1)
            .build();

        let stego_text = cover.hide(&mut data_iterator, &mut method)?;

        assert_eq!(stego_text, "A  little panda \nhas  fallen from \na  tree \n");
        Ok(())
    }

    #[test]
    fn encodes_data_in_cover_with_special_chars() -> Result<(), Box<dyn Error>> {
        let cover = "A little 🐼 has (fallen) from a \\🌳/".to_string();
        let data_input: Vec<u8> = vec![0b11111111];
        let pivot: usize = 10;

        let mut data_iterator = BitIterator::new(&data_input);
        let mut method = ExtendedLineMethod::builder()
            .with_pivot(pivot)
            .with_rng(Box::new(StepRng::new(1, 1)))
            .with_variant(Variant::V1)
            .build();

        let stego_text = cover.hide(&mut data_iterator, &mut method)?;

        assert_eq!(stego_text, "A  little 🐼 has \n(fallen)  from \na  \\🌳/ \n");
        Ok(())
    }


    #[test]
    fn encodes_data_in_html_cover() -> Result<(), Box<dyn Error>> {
        let cover = "<div>\n<button style=\"background-color: red;\">Click me</button>\n<div/>".to_string();
        let data_input: Vec<u8> = vec![0b11111111];
        let pivot: usize = 15;

        let mut data_iterator = BitIterator::new(&data_input);
        let mut method = ExtendedLineMethod::builder()
            .with_pivot(pivot)
            .with_rng(Box::new(StepRng::new(1, 1)))
            .with_variant(Variant::V1)
            .build();

        let stego_text = cover.hide(&mut data_iterator, &mut method)?;

        assert_eq!(stego_text, "<div>  <button style=\"background-color: \nred;\">Click  me</button> \n<div/>  \n");
        Ok(())
    }
}