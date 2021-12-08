use std::error::Error;

use bitvec::prelude::*;
use bitvec::view::BitView;
use rand::RngCore;
use rand::rngs::mock::StepRng;
use rstest::*;

use ptero_common::method::SteganographyMethod;
use ptero_text::extended_line_method::{ConcealError, ExtendedLineMethod, Variant};
use ptero_text::extended_line_method::character_sets::CharacterSetType::OneBit;

use crate::extended_line_method_test::*;

#[rstest]
#[case(4, 97u8, SINGLE_CHAR_TEXT, "a  b \nca b\nca  b\nca b\nca b\nc")]
#[case(10, 255u8, WITH_WORDS_TEXT, "A  little panda \nhas  fallen from \na  tree. The\npanda went\nrolling\ndown the\nhill")]
#[case(10, 255u8, WITH_OTHER_WHITESPACE_TEXT, "A  little panda \nhas  fallen from \na  tree. The\npanda went\nrolling\ndown the\nhill")]
#[case(10, 255u8, WITH_EMOJI_TEXT, "A  little 🐼 has \n(fallen)  from \na  \\🌳/. The 🐼\nwent\nrolling\ndown the\nhill.")]
#[case(15, 255u8, HTML_TEXT, "<div>  <button style=\" \nbackground:  red;\">Click \nme</button>  <div/>\n<footer> This\nis the end\n</footer>")]
#[case(10, 171u8, WITH_WORDS_TEXT, "A little panda \nhas  fallen\nfrom  a tree.\nThe panda\nwent\nrolling\ndown the\nhill")]
fn conceals_data_variant_1<T>(
    #[case] pivot: usize,
    #[case] data: T,
    #[case] cover: &str,
    #[case] expected: &str,
) -> Result<(), Box<dyn Error>>
where T: BitStore {
    let rng = StepRng::new(1, 1);
    let mut method = get_method(pivot, Variant::V1, OneBit, rng);
    let stego_text = method.try_conceal(cover, &mut data.view_bits::<Msb0>().iter())?;

    assert_eq!(stego_text, expected);
    Ok(())
}

#[rstest]
#[case(4, 97u8, SINGLE_CHAR_TEXT, "a  b \nca b\nca b \nca b\nca b\nc")]
#[case(10, 255u8, WITH_WORDS_TEXT, "A  little panda \nhas  fallen from \na tree. The \npanda went\nrolling\ndown the\nhill")]
#[case(10, 255u8, WITH_OTHER_WHITESPACE_TEXT, "A  little panda \nhas  fallen from \na tree. The \npanda went\nrolling\ndown the\nhill")]
#[case(10, 255u8, WITH_EMOJI_TEXT, "A  little 🐼 has \n(fallen)  from \na \\🌳/. The 🐼 \nwent\nrolling\ndown the\nhill.")]
#[case(15, 255u8, HTML_TEXT, "<div>  <button style=\" \nbackground:  red;\">Click \nme</button> <div/> \n<footer> This\nis the end\n</footer>")]
#[case(8, 171u8, WITH_WORDS_TEXT, "A  little panda\nhas \nfallen from \na tree.\nThe\npanda\nwent\nrolling\ndown the\nhill")]
fn conceals_data_variant_2<T>(
    #[case] pivot: usize,
    #[case] data: T,
    #[case] cover: &str,
    #[case] expected: &str,
) -> Result<(), Box<dyn Error>>
where T: BitStore {
    let rng = StepRng::new(1, 1);
    let mut method = get_method(pivot, Variant::V2, OneBit, rng);
    let stego_text = method.try_conceal(cover, &mut data.view_bits::<Msb0>().iter())?;

    assert_eq!(stego_text, expected);
    Ok(())
}

#[rstest]
#[case(4, 97u8, SINGLE_CHAR_TEXT, "a b ca \nb ca\nb ca b\nca b\nc")]
#[case(10, 255u8, WITH_WORDS_TEXT, "A  little panda \nhas  fallen from \na  tree. The\npanda went\nrolling\ndown the\nhill")]
#[case(10, 255u8, WITH_OTHER_WHITESPACE_TEXT, "A  little panda \nhas  fallen from \na  tree. The\npanda went\nrolling\ndown the\nhill")]
#[case(13, 255u8, WITH_EMOJI_TEXT, "A  little 🐼 has \n(fallen)  from a \n\\🌳/.  The 🐼 went\nrolling down\nthe hill.")]
#[case(24, 255u8, HTML_TEXT, "<div>  <button style=\" background: \nred;\">Click  me</button> <div/> \n<footer>  This is the end </footer>")]
#[case(10, 171u8, WITH_WORDS_TEXT, "A  little \npanda has fallen\nfrom  a tree.\nThe panda\nwent\nrolling\ndown the\nhill")]
fn conceals_data_variant_3<T>(
    #[case] pivot: usize,
    #[case] data: T,
    #[case] cover: &str,
    #[case] expected: &str,
) -> Result<(), Box<dyn Error>>
where T: BitStore {
    let rng = StepRng::new(1, 1);
    let mut method = get_method(pivot, Variant::V3, OneBit, rng);
    let stego_text = method.try_conceal(cover, &mut data.view_bits::<Msb0>().iter())?;

    assert_eq!(stego_text, expected);
    Ok(())
}

const STEGO_SINGLE_LETTERS: &str = "a  b \nca b\nca  b\nca b\nca b\nc";
const STEGO_WITH_WORDS: &str = "A  little \npanda has\nfallen  from";
const STEGO_WITH_OTHER_WHITESPACE: &str =
    "A  little panda \nhas  fallen from \na  tree. The\npanda went\nrolling\ndown the\nhill";
const STEGO_WITH_SPECIAL_CHARS: &str =
    "A  little 🐼 has \n(fallen)  from \na  \\🌳/. The 🐼\nwent\nrolling\ndown the\nhill.";
const STEGO_HTML: &str = "<div>  <button style=\" \nbackground:  red;\">Click \nme</button>  <div/>\n<footer> This\nis the end\n</footer>";
const STEGO_V1_CONCEALED: &str =
    "A little panda \nhas  fallen\nfrom  a tree.\nThe panda\nwent\nrolling\ndown the\nhill";
const STEGO_V2_CONCEALED: &str =
    "A  little panda\nhas \nfallen from \na tree.\nThe\npanda\nwent\nrolling\ndown the\nhill";
const STEGO_V3_CONCEALED: &str =
    "A  little \npanda has fallen\nfrom  a tree.\nThe panda\nwent\nrolling\ndown the\nhill";

#[rstest]
#[case(4, STEGO_SINGLE_LETTERS, &[b'a', 0, 0b00], 18)]
#[case(10, STEGO_WITH_WORDS, &[0b01100011, 0b0], 9)]
#[case(10, STEGO_WITH_OTHER_WHITESPACE, &[0b11111111, 0, 0b000000], 21)]
#[case(10, STEGO_WITH_SPECIAL_CHARS, &[0b11111111, 0, 0b00000], 21)]
#[case(15, STEGO_HTML, &[0b11111111, 0, 0b00], 18)]
#[case(10, STEGO_V1_CONCEALED, &[0b10101011, 0, 0], 24)]
fn reveals_data_variant_1(
    #[case] pivot: usize,
    #[case] stego_text: &str,
    #[case] expected: &[u8],
    #[case] expected_bit_len: usize
) -> Result<(), Box<dyn Error>> {
    let rng = StepRng::new(1, 1);
    let mut method = get_method(pivot, Variant::V1, OneBit, rng);
    let data: BitVec<Msb0, u8> = method.try_reveal(stego_text)?;

    assert_eq!(data.len(), expected_bit_len);
    assert_eq!(data.as_raw_slice(), expected);
    Ok(())
}

#[rstest]
#[case(4, STEGO_SINGLE_LETTERS, &[0b01100000, 0b10000000, 0b00], 18)]
#[case(10, STEGO_WITH_WORDS, &[0b01100010, 0b10000000], 9)]
#[case(10, STEGO_WITH_OTHER_WHITESPACE, &[0b11111110, 0b10000000, 0b000000], 21)]
#[case(10, STEGO_WITH_SPECIAL_CHARS, &[0b11111110, 0b10000000, 0b00000], 21)]
#[case(15, STEGO_HTML, &[0b11111110, 0b10000000, 0b00], 18)]
#[case(8, STEGO_V2_CONCEALED, &[0b10101011, 0, 0, 0], 30)]
fn reveals_data_variant_2(
    #[case] pivot: usize,
    #[case] stego_text: &str,
    #[case] expected: &[u8],
    #[case] expected_bit_len: usize
) -> Result<(), Box<dyn Error>> {
    let rng = StepRng::new(1, 1);
    let mut method = get_method(pivot, Variant::V2, OneBit, rng);
    let data: BitVec<Msb0, u8> = method.try_reveal(stego_text)?;

    assert_eq!(data.len(), expected_bit_len);
    assert_eq!(data.as_raw_slice(), expected);
    Ok(())
}

#[rstest]
#[case(4, STEGO_SINGLE_LETTERS, &[0b10100010, 0, 0b00], 18)]
#[case(10, STEGO_WITH_WORDS, &[0b10100011, 0], 9)]
#[case(10, STEGO_WITH_OTHER_WHITESPACE, &[0b11111111, 0, 0b000000], 21)]
#[case(10, STEGO_WITH_SPECIAL_CHARS, &[0b11111111, 0, 0b00000], 21)]
#[case(15, STEGO_HTML, &[0b11111111, 0, 0b00], 18)]
#[case(10, STEGO_V3_CONCEALED, &[0b10101011, 0, 0], 24)]
fn reveals_data_variant_3(
    #[case] pivot: usize,
    #[case] stego_text: &str,
    #[case] expected: &[u8],
    #[case] expected_bit_len: usize
) -> Result<(), Box<dyn Error>> {
    let rng = StepRng::new(1, 1);
    let mut method = get_method(pivot, Variant::V3, OneBit, rng);
    let data: BitVec<Msb0, u8> = method.try_reveal(stego_text)?;

    assert_eq!(data.len(), expected_bit_len);
    assert_eq!(data.as_raw_slice(), expected);
    Ok(())
}


#[test]
fn works_with_empty_data() -> Result<(), Box<dyn Error>> {
    let data_input: Vec<u8> = vec![0b0];
    let rng = StepRng::new(1, 1);
    let mut method = get_method(8, Variant::V1, OneBit, rng);

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
    let mut method = get_method(2, Variant::V1, OneBit, rng);

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
    let mut method = get_method(5, Variant::V3, OneBit, rng);

    let stego_text = method.try_conceal(TINY_TEXT, &mut data_input.iter());

    assert_eq!(stego_text, Err(ConcealError::no_cover_words_left(5, 5)));
    Ok(())
}

#[test]
fn errors_when_too_few_words() -> Result<(), Box<dyn Error>> {
    let data_input = bitvec![1; 8];
    let rng = StepRng::new(1, 1);
    let mut method = get_method(10, Variant::V3, OneBit, rng);

    let stego_text = method.try_conceal(ONE_WORD_TEXT, &mut data_input.iter());

    assert_eq!(stego_text, Err(ConcealError::not_enough_words("Words.")));
    Ok(())
}

#[test]
fn errors_when_cover_is_empty() -> Result<(), Box<dyn Error>> {
    let data_input = bitvec![1; 8];
    let rng = StepRng::new(1, 1);
    let mut method = get_method(5, Variant::V3, OneBit, rng);

    let stego_text = method.try_conceal(EMPTY_TEXT, &mut data_input.iter());

    assert_eq!(stego_text, Err(ConcealError::no_cover_words_left(8, 5)));
    Ok(())
}
