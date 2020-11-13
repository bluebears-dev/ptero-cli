use log::{debug, info, trace, warn};
use ptero::{
    binary::{Bit, BitIterator},
    encoder::{Encoder, ExtendedLineEncoders},
    text::LineByPivotIterator,
};
use spinners::{Spinner, Spinners};
use std::{
    cell::RefCell,
    fmt,
    fs::{self, File},
    io::{self, Write},
    rc::Rc,
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

#[derive(Debug, Clone)]
struct EncodeError {
    count: usize,
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "exceeded cover text capacity by {} bits", self.count)
    }
}

trait Encodable {
    fn encode(&self, cover_text: &str, pivot: usize) -> Result<String, EncodeError>;
}

impl Encodable for Vec<u8> {
    fn encode(&self, cover_text: &str, pivot: usize) -> Result<String, EncodeError> {
        let line_iterator = Rc::new(RefCell::new(LineByPivotIterator::new(&cover_text, pivot)));
        let mut bits = BitIterator::new(self);
        let mut stego_text = String::new();

        let mut no_data_left = false;
        while !no_data_left {
            let mut line: String;
            if let Some(next_line) = line_iterator.borrow_mut().next() {
                line = next_line;
            } else {
                debug!("No words left, stopping...");
                break;
            }

            debug!(
                "Trying to encode the data to line of length {}",
                &line.len()
            );
            trace!("Constructed line: {}", &line);

            if !no_data_left {
                let mut encoder = ExtendedLineEncoders::new(line_iterator.borrow_mut());
                if !encoder.encode(&mut bits, &mut line) {
                    debug!("No data left to encode, setting flag to true");
                    no_data_left = true;
                }
            }

            stego_text.push_str(&format!("{}\n", &line));
        }
        if !no_data_left {
            Err(EncodeError {
                count: bits.count(),
            })
        } else {
            Ok(stego_text)
        }
    }
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
