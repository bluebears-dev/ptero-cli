use std::{convert::TryFrom, error::Error, fs};

use clap::Clap;
use log::{debug, trace};
use spinners::{Spinner, Spinners};

use crate::{binary::{Bit, BitVec}, context::{Context, PivotDecoderContext}, decoder::Decoder, method::complex::extended_line::ExtendedLineMethod};

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
    let decoder = ExtendedLineMethod::new();
    let mut context = PivotDecoderContext::new(cover_text.as_str(), args.pivot);

    // TODO: Add Decodable trait
    let mut secret = Vec::default();
    debug!("Decoding secret from the text");
    while let Ok(line) = context.load_line() {
        trace!("Line {}", line);
        let mut data = decoder.decode(&context)?;
        secret.append(&mut data);
    }
    debug!("Padding bits to byte size boundary");
    while &secret.len() % 8 != 0 {
        secret.push(Bit(0));
    }
    sp.stop();
    println!();
    debug!("Converting bits to bytes");
    let bit_vec: BitVec = secret.into();
    let bytes: Vec<u8> = TryFrom::try_from(bit_vec)?;
    Ok(bytes)
}
