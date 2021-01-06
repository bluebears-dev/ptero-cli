use atty::Stream;
use colored::Colorize;
use log::{error, info, warn};

pub struct Writer;

impl Writer {
    pub fn info(data: &str) {
        info!("{}", data);
        if atty::is(Stream::Stdout) {
            println!("{}", data.green());
        }
    }        
    
    pub fn print(data: &str) {
        if atty::is(Stream::Stdout) {
            println!("{}", data);
        }
    }    
    
    pub fn error(data: &str) {
        error!("{}", data);
        if atty::is(Stream::Stderr) {
            eprintln!("{}", data.red().bold());
        }
    }    

    pub fn warn(data: &str) {
        warn!("{}", data);
        if atty::is(Stream::Stderr) {
            eprintln!("{}", data.yellow());
        }
    }
}