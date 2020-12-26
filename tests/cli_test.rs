use log::{debug, info};
use std::{error::Error, fs, panic, path::PathBuf};
use utils::{global_setup, run_encode_command, run_decode_command};

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
fn correctly_decodes_data_from_utf8_stego_text() -> Result<(), Box<dyn Error>> {
    global_setup();
    let mut res_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    res_dir.push("resources");
    let data_path = res_dir.join("data.txt");
    let stego_path = res_dir.join("stego/stego_utf8.txt");

    debug!("Decoding to JSON format");
    let json_struct = run_decode_command(&stego_path, 50, None)?;
    assert_eq!(json_struct["type"].as_str(), Some("success"));

    debug!("Reading the secret data file");
    let data = fs::read_to_string(&data_path)?;

    info!("Checking if decoded output starts with secret text");
    let has_secret_data = json_struct["result"]
        .as_str()
        .unwrap()
        .starts_with(data.as_str());
    assert!(has_secret_data);

    Ok(())
}

#[test]
fn encodes_and_decodes_the_same_data() -> Result<(), Box<dyn Error>> {
    global_setup();
    let mut res_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    res_dir.push("resources");
    let data_path = res_dir.join("data.txt");
    let cover_path = res_dir.join("cover/cover_ascii.txt");
    let encoding_output = PathBuf::from("encode_out");

    debug!("Encoding and saving to file");
    run_encode_command(&cover_path, &data_path, 50, Some(&encoding_output))?;

    debug!("Decoding from file '{:?}' to JSON", &encoding_output);
    let json_struct = run_decode_command(&encoding_output, 50, None)?;
    assert_eq!(json_struct["type"].as_str(), Some("success"));

    debug!("Reading the secret data file");
    let data = fs::read_to_string(&data_path)?;

    info!("Checking if decoded output starts with secret text");
    let has_secret_data = json_struct["result"]
        .as_str()
        .unwrap()
        .starts_with(data.as_str());
    assert!(has_secret_data);

    Ok(())
}
