use ptero::{
    binary::{Bit, BitIterator, Byte},
    encoder::{
        line_extend_encoder::LineExtendEncoder, random_whitespace_encoder::RandomWhitespaceEncoder,
        trailing_whitespace_encoder::TrailingWhitespaceEncoder, Encoder, NoneEncoder,
    },
    text::WordIterator,
};
use std::{
    cell::RefCell,
    fs::File,
    io::{self, BufReader, Write},
    iter,
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

    let file = File::open(FILENAME).expect("Failed opening the file");
    let mut buf_reader = BufReader::new(file);
    let text_wrapper = WordIterator::from_reader(&mut buf_reader).expect("failed reading the file");

    let values: Vec<u8> = vec![0b11111111, 5, 5, 5, 5, 5, 5, 5, 5, 5];
    let mut data = BitIterator::new(values.iter().map(|v| Byte(*v))).peekable();
    let mut zero_sequence = iter::repeat(Bit(0)).peekable();

    let pivot = 2 * determine_pivot_size(text_wrapper.iter());
    println!("Using determined pivot: {}", pivot);
    let rc_word_iter = Rc::new(RefCell::new(text_wrapper.iter().peekable()));
    let mut stego_text = String::new();
    loop {
        let mut line = String::new();
        {
            let mut word_iter = rc_word_iter.borrow_mut();
            while can_append_word(&line, word_iter.peek(), pivot) {
                line.push_str(word_iter.next().unwrap());
                line.push(' ');
            }
            if word_iter.peek().is_none() {
                break;
            }
        }
        // Encode the bits
        let mut encoder = RandomWhitespaceEncoder::new(LineExtendEncoder::new(
            TrailingWhitespaceEncoder::new(NoneEncoder {}),
            rc_word_iter.borrow_mut(),
        ));

        line = (&line.trim()).to_string();
        if data.peek().is_some() {
            encoder.encode(&mut data, &mut line);
        } else {
            encoder.encode(&mut zero_sequence, &mut line);
        }
        stego_text.push_str(&format!("{}\n", &line))
    }
    if data.next().is_some() {
        panic!(format!(
            "Cover text capacity is too low, {} bits left",
            data.count() + 1
        ));
    }
    println!("Encoded all the data");

    let mut file = File::create("stego.txt").expect("Failed opening file");
    file.write_all(&stego_text.as_bytes())?;
    println!("Saved");
    Ok(())
}
