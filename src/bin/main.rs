use ptero::{
    binary::{self, BitIterator},
    encoder::{random_whitespace_encoder::RandomWhitespaceEncoder, Encoder},
    text::WordIterator,
};
use std::{fs::File, io::{self, BufReader, Write}};

fn can_append_word(line: &String, maybe_word: Option<&&str>, pivot: usize) -> bool {
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

    let bytes = binary::read_file_binary("Cargo.lock")?;
    let mut data = BitIterator::new(bytes.into_iter());

    let pivot = 2 * determine_pivot_size(text_wrapper.iter());
    let mut word_iter = text_wrapper.iter().peekable();
    let mut stego_text = String::new();
    loop {
        let mut line = String::new();
        while can_append_word(&line, word_iter.peek(), pivot) {
            line.push_str(word_iter.next().unwrap());
            line.push(' ');
        }
        if word_iter.peek().is_none() {
            break;
        }
        // Encode the bits
        let encoder = RandomWhitespaceEncoder::default();
        encoder.encode(&mut data, &mut line);
        stego_text.push_str(&format!("{}\n", &line))
    }

    let mut file = File::create("stego.txt").expect("Failed opening file");
    file.write_all(&stego_text.as_bytes())
}
