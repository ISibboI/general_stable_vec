//! A stable vector based on the [`Option`] type.
//! Each element is stored as an `Option`, and a free list is used to keep track of "holes" in the vector.
//! This allows amortised O(1) insertions and deletions, with a memory usage of O(|maximum len|).

use std::marker::PhantomData;

use crate::{
    error::Error,
    interface::{StableVec, StableVecAccess, StableVecIndex},
};

/// A stable vector based on the [`Option`] type with a free list.
/// Each element is stored as an `Option`, and a free list is used to keep track of "holes" in the vector.
/// This allows amortised O(1) insertions and deletions, with a memory usage of O(|maximum len|).
#[derive(Debug)]
pub struct OptionStableVec<Data, Index> {
    vec: Vec<Option<Data>>,
    free_list: Vec<usize>,
    phantom_data: PhantomData<Index>,
}

impl<Data, Index> OptionStableVec<Data, Index> {
    /// Create a new empty [`OptionStableVec`].
    pub fn new() -> Self {
        Self {
            vec: Default::default(),
            free_list: Default::default(),
            phantom_data: Default::default(),
        }
    }
}

impl<Data, Index: StableVecIndex> StableVec<Data, Index> for OptionStableVec<Data, Index> {
    fn insert(&mut self, element: Data) -> Index {
        let index = if let Some(index) = self.free_list.pop() {
            self.vec[index] = Some(element);
            index
        } else {
            let index = self.vec.len();
            self.vec.push(Some(element));
            index
        };
        index.into()
    }

    fn insert_in_place(&mut self, constructor: impl FnOnce(Index) -> Data) -> Index {
        let index = self.free_list.pop().unwrap_or(self.vec.len());
        let element = constructor(index.into());

        if index < self.vec.len() {
            self.vec[index] = Some(element);
        } else {
            self.vec.push(Some(element));
        }
        index.into()
    }

    fn remove(&mut self, index: Index) -> crate::error::Result<Data> {
        let index = index.into();
        if index < self.vec.len() {
            let element = Option::take(self.vec.get_mut(index).unwrap())
                .ok_or(Error::InvalidIndex { index })?;
            self.free_list.push(index);
            Ok(element)
        } else {
            Err(Error::InvalidIndex { index })
        }
    }

    fn clear(&mut self) {
        self.vec.clear();
        self.free_list.clear();
    }
}

impl<Data, Index: StableVecIndex> StableVecAccess<Data, Index> for OptionStableVec<Data, Index> {
    fn get(&self, index: Index) -> crate::error::Result<&Data> {
        let index = index.into();
        match self.vec.get(index) {
            Some(Some(element)) => Ok(element),
            _ => Err(Error::InvalidIndex { index }),
        }
    }

    fn get_mut(&mut self, index: Index) -> crate::error::Result<&mut Data> {
        let index = index.into();
        match self.vec.get_mut(index) {
            Some(Some(element)) => Ok(element),
            _ => Err(Error::InvalidIndex { index }),
        }
    }

    fn len(&self) -> usize {
        self.vec.len() - self.free_list.len()
    }
}

impl<Data, Index> Default for OptionStableVec<Data, Index> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Data: Clone, Index> Clone for OptionStableVec<Data, Index> {
    fn clone(&self) -> Self {
        Self {
            vec: self.vec.clone(),
            free_list: self.free_list.clone(),
            phantom_data: self.phantom_data,
        }
    }
}

impl<Data: Eq, Index> PartialEq for OptionStableVec<Data, Index> {
    fn eq(&self, other: &Self) -> bool {
        self.vec == other.vec
    }
}

impl<Data: Eq, Index> Eq for OptionStableVec<Data, Index> {}
