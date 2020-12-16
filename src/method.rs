use crate::{decoder::Decoder, encoder::Encoder};

/// Encoder which puts extra ASCII space when encoding bit
pub mod random_whitespace;

/// Encoder which puts trailing ASCII space when encoding bit
pub mod trailing_whitespace;

/// Encoder which adds extra word when encoding bit
pub mod line_extend;

/// Encoder which puts trailing Unicode whitespace or invisible chars when encoding bit
pub mod trailing_unicode;

pub mod complex;

pub trait Method: Encoder + Decoder {}