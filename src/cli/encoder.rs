use std::{error::Error, fs::File, io::Read, sync::mpsc::channel};

use clap::Clap;
use log::{info, trace};

use crate::{
    binary::BitIterator,
    context::PivotByLineContext,
    encoder::Encoder,
    method::complex::{eluv::ELUVMethod, extended_line::ExtendedLineMethod},
};

use super::{
    progress::{new_progress_bar, spawn_progress_thread, ProgressStatus},
    writer::Writer,
};

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
    /// Random Whitespace - which puts randomly double whitepsace between words,
    /// Line Extend - which uses pivot to determine the size of the line,
    /// Trailing Unicode - which puts one of the predefined Unicode invisible chars
    /// at the end of the line during encoding.
    ///
    /// It can encode 7 bits in one pass.
    #[clap(long, group = "method_args")]
    eluv: bool,

    /// Use Extended Line method for encoding.
    ///
    /// The Extended Line method is a combination of three smaller encoders.
    /// Random Whitespace - which puts randomly double whitepsace between words,
    /// Line Extend - which uses pivot to determine the size of the line,
    /// Trailing Whitespace - which puts whitespace at the end of the line during encoding.
    ///
    /// It can encode 3 bits in one pass. Relies purely on ASCII characters.
    #[clap(long = "eline", group = "method_args")]
    #[allow(dead_code)]
    extended_line: bool,
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

        let method = self.get_method();
        let mut context = PivotByLineContext::new(&cover_text, pivot);
        let stego_result = method.encode(&mut context, &mut data_iterator, Some(&tx));

        tx.send(ProgressStatus::Finished).ok();
        progress_bar.finish_with_message("Finished encoding");

        Ok(stego_result?.as_bytes().into())
    }

    pub(crate) fn get_method(&self) -> Box<dyn Encoder<PivotByLineContext>> {
        if self.eluv {
            Box::new(ELUVMethod::default())
        } else {
            Box::new(ExtendedLineMethod::default())
        }
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

    use super::EncodeSubCommand;

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
        };

        let result = command.do_encode(cover_input.as_bytes(), data_input.as_slice());
        assert!(result.is_err());
        Ok(())
    }
}
