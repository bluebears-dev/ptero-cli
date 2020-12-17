use predicates::prelude::*;
use std::{error::Error, fs, path::PathBuf};

mod utils;

fn properly_encodes(
    cover: &str,
    data: &str,
    output: &str,
    pivot: usize,
) -> Result<(), Box<dyn Error>> {
    let mut res_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let encoding_output = PathBuf::from(&output);
    res_dir.push("resources");

    let data_file = res_dir.join(&data);
    let encode_cmd = utils::encode_command(
        &res_dir.join(&cover),
        &data_file,
        Some(pivot),
        Some(encoding_output.to_str().unwrap()),
    );
    encode_cmd?.assert().success();

    let data_text = fs::read_to_string(data_file)?;
    let decode_cmd = utils::decode_command(&PathBuf::from(&output), pivot, None);
    println!("DATA TEXT {:?}", &data_text);
    decode_cmd?
        .assert()
        .success()
        .stdout(predicate::str::contains(data_text));
    Ok(())
}

fn output_is_not_malformed(
    cover: &str,
    data: &str,
    output: &str,
    pivot: usize,
) -> Result<(), Box<dyn Error>> {
    let mut res_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let encoding_output = PathBuf::from(&output);
    res_dir.push("resources");

    let data_file = res_dir.join(&data);
    let encode_cmd = utils::encode_command(
        &res_dir.join(&cover),
        &data_file,
        Some(pivot),
        Some(encoding_output.to_str().unwrap()),
    );
    encode_cmd?.assert().success();
    let encoded_text = fs::read(&encoding_output)?;
    String::from_utf8(encoded_text)?;
    Ok(())
}

#[test]
fn output_is_not_malformed_ascii() -> Result<(), Box<dyn Error>> {
    output_is_not_malformed("cover/cover_ascii.txt", "data.txt", "encode_out", 50)
}

#[test]
fn properly_encodes_on_cover_ascii() -> Result<(), Box<dyn Error>> {
    properly_encodes("cover/cover_ascii.txt", "data.txt", "encode_out", 50)
}
#[test]
fn output_is_not_malformed_utf8() -> Result<(), Box<dyn Error>> {
    output_is_not_malformed("cover/cover_utf8.txt", "data.txt", "encode_out", 50)
}

#[test]
fn properly_encodes_on_cover_utf8() -> Result<(), Box<dyn Error>> {
    properly_encodes("cover/cover_utf8.txt", "data.txt", "encode_out", 50)
}
