extern crate alloc;

use alloc::{borrow::Cow, string::String, vec::Vec};
use core::str::FromStr;

/// A string that stores up to `N` bytes inline (on the stack) and spills onto
/// the heap once it grows past that inline capacity.
///
/// `SmallString` behaves like [`alloc::string::String`] for the operations it
/// exposes: it dereferences to `str`, so all string methods are available. The
/// only difference is *where* short strings are stored: values whose UTF-8
/// length fits in `N` bytes never touch the allocator.
///
/// # Examples
///
/// ```
/// use small_collections::SmallString;
///
/// let mut s: SmallString<8> = SmallString::new();
/// s.push_str("hello");
/// assert!(s.is_inline());
/// assert_eq!(&*s, "hello");
///
/// // Pushing past `N` bytes automatically spills onto the heap.
/// s.push_str(" world");
/// assert!(!s.is_inline());
/// assert_eq!(s.as_str(), "hello world");
/// ```
///
/// # Invariants
///
/// * The `Inline` variant never holds more than `N` bytes.
/// * A value only ever moves from `Inline` to `Heap` on growth; it moves back
///   to `Inline` only through [`SmallString::shrink_to_fit`] or
///   [`SmallString::shrink_to`]. [`SmallString::clear`] and
///   [`SmallString::truncate`] keep the current storage.
///
/// # Serialization
///
/// With the `serde` feature enabled, a `SmallString` serializes exactly like a
/// `str` (for example the JSON string `"hello"`), regardless of whether its
/// contents are stored inline or on the heap. This matches the flat sequence
/// representation used by [`SmallVec`](crate::SmallVec).
#[derive(Clone)]
pub enum SmallString<const N: usize> {
    /// Data stored inline, on the stack, with a fixed capacity of `N` bytes.
    Inline(heapless::String<N>),
    /// Data spilled onto the heap, growing as needed.
    Heap(alloc::string::String),
}

impl<const N: usize> SmallString<N> {
    /// Creates a new, empty `SmallString` using its inline storage.
    ///
    /// This never allocates.
    #[inline]
    pub fn new() -> Self {
        SmallString::Inline(heapless::String::new())
    }

    /// Returns the inline capacity, `N`.
    #[inline]
    pub const fn inline_size() -> usize {
        N
    }

    /// Copies the contents of an `&str` into a new [`SmallString`].
    ///
    /// The data is stored inline when its UTF-8 length is at most `N`,
    /// otherwise it is copied onto the heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use small_collections::SmallString;
    ///
    /// let inline: SmallString<8> = SmallString::from_str("hello");
    /// assert!(inline.is_inline());
    ///
    /// let heap: SmallString<2> = SmallString::from_str("hello");
    /// assert!(!heap.is_inline());
    /// ```
    // Kept as an infallible inherent constructor alongside the `FromStr`
    // impl; the name mirrors std's `String`-like constructors.
    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn from_str(s: &str) -> Self {
        let mut out = Self::new();
        out.push_str(s);
        out
    }

    /// Creates a [`SmallString`] from an already heap-allocated `String`.
    ///
    /// The existing allocation is preserved, so the result always uses the
    /// `Heap` variant, except for an empty `String` with no capacity, which
    /// returns an inline `SmallString` (matching
    /// [`SmallVec::from_vec`](crate::SmallVec::from_vec)). Use
    /// [`SmallString::from_str`] if you want a short string to move inline.
    #[inline]
    pub fn from_string(s: String) -> Self {
        if s.capacity() == 0 { Self::new() } else { Self::Heap(s) }
    }

    /// Moves the contents of a `SmallString` with a different inline capacity
    /// into this one.
    ///
    /// The bytes are stored inline when they fit in `N`, otherwise they are
    /// spilled onto the heap. Unlike [`SmallString::from_string`], this does
    /// not preserve an existing heap allocation when the contents would fit
    /// inline.
    #[inline]
    pub fn from_small_string<const M: usize>(input: SmallString<M>) -> Self {
        match input {
            SmallString::Heap(s) if s.len() <= N => SmallString::from_str(&s),
            SmallString::Heap(s) => SmallString::Heap(s),
            SmallString::Inline(s) => SmallString::from_str(s.as_str()),
        }
    }

    /// Converts a `Vec` of bytes to a [`SmallString`].
    ///
    /// The bytes are stored inline when they fit in `N`, otherwise they are
    /// moved onto the heap.
    ///
    /// # Errors
    ///
    /// Returns an error if the bytes are not valid UTF-8.
    #[inline]
    pub fn from_utf8(bytes: Vec<u8>) -> Result<Self, alloc::string::FromUtf8Error> {
        let s = String::from_utf8(bytes)?;
        Ok(if s.len() <= N {
            Self::from_str(&s)
        } else {
            Self::from_string(s)
        })
    }

    /// Converts a slice of bytes to a [`SmallString`].
    ///
    /// The bytes are stored inline when they fit in `N`, otherwise they are
    /// copied onto the heap.
    ///
    /// # Errors
    ///
    /// Returns an error if the bytes are not valid UTF-8.
    #[inline]
    pub fn from_utf8_slice(bytes: &[u8]) -> Result<Self, core::str::Utf8Error> {
        Ok(Self::from_str(core::str::from_utf8(bytes)?))
    }

    /// Converts a slice of bytes to a [`SmallString`], replacing invalid
    /// sequences with U+FFFD.
    ///
    /// Input that is already valid UTF-8 is copied directly, without
    /// allocating. When replacements are needed, the result is stored inline
    /// if it fits in `N` and on the heap otherwise.
    #[inline]
    pub fn from_utf8_lossy(bytes: &[u8]) -> Self {
        match String::from_utf8_lossy(bytes) {
            Cow::Borrowed(borrowed) => Self::from_str(borrowed),
            Cow::Owned(owned) => {
                if owned.len() <= N {
                    Self::from_str(&owned)
                } else {
                    Self::from_string(owned)
                }
            }
        }
    }

    /// Returns `true` if the string is currently storing data inline (on the
    /// stack).
    #[inline]
    pub fn is_inline(&self) -> bool {
        matches!(self, SmallString::Inline(_))
    }

    /// Returns the total capacity (in bytes) of the string.
    ///
    /// For an inline string this is always `N`; for a spilled string it is the
    /// current heap capacity.
    #[inline]
    pub fn capacity(&self) -> usize {
        match self {
            SmallString::Inline(_) => N,
            SmallString::Heap(str) => str.capacity(),
        }
    }

    /// Returns the length of this [`SmallString`] in bytes, not [`char`]s or
    /// graphemes.
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            SmallString::Inline(str) => str.len(),
            SmallString::Heap(str) => str.len(),
        }
    }

    /// Returns `true` if this [`SmallString`] has a length of zero.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Truncates this [`SmallString`], removing all contents.
    ///
    /// While this means the [`SmallString`] will have a length of zero, it
    /// does not touch its capacity or change its storage (inline stays inline,
    /// heap stays heap).
    #[inline]
    pub fn clear(&mut self) {
        match self {
            SmallString::Inline(str) => str.clear(),
            SmallString::Heap(str) => str.clear(),
        }
    }

    /// Extracts a string slice containing the entire `SmallString`.
    #[inline]
    pub fn as_str(&self) -> &str {
        // Leverage Deref
        self
    }

    /// Extracts a mutable string slice containing the entire `SmallString`.
    ///
    /// Mutating the slice must preserve valid UTF-8; methods such as
    /// [`str::make_ascii_uppercase`] do.
    #[inline]
    pub fn as_mut_str(&mut self) -> &mut str {
        // Leverage DerefMut
        self
    }

    /// Converts a [`SmallString`] into a byte slice.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }

    /// Returns a mutable byte slice over the contents.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the content of the slice remains valid UTF-8.
    /// If this invariant is violated, it is Undefined Behavior.
    #[inline]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { self.as_mut_str().as_bytes_mut() }
    }

    /// Removes the last character from the string buffer and returns it.
    /// Returns [`None`] if the string is empty.
    #[inline]
    pub fn pop(&mut self) -> Option<char> {
        match self {
            SmallString::Inline(str) => str.pop(),
            SmallString::Heap(str) => str.pop(),
        }
    }

    /// Shortens this string to the specified length.
    ///
    /// If `new_len` is greater than or equal to the current length, this does
    /// nothing. Keeps the current storage.
    ///
    /// # Panics
    ///
    /// Panics if `new_len` is less than the current length and does not lie on
    /// a [`char`] boundary.
    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        match self {
            SmallString::Inline(str) => str.truncate(new_len),
            SmallString::Heap(str) => str.truncate(new_len),
        }
    }

    /// Appends a string slice to the end of this string, spilling onto the
    /// heap if the inline capacity is exceeded.
    #[inline]
    pub fn push_str(&mut self, input: &str) {
        match self {
            SmallString::Heap(s) => s.push_str(input),
            SmallString::Inline(s) => {
                // `s.len() <= N`, so this subtraction cannot underflow.
                if input.len() <= N - s.len() {
                    // guaranteed to succeed
                    let _ = s.push_str(input);
                } else {
                    // we need to spill on the heap
                    let new_capacity = s.len().saturating_add(input.len()).max(N.saturating_mul(2));
                    let mut heap = String::with_capacity(new_capacity);
                    heap.push_str(s.as_str());
                    heap.push_str(input);
                    *self = SmallString::Heap(heap);
                }
            }
        }
    }

    /// Appends a character to the end of this string, spilling onto the heap
    /// if the inline capacity is exceeded.
    #[inline]
    pub fn push(&mut self, ch: char) {
        match self {
            SmallString::Heap(s) => s.push(ch),
            SmallString::Inline(s) => {
                let char_len = ch.len_utf8();
                // `s.len() <= N`, so this subtraction cannot underflow.
                if char_len <= N - s.len() {
                    // guaranteed to succeed
                    let _ = s.push(ch);
                } else {
                    // we need to spill on the heap
                    let new_capacity = s.len().saturating_add(char_len).max(N.saturating_mul(2));
                    let mut heap = String::with_capacity(new_capacity);
                    heap.push_str(s.as_str());
                    heap.push(ch);
                    *self = SmallString::Heap(heap);
                }
            }
        }
    }

    /// Reserves capacity for at least `additional` more bytes, spilling onto
    /// the heap if needed.
    ///
    /// After this call, [`capacity`](SmallString::capacity) is at least
    /// `len() + additional`.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize` or the allocator reports a
    /// failure.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        match self {
            SmallString::Heap(s) => s.reserve(additional),
            SmallString::Inline(s) => {
                if additional > N - s.len() {
                    // spill to heap
                    let new_capacity = s.len().saturating_add(additional).max(N.saturating_mul(2));
                    let mut heap = String::with_capacity(new_capacity);
                    heap.push_str(s);
                    *self = SmallString::Heap(heap);
                }
            }
        }
    }

    /// Reserves capacity for exactly `additional` more bytes, spilling onto the
    /// heap if needed.
    ///
    /// Prefer [`SmallString::reserve`] when `additional` is only an estimate.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize` or the allocator reports a
    /// failure.
    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        match self {
            SmallString::Heap(s) => s.reserve_exact(additional),
            SmallString::Inline(s) => {
                if additional > N - s.len() {
                    // spill to heap
                    let new_capacity = s.len().saturating_add(additional).max(N.saturating_add(1));
                    let mut heap = String::with_capacity(new_capacity);
                    heap.push_str(s);
                    *self = SmallString::Heap(heap);
                }
            }
        }
    }

    /// Shrinks the string as much as possible: an inline string is unchanged,
    /// while a spilled string whose contents fit in `N` bytes moves back
    /// inline.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        if let SmallString::Heap(s) = self {
            if s.len() <= N {
                *self = SmallString::from_str(s.as_str());
            } else {
                s.shrink_to_fit();
            }
        }
    }

    /// Shrinks the string's capacity to at least `min_capacity` bytes, moving
    /// it back inline when the result fits in `N`.
    ///
    /// Does nothing when the current capacity is already at or below
    /// `min_capacity`.
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        if let SmallString::Heap(s) = self
            && s.capacity() > min_capacity
        {
            let target = s.len().max(min_capacity);
            if target <= N {
                *self = SmallString::from_str(s.as_str());
            } else {
                s.shrink_to(target);
            }
        }
    }

    /// Converts this `SmallString` into an [`alloc::string::String`], reusing
    /// the heap allocation when spilled.
    ///
    /// # Examples
    ///
    /// ```
    /// use small_collections::SmallString;
    ///
    /// let s: SmallString<8> = SmallString::from_str("hello");
    /// assert_eq!(s.into_string(), "hello");
    /// ```
    #[inline]
    pub fn into_string(self) -> String {
        match self {
            SmallString::Inline(s) => String::from(s.as_str()),
            SmallString::Heap(s) => s,
        }
    }

    /// Converts this `SmallString` into a byte vector, reusing the heap
    /// allocation when spilled.
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.into_string().into_bytes()
    }
}

impl<const N: usize> From<&str> for SmallString<N> {
    #[inline]
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl<const N: usize> From<alloc::string::String> for SmallString<N> {
    #[inline]
    fn from(s: alloc::string::String) -> Self {
        Self::from_string(s)
    }
}

impl<const N: usize> From<SmallString<N>> for alloc::string::String {
    #[inline]
    fn from(this: SmallString<N>) -> Self {
        this.into_string()
    }
}

impl<const N: usize> core::ops::Deref for SmallString<N> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            SmallString::Inline(str) => str.as_str(),
            SmallString::Heap(str) => str.as_str(),
        }
    }
}

impl<const N: usize> core::ops::DerefMut for SmallString<N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            SmallString::Inline(str) => str.as_mut_str(),
            SmallString::Heap(str) => str.as_mut_str(),
        }
    }
}

impl<const N: usize> Default for SmallString<N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> core::fmt::Display for SmallString<N> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self.as_str(), f) // Delegate to str implementation
    }
}

impl<const N: usize> core::fmt::Debug for SmallString<N> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Delegate to `str`'s `Debug` so the output is a quoted string,
        // regardless of the underlying storage.
        core::fmt::Debug::fmt(self.as_str(), f)
    }
}

impl<const N: usize> core::fmt::Write for SmallString<N> {
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.push_str(s);
        Ok(())
    }
}

impl<const N: usize, const M: usize> PartialEq<SmallString<M>> for SmallString<N> {
    #[inline]
    fn eq(&self, other: &SmallString<M>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<const N: usize> Eq for SmallString<N> {}

impl<const N: usize> PartialEq<str> for SmallString<N> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<'a, const N: usize> PartialEq<&'a str> for SmallString<N> {
    #[inline]
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}

impl<const N: usize> PartialEq<SmallString<N>> for &str {
    #[inline]
    fn eq(&self, other: &SmallString<N>) -> bool {
        *self == other.as_str()
    }
}

impl<const N: usize> PartialEq<alloc::string::String> for SmallString<N> {
    #[inline]
    fn eq(&self, other: &alloc::string::String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<const N: usize> PartialEq<SmallString<N>> for alloc::string::String {
    #[inline]
    fn eq(&self, other: &SmallString<N>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<const N: usize> PartialEq<SmallString<N>> for str {
    #[inline]
    fn eq(&self, other: &SmallString<N>) -> bool {
        self == other.as_str()
    }
}

impl<const N: usize, const M: usize> PartialOrd<SmallString<M>> for SmallString<N> {
    #[inline]
    fn partial_cmp(&self, other: &SmallString<M>) -> Option<core::cmp::Ordering> {
        Some(self.as_str().cmp(other.as_str()))
    }
}

impl<const N: usize> Ord for SmallString<N> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl<const N: usize> core::hash::Hash for SmallString<N> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl<const N: usize> core::borrow::Borrow<str> for SmallString<N> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<const N: usize> core::borrow::BorrowMut<str> for SmallString<N> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl<const N: usize> AsRef<str> for SmallString<N> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const N: usize> AsRef<[u8]> for SmallString<N> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<const N: usize> AsMut<str> for SmallString<N> {
    #[inline]
    fn as_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl<const N: usize> FromIterator<char> for SmallString<N> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let mut s = Self::new();
        let iter = iter.into_iter();
        s.reserve(iter.size_hint().0);
        for c in iter {
            s.push(c);
        }
        s
    }
}

impl<'a, const N: usize> FromIterator<&'a str> for SmallString<N> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let mut s = Self::new();
        let iter = iter.into_iter();
        s.reserve(iter.size_hint().0);
        for str_slice in iter {
            s.push_str(str_slice);
        }
        s
    }
}

impl<const N: usize> Extend<char> for SmallString<N> {
    #[inline]
    fn extend<I: IntoIterator<Item = char>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve(iter.size_hint().0);
        for c in iter {
            self.push(c);
        }
    }
}

impl<'a, const N: usize> Extend<&'a str> for SmallString<N> {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a str>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve(iter.size_hint().0);
        for str_slice in iter {
            self.push_str(str_slice);
        }
    }
}

impl<const N: usize> FromStr for SmallString<N> {
    type Err = core::convert::Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = Self::new();
        out.push_str(s);
        Ok(out)
    }
}

#[cfg(feature = "serde")]
impl<const N: usize> serde::Serialize for SmallString<N> {
    #[inline]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> serde::Deserialize<'de> for SmallString<N> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SmallStringVisitor<const N: usize>;

        impl<'de, const N: usize> serde::de::Visitor<'de> for SmallStringVisitor<N> {
            type Value = SmallString<N>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(SmallString::from_str(v))
            }

            fn visit_borrowed_str<E: serde::de::Error>(self, v: &'de str) -> Result<Self::Value, E> {
                Ok(SmallString::from_str(v))
            }

            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(if v.len() <= N {
                    SmallString::from_str(&v)
                } else {
                    SmallString::from_string(v)
                })
            }
        }

        deserializer.deserialize_str(SmallStringVisitor::<N>)
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::Write;

    use crate::SmallString;

    // -----------------------------------------------------------------------
    // Existing baseline tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_basic_inline() {
        let mut s: SmallString<16> = SmallString::new();
        assert!(s.is_inline());
        assert!(s.is_empty());
        assert!(s.capacity() == 16);

        s.push_str("Hello,");
        assert_eq!(s.len(), 6);
        assert_eq!(s.as_str(), "Hello,");
        assert!(s.is_inline());
        assert!(s.capacity() == 16);

        s.push(' ');
        s.push_str("World");
        assert_eq!(s.len(), 12);
        assert_eq!(&*s, "Hello, World");
        assert!(s.is_inline());
        assert!(s.capacity() == 16);
    }

    #[test]
    fn test_basic_spill_to_heap() {
        let mut s: SmallString<16> = SmallString::new();
        assert!(s.is_inline());
        assert!(s.is_empty());
        assert!(s.capacity() == 16);

        s.push_str("Hello, ");
        assert_eq!(s.len(), 7);
        assert_eq!(s.as_str(), "Hello, ");
        assert!(s.is_inline());
        assert!(s.capacity() == 16);

        s.push_str(&"a".repeat(30));
        assert_eq!(s.len(), 37);
        assert_eq!(&s, &format!("Hello, {}", "a".repeat(30)));
        assert!(!s.is_inline());
        assert!(s.capacity() >= 37);
    }

    // -----------------------------------------------------------------------
    // Construction methods
    // -----------------------------------------------------------------------

    #[test]
    fn test_new_is_empty_and_default() {
        let s: SmallString<8> = SmallString::new();
        assert!(s.is_empty());
        assert!(s.is_inline());
        let s2: SmallString<8> = Default::default();
        assert!(s2.is_empty());
        assert!(s2.is_inline());
    }

    #[test]
    fn test_from_str_inline() {
        let s: SmallString<32> = SmallString::from_str("hello");
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_from_str_spill() {
        let s: SmallString<4> = SmallString::from_str("hello world");
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), "hello world");
    }

    #[test]
    fn test_from_str_exact_capacity() {
        let s: SmallString<5> = SmallString::from_str("hello");
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_from_string_heap_preserved() {
        let heap = String::from("hello world this is a long string");
        let s: SmallString<4> = SmallString::from_string(heap.clone());
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), heap);
    }

    #[test]
    fn test_from_string_small() {
        let heap = String::from("hi");
        let s: SmallString<32> = SmallString::from_string(heap);
        // from_string always stores in Heap variant
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), "hi");
    }

    #[test]
    fn test_from_utf8_valid() {
        let s: SmallString<16> = SmallString::from_utf8(b"hello".to_vec()).unwrap();
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_from_utf8_invalid() {
        let result: Result<SmallString<16>, _> = SmallString::from_utf8(vec![0xFF, 0xFE]);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_utf8_slice_valid() {
        let s: SmallString<16> = SmallString::from_utf8_slice(b"hello").unwrap();
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_from_utf8_slice_invalid() {
        let result: Result<SmallString<16>, _> = SmallString::from_utf8_slice(&[0xFF, 0xFE]);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_utf8_lossy_valid() {
        let s: SmallString<16> = SmallString::from_utf8_lossy(b"hello");
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_from_utf8_lossy_invalid() {
        let s: SmallString<16> = SmallString::from_utf8_lossy(&[0xFF, 0xFE]);
        // Replacement character(s)
        assert_eq!(s.as_str(), "\u{FFFD}\u{FFFD}");
    }

    #[test]
    fn test_from_utf8_lossy_spill() {
        let bytes = b"hello world this is long";
        let s: SmallString<4> = SmallString::from_utf8_lossy(bytes);
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), "hello world this is long");
    }

    // -----------------------------------------------------------------------
    // From trait impls
    // -----------------------------------------------------------------------

    #[test]
    fn test_from_str_trait_inline() {
        let s: SmallString<16> = SmallString::from("hi");
        assert!(s.is_inline());
    }

    #[test]
    fn test_from_str_trait_spill() {
        let s: SmallString<4> = SmallString::from("long string");
        assert!(!s.is_inline());
    }

    #[test]
    fn test_from_string_trait() {
        let s: SmallString<32> = SmallString::from(String::from("hi"));
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), "hi");
    }

    // -----------------------------------------------------------------------
    // Inspection methods
    // -----------------------------------------------------------------------

    #[test]
    fn test_capacity_inline() {
        let s: SmallString<64> = SmallString::from_str("hello");
        assert_eq!(s.capacity(), 64);
    }

    #[test]
    fn test_capacity_heap() {
        let s: SmallString<4> = SmallString::from_str("hello world, this is a test!");
        assert!(!s.is_inline());
        assert!(s.capacity() >= s.len());
    }

    #[test]
    fn test_len() {
        let s: SmallString<32> = SmallString::from_str("héllo");
        assert_eq!(s.len(), 6);
    }

    // -----------------------------------------------------------------------
    // Mutation methods
    // -----------------------------------------------------------------------

    #[test]
    fn test_clear_inline() {
        let mut s: SmallString<16> = SmallString::from_str("hello");
        assert!(s.is_inline());
        s.clear();
        assert!(s.is_empty());
        assert!(s.is_inline());
        assert_eq!(s.capacity(), 16);
    }

    #[test]
    fn test_clear_heap() {
        let mut s: SmallString<4> = SmallString::from_str("long string content");
        assert!(!s.is_inline());
        s.clear();
        assert!(s.is_empty());
        assert!(!s.is_inline());
    }

    #[test]
    fn test_clear_and_reuse_inline() {
        let mut s: SmallString<16> = SmallString::from_str("hello");
        s.clear();
        s.push_str("world");
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "world");
    }

    #[test]
    fn test_clear_and_reuse_heap() {
        let mut s: SmallString<4> = SmallString::from_str("long string content");
        s.clear();
        s.push_str("abc");
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), "abc");
    }

    #[test]
    fn test_push_char_inline() {
        let mut s: SmallString<16> = SmallString::new();
        s.push('a');
        s.push('b');
        s.push('c');
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "abc");
    }

    #[test]
    fn test_push_char_spill() {
        let mut s: SmallString<4> = SmallString::from_str("abc");
        assert!(s.is_inline());
        s.push('d');
        // exactly at capacity, still inline
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "abcd");

        s.push('e');
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), "abcde");
    }

    #[test]
    fn test_push_multibyte_char_stays_inline() {
        let mut s: SmallString<8> = SmallString::new();
        s.push('€');
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "€");
    }

    #[test]
    fn test_push_multibyte_char_spill() {
        let mut s: SmallString<4> = SmallString::from_str("a");
        // '🦀' is 4 bytes, 'a' is 1 byte, so this spills
        s.push('🦀');
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), "a🦀");
    }

    #[test]
    fn test_push_str_spill_with_char() {
        let mut s: SmallString<4> = SmallString::from_str("a");
        s.push_str("bcd");
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "abcd");

        s.push_str("e");
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), "abcde");
    }

    #[test]
    fn test_push_str_exact_boundary() {
        let mut s: SmallString<5> = SmallString::from_str("hello");
        assert!(s.is_inline());
        // push empty str on full buffer
        s.push_str("");
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_pop_inline() {
        let mut s: SmallString<16> = SmallString::from_str("hello");
        assert_eq!(s.pop(), Some('o'));
        assert_eq!(s.pop(), Some('l'));
        assert_eq!(s.as_str(), "hel");
    }

    #[test]
    fn test_pop_heap() {
        let mut s: SmallString<4> = SmallString::from_str("hello!!!");
        assert!(!s.is_inline());
        assert_eq!(s.pop(), Some('!'));
        assert_eq!(s.pop(), Some('!'));
        assert_eq!(s.as_str(), "hello!");
    }

    #[test]
    fn test_pop_empty() {
        let mut s: SmallString<16> = SmallString::new();
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_pop_empty_after_clear() {
        let mut s: SmallString<16> = SmallString::from_str("a");
        s.clear();
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_pop_multibyte() {
        let mut s: SmallString<16> = SmallString::from_str("a🦀b");
        assert_eq!(s.pop(), Some('b'));
        assert_eq!(s.pop(), Some('🦀'));
        assert_eq!(s.as_str(), "a");
    }

    #[test]
    fn test_truncate_inline() {
        let mut s: SmallString<16> = SmallString::from_str("hello world");
        s.truncate(5);
        assert_eq!(s.as_str(), "hello");
        assert!(s.is_inline());
    }

    #[test]
    fn test_truncate_heap() {
        let mut s: SmallString<4> = SmallString::from_str("hello world");
        assert!(!s.is_inline());
        s.truncate(5);
        assert_eq!(s.as_str(), "hello");
        assert!(!s.is_inline());
    }

    #[test]
    fn test_truncate_zero() {
        let mut s: SmallString<16> = SmallString::from_str("hello");
        s.truncate(0);
        assert!(s.is_empty());
    }

    #[test]
    fn test_truncate_past_len() {
        let mut s: SmallString<16> = SmallString::from_str("hi");
        s.truncate(100);
        assert_eq!(s.as_str(), "hi");
    }

    #[test]
    fn test_reserve_no_spill() {
        let mut s: SmallString<16> = SmallString::from_str("hi");
        s.reserve(4);
        assert!(s.is_inline());
    }

    #[test]
    fn test_reserve_triggers_spill() {
        let mut s: SmallString<8> = SmallString::from_str("hi");
        s.reserve(10);
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), "hi");
    }

    #[test]
    fn test_reserve_on_heap() {
        let mut s: SmallString<4> = SmallString::from_str("hello world");
        assert!(!s.is_inline());
        s.reserve(50);
        // `String::reserve` guarantees room for `len + additional` bytes,
        // regardless of the pre-existing capacity.
        assert!(s.capacity() >= s.len() + 50);
        assert_eq!(s.as_str(), "hello world");
    }

    // -----------------------------------------------------------------------
    // Access methods
    // -----------------------------------------------------------------------

    #[test]
    fn test_as_str() {
        let s: SmallString<16> = SmallString::from_str("hello");
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_as_mut_str() {
        let mut s: SmallString<16> = SmallString::from_str("hello");
        let ms = s.as_mut_str();
        ms.make_ascii_uppercase();
        assert_eq!(s.as_str(), "HELLO");
    }

    #[test]
    fn test_as_bytes() {
        let s: SmallString<16> = SmallString::from_str("hello");
        assert_eq!(s.as_bytes(), b"hello");
    }

    #[test]
    fn test_as_bytes_mut() {
        let mut s: SmallString<16> = SmallString::from_str("hello");
        let bytes = unsafe { s.as_bytes_mut() };
        bytes[0] = b'H';
        assert_eq!(s.as_str(), "Hello");
    }

    #[test]
    fn test_as_bytes_heap() {
        let s: SmallString<4> = SmallString::from_str("hello world");
        assert!(!s.is_inline());
        assert_eq!(s.as_bytes(), b"hello world");
    }

    // -----------------------------------------------------------------------
    // Deref and DerefMut
    // -----------------------------------------------------------------------

    #[test]
    fn test_deref_inline() {
        let s: SmallString<16> = SmallString::from_str("hello");
        let r: &str = &s;
        assert_eq!(r, "hello");
    }

    #[test]
    fn test_deref_heap() {
        let s: SmallString<4> = SmallString::from_str("long string");
        let r: &str = &s;
        assert_eq!(r, "long string");
    }

    #[test]
    fn test_deref_mut() {
        let mut s: SmallString<16> = SmallString::from_str("hello");
        let r: &mut str = &mut s;
        r.make_ascii_uppercase();
        assert_eq!(s.as_str(), "HELLO");
    }

    // -----------------------------------------------------------------------
    // Display, Debug, fmt::Write
    // -----------------------------------------------------------------------

    #[test]
    fn test_display() {
        let s: SmallString<16> = SmallString::from_str("hello");
        assert_eq!(format!("{}", s), "hello");
    }

    #[test]
    fn test_display_heap() {
        let s: SmallString<4> = SmallString::from_str("hello world");
        assert_eq!(format!("{}", s), "hello world");
    }

    #[test]
    fn test_debug_inline() {
        let s: SmallString<16> = SmallString::from_str("hello");
        assert_eq!(format!("{s:?}"), "\"hello\"");
    }

    #[test]
    fn test_debug_heap() {
        let s: SmallString<4> = SmallString::from_str("hello world");
        assert_eq!(format!("{s:?}"), "\"hello world\"");
    }

    #[test]
    fn test_fmt_write() {
        let mut s: SmallString<16> = SmallString::new();
        write!(&mut s, "hello world {}", 42).unwrap();
        assert_eq!(s.as_str(), "hello world 42");
        assert!(s.is_inline());
    }

    #[test]
    fn test_fmt_write_spill() {
        let mut s: SmallString<4> = SmallString::new();
        write!(&mut s, "hello world").unwrap();
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), "hello world");
    }

    // -----------------------------------------------------------------------
    // Equality
    // -----------------------------------------------------------------------

    #[test]
    fn test_eq_inline_inline() {
        let a: SmallString<16> = SmallString::from_str("hello");
        let b: SmallString<16> = SmallString::from_str("hello");
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_inline_heap() {
        let a: SmallString<4> = SmallString::from_str("hello");
        let b: SmallString<16> = SmallString::from_str("hello");
        assert!(!a.is_inline());
        assert!(b.is_inline());
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_cross_n() {
        let a: SmallString<8> = SmallString::from_str("test");
        let b: SmallString<32> = SmallString::from_str("test");
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_inequality() {
        let a: SmallString<16> = SmallString::from_str("abc");
        let b: SmallString<16> = SmallString::from_str("xyz");
        assert_ne!(a, b);
    }

    #[test]
    fn test_partial_eq_str() {
        let s: SmallString<16> = SmallString::from_str("hello");
        assert_eq!(s, *"hello");
    }

    #[test]
    fn test_partial_eq_ref_str() {
        let s: SmallString<16> = SmallString::from_str("hello");
        let r: &str = "hello";
        assert_eq!(s, r);
    }

    #[test]
    fn test_partial_eq_str_ref_left() {
        let s: SmallString<16> = SmallString::from_str("hello");
        assert_eq!("hello", s);
    }

    #[test]
    fn test_partial_eq_string() {
        let s: SmallString<16> = SmallString::from_str("hello");
        let heap = String::from("hello");
        assert_eq!(s, heap);
    }

    // -----------------------------------------------------------------------
    // Ordering
    // -----------------------------------------------------------------------

    #[test]
    fn test_ord() {
        let a: SmallString<16> = SmallString::from_str("abc");
        let b: SmallString<16> = SmallString::from_str("xyz");
        assert!(a < b);
        assert!(b > a);
    }

    #[test]
    fn test_ord_cross_n() {
        let a: SmallString<4> = SmallString::from_str("abc");
        let b: SmallString<32> = SmallString::from_str("xyz");
        assert!(a < b);
    }

    #[test]
    fn test_ord_equal() {
        let a: SmallString<16> = SmallString::from_str("same");
        let b: SmallString<16> = SmallString::from_str("same");
        assert_eq!(a.cmp(&b), core::cmp::Ordering::Equal);
    }

    #[test]
    fn test_partial_ord_cross_m() {
        let a: SmallString<8> = SmallString::from_str("a");
        let b: SmallString<32> = SmallString::from_str("b");
        assert!(a < b);
    }

    // -----------------------------------------------------------------------
    // Hash
    // -----------------------------------------------------------------------

    #[test]
    fn test_hash_inline() {
        use std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
        };

        let s: SmallString<16> = SmallString::from_str("hello");
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        let h1 = hasher.finish();

        let mut hasher = DefaultHasher::new();
        "hello".hash(&mut hasher);
        let h2 = hasher.finish();
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_inline_and_heap_same() {
        use std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
        };

        let inline: SmallString<32> = SmallString::from_str("hello world");
        let heap: SmallString<4> = SmallString::from_str("hello world");

        let mut h1 = DefaultHasher::new();
        inline.hash(&mut h1);
        let mut h2 = DefaultHasher::new();
        heap.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_hash_in_hashmap() {
        use std::collections::HashSet;

        let a: SmallString<16> = SmallString::from_str("key1");
        let b: SmallString<16> = SmallString::from_str("key1");

        let mut set = HashSet::new();
        set.insert(a);
        assert!(set.contains(&b));
    }

    // -----------------------------------------------------------------------
    // Borrow / BorrowMut / AsRef
    // -----------------------------------------------------------------------

    #[test]
    fn test_borrow_str() {
        use std::borrow::Borrow;
        let s: SmallString<16> = SmallString::from_str("hello");
        let r: &str = s.borrow();
        assert_eq!(r, "hello");
    }

    #[test]
    fn test_borrow_mut_str() {
        use std::borrow::BorrowMut;
        let mut s: SmallString<16> = SmallString::from_str("hello");
        let r: &mut str = s.borrow_mut();
        r.make_ascii_uppercase();
        assert_eq!(s.as_str(), "HELLO");
    }

    #[test]
    fn test_as_ref_str() {
        let s: SmallString<16> = SmallString::from_str("hello");
        let r: &str = s.as_ref();
        assert_eq!(r, "hello");
    }

    #[test]
    fn test_as_ref_bytes() {
        let s: SmallString<16> = SmallString::from_str("hello");
        let r: &[u8] = s.as_ref();
        assert_eq!(r, b"hello");
    }

    // -----------------------------------------------------------------------
    // Clone
    // -----------------------------------------------------------------------

    #[test]
    fn test_clone_inline() {
        let s: SmallString<16> = SmallString::from_str("hello");
        let c = s.clone();
        assert!(c.is_inline());
        assert_eq!(c, s);
    }

    #[test]
    fn test_clone_heap() {
        let s: SmallString<4> = SmallString::from_str("hello world");
        assert!(!s.is_inline());
        let c = s.clone();
        assert!(!c.is_inline());
        assert_eq!(c, s);
    }

    #[test]
    fn test_clone_independent() {
        let s: SmallString<16> = SmallString::from_str("hello");
        let mut c = s.clone();
        c.push_str(" world");
        assert_eq!(s.as_str(), "hello");
        assert_eq!(c.as_str(), "hello world");
    }

    // -----------------------------------------------------------------------
    // FromIterator
    // -----------------------------------------------------------------------

    #[test]
    fn test_from_iter_chars_empty() {
        let s: SmallString<16> = SmallString::from_iter("".chars());
        assert!(s.is_empty());
    }

    #[test]
    fn test_from_iter_chars_inline() {
        let s: SmallString<16> = SmallString::from_iter("hello".chars());
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_from_iter_chars_spill() {
        let s: SmallString<4> = SmallString::from_iter("hello world".chars());
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), "hello world");
    }

    #[test]
    fn test_from_iter_strs_empty() {
        let s: SmallString<16> = [""].iter().copied().collect::<SmallString<16>>();
        assert!(s.is_empty());
    }

    #[test]
    fn test_from_iter_strs_inline() {
        let parts = ["hello", " ", "world"];
        let s: SmallString<32> = parts.iter().copied().collect();
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "hello world");
    }

    #[test]
    fn test_from_iter_strs_spill() {
        let parts = ["hello", " ", "world", " ", "this is long"];
        let s: SmallString<4> = parts.iter().copied().collect();
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), "hello world this is long");
    }

    // -----------------------------------------------------------------------
    // Extend
    // -----------------------------------------------------------------------

    #[test]
    fn test_extend_chars_inline() {
        let mut s: SmallString<16> = SmallString::from_str("he");
        s.extend("llo".chars());
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_extend_chars_spill() {
        let mut s: SmallString<4> = SmallString::from_str("a");
        s.extend("bcdef".chars());
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), "abcdef");
    }

    #[test]
    fn test_extend_strs_inline() {
        let mut s: SmallString<16> = SmallString::from_str("hello");
        s.extend([" ", "world"]);
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "hello world");
    }

    #[test]
    fn test_extend_strs_spill() {
        let mut s: SmallString<4> = SmallString::from_str("a");
        s.extend(["bc", "def"]);
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), "abcdef");
    }

    // -----------------------------------------------------------------------
    // Edge cases
    // -----------------------------------------------------------------------

    #[test]
    fn test_zero_capacity() {
        let mut s: SmallString<0> = SmallString::new();
        assert!(s.is_inline());
        assert!(s.is_empty());
        assert_eq!(s.capacity(), 0);

        // Any push should spill immediately
        s.push_str("x");
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), "x");

        // Push char on zero capacity
        let mut s2: SmallString<0> = SmallString::new();
        s2.push('a');
        assert!(!s2.is_inline());
        assert_eq!(s2.as_str(), "a");
    }

    #[test]
    fn test_zero_capacity_from_str() {
        let s: SmallString<0> = SmallString::from_str("");
        assert!(s.is_inline());
        assert!(s.is_empty());

        let s2: SmallString<0> = SmallString::from_str("x");
        assert!(!s2.is_inline());
        assert_eq!(s2.as_str(), "x");
    }

    #[test]
    fn test_multibyte_char_boundary() {
        let mut s: SmallString<8> = SmallString::new();
        s.push_str("a🦀b"); // 'a'=1, '🦀'=4, 'b'=1 => total 6
        assert!(s.is_inline());
        assert_eq!(s.len(), 6);
        assert_eq!(s.as_str(), "a🦀b");

        // Ensure indexing / slicing works correctly
        let chars: Vec<char> = s.chars().collect();
        assert_eq!(chars, vec!['a', '🦀', 'b']);
    }

    #[test]
    fn test_deref_methods_available() {
        let s: SmallString<16> = SmallString::from_str("hello world");
        // Methods from str through Deref
        assert!(s.contains("world"));
        assert!(s.starts_with("hello"));
        assert_eq!(s.find('w'), Some(6));
        let words: Vec<&str> = s.split(' ').collect();
        assert_eq!(words, vec!["hello", "world"]);
    }

    #[test]
    fn test_roundtrip_format() {
        let s: SmallString<16> = SmallString::from_str("test");
        let formatted = format!("{}", s);
        let back: SmallString<16> = SmallString::from_str(&formatted);
        assert_eq!(s, back);
    }

    #[test]
    fn test_empty_str_operations() {
        let mut s: SmallString<16> = SmallString::from_str("");
        assert!(s.is_empty());
        s.push_str("");
        assert!(s.is_empty());
        s.push_str("a");
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn test_from_small_string_inline_to_inline() {
        let source: SmallString<16> = SmallString::from_str("hello");
        assert!(source.is_inline());

        let dest: SmallString<32> = SmallString::from_small_string(source);
        assert!(dest.is_inline());
        assert_eq!(dest.as_str(), "hello");
    }

    #[test]
    fn test_from_small_string_inline_to_spill() {
        let source: SmallString<16> = SmallString::from_str("hello world");
        assert!(source.is_inline());

        let dest: SmallString<4> = SmallString::from_small_string(source);
        assert!(!dest.is_inline());
        assert_eq!(dest.as_str(), "hello world");
    }

    #[test]
    fn test_from_small_string_heap_to_inline() {
        // Force heap by exceeding inline capacity
        let source: SmallString<4> = SmallString::from_str("hello world this is long");
        assert!(!source.is_inline());

        // Target has enough capacity to fit inline
        let dest: SmallString<64> = SmallString::from_small_string(source);
        assert!(dest.is_inline());
        assert_eq!(dest.as_str(), "hello world this is long");
    }

    #[test]
    fn test_from_small_string_heap_to_heap() {
        // Force heap by exceeding inline capacity
        let source: SmallString<16> = SmallString::from_str("hello world this is even longer string content");
        assert!(!source.is_inline());

        // Target too small to fit inline
        let dest: SmallString<4> = SmallString::from_small_string(source);
        assert!(!dest.is_inline());
        assert_eq!(dest.as_str(), "hello world this is even longer string content");
    }

    #[test]
    fn test_from_small_string_empty() {
        let source: SmallString<16> = SmallString::new();
        assert!(source.is_inline());
        assert!(source.is_empty());

        let dest: SmallString<8> = SmallString::from_small_string(source);
        assert!(dest.is_inline());
        assert!(dest.is_empty());
    }

    #[test]
    fn test_from_small_string_exact_capacity() {
        let source: SmallString<16> = SmallString::from_str("abcd");
        assert!(source.is_inline());

        let dest: SmallString<4> = SmallString::from_small_string(source);
        assert!(dest.is_inline());
        assert_eq!(dest.as_str(), "abcd");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_serializes_as_flat_string() {
        let inline: SmallString<16> = SmallString::from_str("hello");
        let heap: SmallString<2> = SmallString::from_str("hello");
        assert!(inline.is_inline());
        assert!(!heap.is_inline());

        // Both representations produce the same, plain JSON string.
        assert_eq!(serde_json::to_string(&inline).unwrap(), "\"hello\"");
        assert_eq!(serde_json::to_string(&heap).unwrap(), "\"hello\"");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_deserializes_into_inline_or_heap() {
        let inline: SmallString<16> = serde_json::from_str("\"hello\"").unwrap();
        assert!(inline.is_inline());
        assert_eq!(inline.as_str(), "hello");

        // The value doesn't fit inline, so it must land on the heap even
        // though the source JSON is identical.
        let heap: SmallString<2> = serde_json::from_str("\"hello\"").unwrap();
        assert!(!heap.is_inline());
        assert_eq!(heap.as_str(), "hello");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_roundtrip_both_variants() {
        let inline: SmallString<16> = SmallString::from_str("hello");
        let json = serde_json::to_string(&inline).unwrap();
        let back: SmallString<16> = serde_json::from_str(&json).unwrap();
        assert_eq!(back, inline);
        assert!(back.is_inline());

        let heap: SmallString<4> = SmallString::from_str("hello world");
        let json = serde_json::to_string(&heap).unwrap();
        let back: SmallString<4> = serde_json::from_str(&json).unwrap();
        assert_eq!(back, heap);
        assert!(!back.is_inline());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_rejects_non_string() {
        let result: Result<SmallString<16>, _> = serde_json::from_str("42");
        assert!(result.is_err());
    }

    // -----------------------------------------------------------------------
    // Inline placement of byte constructors
    // -----------------------------------------------------------------------

    #[test]
    fn test_from_utf8_inline_when_fits() {
        let s: SmallString<16> = SmallString::from_utf8(b"hello".to_vec()).unwrap();
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_from_utf8_spills_when_too_long() {
        let s: SmallString<2> = SmallString::from_utf8(b"hello".to_vec()).unwrap();
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_from_utf8_empty_is_inline() {
        let s: SmallString<4> = SmallString::from_utf8(Vec::new()).unwrap();
        assert!(s.is_inline());
        assert!(s.is_empty());
    }

    #[test]
    fn test_from_utf8_lossy_owned_fits_inline() {
        // Invalid input forces the owned `Cow` path.
        let s: SmallString<16> = SmallString::from_utf8_lossy(&[0xFF, 0xFE]);
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "\u{FFFD}\u{FFFD}");
    }

    // -----------------------------------------------------------------------
    // Construction / conversion additions
    // -----------------------------------------------------------------------

    #[test]
    fn test_inline_size() {
        assert_eq!(<SmallString<8>>::inline_size(), 8);
        assert_eq!(<SmallString<0>>::inline_size(), 0);
    }

    #[test]
    fn test_from_string_empty_has_no_allocation() {
        let s: SmallString<8> = SmallString::from_string(String::new());
        assert!(s.is_inline());
        assert!(s.is_empty());
    }

    #[test]
    fn test_from_string_empty_with_capacity_stays_heap() {
        let source = String::with_capacity(8);
        let s: SmallString<64> = SmallString::from_string(source);
        assert!(!s.is_inline());
        assert!(s.is_empty());
    }

    #[test]
    fn test_into_string() {
        let inline: SmallString<16> = SmallString::from_str("hi");
        assert_eq!(inline.into_string(), "hi");

        let heap: SmallString<2> = SmallString::from_str("hello");
        assert!(!heap.is_inline());
        assert_eq!(heap.into_string(), "hello");
    }

    #[test]
    fn test_into_bytes() {
        let s: SmallString<16> = SmallString::from_str("hé");
        assert_eq!(s.into_bytes(), b"h\xC3\xA9".to_vec());
    }

    #[test]
    fn test_from_small_string_for_string() {
        let s: SmallString<4> = SmallString::from_str("hello");
        let owned: String = String::from(s);
        assert_eq!(owned, "hello");
    }

    #[test]
    fn test_from_str_trait_and_parse() {
        use core::str::FromStr;

        let s = <SmallString<8> as FromStr>::from_str("hi").unwrap();
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "hi");

        let parsed: SmallString<16> = "parsed".parse().unwrap();
        assert_eq!(parsed.as_str(), "parsed");
    }

    #[test]
    fn test_as_mut_str_impl() {
        let mut s: SmallString<16> = SmallString::from_str("hello");
        let r: &mut str = s.as_mut();
        r.make_ascii_uppercase();
        assert_eq!(s.as_str(), "HELLO");
    }

    // -----------------------------------------------------------------------
    // Shrinking
    // -----------------------------------------------------------------------

    #[test]
    fn test_shrink_to_fit_moves_back_inline() {
        let mut s: SmallString<8> = SmallString::from_str("hello world");
        assert!(!s.is_inline());
        s.truncate(5);
        assert!(!s.is_inline());
        s.shrink_to_fit();
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_shrink_to_fit_stays_heap_when_too_long() {
        let mut s: SmallString<2> = SmallString::from_str("hello");
        s.shrink_to_fit();
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_shrink_to_moves_back_inline() {
        let mut s: SmallString<8> = SmallString::from_str("hello world");
        assert!(!s.is_inline());
        s.truncate(5);
        s.shrink_to(1);
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_shrink_to_on_inline_is_noop() {
        let mut s: SmallString<16> = SmallString::from_str("hello");
        s.shrink_to(100);
        assert!(s.is_inline());
        s.shrink_to_fit();
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_reserve_exact_spills() {
        let mut s: SmallString<4> = SmallString::from_str("ab");
        s.reserve_exact(10);
        assert!(!s.is_inline());
        assert!(s.capacity() >= 12);
        assert_eq!(s.as_str(), "ab");
    }

    // -----------------------------------------------------------------------
    // Reverse equality and hashing helpers
    // -----------------------------------------------------------------------

    #[test]
    fn test_reverse_partial_eq_string_and_str() {
        let s: SmallString<16> = SmallString::from_str("hello");
        assert_eq!(String::from("hello"), s);
        assert!(*"hello" == s);
        assert!(*"nope" != s);
    }

    #[test]
    fn test_borrow_str_hashmap_lookup() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let key: SmallString<16> = SmallString::from_str("key1");
        map.insert(key, 1);
        assert_eq!(map.get("key1"), Some(&1));
    }

    // -----------------------------------------------------------------------
    // More edge cases
    // -----------------------------------------------------------------------

    #[test]
    #[should_panic]
    fn test_truncate_non_char_boundary_panics() {
        let mut s: SmallString<16> = SmallString::from_str("a🦀b");
        // 2 is inside the 4-byte crab.
        s.truncate(2);
    }

    #[test]
    fn test_truncate_past_len_is_noop_even_off_boundary() {
        let mut s: SmallString<16> = SmallString::from_str("a🦀b");
        s.truncate(100);
        assert_eq!(s.as_str(), "a🦀b");
    }

    #[test]
    fn test_push_multibyte_exact_boundary() {
        // 'a' (1 byte) + '🦀' (4 bytes) == 5 == N, so it stays inline.
        let mut s: SmallString<5> = SmallString::from_str("a");
        s.push('🦀');
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "a🦀");
    }

    #[test]
    fn test_fmt_write_char() {
        use core::fmt::Write;

        let mut s: SmallString<16> = SmallString::new();
        s.write_char('é').unwrap();
        assert_eq!(s.as_str(), "é");
    }

    #[test]
    fn test_pop_keeps_heap_storage() {
        let mut s: SmallString<2> = SmallString::from_str("hello");
        assert!(!s.is_inline());
        assert_eq!(s.pop(), Some('o'));
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), "hell");
    }
}
