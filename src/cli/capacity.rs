use clap::Clap;
use log::{error, info, warn};
use std::{error::Error, fs};

use crate::{context::{Context, PivotByLineContext}, encoder::{Capacity}, method::complex::{eluv::ELUVMethod, extended_line::ExtendedLineMethod}};

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

    /// Use ELUV method for encoding.
    #[clap(long, group = "method_args")]
    eluv: bool,

    /// Use Extended Line method for encoding.
    #[clap(long = "eline", group = "method_args")]
    #[allow(dead_code)]
    extended_line: bool,
}

pub fn get_method(eluv: bool) -> Box<dyn Capacity> {
    if eluv {
        Box::new(ELUVMethod::default())
    } else {
        Box::new(ExtendedLineMethod::default())
    }
}


pub fn get_cover_text_capacity(args: GetCapacityCommand) -> Result<u32, Box<dyn Error>> {
    let cover_text = fs::read_to_string(args.cover)?;
    let mut pivot_word_context = PivotByLineContext::new(&cover_text, args.pivot);
    let mut text_fragment_count = 0;

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

    info!("Calculating the capacity");
    while let Ok(fragment) = pivot_word_context.load_text() {
        if fragment.is_empty() {
            error!("Pivot is too small, stopping");
            return Err("Could not determine the capacity for the given cover text".into());
        }
        text_fragment_count += 1;
    }
    let method = get_method(args.eluv);
    Ok(text_fragment_count * method.bitrate() as u32)
}
