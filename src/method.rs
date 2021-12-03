use std::error::Error;
use std::sync::mpsc::Sender;
use rand::{Rng, RngCore, SeedableRng};
use rand::rngs::StdRng;
use crate::{context::Context, decoder::Decoder, encoder::Encoder};
use crate::binary::Bit;
use crate::encoder::EncoderResult;
use crate::verify::Verify;

/// Method which adds extra ASCII space when encoding bit
pub mod random_whitespace;

/// Method  which puts trailing ASCII space when encoding bit
pub mod trailing_whitespace;

/// Method which adds extra word when encoding bit (uses pivot, which informs about expected max line length)
pub mod line_extend;

/// Method which puts trailing Unicode whitespace or invisible chars when encoding bit
pub mod trailing_unicode;

/// Module containing complex methods. Complex usually means combination of several other methods.
pub mod complex;

pub mod variant;

pub mod extended_line_method;
pub mod config;

/// Combination of [Encoder](crate::encoder::Encoder) and [Decoder](crate::decoder::Decoder) traits - each method should be able to encode and decode.
pub trait Method<E, D>: Encoder<E> + Decoder<D>
    where
        E: Context,
        D: Context,
{
    fn method_name(&self) -> String;
}

pub trait SteganographyMethod<Cover, Err>: Verify<Cover, Err> {
    type Output;
    type Input;

    fn try_conceal(&mut self, cover: Cover, data: Self::Input) -> Result<Self::Output, Err>;
    fn try_reveal(&mut self, cover: Cover) -> Result<Self::Input, Err>;
}