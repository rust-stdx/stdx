//! Per-thread allocation counting for tests.
//!
//! Linking this crate installs a counting [`GlobalAlloc`](std::alloc::GlobalAlloc)
//! wrapper around [`System`](std::alloc::System). Every test binary that uses it
//! can then measure how many allocations a piece of code performs:
//!
//! ```
//! use alloc_counter::measure;
//!
//! let (value, counters) = measure(|| 1u32 + 1);
//! assert_eq!(value, 2);
//! assert_eq!(counters.calls, 0);
//! ```
//!
//! # Per-thread counting
//!
//! Counters are always on and scoped to the thread that performs the
//! allocation. This keeps measurements exact even when the test harness runs
//! other tests, and their bookkeeping, in parallel: an allocation made by
//! another thread is never attributed to the measuring one. The trade-off is
//! that only the calling thread is observed, so work that must be measured has
//! to run on it.
//!
//! [`measure`] returns the delta across a closure; [`counters`] returns a
//! snapshot for manual before/after comparisons. The `calls` and `bytes`
//! counters are monotonic totals of allocation requests (allocation, zeroed
//! allocation and reallocation); `live_bytes` and `live_allocations` track
//! memory that is still allocated and can be used to detect leaks.
//!
//! The `live_*` gauges assume that an allocation is released by the thread
//! that made it. Freeing on another thread changes that thread's gauge instead,
//! so it can go negative; scope deltas taken on a single thread are exact.
//!
//! # Global allocator constraint
//!
//! Rust allows a single `#[global_allocator]` per binary, and this crate
//! provides one. A test binary that links `alloc_counter` therefore cannot also
//! install a custom allocator (for example one that verifies zeroization).
//! Test binaries that do not use this crate are unaffected: the allocator is
//! only linked in when the crate is referenced.
//!
//! Nothing in this crate panics. Allocations made while a thread's counters
//! are unavailable (during thread-local destruction) are simply not counted.

use core::cell::Cell;
use std::alloc::{GlobalAlloc, Layout, System};

/// Allocation counters for a single thread.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Counters {
    /// Number of allocation requests: allocations, zeroed allocations and
    /// reallocations.
    pub calls: u64,
    /// Total number of bytes requested by those allocations.
    pub bytes: u64,
    /// Bytes currently allocated and not yet freed.
    pub live_bytes: i64,
    /// Number of allocations currently live.
    pub live_allocations: i64,
}

impl Counters {
    /// The all-zero value.
    const ZERO: Counters = Counters {
        calls: 0,
        bytes: 0,
        live_bytes: 0,
        live_allocations: 0,
    };

    /// Returns the counters between `earlier` and `self`.
    ///
    /// # Panics
    ///
    /// Never panics: `calls` and `bytes` are monotonic, and the `live_*`
    /// gauges may legitimately decrease (or go negative across threads).
    #[must_use]
    pub fn delta(self, earlier: Counters) -> Counters {
        Counters {
            calls: self.calls - earlier.calls,
            bytes: self.bytes - earlier.bytes,
            live_bytes: self.live_bytes - earlier.live_bytes,
            live_allocations: self.live_allocations - earlier.live_allocations,
        }
    }
}

thread_local! {
    // The `const` initialiser is load-bearing: the global allocator reads this
    // on every allocation, so a lazily-initialised thread-local would allocate
    // on first access and recurse into the allocator.
    static COUNTERS: Cell<Counters> = const { Cell::new(Counters::ZERO) };
}

/// Returns a snapshot of the current thread's allocation counters.
pub fn counters() -> Counters {
    COUNTERS.try_with(Cell::get).unwrap_or_default()
}

/// Runs `f` and returns its result along with the allocation counters the
/// current thread accumulated during the call.
///
/// Allocations made by other threads are not counted.
pub fn measure<T>(f: impl FnOnce() -> T) -> (T, Counters) {
    let before = counters();
    let value = f();
    (value, counters().delta(before))
}

/// Applies `f` to the current thread's counters, if they are available.
///
/// Safe to call from inside the global allocator: accessing a `const`
/// thread-local never allocates. Threads whose thread-locals are being torn
/// down are skipped, since allocation may still happen during destruction.
fn update(f: impl FnOnce(&mut Counters)) {
    let _ = COUNTERS.try_with(|cell| {
        let mut counters = cell.get();
        f(&mut counters);
        cell.set(counters);
    });
}

fn record_allocation(size: usize) {
    update(|counters| {
        counters.calls += 1;
        counters.bytes += size as u64;
        counters.live_bytes += size as i64;
        counters.live_allocations += 1;
    });
}

fn record_reallocation(old_size: usize, new_size: usize) {
    update(|counters| {
        counters.calls += 1;
        counters.bytes += new_size as u64;
        counters.live_bytes += new_size as i64 - old_size as i64;
    });
}

fn record_deallocation(size: usize) {
    update(|counters| {
        counters.live_bytes -= size as i64;
        counters.live_allocations -= 1;
    });
}

struct CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record_allocation(layout.size());
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        record_allocation(layout.size());
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        record_reallocation(layout.size(), new_size);
        unsafe { System.realloc(ptr, layout, new_size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        record_deallocation(layout.size());
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;
