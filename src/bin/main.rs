use std::{error::Error, fs::File, io::Write, path::PathBuf, process};

use clap::{Parser, Subcommand};
use colored::Colorize;

use ptero::{
    cli::writer::Writer,
    log::{get_file_logger, get_stdout_logger, verbosity_to_level_filter},
};
use serde_json::json;

use ptero::cli::commands::extended_line;

const BANNER: &str = r#"

     ______   ______   ______     ______     ______    
    /\  == \ /\__  _\ /\  ___\   /\  == \   /\  __ \   
    \ \  _-/ \/_/\ \/ \ \  __\   \ \  __<   \ \ \/\ \  
     \ \_\      \ \_\  \ \_____\  \ \_\ \_\  \ \_____\ 
      \/_/       \/_/   \/_____/   \/_/ /_/   \/_____/ 
                                                    
"#;

/// Yet another CLI steganography tool.
#[derive(Debug, Parser)]
#[command(name = "Ptero CLI", author, about, version)]
#[command(group = clap::ArgGroup::new("output_args").multiple(false))]
struct Opts {
    #[command(subcommand)]
    subcommand: AvailableMethods,
    /// Path to file where the result of encoding/decoding should be placed.
    /// If not used, it will print to stdout.
    ///
    /// Cannot be used in conjunction with `--json` flag.
    #[arg(short, long, group = "output_args", value_hint = clap::ValueHint::DirPath)]
    output: Option<PathBuf>,

    /// If present, will print the output of the CLI in JSON format that can be further parsed by other tooling.
    ///
    /// Cannot be used in conjunction with `-o` flag.
    #[arg(long, group = "output_args")]
    json: bool,

    /// Path to log file.
    ///
    /// By default CLI won't save any logs. If this param is used, CLI will append new logs at the end of the file
    /// pointed by the path. It is affected by the verbosity flag (`-v`).
    #[arg(long, value_hint = clap::ValueHint::DirPath)]
    log_file: Option<String>,

    /// Flag for controlling verbosity of the output logs.
    ///
    /// To increase verbosity add additional occurrences e.g. `-v` will print warn logs and so on.
    /// By default only error logs are printed.
    #[arg(short, default_value = "0")]
    verbose: u8,
}

#[derive(Debug, Subcommand)]
enum AvailableMethods {
    /// Select the base Extended Line method. Text steganography.
    ELINE(extended_line::ELINECommand),
    /// Select the ELUV method, a variant of Extended Line method using unicode whitespace. Text steganography.
    ELUV(extended_line::ELUVCommand),
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
fn run_subcommand(subcommand: AvailableMethods) -> Result<Vec<u8>, Box<dyn Error>> {
    match subcommand {
        AvailableMethods::ELINE(command) => command.execute(),
        AvailableMethods::ELUV(command) => command.execute(),
    }
}
#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), Box<dyn Error>> {
    Writer::print(&BANNER.purple().bold().to_string());
    let opts: Opts = Opts::parse();

    enable_logging(opts.verbose, opts.log_file)?;

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
            Writer::info(&format!("Result saved to {:?}", path.as_os_str()));
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
