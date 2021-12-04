use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use std::sync::mpsc::Sender;

use bitvec::prelude::*;
use bitvec::slice::Iter;
use log::trace;
use rand::{Rng, RngCore};

use crate::encoder::EncoderResult;
use crate::method::config::{CommonMethodConfig, CommonMethodConfigBuilder, MethodProgressStatus};

const DEFAULT_ASCII_DELIMITER: &str = " ";
const NEWLINE_STR: &str = "\n";

pub struct RandomWhitespaceMethodBuilder<'a> {
    config_builder: CommonMethodConfigBuilder<'a>,
    whitespace_str: &'static str
}

impl<'a> Default for RandomWhitespaceMethodBuilder<'a> {
    fn default() -> Self {
        RandomWhitespaceMethodBuilder {
            config_builder:  CommonMethodConfig::builder(),
            whitespace_str: DEFAULT_ASCII_DELIMITER
        }
    }
}

impl<'a> RandomWhitespaceMethodBuilder<'a> {
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

    pub fn build(self) -> RandomWhitespaceMethod<'a> {
        RandomWhitespaceMethod {
            config: self.config_builder.build().unwrap(),
            whitespace_str: self.whitespace_str
        }
    }
}

pub struct RandomWhitespaceMethod<'a> {
    config: CommonMethodConfig<'a>,
    whitespace_str: &'static str
}

impl<'a> RandomWhitespaceMethod<'a> {
    pub fn builder() -> RandomWhitespaceMethodBuilder<'a> {
        RandomWhitespaceMethodBuilder::default()
    }

    fn find_approx_whitespace_position(
        &mut self,
        cover: &mut String,
        last_newline_index: usize,
    ) -> usize {
        let rng = &*self.config.rng.upgrade().unwrap();
        let approx_position = rng.borrow_mut().gen_range(last_newline_index, cover.len());

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

    pub(crate) fn conceal_in_random_ascii_whitespace<Order, Type>(
        &mut self,
        data: &mut Iter<Order, Type>,
        cover: &mut String,
    ) -> EncoderResult
        where
            Order: BitOrder,
            Type: BitStore,
    {
        match data.next().as_deref() {
            Some(true) => {
                let last_newline_index = cover.rfind(NEWLINE_STR).unwrap_or(0);
                let position = self.find_approx_whitespace_position(cover, last_newline_index);

                trace!("Putting space at position {}", position);
                cover.insert_str(position, &String::from(self.whitespace_str));
                EncoderResult::Success
            }
            Some(false) => {
                trace!("Skipping double whitespace");
                EncoderResult::Success
            }
            None => EncoderResult::NoDataLeft,
        }
    }
}