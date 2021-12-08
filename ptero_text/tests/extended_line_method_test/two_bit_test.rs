use std::error::Error;

use bitvec::prelude::*;
use bitvec::view::BitView;
use rand::rngs::mock::StepRng;
use rstest::*;

use ptero_common::method::SteganographyMethod;
use ptero_text::extended_line_method::{ConcealError, Variant};
use ptero_text::extended_line_method::character_sets::CharacterSetType::TwoBit;

use crate::extended_line_method_test::*;
use crate::test_resource::ResourceLoader;

#[fixture]
fn stego_text_loader() -> ResourceLoader {
    let dir_path = PathBuf::new()
        .join("resources")
        .join("stego_texts")
        .join("extended_line")
        .join("two_bit");

    ResourceLoader::new(&dir_path)
}

#[fixture]
fn v1_builder() -> ExtendedLineMethodBuilder {
    pre_build_method_with(Variant::V1, TwoBit, StepRng::new(1, 1))
}

#[fixture]
fn v2_builder() -> ExtendedLineMethodBuilder {
    pre_build_method_with(Variant::V2, TwoBit, StepRng::new(1, 1))
}

#[fixture]
fn v3_builder() -> ExtendedLineMethodBuilder {
    pre_build_method_with(Variant::V3, TwoBit, StepRng::new(1, 1))
}

#[rustfmt::skip]
#[rstest]
// Variant 1 test cases
#[case::variant_1(v1_builder(), 4, 97u8, "single_char_short_text", "single_char_short_text_variant_1")]
#[case::variant_1(v1_builder(), 15, [121u8; 3], "single_char_long_text", "single_char_long_text_variant_1")]
#[case::variant_1(v1_builder(), 10, 255u8, "short_text", "short_text_variant_1")]
#[case::variant_1(v1_builder(), 20, [197u8; 18], "long_text", "long_text_variant_1")]
#[case::variant_1(v1_builder(), 20, [726u16; 2], "unicode_text", "unicode_text_variant_1")]
#[case::variant_1(v1_builder(), 28, 65542312u32, "html_text.html", "html_text_variant_1.html")]
// Variant 2 test cases
#[case::variant_2(v2_builder(), 4, 97u8, "single_char_short_text", "single_char_short_text_variant_2")]
#[case::variant_2(v2_builder(), 15, [121u8; 3], "single_char_long_text", "single_char_long_text_variant_2")]
#[case::variant_2(v2_builder(), 10, 255u8, "short_text", "short_text_variant_2")]
#[case::variant_2(v2_builder(), 20, [197u8; 18], "long_text", "long_text_variant_2")]
#[case::variant_2(v2_builder(), 20, [726u16; 2], "unicode_text", "unicode_text_variant_2")]
#[case::variant_2(v2_builder(), 28, 65542312u32, "html_text.html", "html_text_variant_2.html")]
// Variant 3 test cases
#[case::variant_3(v3_builder(), 4, 97u8, "single_char_short_text", "single_char_short_text_variant_3")]
#[case::variant_3(v3_builder(), 15, [121u8; 3], "single_char_long_text", "single_char_long_text_variant_3")]
#[case::variant_3(v3_builder(), 10, 255u8, "short_text", "short_text_variant_3")]
#[case::variant_3(v3_builder(), 20, [197u8; 18], "long_text", "long_text_variant_3")]
#[case::variant_3(v3_builder(), 20, [726u16; 2], "unicode_text", "unicode_text_variant_3")]
#[case::variant_3(v3_builder(), 30, [231u8; 3], "html_text.html", "html_text_variant_3.html")]
fn conceals_data<T>(
    cover_text_loader: ResourceLoader,
    stego_text_loader: ResourceLoader,
    #[case] method_builder: ExtendedLineMethodBuilder,
    #[case] pivot: usize,
    #[case] data: T,
    #[case] cover_path: &str,
    #[case] expected_path: &str,
) -> Result<(), Box<dyn Error>>
    where
        T: BitView,
{
    println!("Testing concealing into: '{}'", cover_path);
    println!("Comparing with result in: '{}'", expected_path);
    let mut method = method_builder.with_pivot(pivot).build()?;

    let cover = cover_text_loader.load_resource(&PathBuf::from(cover_path));
    let expected = stego_text_loader.load_resource(&PathBuf::from(expected_path));

    let stego_text = method.try_conceal(&cover, &mut data.view_bits::<Msb0>().iter())?;

    assert_eq!(stego_text, expected);
    Ok(())
}

#[rustfmt::skip]
#[rstest]
// Variant 1 test cases
#[case::variant_1(v1_builder(), 4, "single_char_short_text_variant_1", &[97], 15 * 4)]
#[case::variant_1(v1_builder(), 15, "single_char_long_text_variant_1", &[121; 3], 39 * 4)]
#[case::variant_1(v1_builder(), 10, "short_text_variant_1", &[255], 13 * 4)]
#[case::variant_1(v1_builder(), 20, "long_text_variant_1", &[197; 18], 65 * 4)]
#[case::variant_1(v1_builder(), 20, "unicode_text_variant_1", &[2, 214, 2, 214], 26 * 4)]
#[case::variant_1(v1_builder(), 28, "html_text_variant_1.html", &[3, 232, 24, 168], 11 * 4)]
// Variant 2 test cases
#[case::variant_2(v2_builder(), 4, "single_char_short_text_variant_2", &[97], 15 * 4)]
#[case::variant_2(v2_builder(), 15, "single_char_long_text_variant_2", &[121; 3], 39 * 4)]
#[case::variant_2(v2_builder(), 10, "short_text_variant_2", &[255], 13 * 4)]
#[case::variant_2(v2_builder(), 20, "long_text_variant_2", &[197; 18], 65 * 4)]
#[case::variant_2(v2_builder(), 20, "unicode_text_variant_2", &[2, 214, 2, 214], 26 * 4)]
#[case::variant_2(v2_builder(), 28, "html_text_variant_2.html", &[3, 232, 24, 168], 11 * 4)]
// Variant 3 test cases
#[case::variant_3(v3_builder(), 4, "single_char_short_text_variant_3", &[97], 14 * 4)]
#[case::variant_3(v3_builder(), 15, "single_char_long_text_variant_3", &[121; 3], 38 * 4)]
#[case::variant_3(v3_builder(), 10, "short_text_variant_3", &[255], 13 * 4)]
#[case::variant_3(v3_builder(), 20, "long_text_variant_3", &[197; 18], 58 * 4)]
#[case::variant_3(v3_builder(), 20, "unicode_text_variant_3", &[2, 214, 2, 214], 25 * 4)]
#[case::variant_3(v3_builder(), 30, "html_text_variant_3.html", &[231; 3], 9 * 4)]
// Additional tests - non-matching stego texts variants
#[case::non_matching(
    v1_builder(), 7, "long_text_variant_2",
    &[174, 174, 174, 174, 174, 174, 174, 174, 174, 174, 174, 174, 174, 174, 174, 174, 174, 174, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 128],
    65 * 4
)]
#[case::non_matching(
    v1_builder(), 7, "long_text_variant_3",
    &[201, 201, 201, 201, 201, 201, 201, 201, 201, 201, 201, 201, 201, 201, 201, 201, 201, 201, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136],
    58 * 4
)]
#[case::non_matching(
    v2_builder(), 7, "long_text_variant_1",
    &[155, 155, 155, 155, 155, 155, 155, 155, 155, 155, 155, 155, 155, 155, 155, 155, 155, 155, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 128],
    65 * 4
)]
#[case::non_matching(
    v2_builder(), 7, "long_text_variant_3",
    &[154, 154, 154, 154, 154, 154, 154, 154, 154, 154, 154, 154, 154, 154, 154, 154, 154, 154, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136],
    58 * 4
)]
#[case::non_matching(
    v3_builder(), 7, "long_text_variant_1",
    &[205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 64],
    65 * 4
)]
#[case::non_matching(
    v3_builder(), 7, "long_text_variant_2",
    &[110, 110, 110, 110, 110, 110, 110, 110, 110, 110, 110, 110, 110, 110, 110, 110, 110, 110, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 64],
    65 * 4
)]
fn reveals_data(
    stego_text_loader: ResourceLoader,
    #[case] method_builder: ExtendedLineMethodBuilder,
    #[case] pivot: usize,
    #[case] stego_path: &str,
    #[case] expected_data: &[u8],
    #[case] expected_bit_amount: usize,
) -> Result<(), Box<dyn Error>> {
    println!("Revealing from: '{}' with pivot '{}'", stego_path, pivot);
    let stego_text = stego_text_loader.load_resource(&PathBuf::from(stego_path));
    let mut method = method_builder.with_pivot(pivot).build()?;
    let data: BitVec<Msb0, u8> = method.try_reveal(&stego_text)?;

    assert_eq!(data.len(), expected_bit_amount);

    let nonzero_data: Vec<u8> = data.as_raw_slice()
        .iter()
        .filter(|v| **v > 0)
        .copied()
        .collect();

    assert_eq!(&nonzero_data, expected_data);
    Ok(())
}

#[test]
fn works_with_empty_data() -> Result<(), Box<dyn Error>> {
    let data_input: Vec<u8> = vec![0b0];
    let rng = StepRng::new(1, 1);
    let mut method = get_method(8, Variant::V1, TwoBit, rng);

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
    let mut method = get_method(2, Variant::V1, TwoBit, rng);

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
    let mut method = get_method(5, Variant::V3, TwoBit, rng);

    let stego_text = method.try_conceal(TINY_TEXT, &mut data_input.iter());

    assert_eq!(stego_text, Err(ConcealError::no_cover_words_left(4, 5)));
    Ok(())
}

#[test]
fn errors_when_too_few_words() -> Result<(), Box<dyn Error>> {
    let data_input = bitvec![1; 8];
    let rng = StepRng::new(1, 1);
    let mut method = get_method(10, Variant::V3, TwoBit, rng);

    let stego_text = method.try_conceal(ONE_WORD_TEXT, &mut data_input.iter());

    assert_eq!(stego_text, Err(ConcealError::not_enough_words("Words.")));
    Ok(())
}

#[test]
fn errors_when_cover_is_empty() -> Result<(), Box<dyn Error>> {
    let data_input = bitvec![1; 8];
    let rng = StepRng::new(1, 1);
    let mut method = get_method(5, Variant::V3, TwoBit, rng);

    let stego_text = method.try_conceal(EMPTY_TEXT, &mut data_input.iter());

    assert_eq!(stego_text, Err(ConcealError::no_cover_words_left(8, 5)));
    Ok(())
}
