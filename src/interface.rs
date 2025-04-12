//! The interfaces that describe a stable vector.

use std::mem;

use crate::error::Result;

/// The interface that defines the full functionality of a stable vector.
pub trait StableVec<Index: StableVecIndex, Data>:
    StableVecAccess<Index, Data> + From<Vec<Data>> + IntoIterator<Item = Data> + FromIterator<Data>
where
    for<'a> &'a Self: IntoIterator<Item = &'a Data>,
    for<'a> &'a mut Self: IntoIterator<Item = &'a mut Data>,
{
    /// Insert a single element into the stable vector at an arbitrary index.
    /// Return the index.
    fn insert(&mut self, element: Data) -> Index;

    /// Insert the default value for a single element into the stable vector at an arbitrary index.
    /// Return the index.
    fn insert_default(&mut self) -> Index
    where
        Data: Default,
    {
        self.insert(Default::default())
    }

    /// Insert a single element into the stable vector by constructing it in place.
    /// This method allows to create the element while already knowing its index.
    /// Returns the index.
    fn insert_in_place(&mut self, constructor: impl FnOnce(Index) -> Data) -> Index;

    /// Inserts a single element into the stable vector at the given index.
    /// The index must be the first index in the iterator returned by [available_insertion_index_iterator](StableVec::available_insertion_index_iterator).
    /// If a different index is given, an [`Error::NotTheNextAvailableInsertionIndex`](crate::error::Error::NotTheNextAvailableInsertionIndex) is returned.
    fn insert_at(&mut self, index: Index, element: Data) -> crate::error::Result<()>;

    /// Inserts a single element into the stable vector at the given index.
    /// This index may be any index that is not currently in use.
    /// If an index that is already mapped to an element is given, an [`Error::IndexAlreadyInUse`](crate::error::Error::IndexAlreadyInUse) is returned.
    ///
    /// **WARNING:** this method may be slower than expected, because it may for example need to update a free list.
    fn insert_at_arbitrary_index(
        &mut self,
        index: Index,
        element: Data,
    ) -> crate::error::Result<()>;

    /// Sets the index to the given value.
    /// If the index was already mapped to a value, the old value is returned.
    /// Otherwise, the given value is newly inserted.
    fn set(&mut self, index: Index, mut element: Data) -> Option<Data> {
        let index: usize = index.into();
        if let Ok(existing_element) = self.get_mut(index.into()) {
            mem::swap(&mut element, existing_element);
            Some(element)
        } else {
            self.insert_at_arbitrary_index(index.into(), element)
                .unwrap();
            None
        }
    }

    /// Insert multiple elements into the stable vector at arbitrary indices.
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

    /// Insert multiple elements into the stable vector at arbitrary indices.
    /// The elements are constructed in place, which allows to create them while already knowing their indices.
    /// The indices are returned as an iterator in the order of the inserted elements.
    ///
    /// **Warning**: the returned iterator must be completely exhausted in order to insert all elements.
    #[must_use = "this iterator must be completely exhausted in order to insert all elements"]
    fn insert_in_place_from_iter(
        &mut self,
        elements: impl IntoIterator<Item = impl FnOnce(Index) -> Data>,
    ) -> impl Iterator<Item = Index> {
        elements
            .into_iter()
            .map(|constructor| self.insert_in_place(constructor))
    }

    /// Remove and return the element at the given index.
    /// If the index is not mapped to any element, an [`Error::UnmappedIndex`](crate::error::Error::UnmappedIndex) is returned.
    fn remove(&mut self, index: Index) -> Result<Data>;

    /// Returns an iterator that iterates over the available insertion indices in this stable vector.
    /// These are the "holes" in the underlying vector,
    /// followed by the indices after the end of the underlying vector.
    fn available_insertion_index_iterator<'result>(&self) -> impl 'result + Iterator<Item = Index>
    where
        Index: 'result;

    /// Return an iterator over the pairs of (index, element) in this stable vec.
    fn iter<'this>(&'this self) -> impl 'this + Iterator<Item = (Index, &'this Data)>
    where
        Data: 'this;

    /// Return an iterator over the pairs of (index, element) in this stable vec.
    fn iter_mut<'this>(&'this mut self) -> impl 'this + Iterator<Item = (Index, &'this mut Data)>
    where
        Data: 'this;

    /// Return an iterator over the elements in this stable vec.
    fn iter_elements<'this>(&'this self) -> impl 'this + Iterator<Item = &'this Data>
    where
        Data: 'this,
    {
        self.iter().map(|(_, element)| element)
    }

    /// Return an iterator over the elements in this stable vec.
    fn iter_elements_mut<'this>(&'this mut self) -> impl 'this + Iterator<Item = &'this mut Data>
    where
        Data: 'this,
    {
        self.iter_mut().map(|(_, element)| element)
    }

    /// Return an iterator over the indices that are currently valid for this stable vec.
    fn iter_indices<'this>(&'this self) -> impl 'this + Iterator<Item = Index>
    where
        Data: 'this,
    {
        self.iter().map(|(index, _)| index)
    }

    /// Remove all elements `e` for which `f(&e)` returns `false`.
    fn retain(&mut self, f: impl FnMut(&Data) -> bool);

    /// Delete all elements from the stable vector.
    fn clear(&mut self);
}

/// The interface that describes methods to access elements inside a stable vector.
///
/// This is separate from the [`StableVec`] trait to allow creating views of a stable vector that do not allow insertion or deletion, but still grants mutable access to contained elements.
pub trait StableVecAccess<Index, Data> {
    /// Get a reference to the element at the given index.
    /// If the index is not mapped to an element, an [`Error::UnmappedIndex`](crate::error::Error::UnmappedIndex) is returned.
    fn get(&self, index: Index) -> Result<&Data>;

    /// Get a mutable reference to the element at the given index.
    /// If the index is not mapped to an element, an [`Error::UnmappedIndex`](crate::error::Error::UnmappedIndex) is returned.
    fn get_mut(&mut self, index: Index) -> Result<&mut Data>;

    /// Return the number of elements in the stable vector.
    fn len(&self) -> usize;

    /// Returns true if the stable vector is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// The interface that describes the index type of a stable vector.
pub trait StableVecIndex: From<usize> + Into<usize> {}
