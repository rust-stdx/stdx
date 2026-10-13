//! Allocation regression tests.
//!
//! Embedded lookups must stay allocation-free. Debug (file system) lookups are
//! allowed a small, fixed number of allocations; they must not grow with the
//! number of files or the file size.

use alloc_counter::measure;
use embed::Embed;

#[derive(Embed)]
#[folder = "tests/fixtures/assets"]
struct Asset;

/// `true` when the generated code uses the embedded (static) table.
fn is_embedded() -> bool {
    !cfg!(debug_assertions) || cfg!(feature = "debug-embed")
}

#[test]
fn embedded_get_and_iter_do_not_allocate() {
    if !is_embedded() {
        return;
    }

    // Warm up lazily initialized state.
    std::hint::black_box(Asset::get("index.html"));
    std::hint::black_box(Asset::iter().count());

    let ((), counters) = measure(|| {
        for _ in 0..1_000 {
            std::hint::black_box(Asset::get("index.html"));
        }
    });
    assert_eq!(counters.calls, 0, "embedded get must not allocate");
    assert_eq!(counters.bytes, 0, "embedded get must not allocate");

    let ((), counters) = measure(|| {
        for _ in 0..1_000 {
            std::hint::black_box(Asset::iter().count());
        }
    });
    assert_eq!(counters.calls, 0, "embedded iter must not allocate");
}

#[test]
fn debug_get_allocations_are_bounded() {
    if is_embedded() {
        return;
    }

    // Warm up the folder root, the matcher and the hash cache.
    std::hint::black_box(Asset::get("index.html"));

    let ((), counters) = measure(|| {
        for _ in 0..1_000 {
            std::hint::black_box(Asset::get("index.html"));
        }
    });
    // A bounded, small number of allocations per lookup (path join and
    // canonicalization, opening the file and reading it). The exact count
    // varies a little per platform, hence the margin.
    assert!(
        counters.calls <= 10_000,
        "debug get allocated {} times for 1000 calls",
        counters.calls
    );
}

#[test]
fn debug_get_does_not_allocate_for_invalid_or_missing_paths() {
    if is_embedded() {
        return;
    }

    std::hint::black_box(Asset::get("index.html"));

    // Invalid keys are rejected before touching the file system.
    let ((), invalid) = measure(|| {
        for _ in 0..1_000 {
            std::hint::black_box(Asset::get("../Cargo.toml"));
        }
    });
    assert_eq!(invalid.calls, 0, "rejecting an invalid key must not allocate");

    let ((), missing) = measure(|| {
        for _ in 0..1_000 {
            std::hint::black_box(Asset::get("missing.html"));
        }
    });
    assert!(
        missing.calls <= 4_000,
        "a missing file allocated {} times for 1000 calls",
        missing.calls
    );
}
