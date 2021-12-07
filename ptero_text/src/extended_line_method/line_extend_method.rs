use std::cell::RefCell;
use std::iter::Peekable;
use std::rc::Rc;

use bitvec::prelude::*;
use bitvec::slice::Iter;
use log::{debug, trace};
use rand::RngCore;

use ptero_common::config::{CommonMethodConfig, CommonMethodConfigBuilder};
use ptero_common::method::{MethodProgressStatus, MethodResult};
use ptero_common::observer::EventNotifier;

use crate::extended_line_method::{ConcealError, graphemes_length, Result};

const DEFAULT_ASCII_DELIMITER: &str = " ";

pub(crate) type VerificationResult = std::result::Result<(), ConcealError>;

pub(crate) struct LineExtendMethodBuilder {
    config_builder: CommonMethodConfigBuilder,
    pivot: usize,
}

impl LineExtendMethodBuilder {
    pub(crate) fn new() -> Self {
        LineExtendMethodBuilder {
            config_builder: CommonMethodConfigBuilder::default(),
            pivot: 0,
        }
    }

    /// Set custom RNG for method.
    pub(crate) fn with_rng(mut self, rng: &Rc<RefCell<dyn RngCore>>) -> Self {
        self.config_builder = self.config_builder.with_rng(rng);
        self
    }

    pub(crate) fn with_notifier(mut self, notifier: EventNotifier<MethodProgressStatus>) -> Self {
        self.config_builder = self.config_builder.with_notifier(notifier);
        self
    }

    pub(crate) fn with_pivot(mut self, pivot: usize) -> Self {
        self.pivot = pivot;
        self
    }

    pub(crate) fn build(self) -> LineExtendMethod {
        LineExtendMethod {
            config: self.config_builder.build().unwrap(),
            pivot: self.pivot,
        }
    }
}

pub(crate) struct LineExtendMethod {
    config: CommonMethodConfig,
    pivot: usize,
}

impl LineExtendMethod {
    const CYCLE_BITRATE: u64 = 1;

    pub(crate) fn builder() -> LineExtendMethodBuilder {
        LineExtendMethodBuilder::new()
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
                    + graphemes_length(DEFAULT_ASCII_DELIMITER);

                if extended_line_length <= self.pivot {
                    let remaining_data_size = data.count();
                    return Err(ConcealError::line_too_short(
                        remaining_data_size,
                        self.pivot,
                    ));
                }
                trace!("Extending line with '{}'", &next_word);
                result.push_str(DEFAULT_ASCII_DELIMITER);
                result.push_str(next_word);

                self.config
                    .notifier
                    .notify(&MethodProgressStatus::DataWritten(Self::CYCLE_BITRATE));

                MethodResult::Success
            }
            Some(false) => {
                trace!("Leaving line as-is");
                self.config
                    .notifier
                    .notify(&MethodProgressStatus::DataWritten(Self::CYCLE_BITRATE));
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
}

pub(crate) fn construct_pivot_line<'b, I>(pivot: usize, word_iter: &mut Peekable<I>) -> String
where
    I: Iterator<Item = &'b str>,
{
    let mut current_line_length = 0;
    let mut result = String::new();

    while let Some(next_word) = word_iter.peek() {
        let line_appendix = if current_line_length > 0 {
            [DEFAULT_ASCII_DELIMITER, next_word].join("")
        } else {
            next_word.to_string()
        };

        if current_line_length + graphemes_length(&line_appendix) > pivot {
            break;
        }

        current_line_length += graphemes_length(&line_appendix);
        result.push_str(&line_appendix);

        word_iter.next();
    }
    trace!(
        "Constructed line of length: '{}' while '{}' is the pivot",
        current_line_length,
        pivot
    );
    result
}

pub(crate) fn verify_pivot(pivot: usize, cover: &str) -> VerificationResult {
    debug!("Checking if pivot is feasible for provided cover");

    let words_longer_than_pivot = cover
        .split_whitespace()
        .filter(|word| word.len() > pivot)
        .collect::<Vec<&str>>();

    if !words_longer_than_pivot.is_empty() {
        let word = words_longer_than_pivot[0];
        Err(ConcealError::pivot_too_small(word.to_string(), pivot))
    } else {
        Ok(())
    }
}
