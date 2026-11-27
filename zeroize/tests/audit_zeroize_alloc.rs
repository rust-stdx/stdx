//! Allocation-level audit: verify that the *original heap buffer* is fully zeroed
//! (including all spare capacity) when it is finally deallocated, for `Vec`, `String`,
//! `CString` and `Box<[T]>`.

use core::{
    alloc::{GlobalAlloc, Layout},
    ptr,
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering::Relaxed},
};
use std::alloc::System;
#[cfg(feature = "std")]
use std::ffi::CString;

use zeroize::{Zeroize, Zeroizing};

/// The next allocation whose deallocation must be all-zero.
static WATCH_PTR: AtomicPtr<u8> = AtomicPtr::new(ptr::null_mut());
static WATCH_ARMED: AtomicUsize = AtomicUsize::new(0);

struct WatchAlloc;

unsafe impl GlobalAlloc for WatchAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, p: *mut u8, layout: Layout) {
        if WATCH_ARMED.load(Relaxed) == 1 && p == WATCH_PTR.load(Relaxed) {
            // `layout.size()` is the allocation size (capacity for Vec/String/CString), so
            // this checks every byte of the buffer, not just its logical length.
            for i in 0..layout.size() {
                // SAFETY: the allocation is still valid in `dealloc`.
                let b = unsafe { ptr::read_volatile(p.add(i)) };
                assert_eq!(b, 0, "byte {i} of {} in the freed buffer is not zero", layout.size());
            }
            WATCH_ARMED.store(0, Relaxed);
        }
        unsafe { System.dealloc(p, layout) }
    }
}

#[global_allocator]
static ALLOC: WatchAlloc = WatchAlloc;

fn arm(p: *mut u8) {
    WATCH_PTR.store(p, Relaxed);
    WATCH_ARMED.store(1, Relaxed);
}

fn assert_observed() {
    assert_eq!(WATCH_ARMED.load(Relaxed), 0, "watched deallocation was not observed");
}

#[test]
fn heap_buffers_zero_on_dealloc() {
    // Vec<u8> with spare capacity.
    {
        let mut v: Vec<u8> = Vec::with_capacity(97);
        v.extend_from_slice(&[0xAAu8; 13]);
        for m in v.spare_capacity_mut() {
            m.write(0xAA);
        }
        arm(v.as_mut_ptr());
        v.zeroize();
        drop(v);
    }
    assert_observed();

    // String with spare capacity.
    {
        let mut s = String::with_capacity(97);
        s.push_str("a secret string");
        for m in unsafe { s.as_mut_vec() }.spare_capacity_mut() {
            m.write(0xA5);
        }
        arm(unsafe { s.as_mut_vec() }.as_mut_ptr());
        s.zeroize();
        drop(s);
    }
    assert_observed();

    // CString: the original buffer (and its full capacity) must be zeroed even though
    // `zeroize` swaps in a fresh one-byte CString. The impl is behind `std`.
    #[cfg(feature = "std")]
    {
        let mut owned = CString::new("a secret c string").unwrap();
        arm(owned.as_bytes_with_nul().as_ptr() as *mut u8);
        owned.zeroize();
        drop(owned);
    }
    assert_observed();

    // Zeroizing<Vec<u8>> on drop.
    {
        let mut v: Zeroizing<Vec<u8>> = Zeroizing::new(Vec::with_capacity(64));
        v.extend_from_slice(&[0xAAu8; 9]);
        for m in v.spare_capacity_mut() {
            m.write(0xAA);
        }
        arm(v.as_mut_ptr());
        drop(v);
    }
    assert_observed();

    // Box<[T]>.
    {
        let mut b: Box<[u64]> = vec![0xAAAA_AAAA_AAAA_AAAAu64; 13].into_boxed_slice();
        arm(b.as_mut_ptr().cast::<u8>());
        b.zeroize();
        drop(b);
    }
    assert_observed();
}
