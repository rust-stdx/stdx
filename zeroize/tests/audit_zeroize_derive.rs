//! Audit tests for `zeroize_derive`: skipped fields/variants stay untouched, all other
//! fields (including heap containers) are zeroized, and the derived `Drop` runs.

#![cfg(feature = "derive")]

use zeroize::{Zeroize, ZeroizeOnDrop};

#[test]
fn derive_struct_skip_leaves_field_untouched() {
    #[derive(Zeroize)]
    struct S {
        a: [u8; 8],
        #[zeroize(skip)]
        skipped: [u8; 8],
    }

    let mut s = S {
        a: [0xAA; 8],
        skipped: [0x55; 8],
    };
    s.zeroize();
    assert!(s.a.iter().all(|&b| b == 0));
    assert!(s.skipped.iter().all(|&b| b == 0x55), "skipped field was modified");
}

#[test]
fn derive_struct_zeroizes_heap_containers() {
    #[derive(Zeroize)]
    struct S {
        string: String,
        bytes: Vec<u8>,
    }

    let mut s = S {
        string: String::from("top secret"),
        bytes: vec![0xAA; 40],
    };
    s.zeroize();
    assert!(s.string.is_empty());
    assert!(s.bytes.is_empty());
}

#[test]
fn derive_enum_skip_variant_and_field() {
    #[derive(Zeroize)]
    enum E {
        A {
            x: [u8; 4],
            #[zeroize(skip)]
            y: [u8; 4],
        },
        #[zeroize(skip)]
        B([u8; 4]),
    }

    let mut e = E::A {
        x: [0xAA; 4],
        y: [0x55; 4],
    };
    e.zeroize();
    match e {
        E::A {
            x,
            y,
        } => {
            assert!(x.iter().all(|&b| b == 0));
            assert!(y.iter().all(|&b| b == 0x55), "skipped enum field modified");
        }
        E::B(_) => panic!("wrong variant"),
    }

    // A skipped variant must not be touched at all.
    let mut e = E::B([0x11; 4]);
    e.zeroize();
    match e {
        E::B(v) => assert!(v.iter().all(|&b| b == 0x11), "skipped variant modified"),
        E::A {
            ..
        } => panic!("wrong variant"),
    }
}

#[test]
fn derive_on_drop_zeroizes() {
    #[derive(Zeroize, ZeroizeOnDrop)]
    struct S {
        a: [u8; 8],
        #[zeroize(skip)]
        kept: u64,
    }

    let s = S {
        a: [0xAA; 8],
        kept: 0x55,
    };
    // Move it into a raw slot and drop it in place so we can inspect memory afterwards.
    let mut slot = core::mem::ManuallyDrop::new(s);
    unsafe { core::ptr::drop_in_place(&mut *slot) };
    assert!(slot.a.iter().all(|&b| b == 0), "derived Drop did not zeroize");
    assert_eq!(slot.kept, 0x55, "derived Drop touched a skipped field");
    assert!(core::mem::needs_drop::<S>());
}

#[test]
fn derive_on_drop_enum() {
    #[derive(Zeroize, ZeroizeOnDrop)]
    enum E {
        V([u8; 6]),
    }

    let mut slot = core::mem::ManuallyDrop::new(E::V([0xAA; 6]));
    unsafe { core::ptr::drop_in_place(&mut *slot) };
    match &*slot {
        E::V(v) => assert!(v.iter().all(|&b| b == 0), "derived enum Drop did not zeroize"),
    }
}

#[test]
fn derive_generic_nested_bound() {
    #[derive(Zeroize)]
    struct Wrapper<T> {
        inner: Option<Vec<T>>,
    }

    let mut w = Wrapper {
        inner: Some(vec![1u64, 2, 3]),
    };
    w.zeroize();
    // The Vec is truncated by its own Zeroize impl; the Option is then None.
    assert!(w.inner.is_none());
}
