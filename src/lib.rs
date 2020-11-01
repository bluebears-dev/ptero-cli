//! # ptero
//!
//! `ptero` is a crate with utilities for the `ptero_cli` frontend

/// Utils for binary data manipulation
#[path = "lib/binary.rs"]
pub mod binary;

/// Provides functions for manipulation text e.g. word iterator
#[path = "lib/text.rs"]
pub mod text;
