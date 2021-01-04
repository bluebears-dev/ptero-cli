use std::{error::Error, fmt, sync::mpsc::Sender};

use log::{debug, trace};

use crate::{binary::Bit, cli::progress::ProgressStatus, context::Context};

/// Possible results of data encoding
#[derive(Debug, Clone)]
pub enum EncoderResult {
    Success,
    NoDataLeft,
}

/// Trait that should be implemented by the [Encoders](crate::econder::Encoder).
/// Gives amount of bits that are encoded per text fragment.
/// Concrete text fragment (e.g. line) is determined by the [Context](crate::context::Context).
pub trait Capacity {
    /// Returns how many bits are encoded per text fragment.
    fn bitrate(&self) -> usize;
}

/// Base trait for all data encoders.
/// The generic type should contain data need by the encoder implementation.
pub trait Encoder<E>: Capacity
where
    E: Context,
{
    /// Encodes bits provided by `data` iterator.
    /// Every Encoder has Context which exposes access to cover text. See [Context] for more info.
    ///
    /// # Arguments
    ///
    /// * `context` - context of the steganography method, can contain various needed info like pivot etc.
    /// * `data` - data iterator which return [Bit] with each iteration
    ///
    /// # Returns
    /// It returns whether the encoding was successful. See [EncoderResult] and [EncodingError].
    ///
    /// [Context]: crate::context::Context
    /// [EncoderResult]: EncoderResult
    /// [EncodingError]: EncodingError
    /// [Bit]: crate::binary::Bit
    fn partial_encode(
        &self,
        context: &mut E,
        data: &mut dyn Iterator<Item = Bit>,
    ) -> Result<EncoderResult, Box<dyn Error>>;

    fn encode(
        &self,
        context: &mut E,
        data: &mut dyn Iterator<Item = Bit>,
        progress_channel: Option<&Sender<ProgressStatus>>,
    ) -> Result<String, Box<dyn Error>> {
        let mut stego_text = String::new();

        let mut no_data_left = false;
        while !no_data_left {
            context.load_text()?;
            trace!("Current line '{}'", context.get_current_text()?);
            match self.partial_encode(context, data)? {
                EncoderResult::Success => {
                    if let Some(tx) = progress_channel {
                        tx.send(ProgressStatus::Step(self.bitrate() as u64)).ok();
                    }
                }
                EncoderResult::NoDataLeft => {
                    debug!("No data left to encode, stopping");
                    no_data_left = true;
                }
            }
            let line = context.get_current_text()?;
            stego_text.push_str(&format!("{}\n", &line));
        }
        // Append the rest of possible missing cover text
        let mut appended_line_count = 0;
        while let Ok(line) = context.load_text() {
            appended_line_count += 1;
            stego_text.push_str(&format!("{}\n", &line));
        }
        debug!("Appended the {} of left lines", appended_line_count);

        if !no_data_left {
            debug!("Capacity exceeded by {} bits", data.count());
            Err(EncodingError::capacity_error().into())
        } else {
            Ok(stego_text)
        }
    }
}

/// Enum for data encoding errors types
#[derive(Debug, Clone)]
pub enum EncodingErrorKind {
    CapacityTooLow,
    NoWordsLeft,
}

/// Represents encoding error. Concrete error if differentiated by the [EncodingErrorKind](EncodingErrorKind)
#[derive(Debug, Clone)]
pub struct EncodingError {
    kind: EncodingErrorKind,
}

impl EncodingError {
    /// Facade for creating [CapacityTooLow](EncodingErrorKind) error.
    pub fn capacity_error() -> Self {
        EncodingError {
            kind: EncodingErrorKind::CapacityTooLow,
        }
    }
    /// Facade for creating [NoWordsLeft](EncodingErrorKind) error.
    pub fn no_words_error() -> Self {
        EncodingError {
            kind: EncodingErrorKind::NoWordsLeft,
        }
    }
}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            EncodingErrorKind::CapacityTooLow => write!(f, "Exceeded cover text capacity"),
            EncodingErrorKind::NoWordsLeft => write!(
                f,
                "No extra words found in cover text when tried to encode a bit"
            ),
        }
    }
}

impl Error for EncodingError {}
