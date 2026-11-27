#![cfg(feature = "std")]

//! Reproducers for RUSTSEC-2026-0190 / GHSA (anyhow <= 1.0.102):
//! unsoundness in `Error::downcast_mut()`.
//!
//! `anyerr` is a fork of anyhow 1.0.86 and carries the pre-fix code. A
//! `downcast_mut` that lands on a value stored in a `ContextError` builds the
//! mutable reference from a `Ref` that was derived through `&T`
//! (`ptr.rs`: `Ref::new` -> `Mut` -> `&mut *ptr`). That pointer->reference->
//! pointer round trip is UB under Stacked Borrows (and rejected by Tree
//! Borrows), even though it "works" on every native target.
//!
//! Miri (Stacked Borrows, default):
//! ```text
//! $ MIRIFLAGS=-Zmiri-strict-provenance \
//!       cargo +nightly miri test -p anyerr --test audit_downcast_mut
//! error: Undefined Behavior: trying to retag from <143738> for Unique
//! permission at alloc44449[0x38], but that tag only grants SharedReadOnly
//! permission for this location
//!   --> anyerr/src/ptr.rs:169:17
//! help: <143738> was created by a SharedReadOnly retag at offsets [0x38..0x48]
//!   --> anyerr/src/ptr.rs:88:17
//!         0: anyerr::ptr::Mut::<'_, ErrorContext>::deref_mut
//!         1: anyerr::error::<impl anyerr::Error>::downcast_mut::<ErrorContext>
//! ```
//!
//! Before the fix this test binary failed under Miri and passed natively. The
//! production code now builds the inner pointer with `ptr::addr_of!` and
//! `Ref::from_raw`, so the same tests are expected to pass everywhere.

use std::{
    error::Error as StdError,
    fmt::{self, Debug, Display},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use anyerr::Error;

#[derive(Debug)]
struct ErrorContext(&'static str);

impl Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

/// Advisory reproducer: add context, then `downcast_mut` to the context type.
#[test]
fn downcast_mut_on_context() {
    let mut error = Error::msg("inner error").context(ErrorContext("old context"));
    let context: &mut ErrorContext = error.downcast_mut().unwrap();
    context.0 = "new context";
    assert_eq!(format!("{}", error), "new context");
}

/// Variant: `downcast_mut` lands on the inner `E` of `ContextError<C, E>`
/// (second branch of `context_downcast`). Reached through the `Context` trait
/// on a `Result`, which uses `Error::from_context` and stores
/// `ContextError<C, E>` directly (whereas `Error::context` stores
/// `ContextError<C, Error>` and recurses through the inner vtable instead).
#[test]
fn downcast_mut_on_inner_error() {
    #[derive(Debug)]
    struct Leaf {
        _nonzero: u64,
    }
    impl Display for Leaf {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("leaf")
        }
    }
    impl StdError for Leaf {}

    use anyerr::Context as _;

    let mut error = Err::<(), Leaf>(Leaf {
        _nonzero: 7,
    })
    .context(ErrorContext("ctx"))
    .unwrap_err();
    let leaf: &mut Leaf = error.downcast_mut().unwrap();
    leaf._nonzero = 8;
}

/// Variant: multi-level context chain (`context_chain_downcast`).
#[test]
fn downcast_mut_on_nested_chain() {
    let mut error = Error::msg("inner").context(ErrorContext("one")).context("two");
    let ctx: &mut ErrorContext = error.downcast_mut().unwrap();
    ctx.0 = "one-mutated";
    assert_eq!(format!("{:#}", error), "two: one-mutated: inner");
}

/// Control: `DerefMut for Error` also ends in `Mut::deref_mut`, but its
/// pointer comes from `object_ref`, which uses `Ref::from_raw(addr_of!(..))`
/// (no reference is materialized), so Miri accepts it.
#[test]
fn deref_mut_of_context_error_is_clean() {
    struct C;
    impl Display for C {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("c")
        }
    }

    let mut err = Error::msg("inner").context(C);
    let dyn_err: &mut (dyn StdError + Send + Sync + 'static) = &mut *err;
    assert_eq!(dyn_err.to_string(), "c");
}

// Drop accounting for the same paths (passes natively, and the *drop* side is
// sound under Miri; only the `&mut` retag is UB).

#[derive(Clone, Debug)]
struct DropCounter(Arc<AtomicUsize>);

impl Drop for DropCounter {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn downcast_mut_sound_without_context() {
    // Control: `object_downcast` for a plain error already uses
    // `Ref::from_raw(addr_of!(..))`, so no shared-reference round trip occurs.
    #[derive(Debug)]
    struct Leaf {
        #[allow(dead_code)]
        counter: DropCounter,
    }
    impl Display for Leaf {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("leaf")
        }
    }
    impl StdError for Leaf {}

    let counter = Arc::new(AtomicUsize::new(0));
    let mut error = Error::new(Leaf {
        counter: DropCounter(counter.clone()),
    });
    let leaf: &mut Leaf = error.downcast_mut().unwrap();
    let _ = leaf;
    drop(error);
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}
