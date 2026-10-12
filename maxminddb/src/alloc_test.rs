//! Allocation regression tests.
//!
//! A counting allocator records allocation calls made by the current thread, so
//! hot paths can be asserted to not allocate. Counting is per-thread and opt-in,
//! so parallel tests do not interfere.

use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
    net::IpAddr,
};

use serde::Deserialize;

use super::{Reader, Value};

struct CountingAlloc;

thread_local! {
    static COUNTING: Cell<bool> = const { Cell::new(false) };
    static ALLOCS: Cell<u64> = const { Cell::new(0) };
}

fn note_alloc() {
    // `try_with` returns an error during thread-local destruction, when
    // allocation may still occur; those allocations are simply not counted.
    if COUNTING.try_with(Cell::get).unwrap_or(false) {
        let _ = ALLOCS.try_with(|c| c.set(c.get() + 1));
    }
}

unsafe impl GlobalAlloc for CountingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        note_alloc();
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        note_alloc();
        unsafe { System.realloc(ptr, layout, new_size) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        note_alloc();
        unsafe { System.alloc_zeroed(layout) }
    }
}

#[global_allocator]
static ALLOCATOR: CountingAlloc = CountingAlloc;

fn count_allocations<T>(f: impl FnOnce() -> T) -> (T, u64) {
    let before = ALLOCS.with(Cell::get);
    COUNTING.with(|c| c.set(true));
    let result = f();
    COUNTING.with(|c| c.set(false));
    (result, ALLOCS.with(Cell::get) - before)
}

#[test]
fn lookup_of_a_missing_address_does_not_allocate() {
    let reader = Reader::open_readfile("test-data/test-data/GeoIP2-City-Test.mmdb").unwrap();
    let addr: IpAddr = "203.0.113.99".parse().unwrap();
    // Make sure the address really is a miss.
    assert!(reader.lookup::<Value>(addr).is_err());

    let (result, allocations) = count_allocations(|| reader.lookup::<super::geoip2::City>(addr));
    assert!(result.is_err());
    assert_eq!(allocations, 0, "a lookup miss should not allocate");
}

#[test]
fn borrowed_lookup_does_not_allocate() {
    #[derive(Deserialize)]
    struct Record<'a> {
        iso: &'a str,
    }

    // A record of borrowed data decodes without copying it.
    let mut data = Vec::new();
    super::test_support::enc_val(
        &super::test_support::Val::Map(vec![("iso", super::test_support::Val::Str("US"))]),
        &mut data,
    );
    let db = super::test_support::db_with_data(data, 4);
    let reader = Reader::from_source(db).unwrap();

    let (result, allocations) = count_allocations(|| reader.lookup::<Record>("1.2.3.4".parse().unwrap()));
    assert_eq!(result.unwrap().iso, "US");
    assert_eq!(allocations, 0, "decoding borrowed data should not allocate");
}
