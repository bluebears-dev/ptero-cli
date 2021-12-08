use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use bitvec::prelude::*;
use bitvec::slice::Iter;
use log::trace;
use rand::{Rng, RngCore};
use unicode_segmentation::UnicodeSegmentation;

use ptero_common::config::{CommonMethodConfig, CommonMethodConfigBuilder};
use ptero_common::method::{MethodProgressStatus, MethodResult};
use ptero_common::observer::{EventNotifier, Observable, Observer};

use crate::extended_line_method::{ConcealError, Result};

const DEFAULT_ASCII_DELIMITER: &str = " ";
const NEWLINE_STR: &str = "\n";

pub(crate) struct RandomWhitespaceMethodBuilder {
    config_builder: CommonMethodConfigBuilder,
    whitespace_str: &'static str,
}

impl RandomWhitespaceMethodBuilder {
    pub(crate) fn new() -> Self {
        RandomWhitespaceMethodBuilder {
            config_builder: CommonMethodConfig::builder(),
            whitespace_str: DEFAULT_ASCII_DELIMITER,
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

    /// Set custom whitespace delimiter
    pub(crate) fn with_custom_whitespace_str(mut self, whitespace_str: &'static str) -> Self {
        self.whitespace_str = whitespace_str;
        self
    }

    pub(crate) fn build(self) -> RandomWhitespaceMethod {
        RandomWhitespaceMethod {
            config: self.config_builder.build().unwrap(),
            whitespace_str: self.whitespace_str,
        }
    }
}

pub(crate) struct RandomWhitespaceMethod {
    config: CommonMethodConfig,
    whitespace_str: &'static str,
}

impl RandomWhitespaceMethod {
    const CYCLE_BITRATE: u64 = 1;

    pub(crate) fn builder() -> RandomWhitespaceMethodBuilder {
        RandomWhitespaceMethodBuilder::new()
    }

    pub(crate) fn subscribe(&mut self, subscriber: Arc<RefCell<dyn Observer<MethodProgressStatus>>>) {
        self.config.notifier.subscribe(subscriber);
    }

    pub(crate) fn notify(&mut self, event: &MethodProgressStatus) {
        self.config.notifier.notify(event);
    }

    fn find_approx_whitespace_position(
        &mut self,
        cover: &mut String,
        last_newline_index: usize,
    ) -> usize {
        let rng = &*self.config.rng.upgrade().unwrap();
        let approx_position = rng.borrow_mut().gen_range(last_newline_index..cover.len());

        let last_line = &cover[last_newline_index..];
        let mut position =
            last_line.find(' ').unwrap_or_else(|| last_line.len()) + last_newline_index;

        for (index, character) in last_line.char_indices() {
            if index + last_newline_index > approx_position {
                break;
            }
            if character.is_whitespace() && !NEWLINE_STR.contains(character) {
                position = index + last_newline_index;
            }
        }
        position
    }

    pub(crate) fn conceal_in_random_whitespace<Order, Type>(
        &mut self,
        data: &mut Iter<Order, Type>,
        cover: &mut String,
    ) -> Result<MethodResult>
    where
        Order: BitOrder,
        Type: BitStore,
    {
        Ok(match data.next().as_deref() {
            Some(true) => {
                let last_newline_index =
                    cover.rfind(NEWLINE_STR).map(|index| index + 1).unwrap_or(0);

                let position = self.find_approx_whitespace_position(cover, last_newline_index);

                if position == cover.len() {
                    return Err(ConcealError::not_enough_words(&cover[last_newline_index..]));
                }

                trace!("Putting space at position {}", position);
                cover.insert_str(position, &String::from(self.whitespace_str));

                self.config
                    .notifier
                    .notify(&MethodProgressStatus::DataWritten(Self::CYCLE_BITRATE));

                MethodResult::Success
            }
            Some(false) => {
                trace!("Skipping double whitespace");

                self.config
                    .notifier
                    .notify(&MethodProgressStatus::DataWritten(Self::CYCLE_BITRATE));

                MethodResult::Success
            }
            None => MethodResult::NoDataLeft,
        })
    }

    pub(crate) fn reveal_in_random_whitespace<Order, Type>(
        &mut self,
        stego_text_line: &mut String,
        revealed_data: &mut BitVec<Order, Type>,
    ) where
        Order: BitOrder,
        Type: BitStore,
    {
        let mut seen_whitespace = false;
        let mut bit = false;
        for (index, cluster) in stego_text_line.graphemes(true).enumerate() {
            let is_methods_whitespace = cluster == self.whitespace_str;
            if seen_whitespace && is_methods_whitespace {
                stego_text_line.remove(index);
                bit = true;
                break;
            }
            seen_whitespace = cluster.contains(char::is_whitespace);
        }
        trace!("Found two consecutive whitespaces: {}", bit);
        revealed_data.push(bit);
    }
}
