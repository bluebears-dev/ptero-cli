use std::{sync::mpsc::{Receiver}, thread};

use indicatif::{ProgressBar, ProgressStyle};
use thread::JoinHandle;

#[derive(Debug, Copy, Clone)]
pub enum ProgressStatus {
    Step(u64),
    Finished,
}

pub fn new_progress_bar(data_count: u64) -> ProgressBar {
    let progress_bar = ProgressBar::new(data_count);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"),
    );
    progress_bar
}

pub fn spawn_progress_thread(bar: ProgressBar, rx: Receiver<ProgressStatus>) -> JoinHandle<()> {
    thread::spawn(move || loop {
        match rx.recv() {
            Ok(ProgressStatus::Finished) => {
                break;
            }
            Ok(ProgressStatus::Step(increment)) => {
                if !bar.is_finished() {
                    bar.inc(increment);
                }
            }
            Err(_) => {}
        }
    })
}