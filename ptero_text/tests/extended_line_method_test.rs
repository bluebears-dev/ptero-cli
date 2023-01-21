use std::path::PathBuf;

use rand::RngCore;
use rstest::*;

use ptero_text::extended_line_method::character_sets::GetCharacterSet;
use ptero_text::extended_line_method::{ExtendedLineMethod, ExtendedLineMethodBuilder, Variant};

use crate::test_resource::ResourceLoader;

#[cfg(test)]
mod one_bit_test;

#[cfg(test)]
mod two_bit_test;

const WITH_WORDS_TEXT: &str =
    "A little panda has fallen from a tree. The panda went rolling down the hill";
const TINY_TEXT: &str = "TI NY COVER";
const ONE_WORD_TEXT: &str = "Words.";
const EMPTY_TEXT: &str = "";

#[fixture]
pub fn cover_text_loader() -> ResourceLoader {
    let dir_path = PathBuf::new().join("resources").join("cover_texts");

    ResourceLoader::new(&dir_path)
}

pub(crate) fn pre_build_method_with<T, CS>(
    variant: Variant,
    charset: CS,
    rng: T,
) -> ExtendedLineMethodBuilder
where
    T: RngCore + 'static,
    CS: GetCharacterSet + 'static,
{
    ExtendedLineMethod::builder()
        .with_rng(rng)
        .with_variant(variant)
        .with_trailing_charset(charset)
}
