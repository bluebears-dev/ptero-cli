use log::{info, warn};
use ptero::{
    binary::{BitIterator, Byte},
    encoder::{Encoder, ExtendedLineEncoders},
    text::WordIterator,
};
use std::{
    cell::RefCell,
    fs::File,
    io::{self, BufReader, Write},
    rc::Rc,
};

fn can_append_word(line: &str, maybe_word: Option<&&str>, pivot: usize) -> bool {
    match maybe_word {
        Some(word) => line.len() + word.len() <= pivot,
        None => false,
    }
}

fn determine_pivot_size<'a>(words: impl Iterator<Item = &'a str>) -> usize {
    words.into_iter().map(|w| w.len()).max().unwrap_or(0)
}

fn main() -> io::Result<()> {
    const FILENAME: &str = "cover.txt";
    pretty_env_logger::init();


    let file = File::open(FILENAME).expect("Failed opening the file");
    let mut buf_reader = BufReader::new(file);
    let text_wrapper = WordIterator::from_reader(&mut buf_reader).expect("failed reading the file");
    let values: Vec<u8> = Vec::from([0b11111100; 39]);
    warn!("Required cover text capacity: {}", BitIterator::new(values.iter().map(|v| Byte(*v))).count());

    let mut data = BitIterator::new(values.iter().map(|v| Byte(*v))).peekable();
    let pivot = 2 * determine_pivot_size(text_wrapper.iter());
    info!("Using determined pivot: {}", pivot);
    let rc_word_iter = Rc::new(RefCell::new(text_wrapper.iter().peekable()));
    let mut stego_text = String::new();
    let mut no_data_left = false;
    let mut no_words_left = false;
    while !(no_data_left && no_words_left) {
        let mut line = String::new();
        if !no_words_left {
            // Scope for borrowed mutable iterator, so we can later borrow it for encoder
            // Probably move to function
            let mut word_iter = rc_word_iter.borrow_mut();
            while can_append_word(&line, word_iter.peek(), pivot) {
                line.push_str(word_iter.next().unwrap());
                line.push(' ');
            }
            line = (&line.trim_end()).to_string();
        }
        // Trim the trailing space after constructing the line
        if !no_data_left {
            let mut encoder = ExtendedLineEncoders::new(rc_word_iter.borrow_mut());
            no_data_left |= !encoder.encode(&mut data, &mut line);
        }
        stego_text.push_str(&format!("{}\n", &line));
        no_words_left |= rc_word_iter.borrow_mut().peek().is_none();
    }
    if !no_data_left && no_words_left {
        panic!(format!(
            "Cover text capacity is too low, {} bits left",
            data.count()
        ));
    }
    info!("Encoded all the data");

    let mut file = File::create("stego.txt").expect("Failed opening file");
    file.write_all(&stego_text.as_bytes())?;
    info!("Saved");
    Ok(())
}
