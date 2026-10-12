//! Allocation and resource-leak regression tests.
//!
//! Counting comes from `alloc_counter`, which attributes allocations to the
//! current thread. These tests still live in their own integration-test binary
//! and run as a single test, since they change the process working directory.
//!
//! `env::temp_dir()` and `std::process::id()` are unsupported on wasm32-wasip1,
//! so the filesystem half of the test below only runs on native targets.
#[cfg(not(target_arch = "wasm32"))]
use std::{env, fs};

use alloc_counter::{counters, measure};

fn count_allocs<T>(f: impl FnOnce() -> T) -> (T, u64) {
    let (out, counts) = measure(f);
    (out, counts.calls)
}

#[cfg(target_os = "linux")]
fn fd_count() -> usize {
    fs::read_dir("/proc/self/fd").map(|it| it.count()).unwrap_or(0)
}

#[test]
fn allocations_are_linear_and_no_leaks() {
    // `parse` must not allocate per pair: escape-free values borrow from the
    // source, so only the returned map allocates (a handful of growth
    // reallocations), independent of the number of pairs.
    for &n in &[500usize, 2000, 8000] {
        let input: String = (0..n).map(|i| format!("KEY_{i}=value {i}\n")).collect();
        let (vars, allocs) = count_allocs(|| dotenv::parse(&input).unwrap());
        assert_eq!(vars.len(), n);
        let cap = n as u64 / 50 + 64;
        assert!(allocs <= cap, "parse allocated {allocs} times for {n} pairs (cap {cap})");
    }

    // Loading the same file repeatedly must not leak memory or file
    // descriptors.
    //
    // Native-only: `env::temp_dir()` and `std::process::id()` panic on
    // wasm32-wasip1 ("not supported by WASI yet" / "no pids on this platform").
    #[cfg(not(target_arch = "wasm32"))]
    {
        let n = 200usize;
        let content: String = (0..n).map(|i| format!("DOTENV_LEAK_{i}=v{i}\n")).collect();
        let dir = env::temp_dir().join(format!("dotenv_alloc_test_{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(".env"), &content).unwrap();

        let old = env::current_dir().unwrap();

        // Warm up lazily-initialised statics.
        env::set_current_dir(&dir).unwrap();
        dotenv::load().unwrap();

        let live_before = counters().live_allocations;
        #[cfg(target_os = "linux")]
        let fds_before = fd_count();

        for _ in 0..100 {
            env::set_current_dir(&dir).unwrap();
            dotenv::load().unwrap();
        }

        let live_after = counters().live_allocations;
        #[cfg(target_os = "linux")]
        let fds_after = fd_count();

        env::set_current_dir(&old).unwrap();
        let _ = fs::remove_dir_all(&dir);

        // Allow a small slack for runtime bookkeeping, but no per-iteration growth.
        assert!(
            (live_after - live_before).abs() < 64,
            "load leaked memory: live allocations went from {live_before} to {live_after}"
        );
        #[cfg(target_os = "linux")]
        assert!(
            fds_after <= fds_before + 1,
            "load leaked file descriptors: {fds_before} -> {fds_after}"
        );
    }
}
