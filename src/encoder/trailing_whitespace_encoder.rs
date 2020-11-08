use crate::binary::Bit;

use super::Encoder;

pub struct TrailingWhitespaceEncoder<T> {
    next: T,
}

impl<T> TrailingWhitespaceEncoder<T>
where
    T: Encoder,
{
    pub fn new(encoder: T) -> Self {
        TrailingWhitespaceEncoder { next: encoder }
    }
}

impl<T> Encoder for TrailingWhitespaceEncoder<T>
where
    T: Encoder,
{
    fn encode(&mut self, data: &mut dyn Iterator<Item = Bit>, line: &mut String) {
        match data.next() {
            Some(Bit(1)) => {
                println!("Adding trailing whitespace: {}", &line);
                line.push(' ');
                Encoder::encode(&mut self.next, data, line);
            }
            Some(Bit(0)) => Encoder::encode(&mut self.next, data, line),
            _ => (),
        }
    }
}
