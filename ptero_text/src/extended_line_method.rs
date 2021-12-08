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
//! let rng = StdRng::seed_from_u64(1337);
//! let cover_text = "This is a sample text that is harmless";
//! let mut ext_line_method = ExtendedLineMethod::builder()
//!     .with_rng(rng)
//!     .with_pivot(11)
//!     .build()
//!     .unwrap();
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
//! let rng = StdRng::seed_from_u64(1337);
//! let stego_text = "This  is a\nsample text \nthat  is\nharmless";
//! let mut ext_line_method = ExtendedLineMethod::builder()
//!     .with_rng(rng)
//!     .with_pivot(11)
//!     .build()
//!     .unwrap();
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
//! let rng = StdRng::seed_from_u64(1337);
//! let cover_text = "This is a sample text that is harmless";
//! let mut ext_line_method = ExtendedLineMethod::builder()
//!     .with_rng(rng)
//!     .with_pivot(11)
//!     .build()
//!     .unwrap();
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
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::rc::Rc;
use std::sync::Arc;

use bitvec::prelude::*;
use bitvec::slice::Iter;
use derive_builder::UninitializedFieldError;
use rand::RngCore;
use snafu::{ErrorCompat, ResultExt, Snafu};
use unicode_segmentation::UnicodeSegmentation;

use ptero_common::config::{
    CommonMethodConfig, CommonMethodConfigBuilder, CommonMethodConfigBuilderError,
};
use ptero_common::method::{MethodProgressStatus, MethodResult, SteganographyMethod};
use ptero_common::observer::{Observable, Observer};

use crate::extended_line_method::character_sets::GetCharacterSet;

use self::line_extend_method::{
    LineExtendMethod, LineExtendMethodBuilder, LineExtendMethodBuilderError,
};
use self::random_whitespace_method::{
    RandomWhitespaceMethod, RandomWhitespaceMethodBuilder, RandomWhitespaceMethodBuilderError,
};
use self::trailing_whitespace_method::{
    TrailingWhitespaceMethod, TrailingWhitespaceMethodBuilder, TrailingWhitespaceMethodBuilderError,
};

const NEWLINE_STR: &str = "\n";

pub mod character_sets;
mod line_extend_method;
mod random_whitespace_method;
mod trailing_whitespace_method;

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
    /// Triggers submethods in given order:
    ///
    /// Line Extension, Random Whitespace, Trailing Whitespace
    V1,
    /// Triggers submethods in given order:
    ///
    /// Line Extension, Trailing Whitespace, Random Whitespace
    V2,
    /// Triggers submethods in given order:
    ///
    /// Random Whitespace, Line Extension, Trailing Whitespace
    V3,
}

pub struct ExtendedLineMethodBuilder {
    rw_submethod_builder: RandomWhitespaceMethodBuilder,
    tw_submethod_builder: TrailingWhitespaceMethodBuilder,
    le_submethod_builder: LineExtendMethodBuilder,
    config_builder: CommonMethodConfigBuilder,
    variant: Variant,
}

impl<'a> Default for ExtendedLineMethodBuilder {
    fn default() -> Self {
        ExtendedLineMethodBuilder {
            rw_submethod_builder: RandomWhitespaceMethod::builder(),
            tw_submethod_builder: TrailingWhitespaceMethod::builder(),
            le_submethod_builder: LineExtendMethod::builder(),
            config_builder: CommonMethodConfig::builder(),
            variant: Variant::V1,
        }
    }
}

impl ExtendedLineMethodBuilder {
    /// Set custom RNG for method.
    pub fn with_rng<T>(mut self, rng: T) -> Self
    where
        T: RngCore + 'static,
    {
        self.config_builder = self.config_builder.with_rng(rng);
        self
    }

    /// Sets custom character set to be used when triggering Trailing Whitespace submethod.
    ///
    /// Possible values are listed in [`CharacterSetType`]. You can implement your custom type
    /// as long as it extends [`GetCharacterSet`].
    ///
    /// By manipulating this value, you can increase bitrate of the method, maximum being 7 bits
    /// per cycle.
    pub fn with_trailing_charset<T>(mut self, character_set: T) -> Self
    where
        T: GetCharacterSet + 'static,
    {
        self.tw_submethod_builder = self.tw_submethod_builder.with_charset(character_set);
        self
    }

    /// Set variant of the method
    pub fn with_variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    /// Set pivot
    pub fn with_pivot(mut self, pivot: usize) -> Self {
        self.le_submethod_builder.with_pivot(pivot);
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
    /// let rng = StdRng::from_entropy();
    /// let method = ExtendedLineMethodBuilder::default()
    ///     .with_rng(rng)
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
    /// let rng = StepRng::new(1, 1);
    /// let method = ExtendedLineMethodBuilder::default()
    ///     .with_rng(rng)
    ///     .with_variant(Variant::V2)
    ///     .with_pivot(20)
    ///     .build();
    /// ```
    pub fn build(self) -> std::result::Result<ExtendedLineMethod, BuilderError> {
        let config = self
            .config_builder
            .build()
            .map_err(|source| {
                BuilderError { source: source.into() }
            })?;

        let config_rc = Rc::new(RefCell::new(config));

        // Refactor to return result like other builders
        Ok(ExtendedLineMethod {
            rw_submethod: self
                .rw_submethod_builder
                .with_shared_config(config_rc.clone())
                .build()
                .map_err(|source| BuilderError { source: source.into() })?,
            tw_submethod: self
                .tw_submethod_builder
                .with_shared_config(config_rc.clone())
                .build()
                .map_err(|source| BuilderError { source: source.into() })?,
            le_submethod: self
                .le_submethod_builder
                .with_shared_config(config_rc.clone())
                .build()
                .map_err(|source| BuilderError { source: source.into() })?,
            config: config_rc,
            variant: self.variant,
        })
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

#[derive(Debug, Snafu)]
#[snafu(display("Couldn't finish building ExtendedLineMethod: {}", source))]
pub struct BuilderError {
    source: Box<dyn Error>
}

fn graphemes_length(text: &str) -> usize {
    text.graphemes(true).count()
}

pub type Result<Success> = std::result::Result<Success, ConcealError>;

/// The main structure describing internal state for the Extended Line method.
pub struct ExtendedLineMethod {
    variant: Variant,
    config: Rc<RefCell<CommonMethodConfig>>,
    rw_submethod: RandomWhitespaceMethod,
    tw_submethod: TrailingWhitespaceMethod,
    le_submethod: LineExtendMethod,
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
    /// let rng = StepRng::new(1, 1);
    /// let method = ExtendedLineMethod::builder()
    ///     .with_rng(rng.clone())
    ///     .with_pivot(20)
    ///     .build();
    /// ```
    ///
    /// With different trailing charset:
    /// ```
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    /// use rand::RngCore;
    /// use rand::rngs::mock::StepRng;
    /// use ptero_text::extended_line_method::character_sets::CharacterSetType;
    /// use ptero_text::extended_line_method::ExtendedLineMethod;
    ///
    /// let rng = StepRng::new(1, 1);
    /// let method = ExtendedLineMethod::builder()
    ///     .with_rng(rng.clone())
    ///     .with_pivot(10)
    ///     .with_trailing_charset(CharacterSetType::Full)
    ///     .build();
    /// ```
    pub fn builder() -> ExtendedLineMethodBuilder {
        ExtendedLineMethodBuilder::default()
    }

    pub fn subscribe(&mut self, subscriber: Arc<RefCell<dyn Observer<MethodProgressStatus>>>) {
        self.config.borrow_mut().notifier.subscribe(subscriber);
    }

    fn notify(&mut self, event: &MethodProgressStatus) {
        self.tw_submethod.notify(event);
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
        let pivot = self.le_submethod.get_pivot();
        let pivot_line = self.le_submethod.construct_pivot_line(word_iterator);
        result.push_str(&pivot_line);

        if pivot_line.is_empty() {
            let remaining_data_size = data.count();
            return Err(ConcealError::no_cover_words_left(
                remaining_data_size,
                pivot,
            ));
        }

        for action in get_variant_methods(&self.variant) {
            let method_result = match action {
                MethodActions::LineExtend => self.le_submethod.conceal_in_extended_line(
                    graphemes_length(&pivot_line),
                    word_iterator,
                    data,
                    result,
                ),
                MethodActions::RandomASCIIWhitespace => {
                    self.rw_submethod.conceal_in_random_whitespace(data, result)
                }
                MethodActions::TrailingASCIIWhitespace => Ok(self
                    .tw_submethod
                    .conceal_in_trailing_whitespace(data, result)),
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
        let actions = get_variant_methods(&self.variant);
        let mut current_line = line.to_string();

        let mut gathered_bits: BitVec<Order, Type> = BitVec::with_capacity(3);
        for action in actions.iter().rev() {
            match action {
                MethodActions::LineExtend => {
                    self.le_submethod
                        .reveal_in_extended_line(&current_line, &mut gathered_bits);
                }
                MethodActions::RandomASCIIWhitespace => {
                    self.rw_submethod
                        .reveal_in_random_whitespace(&mut current_line, &mut gathered_bits);
                }
                MethodActions::TrailingASCIIWhitespace => {
                    self.tw_submethod
                        .reveal_in_trailing_whitespace(&mut current_line, &mut gathered_bits);
                }
            };
        }
        gathered_bits.reverse();
        revealed_data.append(&mut gathered_bits);
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
        self.le_submethod.verify_pivot(cover)?;

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
            let line = self.le_submethod.construct_pivot_line(&mut word_iterator);
            if line.is_empty() {
                break;
            }
            result.push_str(NEWLINE_STR);
            result.push_str(&line);
        }

        self.notify(&MethodProgressStatus::Finished);

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
