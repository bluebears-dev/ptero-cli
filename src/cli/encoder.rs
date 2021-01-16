use std::{convert::TryInto, error::Error, fs::File, io::Read, sync::mpsc::channel};

use clap::Clap;
use log::{info, trace};

use crate::{
    binary::BitIterator,
    context::PivotByLineContext,
    encoder::Encoder,
    method::{
        complex::{eluv::ELUVMethodBuilder, extended_line::ExtendedLineMethodBuilder},
        trailing_unicode::character_sets::CharacterSetType,
    },
};

use super::{
    progress::{new_progress_bar, spawn_progress_thread, ProgressStatus},
    writer::Writer,
};

#[derive(Clap, Debug, PartialEq)]
pub enum ELUVCharacterSet {
    FourBit,
    ThreeBit,
    TwoBit,
    Full,
    Twitter,
}

/// Encode the secret into given cover text
#[derive(Clap)]
pub struct EncodeSubCommand {
    /// Path to cover text used to encoding.
    ///
    /// Please note that original whitespace may not be preserved!
    #[clap(short, long)]
    cover: String,

    /// Path to secret data file which will be encoded.
    #[clap(short, long)]
    data: String,

    /// Pivot i.e. line length used for extended line algorithm.
    ///
    /// If omitted, program will determine minimum pivot that can be used.
    #[clap(long)]
    pivot: Option<usize>,

    /// Use ELUV method for encoding.
    ///
    /// The ELUV method is a combination of three smaller encoders.
    /// Random Whitespace - which puts randomly double whitespace between words,
    /// Line Extend - which uses pivot to determine the size of the line,
    /// Trailing Unicode - which puts one of the predefined Unicode invisible chars
    /// at the end of the line during encoding.
    ///
    /// It can encode 7 bits in one pass.
    /// This method has 3 variant.
    #[clap(long, group = "method_args")]
    eluv: bool,

    /// Override a default set - can only be used with ELUV method!
    ///
    /// Provides a different set for the ELUV command to use.
    /// Please note, that it may change the method's bitrate!
    #[clap(long, arg_enum, requires = "eluv")]
    set: Option<ELUVCharacterSet>,

    /// Variant of the method. See concrete method for possible values.
    ///
    /// Variant is a permutation of methods that can be used during encoding.
    #[clap(long, default_value = "1")]
    variant: u8,

    /// Use Extended Line method for encoding.
    ///
    /// The Extended Line method is a combination of three smaller encoders.
    /// Random Whitespace - which puts randomly double whitespace between words,
    /// Line Extend - which uses pivot to determine the size of the line,
    /// Trailing Whitespace - which puts whitespace at the end of the line during encoding.
    ///
    /// It can encode 3 bits in one pass. Relies purely on ASCII characters.
    /// This method has 3 variant.
    #[clap(long = "eline", group = "method_args")]
    #[allow(dead_code)]
    extended_line: bool,
}

pub fn validate_pivot_smaller_than_text(
    pivot: usize,
    cover_text: &str,
) -> Result<(), Box<dyn Error>> {
    let text_length = cover_text.len();

    if pivot > text_length {
        return Err("Pivot is greater than the cover text length.".into());
    }
    Ok(())
}

impl EncodeSubCommand {
    pub fn run(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let cover_file_input = File::open(&self.cover)?;
        let data_file_input = File::open(&self.data)?;

        self.do_encode(cover_file_input, data_file_input)
    }

    pub(crate) fn do_encode(
        &self,
        mut cover_input: impl Read,
        mut data_input: impl Read,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut cover_text = String::new();
        let mut data = vec![];

        cover_input.read_to_string(&mut cover_text)?;
        data_input.read_to_end(&mut data)?;

        trace!("text: {:?}", data);

        let pivot = pick_pivot_from(
            self.pivot,
            determine_pivot_size(cover_text.split_whitespace()),
        )?;

        validate_pivot_smaller_than_text(pivot, &cover_text)?;

        let capacity_msg = format!(
            "Required cover text capacity: {}",
            BitIterator::new(&data).count()
        );
        Writer::warn(&capacity_msg);
        info!("Encoding secret data");

        let mut data_iterator = BitIterator::new(&data);

        let progress_bar = new_progress_bar(data_iterator.count() as u64);
        let (tx, rx) = channel::<ProgressStatus>();
        progress_bar.set_message("Encoding..");
        spawn_progress_thread(progress_bar.clone(), rx);

        let method = self.get_method()?;
        info!("Using method variant {}", self.variant);
        let mut context = PivotByLineContext::new(&cover_text, pivot);
        let stego_result = method.encode(&mut context, &mut data_iterator, Some(&tx));

        tx.send(ProgressStatus::Finished).ok();
        progress_bar.finish_with_message("Finished encoding");

        Ok(stego_result?.as_bytes().into())
    }

    pub(crate) fn get_method(
        &self,
    ) -> Result<Box<dyn Encoder<PivotByLineContext>>, Box<dyn Error>> {
        Ok(if self.eluv {
            Box::new(
                ELUVMethodBuilder::new()
                    .character_set(get_character_set_type(&self.set))
                    .variant(self.variant.try_into()?)
                    .build(),
            )
        } else {
            Box::new(
                ExtendedLineMethodBuilder::new()
                    .variant(self.variant.try_into()?)
                    .build(),
            )
        })
    }
}

pub fn get_character_set_type(set_option: &Option<ELUVCharacterSet>) -> CharacterSetType {
    if let Some(char_set) = set_option {
        match char_set {
            ELUVCharacterSet::FourBit => CharacterSetType::FourBitUnicodeSet,
            ELUVCharacterSet::ThreeBit => CharacterSetType::ThreeBitUnicodeSet,
            ELUVCharacterSet::TwoBit => CharacterSetType::TwoBitUnicodeSet,
            ELUVCharacterSet::Full => CharacterSetType::FullUnicodeSet,
            ELUVCharacterSet::Twitter => CharacterSetType::TwitterUnicodeSet,
        }
    } else {
        CharacterSetType::FullUnicodeSet
    }
}

pub(crate) fn determine_pivot_size<'a>(words: impl Iterator<Item = &'a str>) -> usize {
    words
        .into_iter()
        .map(|string| string.chars().count() + 1)
        .max()
        .unwrap_or(0)
}

pub(crate) fn pick_pivot_from(
    user_pivot: Option<usize>,
    calculated_pivot: usize,
) -> Result<usize, Box<dyn Error>> {
    Ok(if let Some(value) = user_pivot {
        if value < calculated_pivot {
            return Err("Provided pivot is smaller than the largest word in text! Cannot guarantee encoding will succeed.".into());
        }
        Writer::info(&format!("Using user provided pivot: {}", value));
        value
    } else {
        Writer::info(&format!(
            "Using pivot based on the cover text: {}",
            calculated_pivot
        ));
        calculated_pivot
    })
}

#[allow(unused_imports)]
mod test {
    use std::{error::Error, io::Read};

    use crate::method::trailing_unicode::character_sets::CharacterSetType;

    use super::{get_character_set_type, ELUVCharacterSet, EncodeSubCommand};

    #[test]
    fn fails_when_there_is_not_enough_cover_text() -> Result<(), Box<dyn Error>> {
        let cover_input = "a b c ".repeat(2);
        let data_input: Vec<u8> = vec![0b11111111];

        let command = EncodeSubCommand {
            cover: "stub".into(),
            data: "stub".into(),
            pivot: Some(3),
            eluv: false,
            extended_line: true,
            set: None,
            variant: 1,
        };

        let result = command.do_encode(cover_input.as_bytes(), data_input.as_slice());
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn fails_when_pivot_is_too_small() -> Result<(), Box<dyn Error>> {
        let cover_input = "aaaaa ".repeat(2);
        let data_input: Vec<u8> = vec![0b11111111];

        let command = EncodeSubCommand {
            cover: "stub".into(),
            data: "stub".into(),
            pivot: Some(3),
            eluv: false,
            extended_line: true,
            set: None,
            variant: 1,
        };

        let result = command.do_encode(cover_input.as_bytes(), data_input.as_slice());
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn fails_when_pivot_is_too_large() -> Result<(), Box<dyn Error>> {
        let cover_input = "aaaaa";
        let data_input: Vec<u8> = vec![0b11111111];

        let command = EncodeSubCommand {
            cover: "stub".into(),
            data: "stub".into(),
            pivot: Some(6),
            eluv: false,
            extended_line: true,
            set: None,
            variant: 1,
        };

        let result = command.do_encode(cover_input.as_bytes(), data_input.as_slice());
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn get_character_set_type_returns_default_when_none_is_provided() -> Result<(), Box<dyn Error>>
    {
        assert_eq!(
            get_character_set_type(&None),
            CharacterSetType::FullUnicodeSet
        );
        Ok(())
    }

    #[test]
    fn get_character_set_type_maps_sets_correctly() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            get_character_set_type(&Some(ELUVCharacterSet::Full)),
            CharacterSetType::FullUnicodeSet
        );
        assert_eq!(
            get_character_set_type(&Some(ELUVCharacterSet::FourBit)),
            CharacterSetType::FourBitUnicodeSet
        );
        assert_eq!(
            get_character_set_type(&Some(ELUVCharacterSet::ThreeBit)),
            CharacterSetType::ThreeBitUnicodeSet
        );
        assert_eq!(
            get_character_set_type(&Some(ELUVCharacterSet::TwoBit)),
            CharacterSetType::TwoBitUnicodeSet
        );
        Ok(())
    }
}
