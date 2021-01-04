use std::{error::Error, fs, sync::mpsc::channel};

use clap::Clap;

use crate::{
    context::PivotByRawLineContext,
    decoder::Decoder,
    method::complex::{eluv::ELUVMethod, extended_line::ExtendedLineMethod},
};

use super::progress::{ProgressStatus, new_progress_bar, spawn_progress_thread};

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
    #[clap(long, group = "method_args")]
    eluv: bool,
    
    /// Use Extended Line method for encoding.
    #[clap(long = "eline", group = "method_args")]
    #[allow(dead_code)]
    extended_line: bool,
}

pub fn get_method(eluv: bool) -> Box<dyn Decoder<PivotByRawLineContext>> {
    if eluv {
        Box::new(ELUVMethod::default())
    } else {
        Box::new(ExtendedLineMethod::default())
    }
}

pub fn decode_command(args: DecodeSubCommand) -> Result<Vec<u8>, Box<dyn Error>> {
    let cover_text = fs::read_to_string(args.text)?;
    let decoder = get_method(args.eluv);
    let mut context = PivotByRawLineContext::new(cover_text.as_str(), args.pivot);

    let progress_bar = new_progress_bar(cover_text.len() as u64);
    let (tx, rx) = channel::<ProgressStatus>();
    progress_bar.set_message("Decoding cover text...");
    spawn_progress_thread(progress_bar.clone(), rx);

    let result =  decoder.decode(&mut context, Some(&tx));
    
    tx.send(ProgressStatus::Finished).ok();
    progress_bar.finish_with_message("Finished decoding");

    result
}
