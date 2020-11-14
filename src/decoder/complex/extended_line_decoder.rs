use crate::decoder::Bit;
use crate::decoder::{
    line_extend_decoder, random_whitespace_decoder, trailing_whitespace_decoder, Decoder,
};

pub struct ExtendedLineDecoder<'a> {
    decoders: Vec<Box<dyn Decoder + 'a>>,
}

impl<'a> ExtendedLineDecoder<'a> {
    pub fn new(pivot: usize) -> Self {
        ExtendedLineDecoder {
            decoders: vec![
                Box::new(random_whitespace_decoder::RandomWhitespaceDecoder::new()),
                Box::new(line_extend_decoder::LineExtendDecoder::new(pivot)),
                Box::new(trailing_whitespace_decoder::TrailingWhitespaceDecoder::new()),
            ],
        }
    }
}

impl<'a> Decoder for ExtendedLineDecoder<'a> {
    fn decode(&self, line: &str) -> Vec<Bit> {
        let mut secret_data = Vec::default();
        for decoder in &self.decoders {
            let mut result = decoder.decode(line);
            secret_data.append(&mut result);
        }
        secret_data
    }
}
