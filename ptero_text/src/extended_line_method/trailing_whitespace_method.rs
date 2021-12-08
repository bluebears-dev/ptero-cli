use std::cell::RefCell;
use std::rc::{Rc, Weak};

use bitvec::prelude::*;
use bitvec::slice::Iter;
use log::trace;

use ptero_common::config::CommonMethodConfig;
use ptero_common::method::{MethodProgressStatus, MethodResult};

use crate::extended_line_method::character_sets::{CharacterSetType, GetCharacterSet};

impl TrailingWhitespaceMethodBuilder {
    pub(crate) fn with_shared_config(mut self, config: Rc<RefCell<CommonMethodConfig>>) -> Self {
        self.config_ref = Some(Rc::downgrade(&config));
        self
    }

    pub(crate) fn with_charset<T>(mut self, charset: T) -> Self where T: GetCharacterSet + 'static {
        self.charset = Some(Box::new(charset));
        self
    }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct TrailingWhitespaceMethod {
    #[builder(private)]
    config_ref: Weak<RefCell<CommonMethodConfig>>,
    #[builder(
        private,
        default = "Box::new(CharacterSetType::OneBit)"
    )]
    charset: Box<dyn GetCharacterSet>,
}

impl TrailingWhitespaceMethod {
    pub(crate) fn builder() -> TrailingWhitespaceMethodBuilder {
        TrailingWhitespaceMethodBuilder::default()
    }

    pub(crate) fn notify(&mut self, event: &MethodProgressStatus) {
        let config = self
            .config_ref
            .upgrade()
            .expect("Invalid config reference passed, cannot upgrade weak reference");

        config.borrow_mut().notifier.notify(event);
    }

    fn bitrate(&self) -> usize {
        let amount_of_bits = std::mem::size_of::<usize>() * 8;
        amount_of_bits - self.charset.size().leading_zeros() as usize
    }

    fn assemble_charset_index(&self, next_bits: &BitSlice<Lsb0, usize>) -> usize {
        let bitrate = self.bitrate();
        let index = next_bits.as_raw_slice()[0];
        // Pad to bitrate
        // We might end-up with lower amount of bits than suggested by bitrate
        index << (bitrate - next_bits.len())
    }

    pub(crate) fn conceal_in_trailing_whitespace<Order, Type>(
        &mut self,
        data: &mut Iter<Order, Type>,
        cover: &mut String,
    ) -> MethodResult
    where
        Order: BitOrder,
        Type: BitStore,
    {
        let bitrate = self.bitrate();
        let next_n_bits = data.take(bitrate).collect::<BitVec<Lsb0, usize>>();

        if next_n_bits.is_empty() {
            return MethodResult::NoDataLeft;
        }

        let charset_index = self.assemble_charset_index(&next_n_bits);

        trace!(
            "Took {} bits and assembled a number: {}",
            self.bitrate(),
            charset_index
        );

        if let Some(character) = self.charset.get_character(charset_index) {
            trace!(
                "Putting unicode character {:?} at the end of the line",
                character
            );
            cover.push(*character);
        } else {
            trace!("Skipping trailing whitespace");
        }

        if next_n_bits.len() < bitrate {
            MethodResult::NoDataLeft
        } else {
            self.notify(&MethodProgressStatus::DataWritten(bitrate as u64));
            MethodResult::Success
        }
    }

    pub(crate) fn reveal_in_trailing_whitespace<Order, Type>(
        &mut self,
        stego_text_line: &mut String,
        revealed_data: &mut BitVec<Order, Type>,
    ) where
        Order: BitOrder,
        Type: BitStore,
    {
        if let Some(last_char) = stego_text_line.chars().last() {
            let decoded_number = self.charset.character_to_bits(&last_char);

            trace!(
                "Found {:?} at the end of the line, decoded into {:b}",
                &last_char,
                decoded_number
            );

            let data: &BitSlice<Msb0, usize> = BitSlice::from_element(&decoded_number);
            let data_length = data.len();
            revealed_data.extend(data.into_iter().skip(data_length - self.bitrate()));

            if decoded_number > 0 {
                stego_text_line.remove(stego_text_line.len() - last_char.len_utf8());
            }
        } else {
            trace!("Empty line received, skipping");
        }
    }
}
