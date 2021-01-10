use std::{error::Error, fs::{File}, io::Read, sync::mpsc::channel};

use clap::Clap;

use crate::{
    context::PivotByRawLineContext,
    decoder::Decoder,
    method::complex::{eluv::ELUVMethod, extended_line::ExtendedLineMethod},
};

use super::progress::{ProgressStatus, new_progress_bar, spawn_progress_thread};

/// Decode secret from the stegotext
#[derive(Clap)]
pub struct DecodeSubCommand {
    /// Path to stegotext from which data will be decoded
    #[clap(short, long)]
    text: String,

    /// Pivot i.e. line length used to encode with extended line algorithm
    #[clap(short, long)]
    pivot: usize,

    /// Use ELUV method for encoding.
    #[clap(long, group = "method_args")]
    eluv: bool,
    
    /// Use Extended Line method for encoding.
    #[clap(long = "eline", group = "method_args")]
    #[allow(dead_code)]
    extended_line: bool,
}

impl DecodeSubCommand {
    pub fn run(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let stego_text_file_input= File::open(&self.text)?;

        self.do_decode(stego_text_file_input)
    }

    pub(crate) fn get_method(&self) -> Box<dyn Decoder<PivotByRawLineContext>> {
        if self.eluv {
            Box::new(ELUVMethod::default())
        } else {
            Box::new(ExtendedLineMethod::default())
        }
    }
    
    pub fn do_decode(&self, mut stego_input: impl Read) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut stego_text = String::new();

        stego_input.read_to_string(&mut stego_text)?;
        let decoder = self.get_method();
        let mut context = PivotByRawLineContext::new(stego_text.as_str(), self.pivot);
    
        let progress_bar = new_progress_bar(stego_text.len() as u64);
        let (tx, rx) = channel::<ProgressStatus>();
        progress_bar.set_message("Decoding cover text...");
        spawn_progress_thread(progress_bar.clone(), rx);
    
        let result =  decoder.decode(&mut context, Some(&tx));
        
        tx.send(ProgressStatus::Finished).ok();
        progress_bar.finish_with_message("Finished decoding");
    
        result
    }
}



#[allow(unused_imports)]
mod test {
    use std::{error::Error, io::Read};
    use crate::binary::Bit;

    use super::DecodeSubCommand;

    #[test]
    fn decodes_zeroes_if_not_data_encoded_extended_line() -> Result<(), Box<dyn Error>> {
        let stego_input = "a b";

        let command = DecodeSubCommand {
            text: "stub".into(),
            pivot: 3,
            eluv: false,
            extended_line: true,
        };

        let result = command.do_decode(stego_input.as_bytes());
        assert_eq!(result.ok(), Some(vec![0]));
        Ok(())
    }    
    
    #[test]
    fn decodes_zeroes_if_not_data_encoded_eluv() -> Result<(), Box<dyn Error>> {
        let stego_input = "a b";

        let command = DecodeSubCommand {
            text: "stub".into(),
            pivot: 3,
            eluv: true,
            extended_line: false,
        };

        let result = command.do_decode(stego_input.as_bytes());
        assert_eq!(result.ok(), Some(vec![0]));
        Ok(())
    }
}
