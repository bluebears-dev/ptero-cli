use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use bitvec::prelude::*;
use bitvec::view::AsBits;
use rand::RngCore;
use rand::rngs::mock::StepRng;

use ptero_common::method::SteganographyMethod;
use ptero_text::extended_line_method::{ConcealError, ExtendedLineMethod, Variant};

pub(crate) fn get_method(
    pivot: usize,
    variant: Variant,
    rng: &Rc<RefCell<dyn RngCore>>,
) -> ExtendedLineMethod {
    ExtendedLineMethod::builder()
        .with_pivot(pivot)
        .with_rng(rng)
        .with_variant(variant)
        .build()
}

const SINGLE_CHAR_TEXT: &str = "a b ca b ca b ca b ca b c";
const WITH_WORDS_TEXT: &str =
    "A little panda has fallen from a tree. The panda went rolling down the hill";
const WITH_OTHER_WHITESPACE_TEXT: &str = "A\tlittle  panda \
    has fallen from a tree. \
    The panda went rolling \
    down the\t hill";
const WITH_EMOJI_TEXT: &str =
    "A little 🐼 has (fallen) from a \\🌳/. The 🐼 went rolling down the hill.";
const HTML_TEXT: &str = "<div> \
    <button style=\" background: red;\">Click me</button> \
    <div/> \
    <footer> This is the end \
    </footer>";
const TINY_TEXT: &str = "TI NY COVER";
const ONE_WORD_TEXT: &str = "Words.";
const EMPTY_TEXT: &str = "";

#[test]
fn conceals_text_data() -> Result<(), Box<dyn Error>> {
    let data_input = "a";
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(4, Variant::V1, &rng);

    let stego_text =
        method.try_conceal(SINGLE_CHAR_TEXT, &mut data_input.as_bits::<Msb0>().iter())?;

    assert_eq!(stego_text, "a  b \nca b\nca  b\nca b\nca b\nc");
    Ok(())
}

#[test]
fn conceals_binary_data() -> Result<(), Box<dyn Error>> {
    let data_input = bitvec![1; 8];
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(3, Variant::V1, &rng);

    let stego_text = method.try_conceal(SINGLE_CHAR_TEXT, &mut data_input.iter())?;

    assert_eq!(stego_text, "a  b ca \nb  ca \nb  ca\nb\nca\nb c");
    Ok(())
}

#[test]
fn conceals_data_in_cover_with_words() -> Result<(), Box<dyn Error>> {
    let data_input: u8 = 255;
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(10, Variant::V1, &rng);

    let stego_text =
        method.try_conceal(WITH_WORDS_TEXT, &mut data_input.view_bits::<Msb0>().iter())?;

    assert_eq!(
        stego_text,
        "A  little panda \nhas  fallen from \na  tree. The\npanda went\nrolling\ndown the\nhill"
    );
    Ok(())
}

#[test]
fn conceals_data_in_cover_with_other_whitespace() -> Result<(), Box<dyn Error>> {
    let data_input: u8 = 255;
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(10, Variant::V1, &rng);

    let stego_text = method.try_conceal(
        WITH_OTHER_WHITESPACE_TEXT,
        &mut data_input.view_bits::<Msb0>().iter(),
    )?;

    assert_eq!(
        stego_text,
        "A  little panda \nhas  fallen from \na  tree. The\npanda went\nrolling\ndown the\nhill"
    );
    Ok(())
}

#[test]
fn conceals_data_in_cover_with_special_chars() -> Result<(), Box<dyn Error>> {
    let data_input: u8 = 255;
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(10, Variant::V1, &rng);

    let stego_text =
        method.try_conceal(WITH_EMOJI_TEXT, &mut data_input.view_bits::<Msb0>().iter())?;

    assert_eq!(
        stego_text,
        "A  little 🐼 has \n(fallen)  from \na  \\🌳/. The 🐼\nwent\nrolling\ndown the\nhill."
    );
    Ok(())
}

#[test]
fn conceals_data_in_html_cover() -> Result<(), Box<dyn Error>> {
    let data_input: u8 = 255;
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(15, Variant::V1, &rng);

    let stego_text = method.try_conceal(HTML_TEXT, &mut data_input.view_bits::<Msb0>().iter())?;

    assert_eq!(stego_text, "<div>  <button style=\" \nbackground:  red;\">Click \nme</button>  <div/>\n<footer> This\nis the end\n</footer>");
    Ok(())
}

#[test]
fn conceals_with_variant_v1() -> Result<(), Box<dyn Error>> {
    let data_input: Vec<u8> = vec![0b10101011];
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(10, Variant::V1, &rng);

    let stego_text =
        method.try_conceal(WITH_WORDS_TEXT, &mut data_input.view_bits::<Msb0>().iter())?;

    assert_eq!(
        stego_text,
        "A little panda \nhas  fallen\nfrom  a tree.\nThe panda\nwent\nrolling\ndown the\nhill"
    );
    Ok(())
}

#[test]
fn conceals_with_variant_v2() -> Result<(), Box<dyn Error>> {
    let data_input: Vec<u8> = vec![0b10101011];
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(8, Variant::V2, &rng);

    let stego_text =
        method.try_conceal(WITH_WORDS_TEXT, &mut data_input.view_bits::<Msb0>().iter())?;

    assert_eq!(
        stego_text,
        "A  little panda\nhas \nfallen from \na tree.\nThe\npanda\nwent\nrolling\ndown the\nhill"
    );
    Ok(())
}

#[test]
fn conceals_with_variant_v3() -> Result<(), Box<dyn Error>> {
    let data_input: Vec<u8> = vec![0b10101011];
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(10, Variant::V3, &rng);

    let stego_text =
        method.try_conceal(WITH_WORDS_TEXT, &mut data_input.view_bits::<Msb0>().iter())?;

    assert_eq!(
        stego_text,
        "A  little \npanda has fallen\nfrom  a tree.\nThe panda\nwent\nrolling\ndown the\nhill"
    );
    Ok(())
}

#[test]
fn works_with_empty_data() -> Result<(), Box<dyn Error>> {
    let data_input: Vec<u8> = vec![0b0];
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(8, Variant::V1, &rng);

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
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(2, Variant::V1, &rng);

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
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(5, Variant::V3, &rng);

    let stego_text = method.try_conceal(TINY_TEXT, &mut data_input.iter());

    assert_eq!(stego_text, Err(ConcealError::no_cover_words_left(5, 5)));
    Ok(())
}

#[test]
fn errors_when_too_few_words() -> Result<(), Box<dyn Error>> {
    let data_input = bitvec![1; 8];
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(10, Variant::V3, &rng);

    let stego_text = method.try_conceal(ONE_WORD_TEXT, &mut data_input.iter());

    assert_eq!(stego_text, Err(ConcealError::not_enough_words("Words.")));
    Ok(())
}

#[test]
fn errors_when_cover_is_empty() -> Result<(), Box<dyn Error>> {
    let data_input = bitvec![1; 8];
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(5, Variant::V3, &rng);

    let stego_text = method.try_conceal(EMPTY_TEXT, &mut data_input.iter());

    assert_eq!(stego_text, Err(ConcealError::no_cover_words_left(8, 5)));
    Ok(())
}

const STEGO_SINGLE_LETTERS: &str = "a  b \nca b\nca  b\nca b\nca b\nc";
const STEGO_WITH_WORDS: &str = "A  little \npanda has\nfallen  from";
const STEGO_WITH_OTHER_WHITESPACE: &str = "A  little panda \nhas  fallen from \na  tree. The\npanda went\nrolling\ndown the\nhill";
const STEGO_WITH_SPECIAL_CHARS: &str = "A  little 🐼 has \n(fallen)  from \na  \\🌳/. The 🐼\nwent\nrolling\ndown the\nhill.";
const STEGO_HTML: &str = "<div>  <button style=\" \nbackground:  red;\">Click \nme</button>  <div/>\n<footer> This\nis the end\n</footer>";
const STEGO_V1_CONCEALED: &str = "A little panda \nhas  fallen\nfrom  a tree.\nThe panda\nwent\nrolling\ndown the\nhill";
const STEGO_V2_CONCEALED: &str = "A  little panda\nhas \nfallen from \na tree.\nThe\npanda\nwent\nrolling\ndown the\nhill";
const STEGO_V3_CONCEALED: &str = "A  little \npanda has fallen\nfrom  a tree.\nThe panda\nwent\nrolling\ndown the\nhill";

#[test]
fn reveals_from_one_lettered_text() -> Result<(), Box<dyn Error>> {
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(4, Variant::V1, &rng);

    let data: BitVec<Msb0, u8> = method.try_reveal(STEGO_SINGLE_LETTERS)?;

    assert_eq!(data.len(), 18);
    assert_eq!(
        data.as_raw_slice(),
        &[b'a', 0b00000000, 0b00])
    ;
    Ok(())
}

#[test]
fn reveals_from_text() -> Result<(), Box<dyn Error>> {
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(4, Variant::V1, &rng);

    let data: BitVec<Msb0, u8> = method.try_reveal(STEGO_WITH_WORDS)?;

    assert_eq!(data.len(), 9);
    assert_eq!(
        data.as_raw_slice(),
        &[0b11110011, 0b0]
    );
    Ok(())
}

#[test]
fn reveals_from_text_with_other_whitespace() -> Result<(), Box<dyn Error>> {
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(10, Variant::V1, &rng);

    let revealed_data: BitVec<Msb0, u8> = method.try_reveal(STEGO_WITH_OTHER_WHITESPACE)?;

    assert_eq!(revealed_data.len(), 21);
    assert_eq!(
        revealed_data.as_raw_slice(),
        &[0b11111111, 0b00000000, 0b000000]
    );
    Ok(())
}

#[test]
fn reveals_from_text_with_special_chars() -> Result<(), Box<dyn Error>> {
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(10, Variant::V1, &rng);

    let revealed_data: BitVec<Msb0, u8> = method.try_reveal(STEGO_WITH_SPECIAL_CHARS)?;

    assert_eq!(revealed_data.len(), 21);
    assert_eq!(
        revealed_data.as_raw_slice(),
        &[0b11111111, 0b00000000, 0b00000]
    );
    Ok(())
}

#[test]
fn reveals_from_html() -> Result<(), Box<dyn Error>> {
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(15, Variant::V1, &rng);

    let revealed_data: BitVec<Msb0, u8> = method.try_reveal(STEGO_HTML)?;

    assert_eq!(revealed_data.len(), 18);
    assert_eq!(
        revealed_data.as_raw_slice(),
        &[0b11111111, 0b00000000, 0b00]
    );
    Ok(())
}

#[test]
fn reveals_from_text_with_variant_v1() -> Result<(), Box<dyn Error>> {
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(10, Variant::V1, &rng);

    let revealed_data: BitVec<Msb0, u8> = method.try_reveal(STEGO_V1_CONCEALED)?;

    assert_eq!(revealed_data.len(), 24);
    assert_eq!(
        revealed_data.as_raw_slice(),
        &[0b10101011, 0b00000000, 0b00000000]
    );
    Ok(())
}

#[test]
fn reveals_from_text_with_variant_v2() -> Result<(), Box<dyn Error>> {
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(8, Variant::V2, &rng);

    let revealed_data: BitVec<Msb0, u8> = method.try_reveal(STEGO_V2_CONCEALED)?;

    assert_eq!(revealed_data.len(), 30);
    assert_eq!(
        revealed_data.as_raw_slice(),
        &[0b10101011, 0b00000000, 0b00000000, 0b00000000]
    );
    Ok(())
}

#[test]
fn reveals_from_text_with_variant_v3() -> Result<(), Box<dyn Error>> {
    let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    let mut method = get_method(10, Variant::V3, &rng);

    let revealed_data: BitVec<Msb0, u8> = method.try_reveal(STEGO_V3_CONCEALED)?;

    assert_eq!(revealed_data.len(), 24);
    assert_eq!(
        revealed_data.as_raw_slice(),
        &[0b10101011, 0b00000000, 0b00000000]
    );
    Ok(())
}