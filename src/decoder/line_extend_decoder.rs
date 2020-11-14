//! # Description
//!
//! This decoder checks if the line is extended.
//! If the line is longer than the pivot the bit 1 is decoded, otherwise 0 - 
//! please not that any duplicate or trailing whitespace does not count as extension.
use crate::binary::Bit;

use super::{Decoder};
use log::trace;
use regex::Regex;

pub struct LineExtendDecoder {
    pivot: usize
}

impl LineExtendDecoder {
    pub fn new(pivot: usize) -> Self {
        LineExtendDecoder { pivot }
    }
}

impl Decoder for LineExtendDecoder {
    fn decode(&self, line: &str) -> Vec<Bit> {
        let pattern = Regex::new(r"\s+").unwrap();
        let cleaned_line = pattern.replace_all(line,  " ");
        let bit = if cleaned_line.trim_end().len() > self.pivot {
            trace!("Line is extended over the {} length", self.pivot);
            Bit(1)
        } else {
            Bit(0)
        };
        vec![bit]
    }
}
