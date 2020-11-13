use crate::binary::Bit;

use super::{ASCII_ENCODING_WHITESPACE, Encoder};
use log::trace;
use rand::{thread_rng, Rng};

pub struct RandomWhitespaceEncoder {}

impl Default for RandomWhitespaceEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomWhitespaceEncoder {
    pub fn new() -> Self {
        RandomWhitespaceEncoder {}
    }
}

impl Encoder for RandomWhitespaceEncoder {
    fn encode(&mut self, data: &mut dyn Iterator<Item = Bit>, line: &mut String) -> bool {
        match data.next() {
            Some(Bit(1)) => {
                let mut rng = thread_rng();
                let position_determinant = rng.gen_range(0, &line.len());
                let mut position = line.find(' ').unwrap_or(line.len() - 1);
                for (index, character) in line.char_indices() {
                    if index > position_determinant {
                        break;
                    }
                    if character.is_whitespace() {
                        position = index;
                    }
                }
                trace!("Putting space at position {}", position);
                line.insert_str(position, &String::from(ASCII_ENCODING_WHITESPACE));
                true
            }
            None => false,
            _ => true,
        }
    }
}
