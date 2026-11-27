//! Reproducer: a `DefaultIsZeroes` type whose `Default` has an uninitialized byte.
//!
//! This is legal safe code: `MaybeUninit<u8>` may be uninitialized, `Copy` is satisfied,
//! and `DefaultIsZeroes` is a safe marker trait. Zeroizing an array of such a type must
//! copy `Z::default()` into each element (as the pre-existing per-element path did);
//! reading that byte as a `u8` is undefined behavior.
//!
//! The size-1 fast path in the novel `volatile_set` does exactly that:
//!
//! ```text
//! if size_of::<T>() == 1 {
//!     let byte = unsafe { *(&src as *const T).cast::<u8>() }; // read of `src`
//!     ...
//! }
//! ```
//!
//! Before the fix `bulk_path_reads_uninitialized_size1_default` used no
//! `unsafe` itself and was rejected by Miri. The size-1 fast path has since
//! been removed, so all of these tests are regression tests expected to pass.
//! The size-2 contrast test shows that the per-element typed `write_volatile`
//! path is fine (copying uninitialized bytes is allowed).

use core::mem::MaybeUninit;

use zeroize::{DefaultIsZeroes, Zeroize};

#[derive(Copy, Clone)]
struct UninitByte(#[allow(dead_code)] MaybeUninit<u8>);

impl Default for UninitByte {
    fn default() -> Self {
        // A valid `UninitByte` whose single byte is uninitialized.
        Self(MaybeUninit::uninit())
    }
}

impl DefaultIsZeroes for UninitByte {}

#[derive(Copy, Clone)]
struct UninitWord(#[allow(dead_code)] MaybeUninit<[u8; 2]>);

impl Default for UninitWord {
    fn default() -> Self {
        Self(MaybeUninit::uninit())
    }
}

impl DefaultIsZeroes for UninitWord {}

/// UB: `volatile_set`'s size-1 branch reads the uninitialized byte of `default()`.
#[test]
fn bulk_path_reads_uninitialized_size1_default() {
    let mut arr = [UninitByte(MaybeUninit::new(0xAA)); 8];
    // Safe call; the UB happens inside the crate's bulk fast path.
    arr.zeroize();
}

/// Control: the same uninitialized default with `size_of == 2` goes through the
/// per-element `write_volatile` path, which only copies bytes and is defined.
#[test]
fn size2_uninitialized_default_is_copied_without_reading() {
    let mut arr = [UninitWord(MaybeUninit::new([0xAA, 0xBB])); 4];
    arr.zeroize();
}

/// The same UB is reachable through the `Vec<Z>` container impl.
#[test]
fn bulk_path_via_vec() {
    let mut v = vec![UninitByte(MaybeUninit::new(0xAA)); 8];
    v.zeroize();
}

/// ...and through the `Box<[Z]>` container impl.
#[test]
fn bulk_path_via_box() {
    let mut b: Box<[UninitByte]> = vec![UninitByte(MaybeUninit::new(0xAA)); 8].into_boxed_slice();
    b.zeroize();
}
