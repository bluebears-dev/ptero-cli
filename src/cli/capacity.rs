use clap::Clap;
use log::{error, info, warn};
use std::{cell::RefCell, error::Error, fs, rc::Rc};

use crate::{
    encoder::{complex_encoder::extended_line_encoder::ExtendedLineEncoderFactory, Encoder},
    text::{LineByPivotIterator, WordIterator},
};

use super::encoder::determine_pivot_size;

/// Calculate the minimal capacity for the cover text and given pivot
#[derive(Clap)]
pub struct GetCapacityCommand {
    /// Path to cover text.
    #[clap(short, long)]
    cover: String,

    /// Pivot i.e. line length.
    #[clap(short, long)]
    pivot: usize,
}

pub fn get_cover_text_capacity(args: GetCapacityCommand) -> Result<u32, Box<dyn Error>> {
    let cover_text = fs::read_to_string(args.cover)?;
    let mut text_iterator = LineByPivotIterator::new(&cover_text, args.pivot);
    let mut lines_count = 0;

    let max_word_length = determine_pivot_size(cover_text.split_whitespace());
    let text_length = cover_text
        .split_whitespace()
        .map(|string| string.chars())
        .flatten()
        .count();
    info!("Longest word in the cover text is {}", max_word_length);

    if max_word_length > args.pivot {
        warn!("This pivot might not guarantee the secret data will be encodable!");
    } else if args.pivot >= text_length {
        error!("Pivot greater than the cover text length, stopping");
        return Err("Could not determine the capacity for the given cover text".into());
    }

    while let Some(line) = text_iterator.next() {
        if line.is_empty() {
            error!("Pivot is too small, stopping");
            return Err("Could not determine the capacity for the given cover text".into());
        }
        text_iterator.next_word();
        lines_count += 1;
    }
    let placeholder_ref = Rc::new(RefCell::new(text_iterator));
    let encoder = ExtendedLineEncoderFactory::build(placeholder_ref.borrow_mut());
    Ok(lines_count * encoder.rate())
}