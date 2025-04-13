//! Errors and related types.

use std::slice::GetDisjointMutError;

use thiserror::Error;

/// The error type.
#[derive(Debug, Error)]
pub enum Error {
    /// The given index is not mapped to any element.
    #[error("the given index {index} is not mapped to any element")]
    UnmappedIndex {
        /// The index.
        index: usize,
    },

    /// The given indices contain an overlapping pair of indices.
    #[error("the given indices contain an overlapping pair of indices")]
    OverlappingIndices,

    /// The given index is already mapped to an element.
    #[error("the given index {index} is already mapped to an element")]
    IndexAlreadyInUse {
        /// The index.
        index: usize,
    },

    /// The given index is not the next available insertion index.
    #[error(
        "The given index {actual_index} is not the next available insertion index {expected_index}"
    )]
    NotTheNextAvailableInsertionIndex {
        /// The expected next available insertion index.
        expected_index: usize,
        /// The given invalid insertion index.
        actual_index: usize,
    },
}

impl From<GetDisjointMutError> for Error {
    fn from(value: GetDisjointMutError) -> Self {
        match value {
            GetDisjointMutError::IndexOutOfBounds => {
                unreachable!("We always want to report the index")
            }
            GetDisjointMutError::OverlappingIndices => Self::OverlappingIndices,
        }
    }
}

/// A shortcut result type using this crate's error type.
pub type Result<T> = std::result::Result<T, Error>;
