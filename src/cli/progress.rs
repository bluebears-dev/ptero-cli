use std::{
    marker::PhantomData,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use indicatif::{ProgressBar, ProgressStyle};
use ptero_common::{method::MethodProgressStatus, observer::Observer};
use thread::JoinHandle;

#[derive(Debug, Copy, Clone)]
pub enum ProgressStatus {
    Step(u64),
    Finished,
}

pub struct ProgressBarObserver<Event> {
    pub progress_bar: ProgressBar,
    rx: Receiver<Event>,
    tx: Sender<Event>,
    phantom: PhantomData<Event>,
}

impl ProgressBarObserver<MethodProgressStatus> {
    pub fn new(progress_count: u64) -> Self {
        let (tx, rx) = channel::<MethodProgressStatus>();

        let bar = ProgressBar::new(progress_count);

        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .progress_chars("##-"),
        );

        ProgressBarObserver {
            progress_bar: bar,
            rx,
            tx,
            phantom: PhantomData,
        }
    }

    pub fn spawn_progress_bar(self, rx: Receiver<MethodProgressStatus>) -> JoinHandle<()> {
        thread::spawn(move || loop {
            match rx.recv() {
                Ok(MethodProgressStatus::Finished) => {
                    break;
                }
                Ok(MethodProgressStatus::DataWritten(increment)) => {
                    if !self.progress_bar.is_finished() {
                        self.progress_bar.inc(increment);
                    }
                }
                Err(_) => {}
            }
        })
    }
}

impl Observer<MethodProgressStatus> for ProgressBarObserver<MethodProgressStatus> {
    fn on_notify(&mut self, event: &MethodProgressStatus) {
        self.tx.send(*event);
    }
}
