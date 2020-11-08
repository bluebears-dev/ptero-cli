use crate::binary::Bit;

use super::Encoder;
use rand::{thread_rng, Rng};

pub struct RandomWhitespaceEncoder<T>
where
    T: Encoder,
{
    next: T,
}

impl<T> RandomWhitespaceEncoder<T>
where
    T: Encoder,
{
    pub fn new(encoder: T) -> Self {
        RandomWhitespaceEncoder { next: encoder }
    }
}

impl<T> Encoder for RandomWhitespaceEncoder<T>
where
    T: Encoder,
{
    fn encode(&mut self, data: &mut dyn Iterator<Item = Bit>, line: &mut String) {
        match data.next() {
            Some(Bit(1)) => {
                println!("Adding random whitespace:, {}", &line);
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
                Encoder::encode(&mut self.next, data, line)
            }
            Some(Bit(0)) => Encoder::encode(&mut self.next, data, line),
            _ => (),
        }
    }
}
