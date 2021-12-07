use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::Sender;

use bitvec::prelude::*;
use bitvec::slice::Iter;
use log::trace;
use rand::RngCore;
use unicode_segmentation::UnicodeSegmentation;

use ptero_common::config::{CommonMethodConfig, CommonMethodConfigBuilder};
use ptero_common::method::{MethodProgressStatus, MethodResult};

const DEFAULT_ASCII_DELIMITER: &str = " ";
const NEWLINE_STR: &str = "\n";

pub struct TrailingWhitespaceMethodBuilder<'a> {
    config_builder: CommonMethodConfigBuilder<'a>,
    whitespace_str: &'static str
}

impl<'a> Default for TrailingWhitespaceMethodBuilder<'a> {
    fn default() -> Self {
        TrailingWhitespaceMethodBuilder {
            config_builder:  CommonMethodConfig::builder(),
            whitespace_str: DEFAULT_ASCII_DELIMITER
        }
    }
}

impl<'a> TrailingWhitespaceMethodBuilder<'a> {
    /// Set custom RNG for method.
    pub fn with_rng(mut self, rng: &Rc<RefCell<dyn RngCore>>) -> Self {
        self.config_builder = self.config_builder.with_rng(rng);
        self
    }

    /// Register progress status pipe
    pub fn register(mut self, observer: &'a Sender<MethodProgressStatus>) -> Self {
        self.config_builder = self.config_builder.register(observer);
        self
    }

    /// Proxy method for passing optional, see [`RandomWhitespaceMethodBuilder::register`] for
    /// alternative.
    pub fn maybe_register(mut self, maybe_observer: Option<&'a Sender<MethodProgressStatus>>) -> Self {
        self.config_builder = self.config_builder.maybe_register(maybe_observer);
        self
    }

    /// Set custom whitespace delimiter
    pub fn with_custom_whitespace_str(mut self, whitespace_str: &'static str) -> Self {
        self.whitespace_str = whitespace_str;
        self
    }

    pub fn build(self) -> TrailingWhitespaceMethod<'a> {
        TrailingWhitespaceMethod {
            config: self.config_builder.build().unwrap(),
            whitespace_str: self.whitespace_str
        }
    }
}

pub struct TrailingWhitespaceMethod<'a> {
    config: CommonMethodConfig<'a>,
    whitespace_str: &'static str
}

impl<'a> TrailingWhitespaceMethod<'a> {
    pub fn builder() -> TrailingWhitespaceMethodBuilder<'a> {
        TrailingWhitespaceMethodBuilder::default()
    }

    pub(crate) fn conceal_in_trailing_whitespace<Order, Type>(
        &self,
        data: &mut Iter<Order, Type>,
        cover: &mut String,
    ) -> MethodResult
        where
            Order: BitOrder,
            Type: BitStore,
    {
        match data.next().as_deref() {
            Some(true) => {
                trace!("Putting whitespace at the end of the line");
                cover.push_str(self.whitespace_str);
                MethodResult::Success
            }
            Some(false) => {
                trace!("Skipping trailing whitespace");
                MethodResult::Success
            }
            None => MethodResult::NoDataLeft,
        }
    }

    pub(crate) fn reveal_in_trailing_whitespace<Order, Type>(
        &mut self,
        stego_text_line: &mut String,
        revealed_data: &mut BitVec<Order, Type>
    )
        where
            Order: BitOrder,
            Type: BitStore,
    {
        let bit = stego_text_line.graphemes(true)
            .last()
            .map(|cluster| cluster == self.whitespace_str)
            .unwrap_or(false);

        trace!("Found trailing whitespace: {}", bit);
        if bit {
            stego_text_line.remove(stego_text_line.len() - 1);
        }
        revealed_data.push(bit);
    }
}