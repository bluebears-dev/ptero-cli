use crate::binary::Bit;

pub mod line_extend_encoder;
pub mod random_whitespace_encoder;
pub mod trailing_whitespace_encoder;

pub trait Encoder {
    fn encode(&mut self, data: &mut dyn Iterator<Item = Bit>, line: &mut String);
}

impl<T> Encoder for Option<T> {
    fn encode(&mut self, _: &mut dyn Iterator<Item = Bit>, _: &mut String) {}
}

pub struct NoneEncoder {}

impl Encoder for NoneEncoder {
    fn encode(&mut self, _: &mut dyn std::iter::Iterator<Item = Bit>, _: &mut String) {}
}
