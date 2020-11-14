use std::{error::Error, fs};

use clap::Clap;
use colored::Colorize;
use log::{error, info, warn};
use spinners::{Spinner, Spinners};

use crate::{binary::BitIterator, encodable::Encodable};

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

    /// Pivot i.e. line length used for extended line algorithm.binary
    /// 
    /// If omitted, program will determine minimum pivot that can be used.
    #[clap(long)]
    pivot: Option<usize>
}

fn determine_pivot_size<'a>(words: impl Iterator<Item = &'a str>) -> usize {
    words.into_iter().map(|w| w.len()).max().unwrap_or(0)
}

pub fn encode_command(args: EncodeSubCommand) -> Result<Vec<u8>, Box<dyn Error>>{
    let cover_text = fs::read_to_string(args.cover)?;
    let data = fs::read(args.data)?;
    let mut pivot = determine_pivot_size(cover_text.split_whitespace());

    if let Some(user_pivot) = args.pivot {
        if user_pivot < pivot {
            error!("Provided pivot is smaller than the largest word in text! Cannot guarantee encoding will succeed.");
            return Err("stub".into());
        }
        pivot = user_pivot;
    }

    warn!(
        "Required cover text capacity: {}",
        BitIterator::new(&data).count()
    );
    info!("Using pivot: {}", pivot);

    let sp = Spinner::new(Spinners::Dots12, "Encoding the data".into());
    let stego_result = data.encode(&cover_text, pivot);
    sp.stop();
    println!();
    Ok(stego_result?.as_bytes().into())
}