use std::{error::Error, fs::File, io::Write};

use clap::Clap;
use log::info;

use ptero::{
    cli::decoder::decode_command,
    cli::{
        capacity::get_cover_text_capacity,
        capacity::GetCapacityCommand,
        decoder::DecodeSubCommand,
        encoder::{encode_command, EncodeSubCommand},
        writer::get_writer,
    },
};

const BANNER: &str = r#"

     ______   ______   ______     ______     ______    
    /\  == \ /\__  _\ /\  ___\   /\  == \   /\  __ \   
    \ \  _-/ \/_/\ \/ \ \  __\   \ \  __<   \ \ \/\ \  
     \ \_\      \ \_\  \ \_____\  \ \_\ \_\  \ \_____\ 
      \/_/       \/_/   \/_____/   \/_/ /_/   \/_____/ 
                                                    
"#;

const APP_NAME: &str = "Ptero CLI";

/// The CLI text steganography tool for social media.
#[derive(Clap)]
#[clap(
version = "0.2",
author = "Paweł G. <dev.baymax42@gmail.com>",
name = format ! ("{}{}", BANNER, APP_NAME)
)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,

    /// Path to file where the result of encoding/decoding should be placed.
    /// If not used, it will print to stdout.
    #[clap(short, long)]
    output: Option<String>,

    /// Flag for controlling verbosity of the output logs.
    ///
    /// To increase verbosity add additional occurrences e.g. `-vv` will print info logs.
    /// By default only error logs are printed.
    #[clap(short, parse(from_occurrences))]
    verbose: u8,
    #[clap(long)]
    json: bool,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(name = "encode")]
    Encode(EncodeSubCommand),
    #[clap(name = "decode")]
    Decode(DecodeSubCommand),
    #[clap(name = "capacity")]
    GetCapacity(GetCapacityCommand),
}

fn enable_logging(verbose: u8) {
    let logging_level = match verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };

    pretty_env_logger::formatted_builder()
        .parse_filters(logging_level)
        .init();
}

fn run_subcommand(subcommand: SubCommand) -> Result<String, Box<dyn Error>> {
    let result: String = match subcommand {
        SubCommand::Encode(command) => {
            let result = encode_command(command)?;
            String::from_utf8_lossy(&result).into()
        }
        SubCommand::Decode(command) => {
            let result = decode_command(command)?;
            String::from_utf8_lossy(&result).into()
        }
        SubCommand::GetCapacity(command) => {
            let capacity: u32 = get_cover_text_capacity(command)?;
            format!("{} b", capacity)
        }
    };
    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    enable_logging(opts.verbose);
    let writer = get_writer(opts.json);
    writer.message(BANNER);

    let result = run_subcommand(opts.subcommand);

    if let Err(error) = &result {
        let error_message = format!("{}", error);
        writer.output(&error_message);
    } else {
        let cli_output = &result?;
        if let Some(path) = &opts.output {
            let mut output_file = File::create(path)?;
            output_file.write_all(&cli_output.as_bytes())?;
            info!("Saved to {}", &path);
        } else {
            writer.output(&cli_output);
        }
    }
    Ok(())
}
