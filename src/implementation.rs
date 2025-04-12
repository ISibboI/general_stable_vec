//! Various implementations of stable vector types and index types.

pub mod marked_index;
pub mod option_vec;
pub mod usize_index;

/// Creates an impl for `From<usize>`, `Into<usize>` as well as [`StableVecIndex`](crate::interface::StableVecIndex) for the given type.
///
/// The type is expected to be a tuple struct with a single member that implements `TryFrom<usize>` and `TryInto<usize>`.
///
/// # Example
///
/// ```rust
/// struct Index(u64);
/// general_stable_vec::derive_stable_vec_index!(Index);
/// ```
#[macro_export]
macro_rules! derive_stable_vec_index {
    ($name:ty) => {
        impl From<usize> for $name {
            fn from(value: usize) -> Self {
                Self(value.try_into().unwrap())
            }
        }

        impl From<$name> for usize {
            fn from(value: $name) -> Self {
                value.0.try_into().unwrap()
            }
        }

        impl $crate::interface::StableVecIndex for $name {}
    };
}
