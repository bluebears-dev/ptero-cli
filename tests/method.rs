use std::cell::RefCell;
use std::rc::Rc;

use rand::RngCore;

use ptero::method::extended_line_method::{ExtendedLineMethod, Variant};

mod method {
    mod extended_line_method_test;
}

pub(crate) fn get_method<'a>(
    pivot: usize,
    variant: Variant,
    rng: &Rc<RefCell<dyn RngCore>>,
) -> ExtendedLineMethod<'a> {
    ExtendedLineMethod::builder()
        .with_pivot(pivot)
        .with_rng(rng)
        .with_variant(variant)
        .build()
}
