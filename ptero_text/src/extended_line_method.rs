//! This module contains implementation of Extended Line algorithm that can be used to
//! conceal data inside a text.
//!
//! It implements [`SteganographyMethod`] trait for [`&str`] type.
//!
//! # Examples
//!
//! Concealing data with required properties:
//! ```
//! use std::cell::RefCell;
//! use std::rc::Rc;
//! use bitvec::prelude::*;
//! use bitvec::view::AsBits;
//! use rand::{RngCore, SeedableRng};
//! use rand::rngs::StdRng;
//! use ptero_common::method::SteganographyMethod;
//! use ptero_text::extended_line_method::{ExtendedLineMethod, Variant};
//!
//! let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StdRng::seed_from_u64(1337)));
//! let cover_text = "This is a sample text that is harmless";
//! let mut ext_line_method = ExtendedLineMethod::builder()
//!     .with_rng(&rng)
//!     .with_pivot(11)
//!     .build();
//!
//! let data = "E";
//! let mut data_bit_iter = data.as_bits::<Msb0>().iter();
//!
//! if let Ok(stego_text) = ext_line_method.try_conceal(cover_text, &mut data_bit_iter) {
//!     assert_eq!(&stego_text, "This  is a\nsample text \nthat  is\nharmless");
//! } else {
//!     panic!("Something went wrong");
//! }
//! ```
//!
//! Revealing data:
//! ```
//! use std::cell::RefCell;
//! use std::rc::Rc;
//! use bitvec::prelude::*;
//! use bitvec::view::AsBits;
//! use rand::{RngCore, SeedableRng};
//! use rand::rngs::StdRng;
//! use ptero_common::method::SteganographyMethod;
//! use ptero_text::extended_line_method::{ExtendedLineMethod, Variant};
//!
//! // The RNG must be seeded with the same value as used when concealing
//! let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StdRng::seed_from_u64(1337)));
//! let stego_text = "This  is a\nsample text \nthat  is\nharmless";
//! let mut ext_line_method = ExtendedLineMethod::builder()
//!     .with_rng(&rng)
//!     .with_pivot(11)
//!     .build();
//!
//! if let Ok(bits) = ext_line_method.try_reveal::<Msb0, u8>(stego_text) {
//!     // Most often you'll get your data with trailing noise (probably zeroes)
//!     // If you know the payload's size, you can trim it
//!     let output = String::from_utf8_lossy(&bits.as_raw_slice()[0..1]);
//!     assert_eq!(&output, "E");
//! }
//! ```
//!
//! Tracking progress of concealing:
//! ```
//! use std::cell::RefCell;
//! use std::rc::Rc;
//! use std::sync::Arc;
//! use bitvec::prelude::*;
//! use bitvec::view::AsBits;
//! use rand::{RngCore, SeedableRng};
//! use rand::rngs::StdRng;
//! use ptero_common::method::{MethodProgressStatus, SteganographyMethod};
//! use ptero_common::observer::Observer;
//! use ptero_text::extended_line_method::{ExtendedLineMethod, Variant};
//!
//! struct Listener {
//!     pub amount_written: u64,
//!     pub has_finished: bool
//! }
//!
//! impl Listener {
//!     fn new() -> Self {
//!         Listener {
//!             amount_written: 0,
//!             has_finished: false
//!         }
//!     }
//! }
//!
//! impl Observer<MethodProgressStatus> for Listener {
//!     fn on_notify(&mut self, event: &MethodProgressStatus) {
//!         match event {
//!             MethodProgressStatus::DataWritten(amount) => { self.amount_written += amount; }
//!             MethodProgressStatus::Finished => { self.has_finished = true; }
//!         }
//!     }
//! }
//!
//! // The RNG must be seeded with the same value as used when concealing
//! let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StdRng::seed_from_u64(1337)));
//! let cover_text = "This is a sample text that is harmless";
//! let mut ext_line_method = ExtendedLineMethod::builder()
//!     .with_rng(&rng)
//!     .with_pivot(11)
//!     .build();
//!
//! let listener_arc = Arc::new(RefCell::new(Listener::new()));
//! ext_line_method.subscribe(listener_arc.clone());
//!
//! let data = "E";
//! let mut data_bit_iter = data.as_bits::<Msb0>().iter();
//!
//! ext_line_method.try_conceal(cover_text, &mut data_bit_iter);
//!
//! assert_eq!(listener_arc.borrow().has_finished, true);
//! assert_eq!(listener_arc.borrow().amount_written, 8);
//! ```
//! # Description
//! TBD
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::rc::Rc;
use std::sync::Arc;

use bitvec::prelude::*;
use bitvec::slice::Iter;
use log::{debug, trace};
use rand::RngCore;
use snafu::Snafu;
use unicode_segmentation::UnicodeSegmentation;

use ptero_common::config::{CommonMethodConfig, CommonMethodConfigBuilder};
use ptero_common::method::{MethodProgressStatus, MethodResult, SteganographyMethod};
use ptero_common::observer::{Observable, Observer};

use self::random_whitespace_method::RandomWhitespaceMethod;
use self::trailing_whitespace_method::TrailingWhitespaceMethod;

const ASCII_DELIMITER: &str = " ";
const NEWLINE_STR: &str = "\n";
const DEFAULT_PIVOT: usize = 15;

mod random_whitespace_method;
mod trailing_whitespace_method;
pub mod character_sets;

#[derive(Debug)]
pub(crate) enum MethodActions {
    LineExtend,
    RandomASCIIWhitespace,
    TrailingASCIIWhitespace,
}

/// Extended Line method variant.
/// It describes which variation of internally used algorithms you want to use.
#[derive(Debug)]
pub enum Variant {
    /// Line Extension, Random Whitespace, Trailing Whitespace
    V1,
    /// Line Extension, Trailing Whitespace, Random Whitespace
    V2,
    /// Random Whitespace, Line Extension, Trailing Whitespace
    V3,
}

/// Builder for [`ExtendedLineMethod`] algorithm.
/// It enables you to configure the method to your use case.
pub struct ExtendedLineMethodBuilder {
    pivot: usize,
    variant: Variant,
    config_builder: CommonMethodConfigBuilder,
}

impl<'a> Default for ExtendedLineMethodBuilder {
    fn default() -> Self {
        ExtendedLineMethodBuilder {
            pivot: DEFAULT_PIVOT,
            variant: Variant::V1,
            config_builder: CommonMethodConfig::builder(),
        }
    }
}

impl ExtendedLineMethodBuilder {
    /// Set custom RNG for method.
    pub fn with_rng(mut self, rng: &Rc<RefCell<dyn RngCore>>) -> Self {
        self.config_builder = self.config_builder.with_rng(rng);
        self
    }

    /// Set variant of the method
    pub fn with_variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    /// Set pivot
    pub fn with_pivot(mut self, pivot: usize) -> Self {
        self.pivot = pivot;
        self
    }

    /// Constructs the method
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    /// use rand::{RngCore, SeedableRng};
    /// use rand::rngs::mock::StepRng;
    /// use rand::rngs::StdRng;
    /// use ptero_text::extended_line_method::{ExtendedLineMethodBuilder, Variant};
    ///
    /// let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StdRng::from_entropy()));
    /// let method = ExtendedLineMethodBuilder::default()
    ///     .with_rng(&rng)
    ///     .build();
    /// ```
    ///
    /// Override defaults:
    ///
    /// ```
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    /// use rand::RngCore;
    /// use rand::rngs::mock::StepRng;
    /// use ptero_text::extended_line_method::{ExtendedLineMethodBuilder, Variant};
    ///
    /// let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    /// let method = ExtendedLineMethodBuilder::default()
    ///     .with_rng(&rng)
    ///     .with_variant(Variant::V2)
    ///     .with_pivot(20)
    ///     .build();
    /// ```
    pub fn build(self) -> ExtendedLineMethod {
        ExtendedLineMethod {
            pivot: self.pivot,
            variant: self.variant,
            config: self.config_builder.build().unwrap(),
        }
    }
}

pub(crate) fn get_variant_methods(variant: &Variant) -> &'static [MethodActions; 3] {
    match variant {
        Variant::V1 => &[
            MethodActions::LineExtend,
            MethodActions::RandomASCIIWhitespace,
            MethodActions::TrailingASCIIWhitespace,
        ],
        Variant::V2 => &[
            MethodActions::LineExtend,
            MethodActions::TrailingASCIIWhitespace,
            MethodActions::RandomASCIIWhitespace,
        ],
        Variant::V3 => &[
            MethodActions::RandomASCIIWhitespace,
            MethodActions::LineExtend,
            MethodActions::TrailingASCIIWhitespace,
        ],
    }
}

fn graphemes_length(text: &str) -> usize {
    text.graphemes(true).count()
}

pub type Result<Success> = std::result::Result<Success, ConcealError>;
pub type VerificationResult = std::result::Result<(), ConcealError>;

/// The main structure describing internal state for the Extended Line method.
pub struct ExtendedLineMethod {
    pivot: usize,
    variant: Variant,
    config: CommonMethodConfig,
}

impl ExtendedLineMethod {
    /// Returns a builder for [`ExtendedLineMethod`] algorithm.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    /// use rand::RngCore;
    /// use rand::rngs::mock::StepRng;
    /// use ptero_text::extended_line_method::ExtendedLineMethod;
    ///
    /// let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(1, 1)));
    /// let method = ExtendedLineMethod::builder()
    ///     .with_rng(&rng)
    ///     .with_pivot(20)
    ///     .build();
    /// ```
    pub fn builder() -> ExtendedLineMethodBuilder {
        ExtendedLineMethodBuilder::default()
    }

    pub fn subscribe(&mut self, subscriber: Arc<RefCell<dyn Observer<MethodProgressStatus>>>) {
        self.config.notifier.subscribe(subscriber);
    }

    pub fn verify_pivot(&self, cover: &str) -> VerificationResult {
        debug!("Checking if pivot is feasible for provided cover");

        let words_longer_than_pivot = cover
            .split_whitespace()
            .filter(|word| word.len() > self.pivot)
            .collect::<Vec<&str>>();

        if !words_longer_than_pivot.is_empty() {
            let word = words_longer_than_pivot[0];
            Err(ConcealError::pivot_too_small(word.to_string(), self.pivot))
        } else {
            Ok(())
        }
    }

    fn build_random_whitespace_submethod(&self) -> RandomWhitespaceMethod {
        RandomWhitespaceMethod::builder()
            .with_rng(&self.config.rng.upgrade().unwrap())
            .with_notifier(self.config.notifier.clone())
            .build()
    }

    fn build_trailing_whitespace_submethod(&self) -> TrailingWhitespaceMethod {
        TrailingWhitespaceMethod::builder()
            .with_rng(&self.config.rng.upgrade().unwrap())
            .with_notifier(self.config.notifier.clone())
            .build()
    }

    fn partial_conceal<'b, IteratorType, Order, Type>(
        &mut self,
        word_iterator: &mut Peekable<IteratorType>,
        data: &mut Iter<Order, Type>,
        result: &mut String,
    ) -> Result<MethodResult>
    where
        IteratorType: Iterator<Item = &'b str>,
        Order: BitOrder,
        Type: BitStore,
    {
        let pivot_line = self.construct_pivot_line(word_iterator);
        result.push_str(&pivot_line);

        if pivot_line.is_empty() {
            let remaining_data_size = data.count();
            return Err(ConcealError::no_cover_words_left(
                remaining_data_size,
                self.pivot,
            ));
        }

        let mut random_whitespace_submethod = self.build_random_whitespace_submethod();
        let mut trailing_whitespace_submethod = self.build_trailing_whitespace_submethod();

        for action in get_variant_methods(&self.variant) {
            let method_result = match action {
                MethodActions::LineExtend => self.conceal_in_extended_line(
                    graphemes_length(&pivot_line),
                    word_iterator,
                    data,
                    result,
                ),
                MethodActions::RandomASCIIWhitespace => {
                    random_whitespace_submethod.conceal_in_random_whitespace(data, result)
                }
                MethodActions::TrailingASCIIWhitespace => {
                    Ok(trailing_whitespace_submethod.conceal_in_trailing_whitespace(data, result))
                }
            };

            if let MethodResult::NoDataLeft = method_result? {
                return Ok(MethodResult::NoDataLeft);
            }
        }
        Ok(MethodResult::Success)
    }

    fn partial_reveal<Order, Type>(&mut self, line: &str, revealed_data: &mut BitVec<Order, Type>)
    where
        Order: BitOrder,
        Type: BitStore,
    {
        let mut random_whitespace_submethod = self.build_random_whitespace_submethod();
        let mut trailing_whitespace_submethod = self.build_trailing_whitespace_submethod();

        let actions = get_variant_methods(&self.variant);
        let mut current_line = line.to_string();

        let mut gathered_bits: BitVec<Order, Type> = BitVec::with_capacity(3);
        for action in actions.iter().rev() {
            match action {
                MethodActions::LineExtend => {
                    self.reveal_in_extended_line(&current_line, &mut gathered_bits);
                }
                MethodActions::RandomASCIIWhitespace => {
                    random_whitespace_submethod
                        .reveal_in_random_whitespace(&mut current_line, &mut gathered_bits);
                }
                MethodActions::TrailingASCIIWhitespace => {
                    trailing_whitespace_submethod
                        .reveal_in_trailing_whitespace(&mut current_line, &mut gathered_bits);
                }
            };
        }
        gathered_bits.reverse();
        revealed_data.append(&mut gathered_bits);
    }

    pub(crate) fn conceal_in_extended_line<'b, IteratorType, Order, Type>(
        &mut self,
        pivot_line_length: usize,
        word_iter: &mut Peekable<IteratorType>,
        data: &mut Iter<Order, Type>,
        result: &mut String,
    ) -> Result<MethodResult>
    where
        IteratorType: Iterator<Item = &'b str>,
        Order: BitOrder,
        Type: BitStore,
    {
        Ok(match data.next().as_deref() {
            Some(true) => {
                let next_word = word_iter.next().ok_or_else(|| {
                    let remaining_data_size = data.count();
                    ConcealError::no_cover_words_left(remaining_data_size, self.pivot)
                })?;

                let extended_line_length = pivot_line_length
                    + graphemes_length(next_word)
                    + graphemes_length(ASCII_DELIMITER);

                if extended_line_length <= self.pivot {
                    let remaining_data_size = data.count();
                    return Err(ConcealError::line_too_short(
                        remaining_data_size,
                        self.pivot,
                    ));
                }

                trace!("Extending line with '{}'", &next_word);
                result.push_str(ASCII_DELIMITER);
                result.push_str(next_word);
                self.config.notifier.notify(&MethodProgressStatus::DataWritten(1));
                MethodResult::Success
            }
            Some(false) => {
                trace!("Leaving line as-is");
                self.config.notifier.notify(&MethodProgressStatus::DataWritten(1));
                MethodResult::Success
            }
            None => MethodResult::NoDataLeft,
        })
    }

    pub(crate) fn reveal_in_extended_line<Order, Type>(
        &mut self,
        stego_text_line: &str,
        revealed_data: &mut BitVec<Order, Type>,
    ) where
        Order: BitOrder,
        Type: BitStore,
    {
        let bit = graphemes_length(stego_text_line) > self.pivot;
        trace!("Found extended line: '{}'", bit);
        revealed_data.push(bit)
    }

    pub(crate) fn construct_pivot_line<'b, I>(&self, word_iter: &mut Peekable<I>) -> String
    where
        I: Iterator<Item = &'b str>,
    {
        let mut current_line_length = 0;
        let mut result = String::new();

        while let Some(next_word) = word_iter.peek() {
            let line_appendix = if current_line_length > 0 {
                [ASCII_DELIMITER, next_word].join("")
            } else {
                next_word.to_string()
            };

            if current_line_length + graphemes_length(&line_appendix) > self.pivot {
                break;
            }

            current_line_length += graphemes_length(&line_appendix);
            result.push_str(&line_appendix);

            word_iter.next();
        }
        trace!(
            "Constructed line of length: '{}' while '{}' is the pivot",
            current_line_length,
            self.pivot
        );
        result
    }
}

impl<'a> SteganographyMethod<&'a str, ConcealError> for ExtendedLineMethod {
    type ConcealedOutput = String;

    fn try_conceal<Order, Type>(
        &mut self,
        cover: &str,
        data: &mut Iter<Order, Type>,
    ) -> Result<Self::ConcealedOutput>
    where
        Order: BitOrder,
        Type: BitStore,
    {
        self.verify_pivot(cover)?;

        let mut result = String::with_capacity(cover.len());

        let mut word_iterator = cover
            .split_whitespace()
            .filter(|word| !word.contains(char::is_whitespace))
            .peekable();

        while let MethodResult::Success =
            self.partial_conceal(&mut word_iterator, data, &mut result)?
        {
            result.push_str(NEWLINE_STR);
        }

        loop {
            let line = self.construct_pivot_line(&mut word_iterator);
            if line.is_empty() {
                break;
            }
            result.push_str(NEWLINE_STR);
            result.push_str(&line);
        }

        self.config.notifier.notify(&MethodProgressStatus::Finished);

        Ok(result)
    }

    fn try_reveal<Order, Type>(&mut self, stego_text: &str) -> Result<BitVec<Order, Type>>
    where
        Order: BitOrder,
        Type: BitStore,
    {
        let mut revealed_data: BitVec<Order, Type> = BitVec::new();

        for line in stego_text.split(NEWLINE_STR) {
            self.partial_reveal(line, &mut revealed_data);
        }

        Ok(revealed_data)
    }
}

/// Describes possible errors while concealing data using [`ExtendedLineMethod`].
#[derive(Debug, PartialEq, Snafu)]
pub enum ConcealError {
    /// Used pivot was not suitable for selected cover text.
    /// Cover text has words that are longer the pivot and the method cannot construct a feasible line.
    #[snafu(display(
        "Pivot '{}' is smaller then the longest word in cover: '{}'",
        pivot,
        word
    ))]
    PivotTooSmall { word: String, pivot: usize },
    /// Used cover text can't hide selected amount of data.
    /// Can happen at various stages of concealing.
    #[snafu(display(
        "{}. '{}' bytes left unprocessed while using '{}' as a pivot",
        reason,
        remaining_data_size,
        pivot
    ))]
    CoverTextTooSmall {
        reason: CoverTooSmallErrorReason,
        remaining_data_size: usize,
        pivot: usize,
    },
    /// Used when pivot line contains too few words to input random whitespace.
    /// Adjusting pivot so that more than one word appears on line will mitigate issue.
    #[snafu(display("Line '{}' doesn't have enough words to conceal a bit", line))]
    NotEnoughWordsOnPivotLine { line: String },
}

#[cfg(not(tarpaulin_include))]
impl ConcealError {
    pub fn pivot_too_small(word: String, pivot: usize) -> ConcealError {
        ConcealError::PivotTooSmall { word, pivot }
    }

    pub fn no_cover_words_left(remaining_data_size: usize, pivot: usize) -> ConcealError {
        ConcealError::CoverTextTooSmall {
            reason: CoverTooSmallErrorReason::NoCoverWordsLeft,
            remaining_data_size,
            pivot,
        }
    }

    pub fn line_too_short(remaining_data_size: usize, pivot: usize) -> ConcealError {
        ConcealError::CoverTextTooSmall {
            reason: CoverTooSmallErrorReason::ConstructedTooShortLine,
            remaining_data_size,
            pivot,
        }
    }

    pub fn not_enough_words(line: &str) -> ConcealError {
        ConcealError::NotEnoughWordsOnPivotLine {
            line: line.to_string(),
        }
    }
}

/// Describes the [`MethodError::CoverTextTooSmall`] error with more context.
#[derive(Debug, PartialEq)]
pub enum CoverTooSmallErrorReason {
    /// It can occur during line extension or while constructing a line but got an empty one.
    NoCoverWordsLeft,
    /// Can occur when the last line constructed from cover emptied the cover, but method has to extend the line to hide a bit.
    ConstructedTooShortLine,
}

#[cfg(not(tarpaulin_include))]
impl Display for CoverTooSmallErrorReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CoverTooSmallErrorReason::NoCoverWordsLeft => {
                write!(f, "No cover words left, cannot construct a line.")
            }
            CoverTooSmallErrorReason::ConstructedTooShortLine => {
                write!(
                    f,
                    "Line constructed is too short to extend it above pivot length"
                )
            }
        }
    }
}
