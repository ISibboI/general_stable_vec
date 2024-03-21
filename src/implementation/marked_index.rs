//! An index type that is marked by a type.
//! This prevents to accidentally use a wrong value as the index for a stable vector.

use std::marker::PhantomData;

use crate::interface::StableVecIndex;

/// An index type that is marked by a type `Marker`.
/// This prevents to accidentally use a wrong value as the index for a stable vector.
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
