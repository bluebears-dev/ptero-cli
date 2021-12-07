//! # ptero
//!
//! `ptero` is a crate with utilities for the `ptero_cli` frontend

#[macro_use]
extern crate derive_builder;

/// Utils for binary data manipulation
pub mod binary;

/// Provides functions for manipulation text e.g. word iterator
pub mod text;

/// Contains secret data encoders both simple and complex ones.
pub mod encoder;

/// Contains stegotext decoders.
pub mod decoder;

/// Context containing all needed data (e.g. access to cover text) for the steganography methods.
pub mod context;

/// Module containing all the available methods for text steganography. 
pub mod method;

/// Logger utilities.
pub mod log;
pub mod observer;

pub mod cli {
    pub mod capacity;
    pub mod decoder;
    pub mod encoder;
    pub mod writer;
    pub mod progress;
}

