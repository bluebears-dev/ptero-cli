//! # ptero
//!
//! `ptero` is a crate with utilities for the `ptero_cli` frontend

/// Utils for binary data manipulation
pub mod binary;

/// Provides functions for manipulation text e.g. word iterator
pub mod text;

/// Contains secret data encoders both simple and complex ones.
pub mod encoder;

/// Contains stegotext decoders.
pub mod decoder;

/// Definition of Encodable types.
pub mod encodable;
