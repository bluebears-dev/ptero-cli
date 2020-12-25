use std::{error::Error, path::PathBuf, sync::Once};

use assert_cmd::Command;
use log::LevelFilter;
use serde_json::Value;

static INIT: Once = Once::new();

pub fn global_setup() {
    INIT.call_once(|| {
        fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] - {}",
                    chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    &record.target(),
                    &record.level(),
                    message,
                ));
            })
            .level(LevelFilter::Trace)
            .chain(std::io::stdout())
            .apply()
            .unwrap();
    });
}

pub fn run_encode_command(
    cover_path: &PathBuf,
    data_path: &PathBuf,
    pivot: usize,
    output_path: Option<&PathBuf>,
) -> Result<Value, Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("ptero_cli").unwrap();
    if let Some(path) = output_path {
        cmd.arg("-o").arg(path);
    }
    let assert = cmd
        .arg("--json")
        .arg("encode")
        .arg("-c")
        .arg(&cover_path)
        .arg("-d")
        .arg(&data_path)
        .arg("--pivot")
        .arg(format!("{}", pivot))
        .assert()
        .success();

    let json_out = String::from_utf8_lossy(&assert.get_output().stdout);
    let json_struct: Value = if output_path.is_some() {
        serde_json::from_str("{}")?
    } else {
        serde_json::from_str(&json_out)?
    };

    Ok(json_struct)
}

pub fn run_decode_command(
    stego_text: &PathBuf,
    pivot: usize,
    output_path: Option<&PathBuf>,
) -> Result<Value, Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("ptero_cli").unwrap();
    if let Some(path) = output_path {
        cmd.arg("-o").arg(path);
    }
    let mut cmd = Command::cargo_bin("ptero_cli").unwrap();
    let assert = cmd
        .arg("--json")
        .arg("decode")
        .arg("-t")
        .arg(&stego_text)
        .arg("--pivot")
        .arg(format!("{}", pivot))
        .assert()
        .success();

    let json_out = String::from_utf8_lossy(&assert.get_output().stdout);
    let json_struct: Value = if output_path.is_some() {
        serde_json::from_str("{}")?
    } else {
        serde_json::from_str(&json_out)?
    };

    Ok(json_struct)
}
