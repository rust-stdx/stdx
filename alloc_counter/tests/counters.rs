//! Tests for the per-thread counting allocator.

use alloc_counter::{counters, measure};

#[test]
fn code_that_does_not_allocate_reports_zero() {
    let (value, c) = measure(|| std::hint::black_box(21u64 * 2));
    assert_eq!(value, 42);
    assert_eq!(c.calls, 0);
    assert_eq!(c.bytes, 0);
    assert_eq!(c.live_bytes, 0);
    assert_eq!(c.live_allocations, 0);
}

#[test]
fn allocations_and_frees_are_counted() {
    const SIZE: usize = 4096;

    let (buffer, c) = measure(|| vec![0u8; SIZE]);
    assert_eq!(c.calls, 1, "the buffer needs a single allocation");
    assert_eq!(c.bytes, SIZE as u64);
    assert_eq!(c.live_bytes, SIZE as i64);
    assert_eq!(c.live_allocations, 1);
    assert_eq!(buffer.len(), SIZE);

    let ((), freed) = measure(move || drop(buffer));
    assert_eq!(freed.calls, 0, "freeing is not an allocation");
    assert_eq!(freed.bytes, 0);
    assert_eq!(freed.live_bytes, -(SIZE as i64));
    assert_eq!(freed.live_allocations, -1);
}

#[test]
fn reallocations_are_counted_without_changing_the_live_count() {
    let mut buffer: Vec<u8> = Vec::with_capacity(16);

    let ((), c) = measure(|| buffer.reserve_exact(4096));
    assert_eq!(c.calls, 1, "growing an existing buffer is one reallocation");
    assert!(c.bytes >= 4096, "reallocation bytes: {}", c.bytes);
    assert_eq!(c.live_allocations, 0, "a reallocation keeps one allocation live");
    assert!(c.live_bytes > 0, "reallocation grew live bytes: {}", c.live_bytes);
    assert!(buffer.capacity() >= 4096);
}

/// A sibling thread allocating must not perturb a measurement on this thread.
///
/// This is the regression that a process-wide counter cannot avoid: when the
/// test harness runs another test (or its result bookkeeping) in parallel, its
/// allocations leak into the measured window.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn allocations_on_other_threads_are_not_counted() {
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };

    let started = Arc::new(AtomicBool::new(false));
    let stop = Arc::new(AtomicBool::new(false));
    let sibling = {
        let started = Arc::clone(&started);
        let stop = Arc::clone(&stop);
        std::thread::spawn(move || {
            let mut first = true;
            while !stop.load(Ordering::Relaxed) {
                std::hint::black_box(Vec::<u64>::with_capacity(64));
                if first {
                    started.store(true, Ordering::Relaxed);
                    first = false;
                }
            }
        })
    };

    // Make sure the sibling really is allocating before measuring.
    while !started.load(Ordering::Relaxed) {
        std::hint::spin_loop();
    }

    let (total, c) = measure(|| {
        let mut total = 0u64;
        for i in 0..1_000_000 {
            total = total.wrapping_add(std::hint::black_box(i));
        }
        std::hint::black_box(total)
    });
    assert!(total > 0);

    stop.store(true, Ordering::Relaxed);
    sibling.join().unwrap();

    assert_eq!(c.calls, 0, "another thread's allocations were counted");
    assert_eq!(c.live_allocations, 0);
}

#[test]
fn counters_reports_a_snapshot() {
    let before = counters();
    let (buffer, c) = measure(|| std::hint::black_box(Vec::<u8>::with_capacity(32)));
    let after = counters();

    assert_eq!(c.calls, 1);
    assert_eq!(buffer.capacity(), 32);
    assert_eq!(after.delta(before).calls, 1);
    assert_eq!(after.calls - before.calls, 1);
}
