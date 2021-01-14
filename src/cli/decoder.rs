use std::{convert::TryInto, error::Error, fs::File, io::Read, sync::mpsc::channel};

use clap::Clap;
use log::info;

use crate::{
    context::PivotByRawLineContext,
    decoder::Decoder,
    method::complex::{eluv::ELUVMethodBuilder, extended_line::ExtendedLineMethodBuilder},
};

use super::{
    encoder::{get_character_set_type, ELUVCharacterSet},
    progress::{new_progress_bar, spawn_progress_thread, ProgressStatus},
};

/// Decode secret from the stegotext
#[derive(Clap)]
pub struct DecodeSubCommand {
    /// Path to stegotext from which data will be decoded
    #[clap(short, long)]
    text: String,

    /// Pivot i.e. line length used to encode with extended line algorithm
    #[clap(short, long)]
    pivot: usize,

    /// Use ELUV method for encoding.
    ///
    /// This method has 3 variants.
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
    /// Variant is a permutation of methods that can be used during decoding.
    #[clap(long, default_value = "1")]
    variant: u8,

    /// Use Extended Line method for encoding.
    /// 
    /// This method has 3 variants.
    #[clap(long = "eline", group = "method_args")]
    #[allow(dead_code)]
    extended_line: bool,
}

impl DecodeSubCommand {
    pub fn run(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let stego_text_file_input = File::open(&self.text)?;

        self.do_decode(stego_text_file_input)
    }

    pub(crate) fn get_method(
        &self,
    ) -> Result<Box<dyn Decoder<PivotByRawLineContext>>, Box<dyn Error>> {
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

    pub fn do_decode(&self, mut stego_input: impl Read) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut stego_text = String::new();

        stego_input.read_to_string(&mut stego_text)?;
        let decoder = self.get_method()?;
        info!("Using method variant {}", self.variant);
        let mut context = PivotByRawLineContext::new(stego_text.as_str(), self.pivot);

        let progress_bar = new_progress_bar(stego_text.len() as u64);
        let (tx, rx) = channel::<ProgressStatus>();
        progress_bar.set_message("Decoding cover text...");
        spawn_progress_thread(progress_bar.clone(), rx);

        let result = decoder.decode(&mut context, Some(&tx));

        tx.send(ProgressStatus::Finished).ok();
        progress_bar.finish_with_message("Finished decoding");

        result
    }
}

#[allow(unused_imports)]
mod test {
    use crate::binary::Bit;
    use std::{error::Error, io::Read};

    use super::DecodeSubCommand;

    #[test]
    fn decodes_zeroes_if_not_data_encoded_extended_line() -> Result<(), Box<dyn Error>> {
        let stego_input = "a b";

        let command = DecodeSubCommand {
            text: "stub".into(),
            pivot: 3,
            eluv: false,
            extended_line: true,
            set: None,
            variant: 1,
        };

        let result = command.do_decode(stego_input.as_bytes());
        assert_eq!(result.ok(), Some(vec![0]));
        Ok(())
    }

    #[test]
    fn decodes_zeroes_if_not_data_encoded_eluv() -> Result<(), Box<dyn Error>> {
        let stego_input = "a b";

        let command = DecodeSubCommand {
            text: "stub".into(),
            pivot: 3,
            eluv: true,
            extended_line: false,
            set: None,
            variant: 1,
        };

        let result = command.do_decode(stego_input.as_bytes());
        assert_eq!(result.ok(), Some(vec![0]));
        Ok(())
    }
}
