use ptero::{
    cli::decoder::decode_command,
    cli::{
        decoder::DecodeSubCommand,
        encoder::{encode_command, EncodeSubCommand},
    },
};
use std::{error::Error, fs::File, io::Write};

use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1", author = "Paweł G. <dev.baymax42@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,

    #[clap(short, long)]
    output: Option<String>,

    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(name = "encode")]
    EncodeSubCommand(EncodeSubCommand),
    #[clap(name = "decode")]
    DecodeSubCommand(DecodeSubCommand),
}

fn enable_logging(verbose: u8) {
    let logging_level = match verbose {
        0 => "error",
        1 => "warn",
        2 => "info",
        3 => "debug",
        _ => "trace",
    };
    pretty_env_logger::formatted_builder()
        .parse_filters(logging_level)
        .init();
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();
    enable_logging(opts.verbose);

    let result = match opts.subcommand {
        SubCommand::EncodeSubCommand(command) => encode_command(command)?,
        SubCommand::DecodeSubCommand(command) => decode_command(command)?,
    };

    println!("Finished encoding");
    if let Some(path) = opts.output {
        let mut output_file = File::create(&path).expect("Failed opening file");
        output_file.write_all(&result)?;
        println!("Saved to {}", &path);
    } else {
        println!("\n{}\n", String::from_utf8_lossy(&result));
    }
    Ok(())
}
