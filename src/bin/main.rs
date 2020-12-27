use std::{error::Error, fs::File, io::Write};

use clap::{ArgGroup, Clap};
use log::info;

use ptero::{
    cli::decoder::decode_command,
    cli::{
        capacity::get_cover_text_capacity, capacity::GetCapacityCommand, decoder::DecodeSubCommand,
        encoder::EncodeSubCommand, writer::get_writer,
    },
    log::{get_file_logger, get_stdout_logger, verbosity_to_level_filter},
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
name = format ! ("{}{}", BANNER, APP_NAME),
group = ArgGroup::new("output_args").required(false),
)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,

    /// Path to file where the result of encoding/decoding should be placed.
    /// If not used, it will print to stdout.
    ///
    /// Cannot be used in conjunction with `--json` flag.
    #[clap(short, long, group = "output_args")]
    output: Option<String>,

    /// Flag for controlling verbosity of the output logs.
    ///
    /// To increase verbosity add additional occurrences e.g. `-vv` will print info logs.
    /// By default only error logs are printed.
    #[clap(short, parse(from_occurrences))]
    verbose: u8,
    /// If present, will print the output of the CLI in JSON format that can be further parsed by other tooling.
    ///
    /// Cannot be used in conjunction with `-o` flag.
    #[clap(long, group = "output_args")]
    json: bool,
    /// Path to log file.
    ///
    /// By default CLI won't save any logs. If this param is used, CLI will append new logs at the end of the file
    /// pointed by the path. It is not affected by the `verbose` flag, and saves all the entries (starting from `TRACE`).
    #[clap(long)]
    log_file: Option<String>,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(name = "encode", group = ArgGroup::new("method_args").required(true))]
    Encode(EncodeSubCommand),
    #[clap(name = "decode", group = ArgGroup::new("method_args").required(true))]
    Decode(DecodeSubCommand),
    #[clap(name = "capacity")]
    GetCapacity(GetCapacityCommand),
}

fn enable_logging(
    verbose: u8,
    log_path: Option<String>,
) -> std::result::Result<(), Box<dyn Error>> {
    let level_filter = verbosity_to_level_filter(verbose);
    let mut log_builder = fern::Dispatch::new().chain(get_stdout_logger(level_filter));

    log_builder = if let Some(path) = log_path {
        log_builder.chain(get_file_logger(&path))
    } else {
        log_builder
    };

    log_builder.apply()?;
    Ok(())
}

fn run_subcommand(subcommand: SubCommand) -> Result<Vec<u8>, Box<dyn Error>> {
    let result = match subcommand {
        SubCommand::Encode(command) => {
            command.run()?
        }
        SubCommand::Decode(command) => {
            decode_command(command)?
        }
        SubCommand::GetCapacity(command) => {
            let capacity: u32 = get_cover_text_capacity(command)?;
            let output_str = format!("{} b", capacity);
            output_str.as_bytes().into()
        }
    };
    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    enable_logging(opts.verbose, opts.log_file)?;
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
            output_file.write_all(&cli_output)?;
            info!("Saved to '{}'", &path);
        } else {
            writer.output(&String::from_utf8_lossy(&cli_output));
        }
    }
    Ok(())
}
