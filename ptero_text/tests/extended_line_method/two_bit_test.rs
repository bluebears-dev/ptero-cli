use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use bitvec::prelude::*;
use bitvec::view::BitView;
use rand::RngCore;
use rand::rngs::mock::StepRng;
use rstest::*;
use rstest_reuse::*;
#[cfg(test)]
use rstest_reuse;

use ptero_common::method::SteganographyMethod;
use ptero_text::extended_line_method::{ConcealError, ExtendedLineMethod, Variant};
use ptero_text::extended_line_method::character_sets::CharacterSetType::TwoBit;

use crate::extended_line_method::*;

pub(crate) fn get_method<T>(pivot: usize, variant: Variant, rng: T) -> ExtendedLineMethod
    where
        T: RngCore + 'static,
{
    ExtendedLineMethod::builder()
        .with_pivot(pivot)
        .with_rng(rng)
        .with_variant(variant)
        .with_trailing_charset(TwoBit)
        .build()
        .unwrap()
}

#[rstest]
#[case(4, 97u8, SINGLE_CHAR_TEXT, "a  b \nca b\u{2000}\nca b\nca b\nca b\nc")]
#[case(10, 255u8, WITH_WORDS_TEXT, "A  little panda\u{2001}\nhas  fallen from\u{2001}\na tree.\nThe panda\nwent\nrolling\ndown the\nhill")]
#[case(10, 255u8, WITH_OTHER_WHITESPACE_TEXT, "A  little panda\u{2001}\nhas  fallen from\u{2001}\na tree.\nThe panda\nwent\nrolling\ndown the\nhill")]
#[case(10, 255u8, WITH_EMOJI_TEXT, "A  little 🐼 has\u{2001}\n(fallen)  from\u{2001}\na \\🌳/. The\n🐼 went\nrolling\ndown the\nhill.")]
#[case(15, 255u8, HTML_TEXT, "<div>  <button style=\"\u{2001}\nbackground:  red;\">Click\u{2001}\nme</button>\n<div/> <footer>\nThis is the end\n</footer>")]
#[case(10, 171u8, WITH_WORDS_TEXT, "A little panda \nhas fallen from\u{2001}\na tree.\nThe panda\nwent\nrolling\ndown the\nhill")]
fn conceals_data_variant_1<T>(
    #[case] pivot: usize,
    #[case] data: T,
    #[case] cover: &str,
    #[case] expected: &str,
) -> Result<(), Box<dyn Error>>
    where T: BitStore {
    let rng = StepRng::new(1, 1);
    let mut method = get_method(pivot, Variant::V1, rng);
    let stego_text = method.try_conceal(cover, &mut data.view_bits::<Msb0>().iter())?;

    assert_eq!(stego_text, expected);
    Ok(())
}

#[rstest]
#[case(4, 97u8, SINGLE_CHAR_TEXT, "a b\u{2001}\nca  b\nca b\nca b\nca b\nc")]
#[case(10, 255u8, WITH_WORDS_TEXT, "A  little panda\u{2001}\nhas  fallen from\u{2001}\na tree.\nThe panda\nwent\nrolling\ndown the\nhill")]
#[case(10, 255u8, WITH_OTHER_WHITESPACE_TEXT, "A  little panda\u{2001}\nhas  fallen from\u{2001}\na tree.\nThe panda\nwent\nrolling\ndown the\nhill")]
#[case(10, 255u8, WITH_EMOJI_TEXT, "A  little 🐼 has\u{2001}\n(fallen)  from\u{2001}\na \\🌳/. The\n🐼 went\nrolling\ndown the\nhill.")]
#[case(15, 255u8, HTML_TEXT, "<div>  <button style=\"\u{2001}\nbackground:  red;\">Click\u{2001}\nme</button>\n<div/> <footer>\nThis is the end\n</footer>")]
#[case(8, 171u8, WITH_WORDS_TEXT, "A little panda\u{2000}\nhas  fallen\u{2000}\nfrom a\ntree.\nThe\npanda\nwent\nrolling\ndown the\nhill")]
fn conceals_data_variant_2<T>(
    #[case] pivot: usize,
    #[case] data: T,
    #[case] cover: &str,
    #[case] expected: &str,
) -> Result<(), Box<dyn Error>>
    where T: BitStore {
    let rng = StepRng::new(1, 1);
    let mut method = get_method(pivot, Variant::V2, rng);
    let stego_text = method.try_conceal(cover, &mut data.view_bits::<Msb0>().iter())?;

    assert_eq!(stego_text, expected);
    Ok(())
}

#[rstest]
#[case(4, 97u8, SINGLE_CHAR_TEXT, "a b ca \nb ca\u{2000}\nb ca\nb ca\nb c")]
#[case(10, 255u8, WITH_WORDS_TEXT, "A  little panda\u{2001}\nhas  fallen from\u{2001}\na tree.\nThe panda\nwent\nrolling\ndown the\nhill")]
#[case(10, 255u8, WITH_OTHER_WHITESPACE_TEXT, "A  little panda\u{2001}\nhas  fallen from\u{2001}\na tree.\nThe panda\nwent\nrolling\ndown the\nhill")]
#[case(13, 255u8, WITH_EMOJI_TEXT, "A  little 🐼 has\u{2001}\n(fallen)  from a\u{2001}\n\\🌳/. The 🐼\nwent rolling\ndown the\nhill.")]
#[case(24, 255u8, HTML_TEXT, "<div>  <button style=\" background:\u{2001}\nred;\">Click  me</button> <div/>\u{2001}\n<footer> This is the end\n</footer>")]
#[case(10, 171u8, WITH_WORDS_TEXT, "A  little \npanda  has\u{2001}\nfallen\nfrom a\ntree. The\npanda went\nrolling\ndown the\nhill")]
fn conceals_data_variant_3<T>(
    #[case] pivot: usize,
    #[case] data: T,
    #[case] cover: &str,
    #[case] expected: &str,
) -> Result<(), Box<dyn Error>>
    where T: BitStore {
    let rng = StepRng::new(1, 1);
    let mut method = get_method(pivot, Variant::V3, rng);
    let stego_text = method.try_conceal(cover, &mut data.view_bits::<Msb0>().iter())?;

    assert_eq!(stego_text, expected);
    Ok(())
}

const STEGO_SINGLE_LETTERS: &str = "a  b \nca b\nca  b\nca b\nca b\nc";
const STEGO_SINGLE_LETTERS_WITH_UNICODE_1: &str = "a  b \nca b\u{2000}\nca b\nca b\nca b\nc";
const STEGO_SINGLE_LETTERS_WITH_UNICODE_2: &str = "a  b \nca b\u{2001}\nca b\nca b\nca b\nc";

const STEGO_WITH_WORDS: &str = "A  little\u{2001}\npanda has\nfallen  from";
const STEGO_WITH_OTHER_WHITESPACE: &str =
    "A  little panda \nhas  fallen from\u{2001}\na  tree. The\npanda went\nrolling\ndown the\nhill";
const STEGO_WITH_SPECIAL_CHARS: &str =
    "A  little 🐼 has \n(fallen)  from \na  \\🌳/. The 🐼\nwent\nrolling\ndown the\nhill.";
const STEGO_HTML: &str = "<div>  <button style=\"\u{2001}\nbackground:  red;\">Click\u{2000}\nme</button>\n<div/> <footer>\nThis is the end\n</footer>";
const STEGO_V1_CONCEALED: &str =
    "A  little panda\u{2001}\nhas  fallen from\u{2001}\na tree.\nThe panda\nwent\nrolling\ndown the\nhill";
const STEGO_V2_CONCEALED: &str =
    "A little panda\u{2000}\nhas  fallen\u{2000}\nfrom a\ntree.\nThe\npanda\nwent\nrolling\ndown the\nhill";
const STEGO_V3_CONCEALED: &str =
    "A  little \npanda  has\u{2001}\nfallen\nfrom a\ntree. The\npanda went\nrolling\ndown the\nhill";

#[rstest]
#[case(4, STEGO_SINGLE_LETTERS, &[0b01100000, 0b01000000, 0b00], 24)]
#[case(4, STEGO_SINGLE_LETTERS_WITH_UNICODE_1, &[b'a', 0, 0b00], 24)]
#[case(4, STEGO_SINGLE_LETTERS_WITH_UNICODE_2, &[b'c', 0, 0b00], 24)]
#[case(10, STEGO_WITH_WORDS, &[0b01110000, 0b11000000], 12)]
#[case(10, STEGO_WITH_OTHER_WHITESPACE, &[0b11101111, 0b11000000, 0, 0b0000], 28)]
#[case(10, STEGO_WITH_SPECIAL_CHARS, &[0b11101110, 0b11000000, 0b0000000, 0b0000], 28)]
#[case(15, STEGO_HTML, &[0b11111101, 0, 0b00], 24)]
#[case(10, STEGO_V1_CONCEALED, &[0b11111111, 0, 0, 0], 32)]
fn reveals_data_variant_1(
    #[case] pivot: usize,
    #[case] stego_text: &str,
    #[case] expected: &[u8],
    #[case] expected_bit_len: usize
) -> Result<(), Box<dyn Error>> {
    let rng = StepRng::new(1, 1);
    let mut method = get_method(pivot, Variant::V1, rng);
    let data: BitVec<Msb0, u8> = method.try_reveal(stego_text)?;

    assert_eq!(data.len(), expected_bit_len);
    assert_eq!(data.as_raw_slice(), expected);
    Ok(())
}

#[rstest]
#[case(4, STEGO_SINGLE_LETTERS, &[0b01010000, 0b00010000, 0b00], 24)]
#[case(4, STEGO_SINGLE_LETTERS_WITH_UNICODE_1, &[0b01010010, 0, 0b00], 24)]
#[case(4, STEGO_SINGLE_LETTERS_WITH_UNICODE_2, &[0b01010110, 0, 0b00], 24)]
#[case(10, STEGO_WITH_WORDS, &[0b01110000, 0b10010000], 12)]
#[case(10, STEGO_WITH_OTHER_WHITESPACE, &[0b11011111, 0b10010000, 0, 0b0000], 28)]
#[case(10, STEGO_WITH_SPECIAL_CHARS, &[0b11011101, 0b10010000, 0b0000000, 0b0000], 28)]
#[case(15, STEGO_HTML, &[0b11111011, 0, 0], 24)]
#[case(8, STEGO_V2_CONCEALED, &[0b10101011, 0, 0, 0, 0], 40)]
fn reveals_data_variant_2(
    #[case] pivot: usize,
    #[case] stego_text: &str,
    #[case] expected: &[u8],
    #[case] expected_bit_len: usize
) -> Result<(), Box<dyn Error>> {
    let rng = StepRng::new(1, 1);
    let mut method = get_method(pivot, Variant::V2, rng);
    let data: BitVec<Msb0, u8> = method.try_reveal(stego_text)?;

    assert_eq!(data.len(), expected_bit_len);
    assert_eq!(data.as_raw_slice(), expected);
    Ok(())
}

#[rstest]
#[case(4, STEGO_SINGLE_LETTERS, &[0b10100000, 0b10000000, 0], 24)]
#[case(4, STEGO_SINGLE_LETTERS_WITH_UNICODE_1, &[0b10100001, 0, 0], 24)]
#[case(4, STEGO_SINGLE_LETTERS_WITH_UNICODE_2, &[0b10100011, 0, 0], 24)]
#[case(10, STEGO_WITH_WORDS, &[0b10110000, 0b11000000], 12)]
#[case(10, STEGO_WITH_OTHER_WHITESPACE, &[0b11101111, 0b11000000, 0, 0], 28)]
#[case(10, STEGO_WITH_SPECIAL_CHARS, &[0b11101110, 0b11000000, 0, 0], 28)]
#[case(15, STEGO_HTML, &[0b11111101, 0, 0], 24)]
#[case(10, STEGO_V3_CONCEALED, &[0b10101011, 0, 0, 0, 0], 36)]
fn reveals_data_variant_3(
    #[case] pivot: usize,
    #[case] stego_text: &str,
    #[case] expected: &[u8],
    #[case] expected_bit_len: usize
) -> Result<(), Box<dyn Error>> {
    let rng = StepRng::new(1, 1);
    let mut method = get_method(pivot, Variant::V3, rng);
    let data: BitVec<Msb0, u8> = method.try_reveal(stego_text)?;

    assert_eq!(data.len(), expected_bit_len);
    assert_eq!(data.as_raw_slice(), expected);
    Ok(())
}


#[test]
fn works_with_empty_data() -> Result<(), Box<dyn Error>> {
    let data_input: Vec<u8> = vec![0b0];
    let rng = StepRng::new(1, 1);
    let mut method = get_method(8, Variant::V1, rng);

    let stego_text =
        method.try_conceal(WITH_WORDS_TEXT, &mut data_input.view_bits::<Msb0>().iter())?;

    assert_eq!(
        stego_text,
        "A little\npanda\nhas\nfallen\nfrom a\ntree.\nThe\npanda\nwent\nrolling\ndown the\nhill"
    );
    Ok(())
}

#[test]
fn errors_when_cover_contains_word_longer_than_pivot() -> Result<(), Box<dyn Error>> {
    let data_input = bitvec![1; 8];
    let rng = StepRng::new(1, 1);
    let mut method = get_method(2, Variant::V1, rng);

    let stego_text = method.try_conceal(WITH_WORDS_TEXT, &mut data_input.iter());

    assert_eq!(
        stego_text,
        Err(ConcealError::pivot_too_small("little".to_string(), 2))
    );
    Ok(())
}

#[test]
fn errors_when_cover_is_too_small() -> Result<(), Box<dyn Error>> {
    let data_input = bitvec![1; 8];
    let rng = StepRng::new(1, 1);
    let mut method = get_method(5, Variant::V3, rng);

    let stego_text = method.try_conceal(TINY_TEXT, &mut data_input.iter());

    assert_eq!(stego_text, Err(ConcealError::no_cover_words_left(4, 5)));
    Ok(())
}

#[test]
fn errors_when_too_few_words() -> Result<(), Box<dyn Error>> {
    let data_input = bitvec![1; 8];
    let rng = StepRng::new(1, 1);
    let mut method = get_method(10, Variant::V3, rng);

    let stego_text = method.try_conceal(ONE_WORD_TEXT, &mut data_input.iter());

    assert_eq!(stego_text, Err(ConcealError::not_enough_words("Words.")));
    Ok(())
}

#[test]
fn errors_when_cover_is_empty() -> Result<(), Box<dyn Error>> {
    let data_input = bitvec![1; 8];
    let rng = StepRng::new(1, 1);
    let mut method = get_method(5, Variant::V3, rng);

    let stego_text = method.try_conceal(EMPTY_TEXT, &mut data_input.iter());

    assert_eq!(stego_text, Err(ConcealError::no_cover_words_left(8, 5)));
    Ok(())
}
