use std::error::Error;

use rand::rngs::mock::StepRng;

use ptero_text::extended_line_method::{ExtendedLineMethod, Variant};
use ptero_text::extended_line_method::character_sets::CharacterSetType;

fn report_error<T>(result: Result<T, Box<dyn Error>>) {
    if let Err(e) = result {
        panic!("{}", e);
    }
}

#[test]
#[should_panic(expected = "Couldn't finish building ExtendedLineMethod: `rng` must be initialized")]
fn errors_when_rng_not_provided() {
    let builder = ExtendedLineMethod::builder()
        .with_trailing_charset(CharacterSetType::OneBit)
        .with_pivot(20)
        .with_variant(Variant::V1);

    if let Err(e) = builder.build() {
        panic!("{}", e);
    }
}

#[test]
fn has_default_pivot_provided() {
    ExtendedLineMethod::builder()
        .with_rng(StepRng::new(1, 1))
        .with_trailing_charset(CharacterSetType::OneBit)
        .with_variant(Variant::V1)
        .build()
        .unwrap();
}

#[test]
fn has_default_charset_provided() {
    ExtendedLineMethod::builder()
        .with_rng(StepRng::new(1, 1))
        .with_pivot(20)
        .with_variant(Variant::V1)
        .build()
        .unwrap();
}

#[test]
fn has_default_variant_provided() {
    ExtendedLineMethod::builder()
        .with_rng(StepRng::new(1, 1))
        .with_trailing_charset(CharacterSetType::OneBit)
        .with_pivot(20)
        .build()
        .unwrap();
}
