use std::{error::Error, fs};

use clap::Clap;
use log::debug;
use spinners::{Spinner, Spinners};

use crate::{
    binary::{convert_to_bytes, Bit},
    decoder::{complex::extended_line_decoder::ExtendedLineDecoder, Decoder},
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
}

pub fn decode_command(args: DecodeSubCommand) -> Result<Vec<u8>, Box<dyn Error>> {
    let sp = Spinner::new(Spinners::Dots12, "Decoding the secret".into());
    let cover_text = fs::read_to_string(args.text)?;
    let decoder = ExtendedLineDecoder::new(args.pivot);

    // TODO: Add Decodable trait
    let mut secret = Vec::default();
    for line in cover_text.lines() {
        let mut data = decoder.decode(line);
        secret.append(&mut data);
    }
    debug!("Padding bits to byte size boundary");
    while &secret.len() % 8 != 0 {
        secret.push(Bit(0));
    }
    sp.stop();
    println!();
    debug!("Converting bits to bytes");
    let bytes = convert_to_bytes(&secret)?;
    Ok(bytes)
}
