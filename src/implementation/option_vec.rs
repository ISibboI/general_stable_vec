//! A stable vector based on the [`Option`] type.
//!
//! Each element is stored as an `Option`, and a free list is used to keep track of "holes" in the vector.
//! This allows amortised O(1) insertions and deletions, with a memory usage of O(|maximum len|).

use std::{fmt::Debug, iter, marker::PhantomData, vec};

use crate::{
    error::Error,
    interface::{StableVec, StableVecAccess, StableVecIndex},
};

pub use available_insertion_index_iterator::AvailableInsertionIndexIterator;

mod available_insertion_index_iterator;

/// A stable vector based on the [`Option`] type with a free list.
///
/// Each element is stored as an `Option`, and a free list is used to keep track of "holes" in the vector.
/// This allows amortised O(1) insertions and deletions, with a memory usage of O(|maximum len|).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OptionStableVec<Index, Data> {
    vec: Vec<Option<Data>>,
    free_list: Vec<usize>,
    phantom_data: PhantomData<Index>,
}

impl<Index, Data> OptionStableVec<Index, Data> {
    /// Create a new empty [`OptionStableVec`].
    pub fn new() -> Self {
        Self {
            vec: Default::default(),
            free_list: Default::default(),
            phantom_data: Default::default(),
        }
    }
}

impl<Index: StableVecIndex, Data> StableVec<Index, Data> for OptionStableVec<Index, Data> {
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

    fn insert_at(&mut self, index: Index, element: Data) -> crate::error::Result<()> {
        let expected_index = self.free_list.last().copied().unwrap_or(self.vec.len());
        let index = index.into();
        if expected_index == index {
            let inserted_index = self.insert(element);
            assert_eq!(inserted_index.into(), index);
            Ok(())
        } else {
            Err(Error::NotTheNextAvailableInsertionIndex {
                expected_index,
                actual_index: index,
            })
        }
    }

    fn insert_at_arbitrary_index(
        &mut self,
        index: Index,
        element: Data,
    ) -> crate::error::Result<()> {
        let index = index.into();
        if index >= self.vec.len() {
            self.free_list.extend(self.vec.len()..index);
            self.vec.resize_with(index + 1, || None);
            self.vec[index] = Some(element);
            Ok(())
        } else if self.vec[index].is_some() {
            Err(Error::IndexAlreadyInUse { index })
        } else {
            self.vec[index] = Some(element);
            self.free_list.retain(|&free_index| free_index != index);
            Ok(())
        }
    }

    fn remove(&mut self, index: Index) -> crate::error::Result<Data> {
        let index = index.into();
        if index < self.vec.len() {
            let element = Option::take(self.vec.get_mut(index).unwrap())
                .ok_or(Error::UnmappedIndex { index })?;
            self.free_list.push(index);
            Ok(element)
        } else {
            Err(Error::UnmappedIndex { index })
        }
    }

    fn available_insertion_index_iterator<'result>(&self) -> impl 'result + Iterator<Item = Index>
    where
        Index: 'result,
    {
        AvailableInsertionIndexIterator::new(self.free_list.clone(), self.vec.len())
    }

    fn iter<'this>(&'this self) -> impl 'this + Iterator<Item = (Index, &'this Data)>
    where
        Data: 'this,
    {
        self.vec
            .iter()
            .enumerate()
            .filter_map(|(index, element)| element.as_ref().map(|element| (index.into(), element)))
    }

    fn iter_mut<'this>(&'this mut self) -> impl 'this + Iterator<Item = (Index, &'this mut Data)>
    where
        Data: 'this,
    {
        self.vec
            .iter_mut()
            .enumerate()
            .filter_map(|(index, element)| element.as_mut().map(|element| (index.into(), element)))
    }

    fn retain(&mut self, mut f: impl FnMut(&Data) -> bool) {
        for index in 0..self.vec.len() {
            if let Some(element) = self.vec[index].as_ref() {
                if !f(element) {
                    self.remove(index.into()).unwrap();
                }
            }
        }
    }

    fn clear(&mut self) {
        self.vec.clear();
        self.free_list.clear();
    }
}

impl<Index: StableVecIndex, Data> StableVecAccess<Index, Data> for OptionStableVec<Index, Data> {
    fn get(&self, index: Index) -> crate::error::Result<&Data> {
        let index = index.into();
        match self.vec.get(index) {
            Some(Some(element)) => Ok(element),
            _ => Err(Error::UnmappedIndex { index }),
        }
    }

    fn get_mut(&mut self, index: Index) -> crate::error::Result<&mut Data> {
        let index = index.into();
        match self.vec.get_mut(index) {
            Some(Some(element)) => Ok(element),
            _ => Err(Error::UnmappedIndex { index }),
        }
    }

    fn len(&self) -> usize {
        self.vec.len() - self.free_list.len()
    }
}

impl<Index, Data> Default for OptionStableVec<Index, Data> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Data: Clone, Index> Clone for OptionStableVec<Index, Data> {
    fn clone(&self) -> Self {
        Self {
            vec: self.vec.clone(),
            free_list: self.free_list.clone(),
            phantom_data: self.phantom_data,
        }
    }
}

impl<Data: Eq, Index> PartialEq for OptionStableVec<Index, Data> {
    fn eq(&self, other: &Self) -> bool {
        self.vec == other.vec
    }
}

impl<Data: Eq, Index> Eq for OptionStableVec<Index, Data> {}

impl<Index, Data> From<Vec<Data>> for OptionStableVec<Index, Data> {
    fn from(value: Vec<Data>) -> Self {
        value.into_iter().collect()
    }
}

impl<Index, Data> IntoIterator for OptionStableVec<Index, Data> {
    type Item = Data;
    type IntoIter = iter::Flatten<vec::IntoIter<Option<Data>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter().flatten()
    }
}

impl<Index, Data> FromIterator<Data> for OptionStableVec<Index, Data> {
    fn from_iter<T: IntoIterator<Item = Data>>(iter: T) -> Self {
        Self {
            vec: iter.into_iter().map(Some).collect(),
            free_list: Default::default(),
            phantom_data: Default::default(),
        }
    }
}

impl<Data: Debug, Index: StableVecIndex> Debug for OptionStableVec<Index, Data> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OptionStableVec [")?;

        let mut once = false;
        for (index, element) in self.vec.iter().enumerate() {
            let Some(element) = element else { continue };
            if once {
                write!(f, ", ")?;
            } else {
                once = true;
            }
            write!(f, "({index}, {element:?})")?;
        }

        write!(f, "]")
    }
}
