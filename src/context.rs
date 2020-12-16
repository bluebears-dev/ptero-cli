use std::{cell::RefMut, error::Error, fmt};

use crate::text::WordIterator;

type RefMutWordIterator<'a> = RefMut<'a, >;
pub struct Context<'a> {
    pivot: Option<usize>,
    word_iter: &'a dyn WordIterator,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Context {
            pivot: None,
            word_iter: None,
        }
    }

    pub fn set_pivot(&mut self, pivot: usize) {
        self.pivot = Some(pivot);
    }

    pub fn set_word_iter(&mut self, word_iter: RefMutWordIterator<'a>) {
        self.word_iter = Some(word_iter);
    }

    pub fn get_pivot(&self) -> Result<usize, ContextError> {
        self.pivot.ok_or(ContextError::new("pivot".to_string()))
    }

    pub fn get_word_iter(&self) -> Result<RefMutWordIterator<'a>, ContextError> {
        self.word_iter
            .ok_or(ContextError::new("word_iter".to_string()))
    }
}

#[derive(Debug)]
pub struct ContextError {
    missing_prop: String,
}

impl ContextError {
    fn new(missing_prop: String) -> Self {
        ContextError { missing_prop }
    }
}

impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Incomplete context. Trying to access {} while it's not set.",
            self.missing_prop
        )
    }
}

impl Error for ContextError {}
