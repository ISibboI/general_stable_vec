use std::marker::PhantomData;

/// The iterator over the available insertion indices of an [`OptionStableVec`](super::OptionStableVec).
///
/// **WARNING:** This iterator is lifetime-independent of its underlying vector,
/// but quietly becomes invalid if the underlying vec is changed.
pub struct AvailableInsertionIndexIterator<Index> {
    free_list: Vec<usize>,
    next_index: usize,
    index: PhantomData<Index>,
}

impl<Index> AvailableInsertionIndexIterator<Index> {
    pub(crate) fn new(free_list: Vec<usize>, next_index: usize) -> Self {
        Self {
            free_list,
            next_index,
            index: Default::default(),
        }
    }
}

impl<Index: From<usize>> Iterator for AvailableInsertionIndexIterator<Index> {
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        let next_index = self.free_list.pop().unwrap_or_else(|| {
            let next_index = self.next_index;
            self.next_index = self.next_index.checked_add(1).unwrap();
            next_index
        });

        Some(next_index.into())
    }
}
