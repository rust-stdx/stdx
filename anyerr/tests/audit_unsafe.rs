#![cfg(feature = "std")]

//! Unsafe-code audit tests for `anyerr` (fork of anyhow 1.0.86).
//!
//! Focus: pointer→reference→pointer round trips in the downcast vtable,
//! exactly-once destruction, and auto-trait/thread safety claims.

#![allow(clippy::redundant_clone)]

use std::{
    any::TypeId,
    error::Error as StdError,
    fmt::{self, Debug, Display},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use anyerr::{Context as _, Error};

// ---------------------------------------------------------------------------
// Drop accounting
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct DropCounter(Arc<AtomicUsize>);

impl DropCounter {
    fn new() -> (Self, Arc<AtomicUsize>) {
        let counter = Arc::new(AtomicUsize::new(0));
        (DropCounter(counter.clone()), counter)
    }
}

struct Tracked<const ALIGN: usize> {
    counter: DropCounter,
    _pad: [u8; 0],
}

impl<const ALIGN: usize> Drop for Tracked<ALIGN> {
    fn drop(&mut self) {
        self.counter.0.fetch_add(1, Ordering::SeqCst);
    }
}

impl<const ALIGN: usize> Debug for Tracked<ALIGN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("tracked")
    }
}

impl<const ALIGN: usize> Display for Tracked<ALIGN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("tracked")
    }
}

impl<const ALIGN: usize> StdError for Tracked<ALIGN> {}

#[repr(align(64))]
struct Aligned64 {
    counter: DropCounter,
}

impl Debug for Aligned64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("aligned64")
    }
}
impl Display for Aligned64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("aligned64")
    }
}
impl Drop for Aligned64 {
    fn drop(&mut self) {
        self.counter.0.fetch_add(1, Ordering::SeqCst);
    }
}
impl StdError for Aligned64 {}

struct ZstError;
impl Debug for ZstError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("zst")
    }
}
impl Display for ZstError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("zst")
    }
}
impl StdError for ZstError {}

struct BigPayload {
    counter: DropCounter,
    _payload: [u8; 4096],
}
impl Debug for BigPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("big")
    }
}
impl Display for BigPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("big")
    }
}
impl Drop for BigPayload {
    fn drop(&mut self) {
        self.counter.0.fetch_add(1, Ordering::SeqCst);
    }
}
impl StdError for BigPayload {}

fn tracked(counter: &DropCounter) -> Tracked<0> {
    Tracked {
        counter: counter.clone(),
        _pad: [],
    }
}

#[test]
fn plain_drop_once() {
    let (counter, drops) = DropCounter::new();
    {
        let err = Error::new(tracked(&counter));
        assert_eq!(drops.load(Ordering::SeqCst), 0);
        drop(err);
    }
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}

#[test]
fn msg_drop_once() {
    let (counter, drops) = DropCounter::new();
    struct Msg(DropCounter);
    impl Display for Msg {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("msg")
        }
    }
    impl Debug for Msg {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("msg")
        }
    }
    impl Drop for Msg {
        fn drop(&mut self) {
            self.0.0.fetch_add(1, Ordering::SeqCst);
        }
    }
    drop(Error::msg(Msg(counter)));
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}

#[test]
fn context_drop_once_outer_and_inner() {
    let (c_counter, c_drops) = DropCounter::new();
    let (e_counter, e_drops) = DropCounter::new();
    struct Ctx(DropCounter);
    impl Display for Ctx {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("ctx")
        }
    }
    impl Drop for Ctx {
        fn drop(&mut self) {
            self.0.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    drop(Error::new(tracked(&e_counter)).context(Ctx(c_counter)));
    assert_eq!(c_drops.load(Ordering::SeqCst), 1);
    assert_eq!(e_drops.load(Ordering::SeqCst), 1);
}

#[test]
fn nested_context_chain_drop_once() {
    let (c1, d1) = DropCounter::new();
    let (c2, d2) = DropCounter::new();
    let (c3, d3) = DropCounter::new();
    let (e, de) = DropCounter::new();

    struct Ctx(DropCounter);
    impl Display for Ctx {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("ctx")
        }
    }
    impl Drop for Ctx {
        fn drop(&mut self) {
            self.0.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    drop(
        Error::new(tracked(&e))
            .context(Ctx(c1))
            .context(Ctx(c2))
            .context(Ctx(c3)),
    );
    assert_eq!(d1.load(Ordering::SeqCst), 1);
    assert_eq!(d2.load(Ordering::SeqCst), 1);
    assert_eq!(d3.load(Ordering::SeqCst), 1);
    assert_eq!(de.load(Ordering::SeqCst), 1);
}

#[test]
fn downcast_by_value_success_drops_rest_once() {
    let (c_counter, c_drops) = DropCounter::new();
    let (e_counter, e_drops) = DropCounter::new();

    struct Ctx(DropCounter);
    impl Display for Ctx {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("ctx")
        }
    }
    impl Drop for Ctx {
        fn drop(&mut self) {
            self.0.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    let err = Error::new(tracked(&e_counter)).context(Ctx(c_counter));
    let inner: Tracked<0> = err.downcast().unwrap();
    // The context must have been dropped by object_drop_rest, the payload must
    // still be alive and owned by us.
    assert_eq!(c_drops.load(Ordering::SeqCst), 1);
    assert_eq!(e_drops.load(Ordering::SeqCst), 0);
    drop(inner);
    assert_eq!(e_drops.load(Ordering::SeqCst), 1);
}

#[test]
fn downcast_by_value_context_drops_rest_once() {
    let (c_counter, c_drops) = DropCounter::new();
    let (e_counter, e_drops) = DropCounter::new();

    struct Ctx(DropCounter);
    impl Display for Ctx {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("ctx")
        }
    }
    impl Debug for Ctx {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("ctx")
        }
    }
    impl Drop for Ctx {
        fn drop(&mut self) {
            self.0.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    let err = Error::new(tracked(&e_counter)).context(Ctx(c_counter));
    let ctx: Ctx = err.downcast().unwrap();
    assert_eq!(c_drops.load(Ordering::SeqCst), 0);
    assert_eq!(e_drops.load(Ordering::SeqCst), 1);
    drop(ctx);
    assert_eq!(c_drops.load(Ordering::SeqCst), 1);
}

#[test]
fn downcast_by_value_failure_drops_once() {
    let (counter, drops) = DropCounter::new();
    let err = Error::new(tracked(&counter));
    let err = err.downcast::<ZstError>().unwrap_err();
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    drop(err);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}

#[test]
fn downcast_by_value_nested_chain_each_level() {
    // 3 levels of context; downcast each level by value in a separate chain.
    struct C1;
    struct C2;
    struct C3;
    macro_rules! ctx {
        ($t:ty) => {
            impl Display for $t {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.write_str(stringify!($t))
                }
            }
            impl Debug for $t {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.write_str(stringify!($t))
                }
            }
        };
    }
    ctx!(C1);
    ctx!(C2);
    ctx!(C3);

    let make = || {
        Error::new(tracked(&DropCounter::new().0))
            .context(C1)
            .context(C2)
            .context(C3)
    };

    assert!(make().downcast::<C3>().is_ok());
    assert!(make().downcast::<C2>().is_ok());
    assert!(make().downcast::<C1>().is_ok());
    assert!(make().downcast::<Tracked<0>>().is_ok());

    // No leaks of the innermost payload in any of the above.
    let (counter, drops) = DropCounter::new();
    let err = Error::new(tracked(&counter)).context(C1).context(C2).context(C3);
    let _ = err.downcast::<C2>().unwrap();
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}

#[test]
fn downcast_same_type_context_and_source() {
    // C == E: both downcasts must be well-defined and drop the other once.
    let (counter, drops) = DropCounter::new();
    let err = Error::new(tracked(&counter)).context(tracked(&counter));
    // Downcast to the context.
    let got: Tracked<0> = err.downcast().unwrap();
    assert_eq!(drops.load(Ordering::SeqCst), 1);
    drop(got);
    assert_eq!(drops.load(Ordering::SeqCst), 2);

    let (counter, drops) = DropCounter::new();
    let err = Error::new(tracked(&counter)).context(tracked(&counter));
    // A failed downcast returns the error intact.
    let err = err.downcast::<ZstError>().unwrap_err();
    drop(err);
    assert_eq!(drops.load(Ordering::SeqCst), 2);
}

#[test]
fn downcast_high_alignment_and_zst() {
    let (counter, drops) = DropCounter::new();
    let err = Error::new(Aligned64 {
        counter: counter.clone(),
    });
    drop(err.downcast::<ZstError>().unwrap_err());
    assert_eq!(drops.load(Ordering::SeqCst), 1);

    let (counter, drops) = DropCounter::new();
    let err = Error::new(Aligned64 {
        counter: counter.clone(),
    });
    let aligned: Aligned64 = err.downcast().unwrap();
    assert_eq!(aligned.counter.0.load(Ordering::SeqCst), 0);
    drop(aligned);
    assert_eq!(drops.load(Ordering::SeqCst), 1);

    let err = Error::new(ZstError);
    assert!(err.downcast_ref::<ZstError>().is_some());
}

#[test]
fn downcast_large_payload() {
    let (counter, drops) = DropCounter::new();
    let err = Error::new(BigPayload {
        counter: counter.clone(),
        _payload: [0xAB; 4096],
    })
    .context("wrapped");
    let big = err.downcast::<BigPayload>().unwrap();
    assert_eq!(big._payload[4095], 0xAB);
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    drop(big);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}

#[test]
fn transparent_wrapper_layouts() {
    // `Error::msg` stores `MessageError<M>` and instantiates the vtable for
    // `M`, relying on `#[repr(transparent)]`. Same for `DisplayError<M>` from
    // `Option::context`. Use a high-alignment M to catch layout mistakes.
    let (counter, drops) = DropCounter::new();
    let err = Error::msg(Aligned64 {
        counter: counter.clone(),
    });
    assert!(err.downcast_ref::<Aligned64>().is_some());
    let value: Aligned64 = err.downcast().unwrap();
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    drop(value);
    assert_eq!(drops.load(Ordering::SeqCst), 1);

    let (counter, drops) = DropCounter::new();
    let err = Option::<()>::None
        .context(Aligned64 {
            counter: counter.clone(),
        })
        .unwrap_err();
    assert!(err.downcast_ref::<Aligned64>().is_some());
    let ctx: Aligned64 = err.downcast().unwrap();
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    drop(ctx);
    assert_eq!(drops.load(Ordering::SeqCst), 1);

    // ZST payload and `&'static str` adhoc messages.
    let err = Error::msg(ZstError);
    assert!(err.downcast::<ZstError>().is_ok());
    let err = Error::msg("static literal");
    assert_eq!(err.downcast_ref::<&'static str>(), Some(&"static literal"));
}

#[test]
fn boxed_error_downcast() {
    let (counter, drops) = DropCounter::new();
    let boxed: Box<dyn StdError + Send + Sync> = Box::new(tracked(&counter));
    // `anyhow!` of a `Box<dyn Error + Send + Sync>` goes through
    // `Error::from_boxed` (BoxedKind tag dispatch).
    let err = anyerr::anyhow!(boxed);
    assert!(err.downcast_ref::<Box<dyn StdError + Send + Sync>>().is_some());
    assert!(err.downcast_ref::<Tracked<0>>().is_none());
    {
        let inner: Box<dyn StdError + Send + Sync> = err.downcast().unwrap();
        assert_eq!(drops.load(Ordering::SeqCst), 0);
        drop(inner);
    }
    assert_eq!(drops.load(Ordering::SeqCst), 1);

    // Conversion back into Box<dyn StdError> preserves the value.
    let (counter, drops) = DropCounter::new();
    let err = Error::new(tracked(&counter));
    let boxed: Box<dyn StdError + Send + Sync> = err.into();
    drop(boxed);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}

#[test]
fn chain_order_and_source_traversal() {
    #[derive(Debug)]
    struct Leaf;
    impl Display for Leaf {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("leaf")
        }
    }
    impl StdError for Leaf {}

    let err = Error::new(Leaf).context("middle").context("outer");
    let rendered: Vec<String> = err.chain().map(|e| e.to_string()).collect();
    assert_eq!(rendered, ["outer", "middle", "leaf"]);
    assert_eq!(err.to_string(), "outer");
    assert_eq!(format!("{:#}", err), "outer: middle: leaf");
    assert_eq!(err.root_cause().to_string(), "leaf");
    let dbg = format!("{:?}", err);
    assert!(dbg.contains("Caused by:"), "{dbg}");
    assert!(dbg.contains("leaf"), "{dbg}");
}

// ---------------------------------------------------------------------------
// Auto traits / threading
// ---------------------------------------------------------------------------

fn assert_send_sync<T: Send + Sync>() {}
fn assert_send<T: Send>() {}

#[test]
fn auto_traits() {
    assert_send_sync::<Error>();
    assert_send::<Box<dyn StdError + Send + Sync + 'static>>();
    let _ = TypeId::of::<Error>();
    fn assert_unwind_safe<T: std::panic::UnwindSafe + std::panic::RefUnwindSafe>() {}
    assert_unwind_safe::<Error>();
}

#[test]
fn error_moves_across_threads_and_drops_once() {
    let (counter, drops) = DropCounter::new();
    let err = Error::new(tracked(&counter)).context("ctx");
    let handle = std::thread::spawn(move || {
        assert!(err.downcast_ref::<Tracked<0>>().is_some());
        drop(err);
    });
    handle.join().unwrap();
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}

#[test]
fn shared_error_read_across_threads() {
    let err = Error::new(ZstError).context("ctx");
    std::thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..1000 {
                    assert!(err.downcast_ref::<ZstError>().is_some());
                    assert!(err.downcast_ref::<&'static str>().is_some());
                    assert_eq!(err.to_string(), "ctx");
                }
            });
        }
    });
}

// ---------------------------------------------------------------------------
// Panic safety
// ---------------------------------------------------------------------------

struct PanicDisplay;
impl Display for PanicDisplay {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("display panic");
    }
}
impl Debug for PanicDisplay {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("debug panic");
    }
}
impl StdError for PanicDisplay {}

#[test]
fn panic_in_display_then_drop_error() {
    let err = Error::new(PanicDisplay).context("ctx");
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| format!("{:?}", err)));
    assert!(res.is_err());
    drop(err);

    let (counter, drops) = DropCounter::new();
    struct PanicAfter(DropCounter);
    impl Display for PanicAfter {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            panic!("display panic");
        }
    }
    impl Debug for PanicAfter {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("panic-after")
        }
    }
    impl Drop for PanicAfter {
        fn drop(&mut self) {
            self.0.0.fetch_add(1, Ordering::SeqCst);
        }
    }
    impl StdError for PanicAfter {}
    let err = Error::new(PanicAfter(counter)).context("ctx");
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| format!("{:?}", err)));
    assert!(res.is_err());
    drop(err);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}

#[test]
fn panic_in_source_traversal_then_drop() {
    struct PanicSource;
    impl Debug for PanicSource {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("panicsource")
        }
    }
    impl Display for PanicSource {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("panicsource")
        }
    }
    impl StdError for PanicSource {
        fn source(&self) -> Option<&(dyn StdError + 'static)> {
            panic!("source panic");
        }
    }

    let err = Error::new(PanicSource);
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| format!("{:?}", err)));
    assert!(res.is_err());
    drop(err);
}
