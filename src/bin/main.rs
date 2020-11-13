use log::{info, warn};
use ptero::{
    binary::{Bit, BitIterator, Byte},
    encoder::{Encoder, ExtendedLineEncoders},
    text::LineByPivotIterator,
};
use spinners::{Spinner, Spinners};
use std::{
    cell::RefCell,
    error::Error,
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

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "exceeded cover text capacity {}", self.count)
    }
}

fn encode(
    cover_text: &String,
    pivot: usize,
    mut data: impl Iterator<Item = Bit>,
) -> Result<String, EncodeError> {
    let line_iterator = LineByPivotIterator::new(&cover_text, pivot);

    let rc_word_iter = Rc::new(RefCell::new(line_iterator));
    let mut stego_text = String::new();
    let mut no_data_left = false;
    let mut no_words_left = false;

    while !(no_data_left && no_words_left) {
        let mut line = String::new();
        if let Some(next_line) = rc_word_iter.borrow_mut().next() {
            line = next_line;
        } else {
            no_words_left = true;
        }
        if !no_data_left {
            let mut encoder = ExtendedLineEncoders::new(rc_word_iter.borrow_mut());
            no_data_left |= !encoder.encode(&mut data, &mut line);
        }
        stego_text.push_str(&format!("{}\n", &line));
    }
    if !no_data_left && no_words_left {
        Err(EncodeError {
            count: data.count(),
        })
    } else {
        Ok(stego_text)
    }
}

fn main() -> io::Result<()> {
    let opts: Opts = Opts::parse();
    enable_logging(opts.verbose);

    let cover_text = fs::read_to_string(opts.cover).expect("Failed opening the cover file");
    let pivot = 2 * determine_pivot_size(cover_text.split_whitespace());

    let data = fs::read(opts.data).expect("Failed opening the data file");
    let bit_iter = BitIterator::new(data.iter().map(|v| Byte(*v))).peekable();
    warn!(
        "Required cover text capacity: {}",
        BitIterator::new(data.iter().map(|v| Byte(*v))).count()
    );

    info!("Using determined pivot: {}", pivot);

    let sp = Spinner::new(Spinners::Dots12, "Encoding the data".into());
    let stego_result = encode(&cover_text, pivot, bit_iter);
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
        Err(e) => panic!(e),
    }
    Ok(())
}
