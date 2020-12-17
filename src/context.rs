use std::{
    error::Error,
    fmt,
};

use crate::text::{CoverTextLineIterator, CoverTextWordIterator};

pub trait Context {
    fn get_current_text_mut(&mut self) -> Result<&mut String, ContextError>;

    fn get_current_text(&self) -> Result<&String, ContextError>;

    fn load_line(&mut self) -> Result<&String, ContextError>;
}

pub struct PivotDecoderContext {
    pivot: usize,
    cover_text_iter: CoverTextLineIterator,
    current_text: Option<String>,
}

pub struct PivotEncoderContext {
    pivot: usize,
    cover_text_iter: CoverTextWordIterator,
    current_text: Option<String>,
}

impl PivotDecoderContext {
    pub fn new(cover_text: &str, pivot: usize) -> Self {
        PivotDecoderContext {
            pivot,
            cover_text_iter: CoverTextLineIterator::new(cover_text),
            current_text: None,
        }
    }

    pub fn get_pivot(&self) -> usize {
        self.pivot
    }
}

impl PivotEncoderContext {
    pub fn new(cover_text: &str, pivot: usize) -> Self {
        PivotEncoderContext {
            pivot,
            cover_text_iter: CoverTextWordIterator::new(cover_text),
            current_text: None,
        }
    }

    pub fn get_pivot(&self) -> usize {
        self.pivot
    }

    pub fn get_cover_text_iter_mut(&mut self) -> &mut CoverTextWordIterator {
        &mut self.cover_text_iter
    }
}

impl Context for PivotEncoderContext {
    fn get_current_text_mut(&mut self) -> Result<&mut String, ContextError> {
        self.current_text
            .as_mut()
            .ok_or(ContextError::new(ContextErrorKind::NoTextLeft))
    }

    fn get_current_text(&self) -> Result<&String, ContextError> {
        self.current_text
            .as_ref()
            .ok_or(ContextError::new(ContextErrorKind::NoTextLeft))
    }

    fn load_line(&mut self) -> Result<&String, ContextError> {
        self.current_text = self
            .cover_text_iter
            .construct_line_by_pivot(self.get_pivot());
        self.current_text
            .as_ref()
            .ok_or(ContextError::new(ContextErrorKind::NoTextLeft))
    }
}

impl Context for PivotDecoderContext {
    fn get_current_text_mut(&mut self) -> Result<&mut String, ContextError> {
        self.current_text
            .as_mut()
            .ok_or(ContextError::new(ContextErrorKind::NoTextLeft))
    }

    fn get_current_text(&self) -> Result<&String, ContextError> {
        self.current_text
            .as_ref()
            .ok_or(ContextError::new(ContextErrorKind::NoTextLeft))
    }

    fn load_line(&mut self) -> Result<&String, ContextError> {
        self.current_text = self.cover_text_iter.next().map(|x| x.to_string());
        self.current_text
            .as_ref()
            .ok_or(ContextError::new(ContextErrorKind::NoTextLeft))
    }
}

#[derive(Debug)]
enum ContextErrorKind {
    NoTextLeft,
}

#[derive(Debug)]
pub struct ContextError {
    kind: ContextErrorKind,
}

impl ContextError {
    fn new(kind: ContextErrorKind) -> Self {
        ContextError { kind }
    }
}

impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ContextErrorKind::NoTextLeft => write!(f, "No cover text left.",),
        }
    }
}

impl Error for ContextError {}
