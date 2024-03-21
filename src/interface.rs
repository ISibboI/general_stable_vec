//! The interfaces that describe a stable vector.

use crate::error::Result;

/// The interface that defines the full functionality of a stable vector.
pub trait StableVec<Data, Index>: StableVecAccess<Data, Index> {
    /// Insert a single item into the stable vector at an arbitrary index.
    /// Return the index.
    fn insert(&mut self, element: Data) -> Index;

    /// Insert multiple items into the stable vector at arbitrary indices.
    /// The indices are returned as an iterator in the order of the inserted elements.
    ///
    /// **Warning**: the returned iterator must be completely exhausted in order to insert all elements.
    #[must_use = "this iterator must be completely exhausted in order to insert all given elements"]
    fn insert_from_iter(
        &mut self,
        elements: impl IntoIterator<Item = Data>,
    ) -> impl Iterator<Item = Index> {
        elements.into_iter().map(|element| self.insert(element))
    }

    /// Remove and return the element at the given index.
    /// If the index is invalid, an [`Error::InvalidIndex`](crate::error::Error::InvalidIndex) is returned.
    fn remove(&mut self, index: Index) -> Result<Data>;

    /// Delete all elements from the stable vector.
    fn clear(&mut self);
}

/// The interface that describes methods to access elements inside a stable vector.
/// This is separate from the [`StableVec`] trait to allow creating views of a stable vector that do not allow insertion or deletion, but still grants mutable access to contained elements.
pub trait StableVecAccess<Data, Index> {
    /// Get a reference to the element at the given index.
    /// If the index is invalid, an [`Error::InvalidIndex`](crate::error::Error::InvalidIndex) is returned.
    fn get(&self, index: Index) -> Result<&Data>;

    /// Get a mutable reference to the element at the given index.
    /// If the index is invalid, an [`Error::InvalidIndex`](crate::error::Error::InvalidIndex) is returned.
    fn get_mut(&mut self, index: Index) -> Result<&mut Data>;
}

/// The interface that describes the index type of a stable vector.
pub trait StableVecIndex: From<usize> + Into<usize> {}
