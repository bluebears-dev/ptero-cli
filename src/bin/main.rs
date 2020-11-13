use log::{info, warn};
use ptero::{binary::BitIterator, encodable::Encodable};
use spinners::{Spinner, Spinners};
use std::{
    fs::{self, File},
    io::{self, Write},
};

use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1", author = "Pawel G. <dev.baymax42@gmail.com>")]
struct Opts {
    #[clap(short, long)]
    cover: String,

    #[clap(short, long)]
    data: String,

    #[clap(short, long)]
    output: Option<String>,

    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,
}

fn determine_pivot_size<'a>(words: impl Iterator<Item = &'a str>) -> usize {
    words.into_iter().map(|w| w.len()).max().unwrap_or(0)
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

fn main() -> io::Result<()> {
    let opts: Opts = Opts::parse();
    enable_logging(opts.verbose);

    let cover_text = fs::read_to_string(opts.cover).expect("Failed opening the cover file");
    let pivot = 2 * determine_pivot_size(cover_text.split_whitespace());

    let data = fs::read(opts.data).expect("Failed opening the data file");

    warn!(
        "Required cover text capacity: {}",
        BitIterator::new(&data).count()
    );
    info!("Using determined pivot: {}", pivot);

    let sp = Spinner::new(Spinners::Dots12, "Encoding the data".into());
    let stego_result = data.encode(&cover_text, pivot);
    sp.stop();
    println!();

    match stego_result {
        Ok(stego_text) => {
            println!("Finished encoding");
            if let Some(path) = opts.output {
                let mut output_file = File::create(&path).expect("Failed opening file");
                output_file.write_all(&stego_text.as_bytes())?;
                println!("Saved to {}", &path);
            } else {
                println!("\n{}\n", &stego_text);
            }
        }
        Err(e) => panic!(e.to_string()),
    }
    Ok(())
}
