//! Internal byte storage that works with and without an allocator.

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Encoded CBOR bytes, either borrowed from the caller or owned when `alloc` is enabled.
///
/// With the `zeroize` feature, owned buffers are overwritten before their
/// allocation is released. Borrowed buffers remain the caller's responsibility.
#[derive(Clone, Debug)]
pub(crate) enum Bytes<'a> {
    Borrowed(&'a [u8]),
    #[cfg(feature = "alloc")]
    Owned(Vec<u8>),
}

#[cfg(all(feature = "alloc", feature = "zeroize"))]
impl Drop for Bytes<'_> {
    fn drop(&mut self) {
        if let Self::Owned(bytes) = self {
            use zeroize::Zeroize;
            bytes.zeroize();
        }
    }
}

impl<'a> Bytes<'a> {
    #[inline]
    pub(crate) const fn borrowed(bytes: &'a [u8]) -> Self {
        Self::Borrowed(bytes)
    }

    #[inline]
    pub(crate) fn as_slice(&self) -> &[u8] {
        match self {
            Self::Borrowed(bytes) => bytes,
            #[cfg(feature = "alloc")]
            Self::Owned(bytes) => bytes,
        }
    }
}

#[cfg(feature = "alloc")]
impl Bytes<'static> {
    #[inline]
    pub(crate) fn owned(bytes: Vec<u8>) -> Self {
        Self::Owned(bytes)
    }
}
