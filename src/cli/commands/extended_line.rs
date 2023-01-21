use std::{
    cell::RefCell,
    fs::{read_to_string, File},
    io::Read,
    path::PathBuf,
    sync::Arc,
};

use bitvec::prelude::*;
use bitvec::view::BitView;
use clap::builder::TypedValueParser;
use ptero_common::method::SteganographyMethod;
use ptero_text::{
    extended_line_method::{
        character_sets::CharacterSetType, ExtendedLineMethod, ExtendedLineMethodBuilder, Variant,
    },
    line_separator::LineSeparatorType,
};
use rand::{rngs::StdRng, SeedableRng};
use std::error::Error;

use crate::cli::progress::ProgressBarObserver;

#[derive(clap::Args, Debug)]
struct ExtendedLineCommand {
    #[clap(subcommand)]
    action: MethodActions,

    /// Variant describes order of sub-methods which will be used.
    ///
    ///
    /// V1: Line Extension, Random Whitespace, Trailing Whitespace
    ///
    /// V2: Line Extension, Trailing Whitespace, Random Whitespace
    ///
    /// V3: Random Whitespace, Line Extension, Trailing Whitespace    
    #[arg(long, default_value = "v1", value_parser = clap::builder::PossibleValuesParser::new(["v1", "v2", "v3"])
        .map(|s| s.parse::<Variant>().unwrap())
    )]
    variant: Option<Variant>,

    /// Maximal line length to use when using during hiding.
    ///
    /// Impacts the method's capacity. Used for Line Extension sub-method
    #[arg(long)]
    pivot: Option<usize>,

    /// Line separator which will be used during data hiding.
    ///
    /// Used for Line Extension sub-method.
    #[arg(long, value_parser = clap::builder::PossibleValuesParser::new(["windows", "unix"])
        .map(|s| s.parse::<LineSeparatorType>().unwrap())
    )]
    line_separator: Option<LineSeparatorType>,
}

impl ExtendedLineCommand {
    fn builder(&self) -> ExtendedLineMethodBuilder {
        let mut builder = ExtendedLineMethodBuilder::default().with_rng(StdRng::from_entropy());

        if let Some(variant) = &self.variant {
            builder = builder.with_variant(*variant);
        }
        if let Some(pivot) = &self.pivot {
            builder = builder.with_pivot(*pivot);
        }
        if let Some(sep) = &self.line_separator {
            builder = builder.with_line_separator(*sep);
        }

        builder
    }

    fn execute(&self, method: &mut ExtendedLineMethod) -> Result<Vec<u8>, Box<dyn Error>> {
        match &self.action {
            MethodActions::Conceal {
                cover_path,
                data_path,
            } => {
                let cover_data = read_to_string(&cover_path)?;

                let mut data_file = File::open(&data_path)?;

                let mut data_to_encode = Vec::new();
                data_file.read_to_end(&mut data_to_encode)?;

                Ok(conceal(method, &cover_data, &data_to_encode)?)
            }
            MethodActions::Reveal => Ok(vec![]),
        }
    }
}

#[derive(clap::Parser, Debug)]
pub struct ELUVCommand {
    #[command(flatten)]
    args: ExtendedLineCommand,

    /// Charset for the method to use. Impacts the hiding capacity.
    ///
    /// Used by the Trailing Whitespace sub-method.
    ///
    /// Full: includes all of the possible unicode whitespace characters used by the method.
    ///
    /// Twitter: include only Twitter friendly whitespace characters.
    ///
    /// Three Bit: include only part of the full set, such that three bits are encoded by each character.
    ///
    /// Two Bit: include only part of the full set, such that two bits are encoded by each character.
    #[arg(long, default_value = "two_bit", value_parser = clap::builder::PossibleValuesParser::new(["full", "twitter", "three_bit", "two_bit"])
        .map(|s| s.parse::<CharacterSetType>().unwrap())
    )]
    charset: Option<CharacterSetType>,
}

#[derive(clap::Subcommand, Debug)]
enum MethodActions {
    /// Hides data using cover.
    Conceal {
        /// Path to the cover in which the data will be hidden.
        #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
        cover_path: PathBuf,

        /// Path to data for hiding.
        #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
        data_path: PathBuf,
    },
    /// Retrieves from the stegotext.
    Reveal,
}

fn conceal(
    method: &mut ExtendedLineMethod,
    cover_data: &str,
    data: &[u8],
) -> Result<Vec<u8>, Box<dyn Error>> {
    let progress_bar_observer = ProgressBarObserver::new(data.len() as u64);

    let arc_bar = Arc::new(RefCell::new(progress_bar_observer));

    method.subscribe(arc_bar);
    let concealed_data = method.try_conceal(&cover_data, &mut data.view_bits::<Msb0>().iter())?;

    Ok(concealed_data.as_bytes().to_vec())
}

fn reveal(method: &ExtendedLineMethod) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(vec![])
}

impl ELUVCommand {
    pub fn execute(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut builder = self.args.builder();

        if let Some(charset) = self.charset {
            builder = builder.with_trailing_charset(charset);
        }

        let mut method = builder.build()?;
        self.args.execute(&mut method)
    }
}

#[derive(clap::Parser, Debug)]
pub struct ELINECommand {
    #[command(flatten)]
    args: ExtendedLineCommand,
}

impl ELINECommand {
    pub fn execute(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut method = self.args.builder().build()?;
        self.args.execute(&mut method)
    }
}
