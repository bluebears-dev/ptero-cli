use std::{error::Error, fs, path::PathBuf, sync::Once};

use assert_cmd::Command;
use log::{error, LevelFilter};
use serde_json::Value;

static INIT: Once = Once::new();

pub struct TemporaryFile<'a>(pub &'a str);

impl<'a> Drop for TemporaryFile<'a> {
    fn drop(&mut self) {
        let file_path = PathBuf::from(self.0);

        if file_path.exists() {
            fs::remove_file(file_path)
                .map_err(|e| error!("Failed during teardown: {:?}", e))
                .ok();
        }
    }
}

impl<'a> TemporaryFile<'a> {
    pub fn path(&self) -> PathBuf {
        PathBuf::from(self.0)
    }
}

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
    method: &str,
) -> Result<Value, Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("ptero_cli").unwrap();
    if let Some(path) = output_path {
        cmd.arg("-o").arg(path);
    } else {
        cmd.arg("--json");
    }
    let assert = cmd
        .arg("encode")
        .arg(format!("--{}", method))
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
    method: &str,
) -> Result<Value, Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("ptero_cli").unwrap();
    if let Some(path) = output_path {
        cmd.arg("-o").arg(path);
    } else {
        cmd.arg("--json");
    }
    let assert = cmd
        .arg("decode")
        .arg(format!("--{}", method))
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
