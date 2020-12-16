use std::cell::RefMut;

use crate::text::WordIterator;

pub struct Context<'a, T>
where
    T: WordIterator,
{
    pivot: Option<usize>,
    word_iter: Option<RefMut<'a, T>>,
}

impl<'a, T> Context<'a, T>
where
    T: WordIterator,
{
    pub fn new() -> Self {
        Context {
            pivot: None,
            word_iter: None,
        }
    }

    pub fn set_pivot(&mut self, pivot: usize) -> &Self {
        self.pivot = Some(pivot);
        self
    }

    pub fn set_word_iter(&mut self, word_iter: RefMut<'a, T>) -> &Self {
        self.word_iter = Some(word_iter);
        self
    }
}
