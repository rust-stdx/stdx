//! Exact-accounting and edge-case tests for the counting global allocator.

use std::{
    alloc::{GlobalAlloc, Layout},
    panic::catch_unwind,
};

use alloc_counter::{Counters, counters, measure};

/// Allocates through the global allocator directly, so the expected counters
/// do not depend on what `Vec`/`Box` choose to do.
#[test]
fn alloc_realloc_dealloc_are_exact() {
    let layout = Layout::from_size_align(64, 8).unwrap();
    let grown = Layout::from_size_align(128, 8).unwrap();

    let before = counters();
    let ptr = std::hint::black_box(unsafe { std::alloc::alloc(layout) });
    assert!(!ptr.is_null());
    let after_alloc_snapshot = counters();
    let after_alloc = after_alloc_snapshot.delta(before);
    assert_eq!(
        after_alloc,
        Counters {
            calls: 1,
            bytes: 64,
            live_bytes: 64,
            live_allocations: 1,
        }
    );

    let ptr = std::hint::black_box(unsafe { std::alloc::realloc(ptr, layout, grown.size()) });
    assert!(!ptr.is_null());
    let after_realloc_snapshot = counters();
    let after_realloc = after_realloc_snapshot.delta(after_alloc_snapshot);
    assert_eq!(
        after_realloc,
        Counters {
            calls: 1,
            bytes: 128,
            live_bytes: 64, // 128 - 64: no new live allocation
            live_allocations: 0,
        }
    );

    unsafe { std::alloc::dealloc(std::hint::black_box(ptr), grown) };
    let after_dealloc = counters().delta(after_realloc_snapshot);
    assert_eq!(
        after_dealloc,
        Counters {
            calls: 0,
            bytes: 0,
            live_bytes: -128,
            live_allocations: -1,
        }
    );
}

#[test]
fn alloc_zeroed_is_counted_and_zeroed() {
    let layout = Layout::from_size_align(4096, 16).unwrap();
    let before = counters();
    let ptr = std::hint::black_box(unsafe { std::alloc::alloc_zeroed(layout) });
    assert!(!ptr.is_null());
    let delta = counters().delta(before);
    assert_eq!(delta.calls, 1);
    assert_eq!(delta.bytes, 4096);
    assert_eq!(delta.live_bytes, 4096);
    assert_eq!(delta.live_allocations, 1);
    // `System::alloc_zeroed` must return zeroed memory.
    let bytes = unsafe { std::slice::from_raw_parts(ptr, 4096) };
    assert!(bytes.iter().all(|&b| b == 0));
    unsafe { std::alloc::dealloc(std::hint::black_box(ptr), layout) };
}

#[test]
fn a_large_layout_is_fully_counted() {
    const SIZE: usize = 8 * 1024 * 1024;
    let layout = Layout::from_size_align(SIZE, 64).unwrap();
    let before = counters();
    let ptr = std::hint::black_box(unsafe { std::alloc::alloc(layout) });
    assert!(!ptr.is_null());
    let delta = counters().delta(before);
    assert_eq!(delta.calls, 1);
    assert_eq!(delta.bytes, SIZE as u64);
    assert_eq!(delta.live_bytes, SIZE as i64);
    unsafe { std::alloc::dealloc(ptr, layout) };
}

#[test]
fn delta_is_additive_across_nested_measurements() {
    let outer = measure(|| {
        let inner = measure(|| {
            std::hint::black_box(Vec::<u8>::with_capacity(100));
        });
        assert_eq!(inner.1.calls, 1);
        assert_eq!(inner.1.bytes, 100);
        inner.1.calls
    });
    assert_eq!(outer.0, 1);
    assert_eq!(outer.1.calls, 1, "the outer measurement includes the inner one");
}

#[cfg(panic = "unwind")]
#[test]
fn a_panic_inside_measure_still_records_the_allocation() {
    let before = counters();
    let result = catch_unwind(|| {
        measure(|| {
            std::hint::black_box(Vec::<u8>::with_capacity(32));
            panic!("panic inside the measured closure");
        })
    });
    assert!(result.is_err());
    let delta = counters().delta(before);
    assert!(delta.calls >= 1, "the allocation must have been counted");
    assert!(delta.bytes >= 32, "the requested bytes must have been counted");
    assert!(delta.live_bytes >= 32);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn deallocating_on_another_thread_changes_that_threads_gauge() {
    const SIZE: usize = 1024;
    let buffer = std::hint::black_box(Vec::<u8>::with_capacity(SIZE));

    let handle = std::thread::spawn(move || {
        let before = counters();
        drop(buffer);
        counters().delta(before)
    });
    let thread_delta = handle.join().unwrap();

    assert_eq!(thread_delta.calls, 0, "freeing is not an allocation request");
    assert_eq!(
        thread_delta.live_bytes,
        -(SIZE as i64),
        "the freeing thread's gauge goes negative"
    );
    assert_eq!(thread_delta.live_allocations, -1);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn first_allocation_on_a_fresh_thread_is_counted_once() {
    let counted = std::thread::spawn(|| {
        // The `const` thread-local initialiser must not allocate on first use
        // (a lazy TLS would recurse into the global allocator).
        let (value, delta) = measure(|| std::hint::black_box(Vec::<u8>::with_capacity(777)));
        assert_eq!(value.capacity(), 777);
        (delta.calls, delta.bytes, delta.live_allocations)
    })
    .join()
    .unwrap();
    assert_eq!(counted, (1, 777, 1));
}

#[test]
fn the_global_allocator_is_the_counting_one() {
    // `System` here would not update the counters; this pins the wrapper.
    let before = counters();
    let layout = Layout::from_size_align(8, 8).unwrap();
    let ptr = unsafe { std::alloc::System.alloc(layout) };
    unsafe { std::alloc::System.dealloc(ptr, layout) };
    assert_eq!(counters(), before, "System must bypass the counters");
}

/// `Counters::delta` documents that it never panics, but the type has public
/// fields: a caller can pass a larger `earlier`, which underflows.  Debug
/// builds panic (release wraps).  This is a documentation/contract bug, not
/// memory unsafety.
#[cfg(debug_assertions)]
#[cfg(panic = "unwind")]
#[test]
#[should_panic(expected = "attempt to subtract with overflow")]
fn delta_of_a_larger_snapshot_panics_in_debug() {
    let later = Counters::default();
    let earlier = Counters {
        calls: 1,
        ..Counters::default()
    };
    let _ = later.delta(earlier);
}

/// F-5 regression: a *failed* allocation must still count as a request, but
/// must not appear as live memory. The gauges are documented as "currently
/// allocated", so a leak check must not see a phantom allocation.
#[cfg(all(not(miri), target_pointer_width = "64"))]
#[test]
fn failed_allocation_must_not_create_a_live_allocation() {
    // 4 EiB cannot fit in any current user address space (57-bit VA = 128 PiB)
    // while still being a valid `Layout` (size <= isize::MAX).
    let layout = Layout::from_size_align(1usize << 62, 8).unwrap();
    let before = counters();
    let ptr = std::hint::black_box(unsafe { std::alloc::alloc(layout) });
    assert!(ptr.is_null(), "the test needs an allocation that actually fails");
    let delta = counters().delta(before);
    assert_eq!(
        delta.live_allocations, 0,
        "a null return means no allocation is live (got {delta:?})"
    );
    assert_eq!(delta.live_bytes, 0, "a null return means no live bytes (got {delta:?})");
}

/// A failed reallocation leaves the original block live, so only the request
/// counters may move. The gauges must stay pinned to the original allocation.
#[cfg(all(not(miri), target_pointer_width = "64"))]
#[test]
fn failed_reallocation_keeps_the_original_allocation_live() {
    let layout = Layout::from_size_align(64, 8).unwrap();
    let ptr = std::hint::black_box(unsafe { std::alloc::alloc(layout) });
    assert!(!ptr.is_null());
    let before = counters();

    // 4 EiB cannot succeed on any current target, like the failed-alloc test.
    let huge = 1usize << 62;
    let new_ptr = std::hint::black_box(unsafe { std::alloc::realloc(ptr, layout, huge) });
    assert!(new_ptr.is_null(), "the test needs a reallocation that actually fails");

    let delta = counters().delta(before);
    assert_eq!(delta.calls, 1, "a failed reallocation is still a request");
    assert_eq!(delta.bytes, huge as u64);
    assert_eq!(delta.live_allocations, 0, "the original block is still live (got {delta:?})");
    assert_eq!(delta.live_bytes, 0, "the original block size did not change (got {delta:?})");

    unsafe { std::alloc::dealloc(std::hint::black_box(ptr), layout) };
}
