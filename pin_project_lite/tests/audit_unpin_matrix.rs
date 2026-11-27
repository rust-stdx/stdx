// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Compile-time matrix for the generated conditional `Unpin` impl.
//!
//! Every `assert_*` here is a static assertion: if the macro got the
//! auto-trait computation wrong, this test binary fails to compile. The
//! runtime `#[test]` functions are markers so that the file also reports in
//! `cargo test` output.

#![allow(dead_code)]

#[macro_use]
mod auxiliary;

use core::marker::{PhantomData, PhantomPinned};

use pin_project_lite::pin_project;

// Nested pin_project types: `Inner<T>: Unpin` iff `T: Unpin`; `Outer` only
// looks at its own `#[pin]` fields (`Inner<T>`), not at `Inner`'s `#[pin]`
// field directly -- but transitivity gives the same result.
pin_project! {
    struct Inner<T> {
        #[pin]
        f: T,
    }
}

pin_project! {
    struct Outer<T, U> {
        #[pin]
        inner: Inner<T>,
        u: U,
    }
}

assert_unpin!(Inner<()>);
assert_not_unpin!(Inner<PhantomPinned>);
assert_unpin!(Outer<(), PhantomPinned>);
assert_not_unpin!(Outer<PhantomPinned, ()>);

// `#[pin]` on a reference makes the *reference* the pinned field; references
// are always Unpin, so the container is Unpin regardless of the pointee.
pin_project! {
    struct RefPin<'a, T: ?Sized> {
        #[pin]
        r: &'a mut T,
    }
}
assert_unpin!(RefPin<'_, PhantomPinned>);

// A `PhantomPinned` field *without* `#[pin]` must not affect `Unpin`
// (documented in the macro docs).
pin_project! {
    struct UnpinnedPhantom {
        p: PhantomPinned,
    }
}
assert_unpin!(UnpinnedPhantom);

// ... but with `#[pin]` it must.
pin_project! {
    struct PinnedPhantom {
        #[pin]
        p: PhantomPinned,
    }
}
assert_not_unpin!(PinnedPhantom);

// Explicit `#[project(!Unpin)]`.
pin_project! {
    #[project(!Unpin)]
    struct NeverUnpin {
        #[pin]
        f: u8,
    }
}
assert_not_unpin!(NeverUnpin);

pin_project! {
    #[project(!Unpin)]
    struct NeverUnpinGeneric<T> {
        #[pin]
        f: T,
    }
}
assert_not_unpin!(NeverUnpinGeneric<()>);

// Generic default arguments must be preserved by the macro.
pin_project! {
    struct WithDefault<T = PhantomPinned> {
        #[pin]
        f: T,
    }
}
assert_unpin!(WithDefault<()>);
assert_not_unpin!(WithDefault);

// Enum: a non-pinned field of a `!Unpin` type does not matter; a pinned one
// does.
pin_project! {
    #[project = EUnpinProj]
    enum EUnpin<T> {
        Pinned { #[pin] f: T },
        Unpinned { f: PhantomPinned },
    }
}
assert_unpin!(EUnpin<()>);
assert_not_unpin!(EUnpin<PhantomPinned>);

// Multiple pinned fields: all must be Unpin.
pin_project! {
    struct TwoPinned<A, B> {
        #[pin]
        a: A,
        #[pin]
        b: B,
        other: PhantomData<PhantomPinned>,
    }
}
assert_unpin!(TwoPinned<(), ()>);
assert_not_unpin!(TwoPinned<(), PhantomPinned>);
assert_not_unpin!(TwoPinned<PhantomPinned, ()>);

// Zero-sized pinned field of an Unpin type keeps the container Unpin.
pin_project! {
    struct ZstPinned {
        #[pin]
        z: (),
    }
}
assert_unpin!(ZstPinned);

// `where T: Unpin` bounds are honored (vacuously true for the impl).
pin_project! {
    struct Bounded<T>
    where
        T: Unpin,
    {
        #[pin]
        f: T,
    }
}
assert_unpin!(Bounded<()>);

// A `#[pin]` field whose type is a `?Sized` trait object.
pin_project! {
    struct DynPin<'a> {
        #[pin]
        f: &'a mut (dyn core::fmt::Debug + Unpin),
    }
}
assert_unpin!(DynPin<'_>);

#[test]
fn unpin_matrix_compiles() {
    // The assertions above are the test; reaching this point means the
    // generated `Unpin` impl matched the expected matrix.
}
