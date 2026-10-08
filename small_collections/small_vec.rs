extern crate alloc;

use alloc::{borrow::Cow, collections::VecDeque, vec::Vec};
use core::{
    borrow::{Borrow, BorrowMut},
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut, Index, IndexMut, Range, RangeBounds},
    slice::{Iter, IterMut, SliceIndex},
};

/// A vector that stores its first `N` elements inline (on the stack) and
/// spills onto the heap once it grows past that inline capacity.
///
/// `SmallVec` behaves like [`alloc::vec::Vec`] for the operations it exposes:
/// it dereferences to `[T]`, so all slice methods are available. The only
/// difference is *where* short sequences are stored: values fitting in `N`
/// elements never touch the allocator.
///
/// # Examples
///
/// ```
/// use small_collections::SmallVec;
///
/// let mut v: SmallVec<i32, 4> = SmallVec::new();
/// v.push(1);
/// v.push(2);
/// assert!(v.is_inline());
/// assert_eq!(&*v, &[1, 2]);
///
/// // Pushing past `N` automatically spills onto the heap.
/// for i in 0..4 {
///     v.push(i);
/// }
/// assert!(!v.is_inline());
/// assert_eq!(v.len(), 6);
/// ```
///
/// # Invariants
///
/// * The `Inline` variant never holds more than `N` elements.
/// * A value only ever moves from `Inline` to `Heap` on growth; it moves back
///   to `Inline` only through [`SmallVec::shrink_to_fit`] or
///   [`SmallVec::shrink_to`]. [`SmallVec::clear`] and
///   [`SmallVec::truncate`] keep the current storage.
#[derive(Clone)]
pub enum SmallVec<T, const N: usize> {
    /// Data stored inline, on the stack, with a fixed capacity of `N`.
    Inline(heapless::Vec<T, N>),
    /// Data spilled onto the heap, growing as needed.
    Heap(Vec<T>),
}

impl<T, const N: usize> SmallVec<T, N> {
    /// Creates a new, empty `SmallVec` using its inline storage.
    ///
    /// This never allocates.
    #[inline]
    pub const fn new() -> Self {
        SmallVec::Inline(heapless::Vec::new())
    }

    /// Returns the inline capacity, `N`.
    #[inline]
    pub const fn inline_size() -> usize {
        N
    }

    /// Creates a new, empty `SmallVec` with at least the requested capacity.
    ///
    /// If `capacity <= N`, the result uses its inline storage (and therefore
    /// reports a capacity of `N`). Otherwise a heap buffer with the requested
    /// capacity is allocated.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity <= N {
            Self::new()
        } else {
            SmallVec::Heap(Vec::with_capacity(capacity))
        }
    }

    /// Builds a `SmallVec` from an array, storing it inline.
    ///
    /// The array length `S` must be less than or equal to `N`; this is checked
    /// at compile time and a larger array fails to compile.
    ///
    /// # Examples
    ///
    /// ```
    /// use small_collections::SmallVec;
    ///
    /// let v: SmallVec<i32, 4> = SmallVec::from_array([1, 2, 3]);
    /// assert!(v.is_inline());
    /// assert_eq!(&*v, &[1, 2, 3]);
    /// ```
    ///
    /// An array larger than the inline capacity fails to compile:
    ///
    /// ```compile_fail
    /// use small_collections::SmallVec;
    ///
    /// let v: SmallVec<i32, 2> = SmallVec::from_array([1, 2, 3]);
    /// ```
    #[inline]
    pub const fn from_array<const S: usize>(elements: [T; S]) -> Self {
        SmallVec::Inline(heapless::Vec::from_array(elements))
    }

    /// Builds an inline `SmallVec` containing the first `length` elements of a
    /// full `[T; N]` buffer, dropping the remaining elements.
    ///
    /// # Panics
    ///
    /// Panics if `length > N`.
    #[inline]
    pub fn from_array_and_len(buf: [T; N], length: usize) -> Self {
        assert!(length <= N, "`length` (is {length}) should be <= inline capacity (is {N})");
        let mut v = heapless::Vec::<T, N>::from_array(buf);
        v.truncate(length);
        SmallVec::Inline(v)
    }

    /// Converts an [`alloc::vec::Vec`] into a `SmallVec`.
    ///
    /// An existing heap allocation is always preserved (like
    /// [`SmallString::from_string`](crate::SmallString::from_string)), so even
    /// a short vector stays on the heap. Use [`SmallVec::from_slice`] or the
    /// `From<[T; M]>` impl if you want a short sequence to move inline. An
    /// empty vector with no capacity returns an inline `SmallVec`.
    #[inline]
    pub fn from_vec(vec: Vec<T>) -> Self {
        if vec.capacity() == 0 {
            Self::new()
        } else {
            SmallVec::Heap(vec)
        }
    }

    /// Clones a slice into a `SmallVec`, storing it inline when it fits.
    ///
    /// # Examples
    ///
    /// ```
    /// use small_collections::SmallVec;
    ///
    /// let inline: SmallVec<u8, 4> = SmallVec::from_slice(&[1, 2, 3]);
    /// assert!(inline.is_inline());
    ///
    /// let spilled: SmallVec<u8, 2> = SmallVec::from_slice(&[1, 2, 3]);
    /// assert!(!spilled.is_inline());
    /// ```
    #[inline]
    pub fn from_slice(slice: &[T]) -> Self
    where
        T: Clone,
    {
        if slice.len() <= N {
            SmallVec::Inline(
                heapless::Vec::from_slice(slice).unwrap_or_else(|_| unreachable!("slice length was checked against N")),
            )
        } else {
            SmallVec::Heap(Vec::from(slice))
        }
    }

    /// Moves the contents of a `SmallVec` with a different inline capacity into
    /// this one.
    ///
    /// The elements are stored inline when they fit in `N`, otherwise they are
    /// spilled onto the heap. Unlike [`SmallVec::from_vec`], this does not
    /// preserve an existing heap allocation when it would fit inline.
    #[inline]
    pub fn from_small_vec<const M: usize>(input: SmallVec<T, M>) -> Self {
        match input {
            SmallVec::Heap(vec) => {
                if vec.len() <= N {
                    let mut this = Self::new();
                    this.extend(vec);
                    this
                } else {
                    SmallVec::Heap(vec)
                }
            }
            SmallVec::Inline(vec) => {
                let mut this = Self::new();
                this.extend(vec);
                this
            }
        }
    }

    /// Returns the number of elements in the vector.
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            SmallVec::Inline(v) => v.len(),
            SmallVec::Heap(v) => v.len(),
        }
    }

    /// Returns `true` if the vector contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the total capacity, in elements, of the vector.
    ///
    /// For an inline vector this is always `N`; for a spilled vector it is the
    /// current heap capacity.
    #[inline]
    pub fn capacity(&self) -> usize {
        match self {
            SmallVec::Inline(v) => v.capacity(),
            SmallVec::Heap(v) => v.capacity(),
        }
    }

    /// Returns `true` if the data is currently stored inline, on the stack.
    #[inline]
    pub fn is_inline(&self) -> bool {
        matches!(self, SmallVec::Inline(_))
    }

    /// Extracts a slice containing the entire vector.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        match self {
            SmallVec::Inline(v) => v.as_slice(),
            SmallVec::Heap(v) => v.as_slice(),
        }
    }

    /// Extracts a mutable slice containing the entire vector.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        match self {
            SmallVec::Inline(v) => v.as_mut_slice(),
            SmallVec::Heap(v) => v.as_mut_slice(),
        }
    }

    /// Returns a raw pointer to the first element, or a dangling-but-aligned
    /// pointer when empty.
    ///
    /// The pointer only covers the initialized region (`len` elements), not
    /// any spare capacity.
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.as_slice().as_ptr()
    }

    /// Returns a mutable raw pointer to the first element, or a
    /// dangling-but-aligned pointer when empty.
    ///
    /// The pointer only covers the initialized region (`len` elements), not
    /// any spare capacity.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.as_mut_slice().as_mut_ptr()
    }

    /// Appends an element to the back of the vector, spilling onto the heap if
    /// the inline capacity is exceeded.
    #[inline]
    pub fn push(&mut self, value: T) {
        if let SmallVec::Inline(v) = self
            && v.len() >= N
        {
            // Move everything to the heap, then push `value` below.
            let cap = N.saturating_mul(2).max(1);
            let mut heap: Vec<T> = Vec::with_capacity(cap);
            heap.extend(core::mem::take(v));
            *self = SmallVec::Heap(heap);
        }

        match self {
            SmallVec::Inline(v) => {
                if v.push(value).is_err() {
                    unreachable!("inline capacity was checked before pushing");
                }
            }
            SmallVec::Heap(v) => v.push(value),
        }
    }

    /// Appends an element and returns a mutable reference to it.
    #[inline]
    pub fn push_mut(&mut self, value: T) -> &mut T {
        self.push(value);
        self.last_mut().expect("an element was just pushed")
    }

    /// Removes the last element and returns it, or `None` if empty.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        match self {
            SmallVec::Inline(v) => v.pop(),
            SmallVec::Heap(v) => v.pop(),
        }
    }

    /// Removes and returns the last element if `predicate` returns `true` for
    /// it, otherwise leaves the vector unchanged.
    #[inline]
    pub fn pop_if(&mut self, predicate: impl FnOnce(&mut T) -> bool) -> Option<T> {
        let last = self.last_mut()?;
        if predicate(last) { self.pop() } else { None }
    }

    /// Inserts an element at position `index`, shifting all elements after it
    /// to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    #[inline]
    pub fn insert(&mut self, index: usize, value: T) {
        let len = self.len();
        assert!(index <= len, "insertion index (is {index}) should be <= length (is {len})");
        self.reserve(1);
        match self {
            SmallVec::Inline(v) => {
                if v.insert(index, value).is_err() {
                    unreachable!("inline capacity was reserved before inserting");
                }
            }
            SmallVec::Heap(v) => v.insert(index, value),
        }
    }

    /// Inserts an element at position `index` and returns a mutable reference
    /// to it.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    #[inline]
    pub fn insert_mut(&mut self, index: usize, value: T) -> &mut T {
        self.insert(index, value);
        &mut self[index]
    }

    /// Removes and returns the element at `index`, shifting all elements after
    /// it to the left.
    ///
    /// # Panics
    ///
    /// Panics if `index >= len`.
    #[inline]
    pub fn remove(&mut self, index: usize) -> T {
        let len = self.len();
        assert!(index < len, "removal index (is {index}) should be < length (is {len})");
        match self {
            SmallVec::Inline(v) => v.remove(index),
            SmallVec::Heap(v) => v.remove(index),
        }
    }

    /// Removes and returns the element at `index`, replacing it with the last
    /// element. This does not preserve ordering, but is `O(1)`.
    ///
    /// # Panics
    ///
    /// Panics if `index >= len`.
    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> T {
        let len = self.len();
        assert!(index < len, "swap_remove index (is {index}) should be < length (is {len})");
        match self {
            SmallVec::Inline(v) => v.swap_remove(index),
            SmallVec::Heap(v) => v.swap_remove(index),
        }
    }

    /// Removes all elements, keeping the current storage (inline stays inline,
    /// heap stays heap) and its capacity.
    #[inline]
    pub fn clear(&mut self) {
        match self {
            SmallVec::Inline(v) => v.clear(),
            SmallVec::Heap(v) => v.clear(),
        }
    }

    /// Shortens the vector to `new_len`, dropping the excess elements.
    ///
    /// Does nothing if `new_len >= len`. Keeps the current storage.
    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        match self {
            SmallVec::Inline(v) => v.truncate(new_len),
            SmallVec::Heap(v) => v.truncate(new_len),
        }
    }

    /// Retains only the elements for which `f` returns `true`.
    #[inline]
    pub fn retain<F: FnMut(&T) -> bool>(&mut self, f: F) {
        match self {
            SmallVec::Inline(v) => v.retain(f),
            SmallVec::Heap(v) => v.retain(f),
        }
    }

    /// Retains only the elements for which `f` returns `true`, passing each
    /// element by mutable reference.
    #[inline]
    pub fn retain_mut<F: FnMut(&mut T) -> bool>(&mut self, f: F) {
        match self {
            SmallVec::Inline(v) => v.retain_mut(f),
            SmallVec::Heap(v) => v.retain_mut(f),
        }
    }

    /// Removes consecutive duplicate elements using `PartialEq`.
    #[inline]
    pub fn dedup(&mut self)
    where
        T: PartialEq,
    {
        self.dedup_by(|a, b| a == b);
    }

    /// Removes consecutive elements that map to the same key.
    #[inline]
    pub fn dedup_by_key<F, K>(&mut self, mut key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        self.dedup_by(|a, b| key(a) == key(b));
    }

    /// Removes consecutive elements for which `same_bucket` returns `true`.
    #[inline]
    pub fn dedup_by<F>(&mut self, mut same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        let len = self.len();
        if len <= 1 {
            return;
        }

        let slice = self.as_mut_slice();
        let mut w = 1;
        for r in 1..len {
            let keep = {
                let (left, right) = slice.split_at_mut(r);
                let current = &mut right[0];
                let previous = &mut left[w - 1];
                !same_bucket(current, previous)
            };
            if keep {
                if r != w {
                    slice.swap(r, w);
                }
                w += 1;
            }
        }
        self.truncate(w);
    }

    /// Resizes the vector to `new_len`, filling new slots by cloning `value`.
    #[inline]
    pub fn resize(&mut self, new_len: usize, value: T)
    where
        T: Clone,
    {
        let len = self.len();
        if new_len > len {
            self.extend(core::iter::repeat_n(value, new_len - len));
        } else {
            self.truncate(new_len);
        }
    }

    /// Resizes the vector to `new_len`, filling new slots with `f()`.
    #[inline]
    pub fn resize_with<F: FnMut() -> T>(&mut self, new_len: usize, mut f: F) {
        let len = self.len();
        if new_len > len {
            self.extend(core::iter::repeat_with(&mut f).take(new_len - len));
        } else {
            self.truncate(new_len);
        }
    }

    /// Clones and appends every element of `other`.
    #[inline]
    pub fn extend_from_slice(&mut self, other: &[T])
    where
        T: Clone,
    {
        self.reserve(other.len());
        match self {
            SmallVec::Inline(v) => {
                if v.extend_from_slice(other).is_err() {
                    unreachable!("capacity was reserved before extending");
                }
            }
            SmallVec::Heap(v) => v.extend_from_slice(other),
        }
    }

    /// Clones the elements in `source` and appends them to the end of the
    /// vector.
    ///
    /// # Panics
    ///
    /// Panics if the range is out of bounds.
    #[inline]
    pub fn extend_from_within<R: RangeBounds<usize>>(&mut self, source: R)
    where
        T: Clone,
    {
        let range = bounded_range(source, self.len());
        let count = range.end - range.start;
        if count == 0 {
            return;
        }
        self.reserve(count);
        match self {
            // `Vec::extend_from_within` clones the range in bulk.
            SmallVec::Heap(v) => v.extend_from_within(range),
            SmallVec::Inline(v) => {
                for i in 0..count {
                    let value = v[range.start + i].clone();
                    if v.push(value).is_err() {
                        unreachable!("capacity was reserved before extending");
                    }
                }
            }
        }
    }

    /// Reserves room for at least `additional` more elements, growing the
    /// heap buffer (or spilling to it) if needed.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        match self {
            SmallVec::Heap(v) => v.reserve(additional),
            SmallVec::Inline(v) => {
                if additional > N - v.len() {
                    let cap = N.saturating_mul(2).max(v.len().saturating_add(additional));
                    let mut heap: Vec<T> = Vec::with_capacity(cap);
                    heap.extend(core::mem::take(v));
                    *self = SmallVec::Heap(heap);
                }
            }
        }
    }

    /// Reserves room for exactly `additional` more elements, growing the heap
    /// buffer (or spilling to it) if needed.
    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        match self {
            SmallVec::Heap(v) => v.reserve_exact(additional),
            SmallVec::Inline(v) => {
                if additional > N - v.len() {
                    let cap = v.len().saturating_add(additional).max(N.saturating_add(1));
                    let mut heap: Vec<T> = Vec::with_capacity(cap);
                    heap.extend(core::mem::take(v));
                    *self = SmallVec::Heap(heap);
                }
            }
        }
    }

    /// Shrinks the vector as much as possible: an inline vector is unchanged,
    /// while a spilled vector whose length fits in `N` moves back inline.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        if let SmallVec::Heap(v) = self {
            if v.len() <= N {
                let inline = heapless::Vec::<T, N>::from_iter(v.drain(..));
                *self = SmallVec::Inline(inline);
            } else {
                v.shrink_to_fit();
            }
        }
    }

    /// Shrinks the vector's capacity to at least `min_capacity`, moving it back
    /// inline when the result fits in `N`.
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        if let SmallVec::Heap(v) = self
            && v.capacity() > min_capacity
        {
            let target = v.len().max(min_capacity);
            if target <= N {
                let inline = heapless::Vec::<T, N>::from_iter(v.drain(..));
                *self = SmallVec::Inline(inline);
            } else {
                v.shrink_to(target);
            }
        }
    }

    /// Moves every element out of `other` and appends them here, leaving
    /// `other` empty.
    #[inline]
    pub fn append<const M: usize>(&mut self, other: &mut SmallVec<T, M>) {
        self.reserve(other.len());
        self.extend(other.drain(..));
    }

    /// Splits the vector into two at `at`: `self` keeps `[0, at)`, and the
    /// returned vector holds `[at, len)`.
    ///
    /// # Panics
    ///
    /// Panics if `at > len`.
    #[inline]
    pub fn split_off(&mut self, at: usize) -> Self {
        let len = self.len();
        assert!(at <= len, "split index (is {at}) should be <= length (is {len})");
        self.drain(at..).collect()
    }

    /// Removes the given range from the vector and returns an iterator over the
    /// removed elements.
    ///
    /// The returned iterator keeps a mutable borrow on the vector. If it is
    /// dropped without being fully consumed, the remaining removed elements are
    /// dropped and the tail is moved into place.
    ///
    /// # Panics
    ///
    /// Panics if the range is out of bounds.
    #[inline]
    pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> Drain<'_, T, N> {
        match self {
            SmallVec::Inline(v) => Drain::Inline(v.drain(range)),
            SmallVec::Heap(v) => Drain::Heap(v.drain(range)),
        }
    }
}

/// Converts a `RangeBounds` into a concrete, validated `Range`, panicking when
/// it is out of bounds for a vector of `len` elements.
fn bounded_range<R: RangeBounds<usize>>(range: R, len: usize) -> Range<usize> {
    use core::ops::Bound;

    let start = match range.start_bound() {
        Bound::Included(&n) => n,
        Bound::Excluded(&n) => n.checked_add(1).expect("range start overflow"),
        Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        Bound::Included(&n) => n.checked_add(1).expect("range end overflow"),
        Bound::Excluded(&n) => n,
        Bound::Unbounded => len,
    };
    assert!(start <= end && end <= len, "range out of bounds");
    start..end
}

/// An iterator that moves elements out of a [`SmallVec`].
///
/// Created by [`SmallVec::into_iter`]. It yields owned values, taking them
/// either from the inline storage or from the heap buffer.
pub enum IntoIter<T, const N: usize> {
    /// Iterating over the inline storage.
    Inline(heapless::vec::IntoIter<T, N, usize>),
    /// Iterating over the heap buffer.
    Heap(alloc::vec::IntoIter<T>),
}

impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IntoIter::Inline(v) => v.next(),
            IntoIter::Heap(v) => v.next(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            IntoIter::Inline(v) => v.size_hint(),
            IntoIter::Heap(v) => v.size_hint(),
        }
    }
}

impl<T, const N: usize> DoubleEndedIterator for IntoIter<T, N> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            IntoIter::Inline(v) => v.next_back(),
            IntoIter::Heap(v) => v.next_back(),
        }
    }
}

impl<T, const N: usize> ExactSizeIterator for IntoIter<T, N> {
    #[inline]
    fn len(&self) -> usize {
        match self {
            IntoIter::Inline(v) => v.len(),
            IntoIter::Heap(v) => v.len(),
        }
    }
}

impl<T, const N: usize> core::iter::FusedIterator for IntoIter<T, N> {}

impl<T: Clone, const N: usize> Clone for IntoIter<T, N> {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            IntoIter::Inline(v) => IntoIter::Inline(v.clone()),
            IntoIter::Heap(v) => IntoIter::Heap(v.clone()),
        }
    }
}

impl<T: fmt::Debug, const N: usize> fmt::Debug for IntoIter<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntoIter::Inline(v) => fmt::Debug::fmt(v, f),
            IntoIter::Heap(v) => fmt::Debug::fmt(v, f),
        }
    }
}

/// A draining iterator for a [`SmallVec`].
///
/// Created by [`SmallVec::drain`]. If the iterator is dropped before being
/// fully consumed, the remaining removed elements are dropped and the
/// unconsumed tail is moved back into place.
pub enum Drain<'a, T, const N: usize> {
    /// Draining the inline storage.
    Inline(heapless::vec::Drain<'a, T, usize>),
    /// Draining the heap buffer.
    Heap(alloc::vec::Drain<'a, T>),
}

impl<T, const N: usize> Drain<'_, T, N> {
    /// Returns the remaining elements to be yielded as a slice.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        match self {
            Drain::Inline(v) => v.as_slice(),
            Drain::Heap(v) => v.as_slice(),
        }
    }
}

impl<T, const N: usize> Iterator for Drain<'_, T, N> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Drain::Inline(v) => v.next(),
            Drain::Heap(v) => v.next(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Drain::Inline(v) => v.size_hint(),
            Drain::Heap(v) => v.size_hint(),
        }
    }
}

impl<T, const N: usize> DoubleEndedIterator for Drain<'_, T, N> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Drain::Inline(v) => v.next_back(),
            Drain::Heap(v) => v.next_back(),
        }
    }
}

impl<T, const N: usize> ExactSizeIterator for Drain<'_, T, N> {
    #[inline]
    fn len(&self) -> usize {
        match self {
            Drain::Inline(v) => v.len(),
            Drain::Heap(v) => v.len(),
        }
    }
}

impl<T, const N: usize> core::iter::FusedIterator for Drain<'_, T, N> {}

impl<T: fmt::Debug, const N: usize> fmt::Debug for Drain<'_, T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Drain::Inline(v) => fmt::Debug::fmt(v, f),
            Drain::Heap(v) => fmt::Debug::fmt(v, f),
        }
    }
}

impl<T, const N: usize> Default for SmallVec<T, N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Deref for SmallVec<T, N> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, const N: usize> DerefMut for SmallVec<T, N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T, const N: usize> AsRef<[T]> for SmallVec<T, N> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const N: usize> AsMut<[T]> for SmallVec<T, N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, const N: usize> Borrow<[T]> for SmallVec<T, N> {
    #[inline]
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const N: usize> BorrowMut<[T]> for SmallVec<T, N> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, const N: usize, I: SliceIndex<[T]>> Index<I> for SmallVec<T, N> {
    type Output = <I as SliceIndex<[T]>>::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T, const N: usize, I: SliceIndex<[T]>> IndexMut<I> for SmallVec<T, N> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}

impl<T: fmt::Debug, const N: usize> fmt::Debug for SmallVec<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: Hash, const N: usize> Hash for SmallVec<T, N> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<T: Eq, const N: usize> Eq for SmallVec<T, N> {}

impl<T, U, const N: usize, const M: usize> PartialOrd<SmallVec<U, M>> for SmallVec<T, N>
where
    T: PartialOrd<U>,
{
    #[inline]
    fn partial_cmp(&self, other: &SmallVec<U, M>) -> Option<Ordering> {
        self.as_slice().iter().partial_cmp(other.as_slice().iter())
    }
}

impl<T: Ord, const N: usize> Ord for SmallVec<T, N> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

/// Implements `PartialEq` between two slice-like types by comparing their
/// dereferenced slices.
macro_rules! impl_slice_eq {
    ([$($vars:tt)*] $lhs:ty, $rhs:ty) => {
        impl<T, U, const N: usize, $($vars)*> PartialEq<$rhs> for $lhs
        where
            T: PartialEq<U>,
        {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                self[..] == other[..]
            }
        }
    };
}

impl_slice_eq! { [const M: usize] SmallVec<T, M>, SmallVec<U, N> }
impl_slice_eq! { [const M: usize] SmallVec<T, M>, [U; N] }
impl_slice_eq! { [const M: usize] SmallVec<T, M>, &[U; N] }
impl_slice_eq! { [] SmallVec<T, N>, [U] }
impl_slice_eq! { [] SmallVec<T, N>, &[U] }
impl_slice_eq! { [] SmallVec<T, N>, &mut [U] }
impl_slice_eq! { [] [T], SmallVec<U, N> }
impl_slice_eq! { [] &[T], SmallVec<U, N> }
impl_slice_eq! { [] &mut [T], SmallVec<U, N> }
impl_slice_eq! { [] Vec<T>, SmallVec<U, N> }
impl_slice_eq! { [] SmallVec<T, N>, Vec<U> }

impl<T, U, const N: usize> PartialEq<SmallVec<U, N>> for Cow<'_, [T]>
where
    T: Clone + PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &SmallVec<U, N>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U, const N: usize> PartialEq<Cow<'_, [U]>> for SmallVec<T, N>
where
    U: Clone,
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &Cow<'_, [U]>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U, const N: usize> PartialEq<SmallVec<U, N>> for VecDeque<T>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &SmallVec<U, N>) -> bool {
        let other = other.as_slice();
        if self.len() != other.len() {
            return false;
        }
        let (sa, sb) = self.as_slices();
        let (oa, ob) = other.split_at(sa.len());
        sa == oa && sb == ob
    }
}

impl<T, const N: usize> IntoIterator for SmallVec<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        match self {
            SmallVec::Inline(v) => IntoIter::Inline(v.into_iter()),
            SmallVec::Heap(v) => IntoIter::Heap(v.into_iter()),
        }
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a SmallVec<T, N> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut SmallVec<T, N> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T, const N: usize> Extend<T> for SmallVec<T, N> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();
        self.reserve(lower);
        for item in iter {
            self.push(item);
        }
    }
}

impl<'a, T: Clone + 'a, const N: usize> Extend<&'a T> for SmallVec<T, N> {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
}

impl<T, const N: usize> FromIterator<T> for SmallVec<T, N> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();
        let mut this = Self::with_capacity(lower);
        this.extend(iter);
        this
    }
}

impl<T, const N: usize> From<&[T]> for SmallVec<T, N>
where
    T: Clone,
{
    #[inline]
    fn from(slice: &[T]) -> Self {
        Self::from_slice(slice)
    }
}

impl<T, const N: usize> From<&mut [T]> for SmallVec<T, N>
where
    T: Clone,
{
    #[inline]
    fn from(slice: &mut [T]) -> Self {
        Self::from_slice(slice)
    }
}

impl<T, const M: usize, const N: usize> From<&[T; M]> for SmallVec<T, N>
where
    T: Clone,
{
    #[inline]
    fn from(slice: &[T; M]) -> Self {
        Self::from_slice(slice)
    }
}

impl<T, const M: usize, const N: usize> From<&mut [T; M]> for SmallVec<T, N>
where
    T: Clone,
{
    #[inline]
    fn from(slice: &mut [T; M]) -> Self {
        Self::from_slice(slice)
    }
}

impl<T, const M: usize, const N: usize> From<[T; M]> for SmallVec<T, N> {
    #[inline]
    fn from(array: [T; M]) -> Self {
        if M <= N {
            SmallVec::Inline(heapless::Vec::from_iter(array))
        } else {
            Self::from_vec(Vec::from(array))
        }
    }
}

impl<T, const N: usize> From<Vec<T>> for SmallVec<T, N> {
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        Self::from_vec(vec)
    }
}

impl<T, const N: usize> From<SmallVec<T, N>> for Vec<T> {
    #[inline]
    fn from(this: SmallVec<T, N>) -> Self {
        match this {
            SmallVec::Inline(v) => v.into_iter().collect(),
            SmallVec::Heap(v) => v,
        }
    }
}

impl<T, const N: usize> From<SmallVec<T, N>> for alloc::boxed::Box<[T]> {
    #[inline]
    fn from(this: SmallVec<T, N>) -> Self {
        Vec::from(this).into_boxed_slice()
    }
}

impl<T, const M: usize, const N: usize> TryFrom<SmallVec<T, N>> for [T; M] {
    type Error = SmallVec<T, N>;

    #[inline]
    fn try_from(this: SmallVec<T, N>) -> Result<[T; M], Self::Error> {
        if this.len() != M {
            return Err(this);
        }

        match this {
            SmallVec::Inline(v) => match v.into_array::<M>() {
                Ok(array) => Ok(array),
                Err(v) => Err(SmallVec::Inline(v)),
            },
            SmallVec::Heap(v) => match <[T; M]>::try_from(v) {
                Ok(array) => Ok(array),
                Err(v) => Err(SmallVec::Heap(v)),
            },
        }
    }
}

#[cfg(feature = "serde")]
impl<T, const N: usize> serde::Serialize for SmallVec<T, N>
where
    T: serde::Serialize,
{
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, T, const N: usize> serde::Deserialize<'de> for SmallVec<T, N>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use core::marker::PhantomData;

        use serde::de::{SeqAccess, Visitor};

        struct SmallVecVisitor<T, const N: usize> {
            marker: PhantomData<T>,
        }

        impl<'de, T, const N: usize> Visitor<'de> for SmallVecVisitor<T, N>
        where
            T: serde::Deserialize<'de>,
        {
            type Value = SmallVec<T, N>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut this = SmallVec::new();
                if let Some(size) = seq.size_hint() {
                    this.reserve(size);
                }
                while let Some(value) = seq.next_element()? {
                    this.push(value);
                }
                Ok(this)
            }
        }

        deserializer.deserialize_seq(SmallVecVisitor {
            marker: PhantomData,
        })
    }
}

#[cfg(feature = "std")]
impl<const N: usize> std::io::Write for SmallVec<u8, N> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.extend_from_slice(buf);
        Ok(buf.len())
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.extend_from_slice(buf);
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(feature = "bytes")]
unsafe impl<const N: usize> bytes::BufMut for SmallVec<u8, N> {
    #[inline]
    fn remaining_mut(&self) -> usize {
        // A vector can never have more than `isize::MAX` bytes.
        isize::MAX as usize - self.len()
    }

    #[inline]
    unsafe fn advance_mut(&mut self, cnt: usize) {
        let len = self.len();
        let remaining = self.capacity() - len;
        assert!(
            cnt <= remaining,
            "advance out of bounds: the capacity is {remaining} but advancing by {cnt}"
        );

        let new_len = len + cnt;
        match self {
            SmallVec::Inline(v) => unsafe { v.set_len(new_len) },
            SmallVec::Heap(v) => unsafe { v.set_len(new_len) },
        }
    }

    #[inline]
    fn chunk_mut(&mut self) -> &mut bytes::buf::UninitSlice {
        if self.capacity() == self.len() {
            // Grow the vector so there is always somewhere to write.
            self.reserve(64);
        }

        match self {
            SmallVec::Inline(v) => {
                let spare = v.spare_capacity_mut();
                let ptr = spare.as_mut_ptr().cast::<u8>();
                let len = spare.len();
                // SAFETY: `ptr` points to `len` contiguous, writable bytes of
                // uninitialized storage owned by `self`.
                unsafe { bytes::buf::UninitSlice::from_raw_parts_mut(ptr, len) }
            }
            SmallVec::Heap(v) => v.chunk_mut(),
        }
    }

    #[inline]
    fn put<T: bytes::Buf>(&mut self, mut source: T)
    where
        Self: Sized,
    {
        self.reserve(source.remaining());
        while source.has_remaining() {
            let chunk = source.chunk();
            let len = chunk.len();
            self.extend_from_slice(chunk);
            source.advance(len);
        }
    }

    #[inline]
    fn put_slice(&mut self, source: &[u8]) {
        self.extend_from_slice(source);
    }

    #[inline]
    fn put_bytes(&mut self, val: u8, cnt: usize) {
        let new_len = self.len().saturating_add(cnt);
        self.resize(new_len, val);
    }
}

#[cfg(test)]
mod tests {
    use alloc::{boxed::Box, vec, vec::Vec};
    use core::cmp::Ordering;

    use super::SmallVec;

    #[test]
    fn new_is_inline_and_empty() {
        let v: SmallVec<i32, 4> = SmallVec::new();
        assert!(v.is_inline());
        assert!(v.is_inline());
        assert!(v.is_empty());
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 4);
        assert_eq!(<SmallVec<i32, 4>>::inline_size(), 4);
    }

    #[test]
    fn push_stays_inline_until_full() {
        let mut v: SmallVec<i32, 3> = SmallVec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        assert!(v.is_inline());
        assert_eq!(&*v, &[1, 2, 3]);

        v.push(4);
        assert!(!v.is_inline());
        assert_eq!(&*v, &[1, 2, 3, 4]);
        assert!(v.capacity() >= 4);
    }

    #[test]
    fn zero_inline_capacity_spills_immediately() {
        let mut v: SmallVec<i32, 0> = SmallVec::new();
        assert!(v.is_inline());
        assert_eq!(v.capacity(), 0);
        v.push(1);
        assert!(!v.is_inline());
        assert_eq!(&*v, &[1]);
    }

    #[test]
    fn pop_and_pop_if() {
        let mut v: SmallVec<i32, 4> = SmallVec::from([1, 2, 3]);
        assert_eq!(v.pop(), Some(3));
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.pop_if(|x| *x == 1), Some(1));
        assert_eq!(v.pop(), None);
        assert_eq!(v.pop_if(|_| true), None);
    }

    #[test]
    fn insert_and_remove() {
        let mut v: SmallVec<i32, 4> = SmallVec::from([1, 2, 3]);
        v.insert(1, 9);
        assert_eq!(&*v, &[1, 9, 2, 3]);
        assert!(v.is_inline());

        // Inserting past the inline capacity spills.
        v.insert(4, 8);
        assert!(!v.is_inline());
        assert_eq!(&*v, &[1, 9, 2, 3, 8]);

        assert_eq!(v.remove(0), 1);
        assert_eq!(&*v, &[9, 2, 3, 8]);
    }

    #[test]
    #[should_panic(expected = "insertion index")]
    fn insert_out_of_bounds_panics() {
        let mut v: SmallVec<i32, 4> = SmallVec::from([1]);
        v.insert(3, 2);
    }

    #[test]
    #[should_panic(expected = "removal index")]
    fn remove_out_of_bounds_panics() {
        let mut v: SmallVec<i32, 4> = SmallVec::from([1]);
        v.remove(1);
    }

    #[test]
    fn swap_remove() {
        let mut v: SmallVec<i32, 4> = SmallVec::from([1, 2, 3, 4]);
        assert_eq!(v.swap_remove(1), 2);
        assert_eq!(&*v, &[1, 4, 3]);
        assert_eq!(v.swap_remove(2), 3);
        assert_eq!(&*v, &[1, 4]);
    }

    #[test]
    fn push_mut_and_insert_mut() {
        let mut v: SmallVec<i32, 2> = SmallVec::new();
        *v.push_mut(1) += 10;
        *v.insert_mut(0, 5) *= 2;
        assert_eq!(&*v, &[10, 11]);
    }

    #[test]
    fn clear_keeps_storage() {
        let mut inline: SmallVec<i32, 4> = SmallVec::from([1, 2]);
        inline.clear();
        assert!(inline.is_inline());
        assert!(inline.is_empty());

        let mut spilled: SmallVec<i32, 2> = SmallVec::from([1, 2, 3]);
        spilled.clear();
        assert!(!spilled.is_inline());
        assert!(spilled.is_empty());
    }

    #[test]
    fn truncate_keeps_storage() {
        let mut v: SmallVec<i32, 4> = SmallVec::from([1, 2, 3]);
        v.truncate(1);
        assert!(v.is_inline());
        assert_eq!(&*v, &[1]);
        v.truncate(10);
        assert_eq!(&*v, &[1]);
    }

    #[test]
    fn retain_and_retain_mut() {
        let mut v: SmallVec<i32, 8> = SmallVec::from([1, 2, 3, 4, 5]);
        v.retain(|x| x % 2 == 1);
        assert_eq!(&*v, &[1, 3, 5]);

        v.retain_mut(|x| {
            *x *= 2;
            *x < 6
        });
        assert_eq!(&*v, &[2]);
    }

    #[test]
    fn dedup() {
        let mut v: SmallVec<i32, 8> = SmallVec::from([1, 1, 2, 2, 2, 3, 1]);
        v.dedup();
        assert_eq!(&*v, &[1, 2, 3, 1]);

        let mut v: SmallVec<i32, 8> = SmallVec::from([1, 3, 2, 4, 5]);
        v.dedup_by_key(|x| *x % 2);
        assert_eq!(&*v, &[1, 2, 5]);
    }

    #[test]
    fn resize_and_resize_with() {
        let mut v: SmallVec<i32, 4> = SmallVec::from([1, 2]);
        v.resize(6, 0);
        assert!(!v.is_inline());
        assert_eq!(&*v, &[1, 2, 0, 0, 0, 0]);
        v.resize(2, 0);
        assert_eq!(&*v, &[1, 2]);

        let mut counter = 0;
        let mut w: SmallVec<i32, 2> = SmallVec::new();
        w.resize_with(4, || {
            counter += 1;
            counter
        });
        assert_eq!(&*w, &[1, 2, 3, 4]);
    }

    #[test]
    fn extend_from_slice_and_within() {
        let mut v: SmallVec<i32, 4> = SmallVec::from([1, 2]);
        v.extend_from_slice(&[3, 4]);
        assert!(v.is_inline());
        assert_eq!(&*v, &[1, 2, 3, 4]);

        v.extend_from_within(0..2);
        assert!(!v.is_inline());
        assert_eq!(&*v, &[1, 2, 3, 4, 1, 2]);
    }

    #[test]
    fn reserve_and_reserve_exact() {
        let mut v: SmallVec<i32, 4> = SmallVec::from([1, 2]);
        v.reserve(1);
        assert!(v.is_inline());
        v.reserve(10);
        assert!(!v.is_inline());
        assert!(v.capacity() >= 12);
        let before = v.capacity();
        v.reserve_exact(2);
        assert!(v.capacity() >= before);
        assert_eq!(&*v, &[1, 2]);
    }

    #[test]
    fn shrink_to_fit_moves_back_inline() {
        let mut v: SmallVec<i32, 4> = SmallVec::from([1, 2, 3, 4]);
        v.push(5);
        assert!(!v.is_inline());
        v.pop();
        v.shrink_to_fit();
        assert!(v.is_inline());
        assert_eq!(&*v, &[1, 2, 3, 4]);
    }

    #[test]
    fn shrink_to_moves_back_inline() {
        let mut v: SmallVec<i32, 4> = SmallVec::from([1, 2, 3, 4]);
        v.push(5);
        assert!(!v.is_inline());
        v.pop();
        v.shrink_to(2);
        assert!(v.is_inline());
        assert_eq!(&*v, &[1, 2, 3, 4]);
    }

    #[test]
    fn append_moves_between_capacities() {
        let mut a: SmallVec<i32, 4> = SmallVec::from([1, 2]);
        let mut b: SmallVec<i32, 8> = SmallVec::from([3, 4, 5]);
        a.append(&mut b);
        assert_eq!(&*a, &[1, 2, 3, 4, 5]);
        assert!(b.is_empty());
    }

    #[test]
    fn split_off() {
        let mut v: SmallVec<i32, 8> = SmallVec::from([1, 2, 3, 4, 5]);
        let tail: SmallVec<i32, 8> = v.split_off(2);
        assert_eq!(&*v, &[1, 2]);
        assert_eq!(&*tail, &[3, 4, 5]);
        assert!(tail.is_inline());
    }

    #[test]
    fn drain_inline_and_heap() {
        let mut inline: SmallVec<i32, 8> = SmallVec::from([1, 2, 3, 4]);
        let drained: Vec<i32> = inline.drain(1..3).collect();
        assert_eq!(drained, vec![2, 3]);
        assert_eq!(&*inline, &[1, 4]);

        let mut heap: SmallVec<i32, 2> = SmallVec::from([1, 2, 3, 4]);
        assert!(!heap.is_inline());
        heap.drain(..);
        assert!(heap.is_empty());
    }

    #[test]
    fn drain_dropped_early_restores_tail() {
        let mut v: SmallVec<i32, 8> = SmallVec::from([1, 2, 3, 4, 5]);
        {
            let mut d = v.drain(1..4);
            assert_eq!(d.next(), Some(2));
        }
        assert_eq!(&*v, &[1, 5]);
    }

    #[test]
    fn into_iter_inline_and_heap() {
        let inline: SmallVec<i32, 8> = SmallVec::from([1, 2, 3]);
        let collected: Vec<i32> = inline.into_iter().collect();
        assert_eq!(collected, vec![1, 2, 3]);

        let heap: SmallVec<i32, 2> = SmallVec::from([1, 2, 3, 4]);
        assert!(!heap.is_inline());
        let collected: Vec<i32> = heap.into_iter().collect();
        assert_eq!(collected, vec![1, 2, 3, 4]);
    }

    #[test]
    fn into_iter_double_ended() {
        let v: SmallVec<i32, 8> = SmallVec::from([1, 2, 3]);
        let collected: Vec<i32> = v.into_iter().rev().collect();
        assert_eq!(collected, vec![3, 2, 1]);
    }

    #[test]
    fn from_vec_preserves_heap() {
        let heap: Vec<i32> = vec![1, 2];
        let v: SmallVec<i32, 16> = SmallVec::from_vec(heap);
        assert!(!v.is_inline());
        assert_eq!(&*v, &[1, 2]);
    }

    #[test]
    fn from_vec_empty_is_inline() {
        let v: SmallVec<i32, 4> = SmallVec::from_vec(Vec::new());
        assert!(v.is_inline());
    }

    #[test]
    fn from_array_inline_and_heap() {
        let inline: SmallVec<i32, 4> = SmallVec::from([1, 2, 3]);
        assert!(inline.is_inline());

        let heap: SmallVec<i32, 2> = SmallVec::from([1, 2, 3]);
        assert!(!heap.is_inline());
        assert_eq!(&*heap, &[1, 2, 3]);
    }

    #[test]
    fn from_array_and_len() {
        let v: SmallVec<i32, 4> = SmallVec::from_array_and_len([1, 2, 3, 4], 2);
        assert!(v.is_inline());
        assert_eq!(&*v, &[1, 2]);
    }

    #[test]
    fn from_slice_inline_and_heap() {
        let inline: SmallVec<i32, 4> = SmallVec::from_slice(&[1, 2]);
        assert!(inline.is_inline());

        let heap: SmallVec<i32, 1> = SmallVec::from_slice(&[1, 2]);
        assert!(!heap.is_inline());
    }

    #[test]
    fn from_small_vec_cross_capacity() {
        let source: SmallVec<i32, 8> = SmallVec::from([1, 2, 3]);
        let fits: SmallVec<i32, 8> = SmallVec::from_small_vec(source);
        assert!(fits.is_inline());
        assert_eq!(&*fits, &[1, 2, 3]);

        let spilled: SmallVec<i32, 2> = SmallVec::from([1, 2, 3]);
        let target: SmallVec<i32, 8> = SmallVec::from_small_vec(spilled);
        assert!(target.is_inline());
        assert_eq!(&*target, &[1, 2, 3]);

        let big: SmallVec<i32, 8> = SmallVec::from([1, 2, 3, 4, 5]);
        let target: SmallVec<i32, 2> = SmallVec::from_small_vec(big);
        assert!(!target.is_inline());
        assert_eq!(&*target, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn with_capacity() {
        let inline: SmallVec<i32, 8> = SmallVec::with_capacity(3);
        assert!(inline.is_inline());

        let heap: SmallVec<i32, 2> = SmallVec::with_capacity(10);
        assert!(!heap.is_inline());
        assert!(heap.capacity() >= 10);
    }

    #[test]
    fn conversions_to_and_from_vec() {
        let v: SmallVec<i32, 4> = SmallVec::from([1, 2, 3]);
        let heap: Vec<i32> = Vec::from(v);
        assert_eq!(heap, vec![1, 2, 3]);

        let v: SmallVec<i32, 2> = SmallVec::from([1, 2, 3]);
        let heap: Vec<i32> = Vec::from(v);
        assert_eq!(heap, vec![1, 2, 3]);

        let boxed: Box<[i32]> = Box::from(SmallVec::<i32, 4>::from([1, 2]));
        assert_eq!(&*boxed, &[1, 2]);
    }

    #[test]
    fn try_into_array() {
        let v: SmallVec<i32, 4> = SmallVec::from([1, 2, 3]);
        let array: [i32; 3] = v.try_into().unwrap();
        assert_eq!(array, [1, 2, 3]);

        let v: SmallVec<i32, 4> = SmallVec::from([1, 2, 3]);
        assert!(<[i32; 2]>::try_from(v).is_err());

        let v: SmallVec<i32, 2> = SmallVec::from([1, 2, 3]);
        let array: [i32; 3] = v.try_into().unwrap();
        assert_eq!(array, [1, 2, 3]);
    }

    #[test]
    fn from_iterator_collects_and_spills() {
        let inline: SmallVec<i32, 8> = (1..=3).collect();
        assert!(inline.is_inline());

        let heap: SmallVec<i32, 2> = (1..=5).collect();
        assert!(!heap.is_inline());
        assert_eq!(&*heap, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn extend_trait() {
        let mut v: SmallVec<i32, 4> = SmallVec::from([1]);
        v.extend([2, 3, 4, 5]);
        assert_eq!(&*v, &[1, 2, 3, 4, 5]);
        v.extend([6, 7].iter());
        assert_eq!(&*v, &[1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn comparisons_across_capacities_and_types() {
        let a: SmallVec<i32, 4> = SmallVec::from([1, 2, 3]);
        let b: SmallVec<i32, 8> = SmallVec::from([1, 2, 3]);
        assert_eq!(a, b);
        assert_eq!(a, [1, 2, 3]);
        assert_eq!(a, &[1, 2, 3][..]);
        assert_eq!(a, vec![1, 2, 3]);
        assert_eq!(vec![1, 2, 3], a);
        assert_eq!(a, *alloc::borrow::Cow::Borrowed(&[1, 2, 3][..]));

        let c: SmallVec<i32, 4> = SmallVec::from([4]);
        assert_ne!(a, c);
        assert!(a < c);
    }

    #[test]
    fn deref_and_slice_methods() {
        let v: SmallVec<i32, 8> = SmallVec::from([3, 1, 2]);
        assert_eq!(v.iter().copied().max(), Some(3));
        assert_eq!(v.first(), Some(&3));
        assert_eq!(&v[1..], &[1, 2]);
    }

    #[test]
    fn debug_and_default() {
        let v: SmallVec<i32, 4> = SmallVec::from([1, 2, 3]);
        assert_eq!(alloc::format!("{v:?}"), "[1, 2, 3]");

        let d: SmallVec<i32, 4> = Default::default();
        assert!(d.is_empty());
    }

    #[test]
    fn hash_matches_slice() {
        use core::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let v: SmallVec<i32, 4> = SmallVec::from([1, 2, 3]);
        let mut h1 = DefaultHasher::new();
        v.hash(&mut h1);

        let mut h2 = DefaultHasher::new();
        [1, 2, 3].hash(&mut h2);

        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_is_flat_sequence() {
        let v: SmallVec<i32, 4> = SmallVec::from([1, 2, 3]);
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, "[1,2,3]");

        let back: SmallVec<i32, 4> = serde_json::from_str(&json).unwrap();
        assert!(back.is_inline());
        assert_eq!(back, v);

        let long = "[1,2,3,4,5,6]";
        let back: SmallVec<i32, 2> = serde_json::from_str(long).unwrap();
        assert!(!back.is_inline());
        assert_eq!(&*back, &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    #[cfg(feature = "std")]
    fn io_write() {
        use std::io::Write;

        let mut v: SmallVec<u8, 4> = SmallVec::new();
        v.write_all(b"hello ").unwrap();
        write!(v, "world").unwrap();
        assert_eq!(&*v, b"hello world");
        assert!(!v.is_inline());
    }

    #[test]
    #[cfg(feature = "bytes")]
    fn buf_mut() {
        use bytes::{BufMut, BytesMut};

        let mut v: SmallVec<u8, 4> = SmallVec::new();
        v.put_slice(b"ab");
        assert!(v.is_inline());
        v.put_u8(b'c');
        // `put` exercises `chunk_mut`/`advance_mut`.
        v.put(BytesMut::from(&b"de"[..]));
        assert_eq!(&*v, b"abcde");

        let mut v: SmallVec<u8, 8> = SmallVec::new();
        v.put_u32(0x0102_0304);
        assert_eq!(&*v, &[1, 2, 3, 4]);
    }

    // -----------------------------------------------------------------------
    // Additional coverage
    // -----------------------------------------------------------------------

    #[test]
    fn inline_size_is_n() {
        assert_eq!(<SmallVec<i32, 4>>::inline_size(), 4);
        assert_eq!(<SmallVec<i32, 0>>::inline_size(), 0);
    }

    #[test]
    fn capacity_invariants_across_operations() {
        let mut v: SmallVec<i32, 4> = SmallVec::new();
        v.reserve(2);
        assert!(v.capacity() >= v.len() + 2);
        v.extend([1, 2]);
        v.reserve(10);
        assert!(v.capacity() >= v.len() + 10);

        let mut e: SmallVec<i32, 2> = SmallVec::new();
        e.reserve_exact(8);
        assert!(e.capacity() >= 8);
    }

    #[test]
    fn push_growth_loop_preserves_all_elements() {
        let mut v: SmallVec<usize, 4> = SmallVec::new();
        for i in 0..1000 {
            v.push(i);
        }
        assert!(!v.is_inline());
        assert_eq!(v.len(), 1000);
        for (i, x) in v.iter().enumerate() {
            assert_eq!(*x, i);
        }
    }

    #[test]
    fn insert_at_end_and_into_empty() {
        let mut v: SmallVec<i32, 4> = SmallVec::new();
        v.insert(0, 1);
        assert_eq!(&*v, &[1]);
        v.insert(1, 2);
        assert_eq!(&*v, &[1, 2]);
    }

    #[test]
    #[should_panic(expected = "range out of bounds")]
    fn extend_from_within_out_of_bounds_panics() {
        let mut v: SmallVec<i32, 8> = SmallVec::from([1, 2]);
        v.extend_from_within(0..3);
    }

    #[test]
    fn extend_from_within_empty_range_is_noop() {
        let mut v: SmallVec<i32, 4> = SmallVec::from([1, 2]);
        v.extend_from_within(1..1);
        assert!(v.is_inline());
        assert_eq!(&*v, &[1, 2]);
    }

    #[test]
    #[should_panic]
    fn split_off_out_of_bounds_panics() {
        let mut v: SmallVec<i32, 4> = SmallVec::from([1, 2]);
        let _ = v.split_off(3);
    }

    #[test]
    fn split_off_boundaries() {
        let mut v: SmallVec<i32, 4> = SmallVec::from([1, 2, 3]);
        let empty = v.split_off(3);
        assert!(empty.is_empty());
        assert_eq!(&*v, &[1, 2, 3]);

        let mut w: SmallVec<i32, 4> = SmallVec::from([1, 2, 3]);
        let all = w.split_off(0);
        assert!(w.is_empty());
        assert_eq!(&*all, &[1, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn drain_out_of_bounds_panics() {
        let mut v: SmallVec<i32, 8> = SmallVec::from([1, 2]);
        let _ = v.drain(0..3);
    }

    #[test]
    fn drain_as_slice_and_double_ended() {
        let mut v: SmallVec<i32, 8> = SmallVec::from([1, 2, 3, 4]);
        {
            let mut d = v.drain(1..4);
            assert_eq!(d.as_slice(), &[2, 3, 4]);
            assert_eq!(d.next(), Some(2));
            assert_eq!(d.next_back(), Some(4));
            assert_eq!(d.as_slice(), &[3]);
            assert_eq!(d.next(), Some(3));
            assert_eq!(d.next(), None);
        }
        assert_eq!(&*v, &[1]);
    }

    #[test]
    fn clear_then_shrink_to_fit_moves_inline() {
        let mut v: SmallVec<i32, 4> = SmallVec::from([1, 2, 3, 4, 5]);
        assert!(!v.is_inline());
        v.clear();
        assert!(!v.is_inline());
        v.shrink_to_fit();
        assert!(v.is_inline());
    }

    #[test]
    fn shrink_to_keeps_min_capacity_on_heap() {
        let mut v: SmallVec<i32, 2> = SmallVec::from([1, 2, 3, 4]);
        assert!(!v.is_inline());
        v.shrink_to(3);
        assert!(!v.is_inline());
        assert!(v.capacity() >= 3);
        assert_eq!(&*v, &[1, 2, 3, 4]);
    }

    #[test]
    fn zero_capacity_vector_operations() {
        let mut v: SmallVec<i32, 0> = SmallVec::new();
        assert!(v.is_inline());
        assert_eq!(v.capacity(), 0);
        v.reserve(0);
        assert!(v.is_inline());
        v.push(1);
        assert!(!v.is_inline());
        v.insert(0, 0);
        assert_eq!(&*v, &[0, 1]);
        v.shrink_to_fit();
        v.truncate(1);
        assert_eq!(&*v, &[0]);
    }

    #[test]
    fn dedup_by_preserves_order_and_argument_order() {
        // `same_bucket` is called with (current, previous), like std.
        let mut v: SmallVec<i32, 8> = SmallVec::from([1, 2, 2, 3, 3, 3, 4]);
        let mut calls = Vec::new();
        v.dedup_by(|a, b| {
            calls.push((*a, *b));
            a == b
        });
        assert_eq!(&*v, &[1, 2, 3, 4]);
        assert_eq!(calls[0], (2, 1));
        assert_eq!(calls[1], (2, 2));
        assert_eq!(calls[2], (3, 2));
    }

    #[test]
    fn dedup_on_empty_and_single() {
        let mut empty: SmallVec<i32, 4> = SmallVec::new();
        empty.dedup();
        assert!(empty.is_empty());

        let mut one: SmallVec<i32, 4> = SmallVec::from([1]);
        one.dedup();
        assert_eq!(&*one, &[1]);
    }

    #[test]
    fn retain_removes_nothing_and_everything() {
        let mut keep: SmallVec<i32, 8> = SmallVec::from([1, 2, 3]);
        keep.retain(|_| true);
        assert_eq!(&*keep, &[1, 2, 3]);

        let mut drop: SmallVec<i32, 8> = SmallVec::from([1, 2, 3]);
        drop.retain(|_| false);
        assert!(drop.is_empty());
    }

    #[test]
    fn try_from_error_preserves_contents() {
        let v: SmallVec<i32, 4> = SmallVec::from([1, 2, 3]);
        let err = <[i32; 2]>::try_from(v).unwrap_err();
        assert_eq!(&*err, &[1, 2, 3]);

        let v: SmallVec<i32, 2> = SmallVec::from([1, 2, 3]);
        assert!(!v.is_inline());
        let err = <[i32; 2]>::try_from(v).unwrap_err();
        assert_eq!(&*err, &[1, 2, 3]);
    }

    #[test]
    fn into_vec_preserves_heap_allocation() {
        let mut v: SmallVec<i32, 2> = SmallVec::new();
        v.extend([1, 2, 3, 4]);
        assert!(!v.is_inline());
        let ptr = v.as_ptr();
        let vec: Vec<i32> = Vec::from(v);
        assert_eq!(vec.as_ptr(), ptr);
        assert_eq!(vec, vec![1, 2, 3, 4]);
    }

    #[test]
    fn hash_inline_and_heap_agree() {
        use core::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let inline: SmallVec<i32, 8> = SmallVec::from([1, 2, 3]);
        let heap: SmallVec<i32, 1> = SmallVec::from([1, 2, 3]);
        assert!(inline.is_inline());
        assert!(!heap.is_inline());

        let mut h1 = DefaultHasher::new();
        inline.hash(&mut h1);
        let mut h2 = DefaultHasher::new();
        heap.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn vecdeque_equality() {
        use alloc::collections::VecDeque;

        let v: SmallVec<i32, 2> = SmallVec::from([1, 2, 3, 4]);
        assert!(!v.is_inline());
        let mut dq: VecDeque<i32> = VecDeque::new();
        dq.extend([1, 2, 3, 4]);
        assert_eq!(dq, v);

        // A deque whose contents wrap around still compares equal.
        let mut wrapped: VecDeque<i32> = VecDeque::new();
        wrapped.extend([9, 1, 2, 3, 4]);
        wrapped.pop_front();
        assert_eq!(wrapped, v);

        let mut shorter: VecDeque<i32> = VecDeque::new();
        shorter.extend([1, 2, 3]);
        assert_ne!(shorter, v);
    }

    #[test]
    fn cow_equality() {
        use alloc::borrow::Cow;

        let v: SmallVec<i32, 4> = SmallVec::from([1, 2, 3]);
        let borrowed: Cow<'_, [i32]> = Cow::Borrowed(&[1, 2, 3]);
        assert_eq!(borrowed, v);
        assert_eq!(v, borrowed);

        let mismatched: Cow<'_, [i32]> = Cow::Borrowed(&[1, 2]);
        assert_ne!(mismatched, v);
    }

    #[test]
    fn cross_capacity_ordering() {
        let a: SmallVec<i32, 2> = SmallVec::from([1, 2]);
        let b: SmallVec<i32, 8> = SmallVec::from([1, 3]);
        assert!(a < b);
        assert!(b > a);
        let c: SmallVec<i32, 8> = SmallVec::from([1, 2]);
        assert_eq!(a.partial_cmp(&c), Some(Ordering::Equal));
    }

    #[test]
    fn into_iter_exact_size_and_clone() {
        let v: SmallVec<i32, 4> = SmallVec::from([1, 2, 3]);
        let mut it = v.into_iter();
        assert_eq!(it.len(), 3);
        assert_eq!(it.size_hint(), (3, Some(3)));
        let mut cloned = it.clone();
        assert_eq!(cloned.next(), Some(1));
        // The original is unaffected by the clone.
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn default_is_inline_empty() {
        let v: SmallVec<i32, 4> = Default::default();
        assert!(v.is_inline());
        assert!(v.is_empty());
    }

    #[test]
    fn debug_inline_and_heap() {
        let inline: SmallVec<i32, 4> = SmallVec::from([1, 2, 3]);
        assert_eq!(alloc::format!("{inline:?}"), "[1, 2, 3]");
        let heap: SmallVec<i32, 1> = SmallVec::from([1, 2, 3]);
        assert_eq!(alloc::format!("{heap:?}"), "[1, 2, 3]");
    }

    #[test]
    #[cfg(feature = "bytes")]
    #[should_panic(expected = "advance out of bounds")]
    fn buf_mut_advance_out_of_bounds_panics() {
        use bytes::BufMut;

        let mut v: SmallVec<u8, 4> = SmallVec::new();
        // SAFETY: this deliberately violates the `BufMut` contract to check
        // the bounds assertion inside `advance_mut`.
        unsafe { v.advance_mut(10) };
    }

    #[test]
    #[cfg(feature = "bytes")]
    fn buf_mut_chunk_mut_grows_when_full() {
        use bytes::BufMut;

        let mut v: SmallVec<u8, 4> = SmallVec::new();
        v.put_slice(b"abcd");
        assert!(v.is_inline());
        // `chunk_mut` must grow (spill) so there is always somewhere to write.
        let chunk = v.chunk_mut();
        assert!(chunk.len() > 0);
        assert!(!v.is_inline());
        assert_eq!(&*v, b"abcd");
    }
}
