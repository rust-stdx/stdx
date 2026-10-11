//! Allocation and resource-leak regression tests.
//!
//! These live in their own integration-test binary because they install a
//! counting global allocator, which is process-wide. They therefore run as a
//! single test.

use std::{
    alloc::{GlobalAlloc, Layout, System},
    env, fs,
    sync::atomic::{AtomicIsize, AtomicU64, Ordering},
};

struct Counting;

static ALLOCS: AtomicU64 = AtomicU64::new(0);
static LIVE: AtomicIsize = AtomicIsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        LIVE.fetch_add(1, Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        LIVE.fetch_sub(1, Ordering::Relaxed);
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

fn count_allocs<T>(f: impl FnOnce() -> T) -> (T, u64) {
    let before = ALLOCS.load(Ordering::Relaxed);
    let out = f();
    let after = ALLOCS.load(Ordering::Relaxed);
    (out, after - before)
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
    let n = 200usize;
    let content: String = (0..n).map(|i| format!("DOTENV_LEAK_{i}=v{i}\n")).collect();
    let dir = env::temp_dir().join(format!("dotenv_alloc_test_{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join(".env"), &content).unwrap();

    let old = env::current_dir().unwrap();

    // Warm up lazily-initialised statics.
    env::set_current_dir(&dir).unwrap();
    dotenv::load().unwrap();

    let live_before = LIVE.load(Ordering::Relaxed);
    #[cfg(target_os = "linux")]
    let fds_before = fd_count();

    for _ in 0..100 {
        env::set_current_dir(&dir).unwrap();
        dotenv::load().unwrap();
    }

    let live_after = LIVE.load(Ordering::Relaxed);
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
