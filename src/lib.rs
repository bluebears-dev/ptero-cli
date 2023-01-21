//! # ptero
//!
//! `ptero` is a crate with utilities for the `ptero_cli` frontend

/// Provides functions for manipulation text e.g. word iterator
pub mod text;

/// Logger utilities.
pub mod log;

pub mod cli {
    pub mod commands;
    pub mod progress;
    pub mod writer;
}
