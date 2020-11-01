use std::{error::Error, fs::File, io::BufReader};

use ptero::{
    binary::{self, BitIterator},
    text::WordIterator,
};

fn main() -> Result<(), Box<dyn Error>> {
    const FILENAME: &str = "cover.txt";

    let file = File::open(FILENAME).expect("Failed opening the file");
    let mut buf_reader = BufReader::new(file);
    let text_wrapper = WordIterator::from_reader(&mut buf_reader).expect("failed reading the file");
    for word in text_wrapper.iter() {
        println!("{}\t", word)
    }

    let bytes = binary::read_file_binary("Cargo.lock")?;
    for bit in BitIterator::new(bytes.into_iter()) {
        print!("{}", bit);
    }

    Ok(())
}
