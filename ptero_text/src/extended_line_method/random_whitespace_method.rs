use std::cell::RefCell;
use std::rc::{Rc, Weak};

use bitvec::prelude::*;
use bitvec::slice::Iter;
use log::trace;
use rand::Rng;
use unicode_segmentation::UnicodeSegmentation;

use ptero_common::config::CommonMethodConfig;
use ptero_common::method::{MethodProgressStatus, MethodResult};

use crate::extended_line_method::{ConcealError, Result};

const DEFAULT_ASCII_DELIMITER: &str = " ";
const NEWLINE_STR: &str = "\n";

impl RandomWhitespaceMethodBuilder {
    pub(crate) fn with_shared_config(mut self, config: Rc<RefCell<CommonMethodConfig>>) -> Self {
        self.config_ref = Some(Rc::downgrade(&config));
        self
    }
}

#[derive(Builder)]
pub struct RandomWhitespaceMethod {
    #[builder(private, setter(into))]
    config_ref: Weak<RefCell<CommonMethodConfig>>,
    #[builder(
        setter(into, prefix = "with"),
        default = "DEFAULT_ASCII_DELIMITER"
    )]
    whitespace_str: &'static str,
}

impl RandomWhitespaceMethod {
    const CYCLE_BITRATE: u64 = 1;

    pub(crate) fn builder() -> RandomWhitespaceMethodBuilder {
        RandomWhitespaceMethodBuilder::default()
    }

    pub(crate) fn notify(&mut self, event: &MethodProgressStatus) {
        let config = self
            .config_ref
            .upgrade()
            .expect("Invalid config reference passed, cannot upgrade weak reference");

        config.borrow_mut().notifier.notify(event);
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

                self.notify(&MethodProgressStatus::DataWritten(Self::CYCLE_BITRATE));

                MethodResult::Success
            }
            Some(false) => {
                trace!("Skipping double whitespace");

                self.notify(&MethodProgressStatus::DataWritten(Self::CYCLE_BITRATE));

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

    fn find_approx_whitespace_position(
        &mut self,
        cover: &mut String,
        last_newline_index: usize,
    ) -> usize {
        let approx_position = self.generate_random_position(last_newline_index, cover.len());

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

    fn generate_random_position(
        &mut self,
        last_newline_index: usize,
        cover_length: usize,
    ) -> usize {
        let config = self
            .config_ref
            .upgrade()
            .expect("Invalid config reference passed, cannot upgrade weak reference");

        let rng = &mut config.borrow_mut().rng;
        rng.gen_range(last_newline_index..cover_length)
    }
}
