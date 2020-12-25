use log::{debug, info};
use utils::{global_setup, run_encode_command};
use std::{error::Error, fs, panic, path::PathBuf};

mod utils;

#[test]
fn does_not_fail_when_encoding_over_ascii_cover() -> Result<(), Box<dyn Error>> {
    global_setup();
    let mut res_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    res_dir.push("resources");
    let data_path = res_dir.join("data.txt");
    let cover_path = res_dir.join("cover/cover_ascii.txt");

    debug!("Encoding to JSON format");
    let json_struct = run_encode_command(&cover_path, &data_path, 50, None)?;
    assert_eq!(json_struct["type"].as_str(), Some("success"));
    Ok(())
}

#[test]
fn does_not_fail_when_encoding_over_uft8_cover() -> Result<(), Box<dyn Error>> {
    global_setup();
    let mut res_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    res_dir.push("resources");
    let data_path = res_dir.join("data.txt");
    let cover_path = res_dir.join("cover/cover_utf8.txt");

    debug!("Encoding to JSON format");
    let json_struct = run_encode_command(&cover_path, &data_path, 50, None)?;
    assert_eq!(json_struct["type"].as_str(), Some("success"));
    Ok(())
}

#[test]
fn encoding_output_is_not_malformed_utf8_when_saving_to_file() -> Result<(), Box<dyn Error>> {
    global_setup();
    let mut res_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let encoding_output = PathBuf::from("encode_out");
    res_dir.push("resources");
    let data_path = res_dir.join("data.txt");
    let cover_path = res_dir.join("cover/cover_utf8.txt");

    debug!("Encoding and saving to file");
    run_encode_command(&cover_path, &data_path, 50, Some(&encoding_output))?;

    info!("Reading UTF-8 from output stego text");
    let encoded_text = fs::read(&encoding_output)?;
    String::from_utf8(encoded_text)?;

    debug!("Removing the '{:?}' file", &encoding_output);
    fs::remove_file(&encoding_output)?;
    Ok(())
}

