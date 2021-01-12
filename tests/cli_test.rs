use assert_cmd::Command;
use log::{debug, info};
use std::{error::Error, fs, panic, path::PathBuf};
use utils::{global_setup, run_decode_command, run_encode_command};

mod utils;

fn check_if_does_not_fail_when_encoding_over_ascii_cover(
    data: &PathBuf,
    cover: &PathBuf,
    method: &str,
) -> Result<(), Box<dyn Error>> {
    debug!("Checking for method: {}", method);

    debug!("Encoding to JSON format");
    let json_struct = run_encode_command(cover, data, 50, None, method)?;
    assert_eq!(json_struct["type"].as_str(), Some("success"));
    Ok(())
}

fn check_if_correctly_decodes_data_from_utf8_stego_text(
    data: &PathBuf,
    stego_text: &PathBuf,
    method: &str,
) -> Result<(), Box<dyn Error>> {
    debug!("Checking for method: {}", method);

    debug!("Decoding to JSON format");
    let json_struct = run_decode_command(&stego_text, 50, None, method)?;
    assert_eq!(json_struct["type"].as_str(), Some("success"));

    debug!("Reading the secret data file");
    let data = fs::read_to_string(&data)?;

    info!("Checking if decoded output starts with secret text");
    let has_secret_data = json_struct["result"]
        .as_str()
        .unwrap()
        .starts_with(data.as_str());
    assert!(has_secret_data);

    Ok(())
}

fn check_if_encodes_and_decodes_the_same_data(
    data: &PathBuf,
    cover: &PathBuf,
    method: &str,
) -> Result<(), Box<dyn Error>> {
    debug!("Checking for method: {}", method);
    let encoding_output_path = PathBuf::from("encode_out");

    info!("Encoding and saving to file");
    run_encode_command(&cover, &data, 50, Some(&encoding_output_path), method)?;

    debug!("Decoding from file {:?} to JSON", &encoding_output_path);
    let json_struct = run_decode_command(&encoding_output_path, 50, None, method)?;
    assert_eq!(json_struct["type"].as_str(), Some("success"));
    debug!("Decoded data: {:?}", &json_struct["result"]);

    let data = fs::read_to_string(&data)?;
    debug!("Secret data: {:?}", &data);

    info!("Checking if decoded output starts with secret text");
    let has_secret_data = json_struct["result"]
        .as_str()
        .unwrap()
        .starts_with(data.as_str());
    assert!(has_secret_data);

    Ok(())
}

#[test]
fn does_not_fail_when_encoding_over_ascii_cover() -> Result<(), Box<dyn Error>> {
    global_setup();
    let mut res_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    res_dir.push("resources");
    let data_path = res_dir.join("data.txt");
    let cover_path = res_dir.join("cover/cover_ascii.txt");

    check_if_does_not_fail_when_encoding_over_ascii_cover(&data_path, &cover_path, "eline")?;
    check_if_does_not_fail_when_encoding_over_ascii_cover(&data_path, &cover_path, "eluv")
}

#[test]
fn does_not_fail_when_encoding_over_utf8_cover() -> Result<(), Box<dyn Error>> {
    global_setup();
    let mut res_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    res_dir.push("resources");
    let data_path = res_dir.join("data.txt");
    let cover_path = res_dir.join("cover/cover_utf8.txt");

    check_if_does_not_fail_when_encoding_over_ascii_cover(&data_path, &cover_path, "eline")?;
    check_if_does_not_fail_when_encoding_over_ascii_cover(&data_path, &cover_path, "eluv")
}

#[test]
fn correctly_decodes_data_from_utf8_stego_text_when_eline_method_is_used(
) -> Result<(), Box<dyn Error>> {
    global_setup();
    let mut res_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    res_dir.push("resources");

    let data_path = res_dir.join("data.txt");
    let stego_path_eline = res_dir.join("stego/stego_utf8_eline.txt");
    let stego_path_eluv = res_dir.join("stego/stego_utf8_eluv.txt");

    check_if_correctly_decodes_data_from_utf8_stego_text(&data_path, &stego_path_eline, "eline")?;
    check_if_correctly_decodes_data_from_utf8_stego_text(&data_path, &stego_path_eluv, "eluv")
}

#[test]
fn encodes_and_decodes_the_same_data() -> Result<(), Box<dyn Error>> {
    global_setup();
    let mut res_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    res_dir.push("resources");

    let data_path = res_dir.join("data.txt");
    let cover_path = res_dir.join("cover/cover_ascii.txt");

    check_if_encodes_and_decodes_the_same_data(&data_path, &cover_path, "eline")?;
    check_if_encodes_and_decodes_the_same_data(&data_path, &cover_path, "eluv")
}

#[test]
fn eluv_encode_command_passes_without_set_param() -> Result<(), Box<dyn Error>> {
    global_setup();
    let mut res_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    res_dir.push("resources");

    let data_path = res_dir.join("data.txt");
    let cover_path = res_dir.join("cover/cover_ascii.txt");

    Command::cargo_bin("ptero_cli")
        .unwrap()
        .arg("encode")
        .arg("--eluv")
        .arg("-c")
        .arg(&cover_path)
        .arg("-d")
        .arg(&data_path)
        .arg("--pivot")
        .arg("50")
        .assert()
        .success();
    Ok(())
}

#[test]
fn eluv_encode_command_passes_with_set_param() -> Result<(), Box<dyn Error>> {
    global_setup();
    let mut res_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    res_dir.push("resources");

    let data_path = res_dir.join("data.txt");
    let cover_path = res_dir.join("cover/cover_ascii.txt");

    Command::cargo_bin("ptero_cli")
        .unwrap()
        .arg("encode")
        .arg("--eluv")
        .arg("-c")
        .arg(&cover_path)
        .arg("-d")
        .arg(&data_path)
        .arg("--pivot")
        .arg("50")
        .arg("--set")
        .arg("full")
        .assert()
        .success();
    Ok(())
}

#[test]
fn eluv_encode_command_fail_when_wrong_set_param_provided() -> Result<(), Box<dyn Error>> {
    global_setup();
    let mut res_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    res_dir.push("resources");

    let data_path = res_dir.join("data.txt");
    let cover_path = res_dir.join("cover/cover_ascii.txt");

    Command::cargo_bin("ptero_cli")
        .unwrap()
        .arg("encode")
        .arg("--eluv")
        .arg("-c")
        .arg(&cover_path)
        .arg("-d")
        .arg(&data_path)
        .arg("--pivot")
        .arg("50")
        .arg("--set")
        .arg("WRONG_SET_PARAM")
        .assert()
        .failure();
    Ok(())
}
