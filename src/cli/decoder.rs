use std::{error::Error, fs};

use clap::Clap;

use crate::{
    context::PivotByRawLineContext,
    decoder::Decoder,
    method::complex::{eluv::ELUVMethod, extended_line::ExtendedLineMethod},
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

    Ok(decoder.decode(&mut context)?)
}
