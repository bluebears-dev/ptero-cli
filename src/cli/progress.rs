use std::{
    marker::PhantomData,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use indicatif::{ProgressBar, ProgressStyle};
use ptero_common::{method::MethodProgressStatus, observer::Observer};
use thread::JoinHandle;

#[derive(Clone, Debug)]
pub struct SenderProxyObserver<Event> {
    sender: Sender<Event>,
    phantom: PhantomData<Event>,
}

impl SenderProxyObserver<MethodProgressStatus> {
    pub fn new(sender: Sender<MethodProgressStatus>) -> Self {
        SenderProxyObserver {
            sender,
            phantom: PhantomData,
        }
    }
}

impl Observer<MethodProgressStatus> for SenderProxyObserver<MethodProgressStatus> {
    fn on_notify(&mut self, event: &MethodProgressStatus) {
        self.sender.send(*event);
    }
}

pub fn create_progress_bar(progress_count: u64) -> ProgressBar {
    let bar = ProgressBar::new(progress_count);

    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"),
    );

    bar
}

pub fn spawn_progress_bar(
    progres_bar: ProgressBar,
    rx: Receiver<MethodProgressStatus>,
) -> JoinHandle<()> {
    thread::spawn(move || loop {
        match rx.recv() {
            Ok(MethodProgressStatus::Finished) => {
                break;
            }
            Ok(MethodProgressStatus::DataWritten(increment)) => {
                if !progres_bar.is_finished() {
                    progres_bar.inc(increment);
                }
            }
            Err(_) => {}
        }
    })
}
