use std::{error::Error, fs::File, io::Write, process};

use clap::{ArgGroup, Clap};
use colored::Colorize;

use ptero::{
    cli::{
        capacity::GetCapacityCommand, decoder::DecodeSubCommand,
        encoder::EncodeSubCommand, writer::Writer,
    },
    log::{get_file_logger, get_stdout_logger, verbosity_to_level_filter},
};
use serde_json::json;

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
    /// To increase verbosity add additional occurrences e.g. `-v` will print warn logs and so on.
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
    /// pointed by the path. It is affected by the verbosity flag (`-v`).
    #[clap(long)]
    log_file: Option<String>,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(name = "encode", group = ArgGroup::new("method_args").required(true))]
    Encode(EncodeSubCommand),
    #[clap(name = "decode", group = ArgGroup::new("method_args").required(true))]
    Decode(DecodeSubCommand),
    #[clap(name = "capacity", group = ArgGroup::new("method_args").required(true))]
    GetCapacity(GetCapacityCommand),
}

#[cfg(not(tarpaulin_include))]
fn enable_logging(
    verbose: u8,
    log_path: Option<String>,
) -> std::result::Result<(), Box<dyn Error>> {
    let level_filter = verbosity_to_level_filter(verbose);

    let mut log_builder = fern::Dispatch::new()
        .level(level_filter)
        .chain(get_stdout_logger());

    log_builder = if let Some(path) = log_path {
        log_builder.chain(get_file_logger(&path))
    } else {
        log_builder
    };

    log_builder.apply()?;
    Ok(())
}

#[cfg(not(tarpaulin_include))]
fn run_subcommand(subcommand: SubCommand) -> Result<Vec<u8>, Box<dyn Error>> {
    let result = match subcommand {
        SubCommand::Encode(command) => command.run()?,
        SubCommand::Decode(command) => command.run()?,
        SubCommand::GetCapacity(command) => {
            let capacity: u32 = command.run()?;
            let output_str = format!("{} b", capacity);
            output_str.as_bytes().into()
        }
    };
    Ok(result)
}
#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    enable_logging(opts.verbose, opts.log_file)?;
    Writer::print(&BANNER.purple().bold().to_string());

    let result = run_subcommand(opts.subcommand);

    if let Err(error) = &result {
        let error_message = format!("{}", error);
        Writer::error(&error_message);
        process::exit(1);
    } else {
        let cli_output = &result?;
        if let Some(path) = &opts.output {
            let mut output_file = File::create(path)?;
            output_file.write_all(&cli_output)?;
            Writer::info(&format!("Saved to '{}'", &path));
        } else {
            let output = &String::from_utf8_lossy(&cli_output);
            if opts.json {
                println!(
                    "{}",
                    json!({
                        "type": "success",
                        "result": output,
                    })
                );
            } else {
                println!("{}", output);
            }
        }
    }
    Ok(())
}
