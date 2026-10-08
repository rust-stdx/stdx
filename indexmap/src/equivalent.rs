//! Key equivalence for hash table lookups.
//!
//! This module is a vendored, dependency-free version of the `equivalent`
//! crate (<https://github.com/indexmap-rs/equivalent>). It is kept internal so
//! that `indexmap` has no external dependency for its public [`Equivalent`]
//! trait, while preserving the exact public API of the original crate.

use core::borrow::Borrow;

/// Key equivalence trait.
///
/// This trait allows hash table lookup to be customized. It has one blanket
/// implementation that uses the regular solution with `Borrow` and `Eq`, just
/// like `HashMap` does, so that you can pass `&str` to lookup into a map with
/// `String` keys and so on.
///
/// # Contract
///
/// The implementor **must** hash like `K`, if it is hashable.
pub trait Equivalent<K: ?Sized> {
    /// Compare self to `key` and return `true` if they are equal.
    fn equivalent(&self, key: &K) -> bool;
}

impl<Q: ?Sized, K: ?Sized> Equivalent<K> for Q
where
    Q: Eq,
    K: Borrow<Q>,
{
    #[inline]
    fn equivalent(&self, key: &K) -> bool {
        PartialEq::eq(self, key.borrow())
    }
}
