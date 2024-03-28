//! An index type that is marked by a type.
//! This prevents to accidentally use a wrong value as the index for a stable vector.

use std::marker::PhantomData;

use crate::interface::StableVecIndex;

/// An index type that is marked by a type `Marker`.
/// This prevents to accidentally use a wrong value as the index for a stable vector.
#[derive(Debug)]
pub struct MarkedIndex<Marker> {
    index: usize,
    marker: PhantomData<Marker>,
}

impl<Marker> StableVecIndex for MarkedIndex<Marker> {}

impl<Marker> From<usize> for MarkedIndex<Marker> {
    fn from(index: usize) -> Self {
        Self {
            index,
            marker: Default::default(),
        }
    }
}

impl<Marker> From<MarkedIndex<Marker>> for usize {
    fn from(value: MarkedIndex<Marker>) -> Self {
        value.index
    }
}

impl<Marker> Clone for MarkedIndex<Marker> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Marker> Copy for MarkedIndex<Marker> {}

impl<Marker> PartialEq for MarkedIndex<Marker> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<Marker> Eq for MarkedIndex<Marker> {}

impl<Marker> PartialOrd for MarkedIndex<Marker> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<Marker> Ord for MarkedIndex<Marker> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl<Marker> core::hash::Hash for MarkedIndex<Marker> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.marker.hash(state);
    }
}
