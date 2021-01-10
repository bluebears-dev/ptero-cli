use clap::Clap;
use context::ContextErrorKind;
use log::debug;
use std::{error::Error, fs::{File}, io::Read, sync::mpsc::channel};

use crate::{
    context::{self, Context, PivotByLineContext},
    encoder::Capacity,
    method::complex::{eluv::ELUVMethod, extended_line::ExtendedLineMethod},
};

use super::{
    encoder::determine_pivot_size,
    progress::{new_progress_bar, spawn_progress_thread, ProgressStatus},
    writer::Writer,
};

/// Calculate the minimal capacity for the cover text and given pivot
#[derive(Clap)]
pub struct GetCapacityCommand {
    /// Path to cover text.
    #[clap(short, long)]
    cover: String,

    /// Pivot i.e. line length.
    #[clap(short, long)]
    pivot: usize,

    /// Use ELUV method for encoding.
    #[clap(long, group = "method_args")]
    eluv: bool,

    /// Use Extended Line method for encoding.
    #[clap(long = "eline", group = "method_args")]
    #[allow(dead_code)]
    extended_line: bool,
}

impl GetCapacityCommand {
    pub fn run(&self) -> Result<u32, Box<dyn Error>> {
        let cover_file_input = File::open(&self.cover)?;

        self.get_cover_text_capacity(cover_file_input)
    }

    pub(crate) fn get_cover_text_capacity(
        &self,
        mut cover_input: impl Read,
    ) -> Result<u32, Box<dyn Error>> {
        let mut cover_text = String::new();

        cover_input.read_to_string(&mut cover_text)?;
        let mut pivot_word_context = PivotByLineContext::new(&cover_text, self.pivot);
        let mut text_fragment_count = 0;

        let max_word_length = determine_pivot_size(cover_text.split_whitespace());
        let text_length = cover_text
            .split_whitespace()
            .map(|string| string.chars())
            .flatten()
            .count();
        debug!("Longest word in the cover text is {}", max_word_length);

        if max_word_length > self.pivot {
            Writer::warn("This pivot might not guarantee the secret data will be encodable!");
        } else if self.pivot >= text_length {
            return Err("Pivot is greater than the cover text length.".into());
        }

        let progress_bar = new_progress_bar(cover_text.len() as u64);
        let (tx, rx) = channel::<ProgressStatus>();
        progress_bar.set_message("Calculating the capacity...");
        spawn_progress_thread(progress_bar.clone(), rx);

        loop {
            let result = pivot_word_context.load_text();

            match result {
                Ok(fragment) => {
                    tx.send(ProgressStatus::Step(fragment.len() as u64)).ok();
                    text_fragment_count += 1;
                }
                Err(error) => match error.kind() {
                    ContextErrorKind::CannotConstructLine => {
                        tx.send(ProgressStatus::Finished).ok();
                        progress_bar.abandon_with_message("Error occurred");
                        return Err(error.into());
                    }
                    ContextErrorKind::NoTextLeft => {
                        tx.send(ProgressStatus::Finished).ok();
                        progress_bar.finish_with_message("Capacity calculated");
                        break;
                    }
                },
            }

            pivot_word_context.next_word();
        }

        let method = self.get_method();
        Ok(text_fragment_count * method.bitrate() as u32)
    }

    pub(crate) fn get_method(&self) -> Box<dyn Capacity> {
        if self.eluv {
            Box::new(ELUVMethod::default())
        } else {
            Box::new(ExtendedLineMethod::default())
        }
    }
}



#[allow(unused_imports)]
mod test {
    use std::{error::Error, io::Read};

    use super::GetCapacityCommand;

    #[test]
    fn returns_capacity_for_given_method() -> Result<(), Box<dyn Error>> {
        let cover_input = "a b c ".repeat(2);

        let command = GetCapacityCommand {
            cover: "stub".into(),
            pivot: 3,
            eluv: false,
            extended_line: true,
        };

        let result = command.get_cover_text_capacity(cover_input.as_bytes());
        assert_eq!(result.ok(), Some(6 as u32));
        Ok(())
    }
}
