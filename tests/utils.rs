use std::{error::Error, fmt::Display, path::PathBuf};

use assert_cmd::Command;

pub const CLI_BIN_NAME: &str = "ptero_cli";

fn generic_command(args: &[&str]) -> Result<Command, Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(CLI_BIN_NAME)?;
    cmd.args(args);

    return Ok(cmd);
}

fn construct_optional_arg<T: Display>(flag: &str, value: Option<T>) -> Option<String> {
    value.map(|out| format!("{} {}", flag, out))
}

pub fn encode_command(
    cover_path: &PathBuf,
    data_path: &PathBuf,
    pivot: Option<usize>,
    output: Option<&str>,
) -> Result<Command, Box<dyn Error>> {
    let mut arguments = vec![];
    let out_arg = construct_optional_arg("-o", output);
    if let Some(arg) = &out_arg {
        arguments.extend(arg.split(" "));
    }
    arguments.extend_from_slice(&[
        "encode",
        "-c",
        cover_path.to_str().unwrap(),
        "-d",
        data_path.to_str().unwrap(),
    ]);
    let pivot_arg = construct_optional_arg("--pivot", pivot);
    if let Some(arg) = &pivot_arg {
        arguments.extend(arg.split(" "));
    }

    return generic_command(arguments.as_slice());
}

pub fn decode_command(
    stego_text_path: &PathBuf,
    pivot: usize,
    output: Option<&str>,
) -> Result<Command, Box<dyn Error>> {
    let mut arguments = vec![];
    let out_arg = construct_optional_arg("-o", output);
    if let Some(arg) = &out_arg {
        arguments.extend(arg.split(" "));
    }
    let pivot_str = pivot.to_string();
    arguments.extend_from_slice(&[
        "decode",
        "-t",
        stego_text_path.to_str().unwrap(),
        "--pivot",
        pivot_str.as_str(),
    ]);

    return generic_command(arguments.as_slice());
}
