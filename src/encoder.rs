use crate::{binary::Bit};

pub mod extended_line_encoder;
pub mod line_extend_encoder;
pub mod random_whitespace_encoder;
pub mod trailing_whitespace_encoder;

pub const ASCII_ENCODING_WHITESPACE: char = ' ';

pub trait Encoder {
    fn encode(&mut self, data: &mut dyn Iterator<Item = Bit>, line: &mut String) -> bool;
}
