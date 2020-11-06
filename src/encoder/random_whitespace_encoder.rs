use super::Encoder;
use crate::encoder::Bit;
use rand::{thread_rng, Rng};

pub struct RandomWhitespaceEncoder {
    next: Option<Box<dyn Encoder>>,
}

impl Default for RandomWhitespaceEncoder {
    fn default() -> Self {
        Self::new(None)
    }
}

impl RandomWhitespaceEncoder {
    pub fn new(encoder: Option<Box<dyn Encoder>>) -> Self {
        RandomWhitespaceEncoder { next: encoder }
    }
}

impl Encoder for RandomWhitespaceEncoder {
    fn encode(&self, data: &mut dyn Iterator<Item = Bit>, line: &mut String) {
        if let Some(Bit(1)) = data.next() {
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
            line.insert_str(position, " ");
        }
        Encoder::encode(&self.next, data, line);
    }
}
