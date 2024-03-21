//! Errors and related types.

use thiserror::Error;

/// The error type.
#[derive(Debug, Error)]
pub enum Error {
    /// The given index is not mapped to any data item.
    #[error("the given index {index} is not mapped to any data item")]
    InvalidIndex {
        /// The index.
        index: usize,
    },
}

/// A shortcut result type using this crate's error type.
pub type Result<T> = std::result::Result<T, Error>;
