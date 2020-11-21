use colored::Colorize;
use ptero::{cli::decoder::decode_command, cli::{capacity::GetCapacityCommand, decoder::DecodeSubCommand, encoder::{encode_command, EncodeSubCommand}, capacity::get_cover_text_capacity}, success};
use std::{error::Error, fs::File, io::Write};

use clap::Clap;
/// The CLI text steganography tool for social media.
#[derive(Clap)]
#[clap(version = "0.1", author = "Paweł G. <dev.baymax42@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,

    /// Path to file where the result of encoding/decoding should be placed.
    /// If not used, it will print to stdout.
    #[clap(short, long)]
    output: Option<String>,

    /// Flag for controlling verbosity of the output logs.
    /// To increase verbosity add additional occurrences e.g. `-vv` will print info logs.
    ///
    /// By default only error logs are printed.
    #[clap(short, parse(from_occurrences))]
    verbose: u8,
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
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };
    pretty_env_logger::formatted_builder()
        .parse_filters(logging_level)
        .init();
}

fn print_logo() {
    println!(
        r#"

     ______   ______   ______     ______     ______    
    /\  == \ /\__  _\ /\  ___\   /\  == \   /\  __ \   
    \ \  _-/ \/_/\ \/ \ \  __\   \ \  __<   \ \ \/\ \  
     \ \_\      \ \_\  \ \_____\  \ \_\ \_\  \ \_____\ 
      \/_/       \/_/   \/_____/   \/_/ /_/   \/_____/ 
                                                       
"#
    )
}

fn print_result(output: Option<String>, result: Vec<u8>) -> Result<(), Box<dyn Error>> {
    success!("Finished!");
    if let Some(path) = output {
        let mut output_file = File::create(&path)?;
        output_file.write_all(&result)?;
        success!("Saved to {}", &path);
    } else {
        success!("Printing data to stdout");
        println!("---\n{}\n---", String::from_utf8_lossy(&result).dimmed());
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    print_logo();
    let opts: Opts = Opts::parse();
    enable_logging(opts.verbose);

    match opts.subcommand {
        SubCommand::Encode(command) => {
            let result = encode_command(command)?;
            print_result(opts.output, result)?;
        },
        SubCommand::Decode(command) => {
            let result = decode_command(command)?;
            print_result(opts.output, result)?;
        },
        SubCommand::GetCapacity(command) => {
            let capacity: u32 = get_cover_text_capacity(command)?;
            success!("Capacity is {} bits", capacity);
        }
    };

    
    Ok(())
}
