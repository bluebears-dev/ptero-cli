use crate::binary::Bit;

pub mod random_whitespace_encoder;

pub trait Encoder {
    fn encode(&self, data: &mut dyn Iterator<Item = Bit>, line: &mut String);
}

impl<T> Encoder for Option<T> {
    fn encode(&self, _: &mut dyn Iterator<Item = Bit>, _: &mut String) {}
}
