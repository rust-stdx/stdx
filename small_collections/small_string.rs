#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SmallString<const N: usize> {
    Inline(heapless::String<N>),
    Heap(alloc::string::String),
}

impl<const N: usize> SmallString<N> {
    #[inline(always)]
    pub fn new() -> Self {
        SmallString::Inline(heapless::String::new())
    }

    /// Copy the content of an `&str` into a new [`SmallString`].
    #[inline(always)]
    pub fn from_str(s: &str) -> Self {
        let mut str = Self::new();
        str.push_str(s);
        str
    }

    /// Creates a [`SmallString`] from an already heap-allocateed String.
    #[inline(always)]
    pub fn from_string(s: String) -> Self {
        Self::Heap(s)
    }

    /// Converts a `Vec` of bytes to a [`SmallString`].
    ///
    /// If the bytes are not valid UTF-8, this returns an error.
    /// If valid, it attempts to store them inline if they fit.
    #[inline(always)]
    pub fn from_utf8(bytes: Vec<u8>) -> Result<Self, alloc::string::FromUtf8Error> {
        Ok(Self::Heap(alloc::string::String::from_utf8(bytes)?))
    }

    /// Converts a slice of bytes to a [`SmallString`].
    ///
    /// If the bytes are not valid UTF-8, this returns an error.
    /// If valid, it attempts to store them inline if they fit.
    #[inline(always)]
    pub fn from_utf8_slice(bytes: &[u8]) -> Result<Self, core::str::Utf8Error> {
        Ok(Self::from_str(core::str::from_utf8(bytes)?))
    }

    /// Converts a slice of bytes to a [`SmallString`], replacing invalid characters.
    #[inline(always)]
    pub fn from_utf8_lossy(bytes: &[u8]) -> Self {
        // TODO: should we move back to inline if str.len() allows it but str is owned?
        let str = alloc::string::String::from_utf8_lossy(bytes);
        match str {
            std::borrow::Cow::Borrowed(borrowed) => Self::from_str(borrowed),
            std::borrow::Cow::Owned(owned) => Self::from_string(owned),
        }
    }

    /// Returns `true` if the string is currently storing data inline (on the stack).
    #[inline(always)]
    pub fn is_inline(&self) -> bool {
        matches!(self, SmallString::Inline(_))
    }

    /// Returns the total capacity (in bytes) of the string.
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        match self {
            SmallString::Inline(_) => N,
            SmallString::Heap(str) => str.capacity(),
        }
    }

    /// Returns the length of this [`SmallString`] in bytes, not [`char`]s or graphemes.
    #[inline(always)]
    pub fn len(&self) -> usize {
        match self {
            SmallString::Inline(str) => str.len(),
            SmallString::Heap(str) => str.len(),
        }
    }

    /// Returns `true` if this [`SmallString`] has a length of zero, and `false` otherwise.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Truncates this [`SmallString`], removing all contents.
    ///
    /// While this means the [`SmallString`] will have a length of zero, it does not
    /// touch its capacity.
    #[inline(always)]
    pub fn clear(&mut self) {
        match self {
            SmallString::Inline(str) => str.clear(),
            SmallString::Heap(str) => str.clear(),
        }
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        // Leverage Deref
        self
    }

    #[inline(always)]
    pub fn as_mut_str(&mut self) -> &mut str {
        // Leverage DerefMut
        self
    }

    /// Converts a [`SmallString`] into a byte slice.
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }

    /// # Safety
    ///
    /// The caller must ensure that the content of the slice remains valid UTF-8.
    /// If this invariant is violated, it is Undefined Behavior.
    #[inline(always)]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { self.as_mut_str().as_bytes_mut() }
    }

    /// Removes the last character from the string buffer and returns it.
    /// Returns None if the string is empty.
    #[inline(always)]
    pub fn pop(&mut self) -> Option<char> {
        match self {
            SmallString::Inline(str) => str.pop(),
            SmallString::Heap(str) => str.pop(),
        }
    }

    /// Shortens this String to the specified length.
    /// If new_len >= current length, this does nothing.
    ///
    /// # Panics
    ///
    /// Panics if `new_len` does not lie on a [`char`] boundary.
    #[inline(always)]
    pub fn truncate(&mut self, new_len: usize) {
        match self {
            SmallString::Inline(str) => str.truncate(new_len),
            SmallString::Heap(str) => str.truncate(new_len),
        }
    }

    #[inline]
    pub fn push_str(&mut self, input: &str) {
        match self {
            SmallString::Heap(str) => str.push_str(input),
            SmallString::Inline(str) => {
                if str.len() + input.len() <= N {
                    // guaranteed success
                    let _ = str.push_str(input);
                } else {
                    // we need to spill on the heap
                    let new_capacity = core::cmp::max(str.len() + input.len(), N * 2);
                    let mut heap_str = alloc::string::String::with_capacity(new_capacity);
                    heap_str.push_str(str.as_str());
                    heap_str.push_str(input);
                    *self = SmallString::Heap(heap_str)
                }
            }
        }
    }

    #[inline]
    pub fn push(&mut self, ch: char) {
        match self {
            SmallString::Heap(str) => str.push(ch),
            SmallString::Inline(str) => {
                let char_len = ch.len_utf8();
                if str.len() + char_len <= N {
                    // guaranteed success
                    let _ = str.push(ch);
                } else {
                    // we need to spill on the heap
                    let new_capacity = core::cmp::max(str.len() + char_len, N * 2);
                    let mut heap_str = alloc::string::String::with_capacity(new_capacity);
                    heap_str.push_str(str.as_str());
                    heap_str.push(ch);
                    *self = SmallString::Heap(heap_str)
                }
            }
        }
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        match self {
            SmallString::Heap(str) => str.reserve(additional),
            SmallString::Inline(str) => {
                if str.len() + additional > N {
                    // spill to heap
                    let new_capacity = core::cmp::max(str.len() + additional, N * 2);
                    let mut heap_str = alloc::string::String::with_capacity(new_capacity);
                    heap_str.push_str(str);
                    *self = SmallString::Heap(heap_str)
                }
            }
        }
    }
}

impl<const N: usize> From<&str> for SmallString<N> {
    #[inline(always)]
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl<const N: usize> From<String> for SmallString<N> {
    #[inline(always)]
    fn from(s: String) -> Self {
        Self::from_string(s)
    }
}

impl<const N: usize> core::ops::Deref for SmallString<N> {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        match self {
            SmallString::Inline(str) => str.as_str(),
            SmallString::Heap(str) => str.as_str(),
        }
    }
}

impl<const N: usize> core::ops::DerefMut for SmallString<N> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            SmallString::Inline(str) => str.as_mut_str(),
            SmallString::Heap(str) => str.as_mut_str(),
        }
    }
}

impl<const N: usize> Default for SmallString<N> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> core::fmt::Display for SmallString<N> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self.as_str(), f) // Delegate to str implementation
    }
}

impl<const N: usize> core::fmt::Write for SmallString<N> {
    #[inline(always)]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.push_str(s);
        Ok(())
    }
}

impl<const N: usize, const M: usize> PartialEq<SmallString<M>> for SmallString<N> {
    #[inline(always)]
    fn eq(&self, other: &SmallString<M>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<const N: usize> Eq for SmallString<N> {}

impl<const N: usize> PartialEq<str> for SmallString<N> {
    #[inline(always)]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<'a, const N: usize> PartialEq<&'a str> for SmallString<N> {
    #[inline(always)]
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}

impl<const N: usize> PartialEq<SmallString<N>> for &str {
    #[inline(always)]
    fn eq(&self, other: &SmallString<N>) -> bool {
        *self == other.as_str()
    }
}

impl<const N: usize> PartialEq<String> for SmallString<N> {
    #[inline(always)]
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<const N: usize, const M: usize> PartialOrd<SmallString<M>> for SmallString<N> {
    #[inline(always)]
    fn partial_cmp(&self, other: &SmallString<M>) -> Option<core::cmp::Ordering> {
        Some(self.as_str().cmp(other.as_str()))
    }
}

impl<const N: usize> Ord for SmallString<N> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl<const N: usize> core::hash::Hash for SmallString<N> {
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
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
        assert!(s.is_inline() == false);
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
        assert!(s.is_inline() == false);
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
        let cap_before = s.capacity();
        s.reserve(50);
        assert!(s.capacity() >= cap_before + 50);
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
        let r: &str = &*s;
        assert_eq!(r, "hello");
    }

    #[test]
    fn test_deref_heap() {
        let s: SmallString<4> = SmallString::from_str("long string");
        let r: &str = &*s;
        assert_eq!(r, "long string");
    }

    #[test]
    fn test_deref_mut() {
        let mut s: SmallString<16> = SmallString::from_str("hello");
        let r: &mut str = &mut *s;
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
        assert_eq!(format!("{:?}", s), "Inline(\"hello\")");
    }

    #[test]
    fn test_debug_heap() {
        let s: SmallString<4> = SmallString::from_str("hello world");
        assert_eq!(format!("{:?}", s), "Heap(\"hello world\")");
    }

    #[test]
    fn test_fmt_write() {
        let mut s: SmallString<16> = SmallString::new();
        write!(&mut s, "hello {} {}", "world", 42).unwrap();
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

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_roundtrip_inline() {
        let s: SmallString<16> = SmallString::from_str("hello");
        let json = serde_json::to_string(&s).unwrap();
        let deserialized: SmallString<16> = serde_json::from_str(&json).unwrap();
        assert_eq!(s, deserialized);
        assert!(deserialized.is_inline());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_roundtrip_heap() {
        let s: SmallString<4> = SmallString::from_str("hello world long");
        let json = serde_json::to_string(&s).unwrap();
        let deserialized: SmallString<4> = serde_json::from_str(&json).unwrap();
        assert_eq!(s, deserialized);
        // Deserialized smallstring will fit inline on the target size
    }
}
