use std::{error::Error, fmt};

use log::error;

use crate::text::{CoverTextLineIterator, CoverTextWordIterator};

/// Context with essential methods for every encoder/decoder.
pub trait Context {
    /// Gets currently loaded cover text fragment *mutably*.
    ///
    /// # Returns
    /// Result which is either `&mut String` or [ContextError].
    fn get_current_text_mut(&mut self) -> Result<&mut String, ContextError>;

    /// Gets currently loaded cover text fragment as read-only.
    ///
    /// # Returns
    /// Result which is either `&String` or [ContextError].
    fn get_current_text(&self) -> Result<&String, ContextError>;

    /// Loads next cover text fragment.
    ///
    /// # Returns
    /// Result which is either `&String` or [ContextError]. Returned string is the newly loaded fragment.
    fn load_text(&mut self) -> Result<&str, ContextError>;
}

/// Context used by methods requiring pivot.
/// Loads and returns cover text line by line - *not bound by pivot*.
pub struct PivotByRawLineContext {
    pivot: usize,
    cover_text_iter: CoverTextLineIterator,
    current_text: Option<String>,
}
/// Context used by methods requiring pivot.
/// Loads cover text line by line, uses pivot and does not preserve original whitespace.
/// It also exposes the word iterator for purpose of peeking and/or traversing.
pub struct PivotByLineContext {
    pivot: usize,
    cover_text_iter: CoverTextWordIterator,
    current_text: Option<String>,
}

impl PivotByRawLineContext {
    pub fn new(cover_text: &str, pivot: usize) -> Self {
        PivotByRawLineContext {
            pivot,
            cover_text_iter: CoverTextLineIterator::new(cover_text),
            current_text: None,
        }
    }

    pub fn get_pivot(&self) -> usize {
        self.pivot
    }
}

impl PivotByLineContext {
    const WORD_DELIMITER: char = ' ';

    pub fn new(cover_text: &str, pivot: usize) -> Self {
        PivotByLineContext {
            pivot,
            cover_text_iter: CoverTextWordIterator::new(cover_text),
            current_text: None,
        }
    }

    pub fn get_pivot(&self) -> usize {
        self.pivot
    }

    // Peeks the next word without forwarding the iterator.
    //
    // # Returns
    // Returns the next word or none if the iterator gets out-of-bounds.
    pub fn peek_word(&mut self) -> Option<String> {
        self.cover_text_iter.peek()
    }
    
    // Gets the next word and proceeds the iterator.
    //
    // # Returns
    // Returns the next word or none if the iterator gets out-of-bounds.
    pub fn next_word(&mut self) -> Option<String> {
        self.cover_text_iter.next()
    }

    // Constructs line of maximum length determined by pivot.
    //
    // # Returns
    // Returns the line or none if there are no words left. It is a result, 
    // throws an error if line cannot be constructed but there are still words left.
    fn construct_line_by_pivot(&mut self) -> Result<Option<String>, ContextError> {
        let maybe_word = self.cover_text_iter.peek();

        if maybe_word.is_none() {
            return Ok(None); 
        }

        let mut word = maybe_word.unwrap();
        
        if word.len() > self.pivot {
            error!("Stuck at word of length {}.", word.len());
            return Err(ContextError { kind: ContextErrorKind::CannotConstructLine });
        }
        let mut line = String::new();
        while line.len() + word.len() <= self.pivot {
            line.push_str(&word);
            line.push(Self::WORD_DELIMITER);
            // Skip the peeked word
            self.cover_text_iter.next();

            if let Some(next_word) = self.cover_text_iter.peek() {
                word = next_word;
            } else {
                return Ok(Some(line));
            }
        }
        Ok(Some(line.trim_end().to_string()))
    }
}

impl Context for PivotByLineContext {
    fn get_current_text_mut(&mut self) -> Result<&mut String, ContextError> {
        self.current_text
            .as_mut()
            .ok_or_else(|| ContextError::new(ContextErrorKind::NoTextLeft))
    }

    fn get_current_text(&self) -> Result<&String, ContextError> {
        self.current_text
            .as_ref()
            .ok_or_else(|| ContextError::new(ContextErrorKind::NoTextLeft))
    }

    // Loads the line considering the pivot. Line length is smaller or equal the pivot value.
    // This function does not preserve original whitespace. By default words are delimited by standard ASCII space (0x20). 
    //
    // # Returns
    // Result which is either the line or [ContextError] if anything fails. 
    fn load_text(&mut self) -> Result<&str, ContextError> {
        self.current_text = self.construct_line_by_pivot()?;
        self.current_text
            .as_deref()
            .ok_or_else(|| ContextError::new(ContextErrorKind::NoTextLeft))
    }
}

impl Context for PivotByRawLineContext {
    fn get_current_text_mut(&mut self) -> Result<&mut String, ContextError> {
        self.current_text
            .as_mut()
            .ok_or_else(|| ContextError::new(ContextErrorKind::NoTextLeft))
    }

    fn get_current_text(&self) -> Result<&String, ContextError> {
        self.current_text
            .as_ref()
            
            .ok_or_else(|| ContextError::new(ContextErrorKind::NoTextLeft))
    }

    // Loads the raw line. By raw it means preserving the whitespace
    //
    // # Returns
    // Result which is either the line or [ContextError] if anything fails. 
    fn load_text(&mut self) -> Result<&str, ContextError> {
        self.current_text = self.cover_text_iter.next();
        self.current_text
            .as_deref()
            .ok_or_else(|| ContextError::new(ContextErrorKind::NoTextLeft))
    }
}

/// Enum determining the exact context error.
#[derive(Debug, Clone, Copy)]
pub enum ContextErrorKind {
    NoTextLeft,
    CannotConstructLine
}

/// Error implementation for [Context]. Exact error message is determined by [ContextErrorKind].
#[derive(Debug)]
pub struct ContextError {
    kind: ContextErrorKind,
}

impl ContextError {
    fn new(kind: ContextErrorKind) -> Self {
        ContextError { kind }
    }

    pub fn kind(&self) -> ContextErrorKind {
        self.kind
    }
}

impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ContextErrorKind::NoTextLeft => write!(f, "No cover text left.",),
            ContextErrorKind::CannotConstructLine => write!(f, "Pivot is too small. Couldn't load the line no longer than the pivot.",),
        }
    }
}

impl Error for ContextError {}
