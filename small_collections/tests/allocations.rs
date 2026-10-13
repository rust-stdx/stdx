//! Allocation regression tests for [`SmallString`].
//!
//! Construction must never touch the heap while the contents fit in the inline
//! capacity — including the lossy UTF-8 path, which must not create an
//! intermediate `String`. Counting comes from `alloc_counter`, which attributes
//! allocations to the current thread, so these assertions are not perturbed by
//! parallel tests.

use alloc_counter::measure;
use small_collections::SmallString;

#[test]
fn from_utf8_lossy_valid_short_is_allocation_free() {
    let (s, counters) = measure(|| SmallString::<16>::from_utf8_lossy(b"hello"));
    assert!(s.is_inline());
    assert_eq!(s.as_str(), "hello");
    assert_eq!(counters.calls, 0, "valid lossy conversion allocated {} time(s)", counters.calls);
}

#[test]
fn from_utf8_lossy_invalid_short_is_allocation_free() {
    // Invalid input previously forced the owned `Cow` path, allocating a
    // temporary `String` that was dropped after being copied inline.
    let (s, counters) = measure(|| SmallString::<16>::from_utf8_lossy(&[0xFF, 0xFE]));
    assert!(s.is_inline());
    assert_eq!(s.as_str(), "\u{FFFD}\u{FFFD}");
    assert_eq!(
        counters.calls, 0,
        "lossy conversion of invalid input allocated {} time(s)",
        counters.calls
    );
}

#[test]
fn from_utf8_lossy_spills_exactly_once() {
    let bytes = b"hello world this is long";
    let (s, counters) = measure(|| SmallString::<4>::from_utf8_lossy(bytes));
    assert!(!s.is_inline());
    assert_eq!(s.as_str(), "hello world this is long");
    assert_eq!(
        counters.calls, 1,
        "spilling should allocate exactly once, got {}",
        counters.calls
    );
}
