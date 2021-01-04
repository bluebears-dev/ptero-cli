use atty::Stream;
use colored::Colorize;
use log::{error, info, warn};

pub struct Writer;

impl Writer {
    pub fn info(data: &str) {
        if atty::is(Stream::Stdout) {
            println!("{}", data.green());
        }
        info!("{}", data);
    }        
    
    pub fn print(data: &str) {
        if atty::is(Stream::Stdout) {
            println!("{}", data);
        }
    }    
    
    pub fn error(data: &str) {
        if atty::is(Stream::Stderr) {
            eprintln!("{}", data.red().bold());
        }
        error!("{}", data);
    }    

    pub fn warn(data: &str) {
        if atty::is(Stream::Stderr) {
            eprintln!("{}", data.yellow());
        }
        warn!("{}", data);
    }
}