use log::{info, warn};
use ptero::{
    binary::{BitIterator, Byte},
    encoder::{Encoder, ExtendedLineEncoders},
    text::WordIterator,
};
use spinners::{Spinner, Spinners};
use std::{time::Duration, cell::RefCell, cell::RefMut, fs::{self, File}, io::{self, BufReader, Write}, iter::Peekable, rc::Rc, thread::sleep};

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

fn can_append_word(line: &str, maybe_word: Option<&&str>, pivot: usize) -> bool {
    match maybe_word {
        Some(word) => line.len() + word.len() <= pivot,
        None => false,
    }
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

fn construct_line<'a, T: Iterator<Item = &'a str>>(
    word_iter: RefMut<Peekable<T>>,
    line: &mut String,
    pivot: usize,
) {
    let mut word_iter = word_iter;
    while can_append_word(&line, word_iter.peek(), pivot) {
        line.push_str(word_iter.next().unwrap());
        line.push(' ');
    }
}

fn main() -> io::Result<()> {
    let opts: Opts = Opts::parse();
    enable_logging(opts.verbose);

    let cover_text_file = File::open(opts.cover).expect("Failed opening the cover file");
    let mut cover_reader = BufReader::new(cover_text_file);
    let text_wrapper =
        WordIterator::from_reader(&mut cover_reader).expect("Failed reading the file");

    let data = fs::read(opts.data).expect("Failed opening the data file");
    warn!(
        "Required cover text capacity: {}",
        BitIterator::new(data.iter().map(|v| Byte(*v))).count()
    );

    let mut bit_iter = BitIterator::new(data.iter().map(|v| Byte(*v))).peekable();
    let pivot = 2 * determine_pivot_size(text_wrapper.iter());
    info!("Using determined pivot: {}", pivot);
    let rc_word_iter = Rc::new(RefCell::new(text_wrapper.iter().peekable()));
    let mut stego_text = String::new();
    let mut no_data_left = false;
    let mut no_words_left = false;

    let sp = Spinner::new(Spinners::Dots12, "Encoding the data".into());
    while !(no_data_left && no_words_left) {
        sleep(Duration::from_secs(1));
        let mut line = String::new();
        if !no_words_left {
            construct_line(rc_word_iter.borrow_mut(), &mut line, pivot);
            // Trim the trailing space after constructing the line
            line = line.trim_end().into();
        }
        if !no_data_left {
            let mut encoder = ExtendedLineEncoders::new(rc_word_iter.borrow_mut());
            no_data_left |= !encoder.encode(&mut bit_iter, &mut line);
        }
        stego_text.push_str(&format!("{}\n", &line));
        no_words_left |= rc_word_iter.borrow_mut().peek().is_none();
    }
    sp.stop();
    println!();

    if !no_data_left && no_words_left {
        panic!(format!(
            "Cover text capacity is too low, {} bits left",
            bit_iter.count()
        ));
    }
    info!("Encoded all the data");

    if let Some(path) = opts.output {
        let mut output_file = File::create(path).expect("Failed opening file");
        output_file.write_all(&stego_text.as_bytes())?;
        info!("Saved");
    } else {
        println!("\n{}\n", &stego_text);
    }
    Ok(())
}
