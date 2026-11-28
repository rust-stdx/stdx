// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Audit tests for projection lifetime/aliasing behaviour, drop order and
//! panic safety of `pin_project!` on pin-project-lite 0.2.18.
//!
//! These tests only use the public safe API (plus `PhantomPinned` to model
//! genuinely immovable fields), so they must pass on any sound version.

#![allow(dead_code)]

use core::{
    cell::RefCell,
    marker::PhantomPinned,
    mem,
    pin::{Pin, pin},
};
use std::panic::{self, AssertUnwindSafe};

use pin_project_lite::pin_project;

// ---------------------------------------------------------------------------
// 1. Projection basics: all three projection methods agree on addresses and
//    never move a #[pin] field.
// ---------------------------------------------------------------------------

pin_project! {
    #[project = PProj]
    #[project_ref = PProjRef]
    #[project_replace = PProjReplace]
    struct P<T, U> {
        #[pin]
        pinned: T,
        unpinned: U,
    }
}

#[test]
fn projection_addresses_are_stable() {
    let mut v = P { pinned: 1_u32, unpinned: 2_u64 };
    let addr_pinned = (&raw const v.pinned) as usize;

    let mut pinned = Pin::new(&mut v);
    {
        let p: PProj<'_, u32, u64> = pinned.as_mut().project();
        // `Pin<&mut T>` points at the field itself, no temporary copy.
        assert_eq!((&raw const *p.pinned) as usize, addr_pinned);
        assert_eq!(*p.pinned, 1);
        assert_eq!(*p.unpinned, 2);
    }
    {
        let p: PProjRef<'_, u32, u64> = pinned.as_ref().project_ref();
        assert_eq!((&raw const *p.pinned) as usize, addr_pinned);
    }
    {
        let r: PProjReplace<u32, u64> =
            pinned.as_mut().project_replace(P { pinned: 9, unpinned: 8 });
        assert_eq!(r.unpinned, 2);
        assert_eq!(mem::size_of_val(&r.pinned), 0); // PhantomData<u32>
    }
    assert_eq!(pinned.pinned, 9);
    assert_eq!(pinned.unpinned, 8);
}

// ---------------------------------------------------------------------------
// 2. A `!Unpin` field projected through `Pin<&mut P<..>>` keeps its pin.
// ---------------------------------------------------------------------------

#[test]
fn pinned_field_is_not_unpin() {
    // `P<PhantomPinned, ()>` is `!Unpin`, so `Pin::new` is rejected (negative
    // half in tests/audit_compile.rs); `pin!` is the safe way to pin it.
    let v = pin!(P { pinned: PhantomPinned, unpinned: () });
    let mut v = v;
    let p = v.as_mut().project();
    let _: Pin<&mut PhantomPinned> = p.pinned;
    let _: &mut () = p.unpinned;
    let _ = v.as_ref().project_ref();
}

// ---------------------------------------------------------------------------
// 3. Double projection through `as_mut()` is allowed and does not alias:
//    each `project()` consumes a fresh reborrow.
// ---------------------------------------------------------------------------

#[test]
fn double_projection_sequential() {
    let mut v = P { pinned: 1_u8, unpinned: 2_u8 };
    let mut v = Pin::new(&mut v);

    {
        let p = v.as_mut().project();
        *p.unpinned = 10;
    }
    {
        let p = v.as_mut().project();
        assert_eq!(*p.unpinned, 10);
        *p.unpinned = 20;
    }
    assert_eq!(v.unpinned, 20);
}

// ---------------------------------------------------------------------------
// 4. Enum projection: variants, unit variants, multi-field, replace.
// ---------------------------------------------------------------------------

pin_project! {
    #[project = EProj]
    #[project_ref = EProjRef]
    #[project_replace = EProjReplace]
    #[derive(Debug, PartialEq, Eq)]
    enum E<A, B> {
        Both { #[pin] a: A, b: B },
        OnlyPinned { #[pin] a: A },
        Unit,
    }
}

#[test]
fn enum_variants_project() {
    let mut e = E::Both { a: 1_u16, b: 2_u16 };
    let mut e = Pin::new(&mut e);

    match e.as_mut().project() {
        EProj::Both { a, b } => {
            assert_eq!(*a, 1);
            *b += 1;
        }
        _ => unreachable!(),
    }
    assert_eq!(&*e, &E::Both { a: 1, b: 3 });

    // project_replace changes the variant, dropping the old pinned field.
    let r = e.as_mut().project_replace(E::Unit);
    match r {
        EProjReplace::Both { a: _, b } => assert_eq!(b, 3),
        _ => unreachable!(),
    }
    assert!(matches!(&*e, E::Unit));

    let mut e2 = E::<u16, u16>::OnlyPinned { a: 7 };
    let e2 = Pin::new(&mut e2);
    match e2.as_ref().project_ref() {
        EProjRef::OnlyPinned { a } => assert_eq!(*a, 7),
        _ => unreachable!(),
    }
    // project_replace on a unit variant.
    let mut unit = E::<u16, u16>::Unit;
    let mut unit = Pin::new(&mut unit);
    let r = unit.as_mut().project_replace(E::Unit);
    assert!(matches!(r, EProjReplace::Unit));
    assert!(matches!(&*unit, E::Unit));

    // Replacement that is a different variant.
    let mut e3 = E::<u16, u16>::OnlyPinned { a: 5 };
    let mut e3 = Pin::new(&mut e3);
    let _ = e3.as_mut().project_replace(E::Both { a: 6, b: 7 });
    match &*e3 {
        E::Both { a, b } => assert_eq!((*a, *b), (6, 7)),
        _ => unreachable!(),
    }
}

// ---------------------------------------------------------------------------
// 5. Drop order with counters: plain drop, project_replace, and the
//    "pinned field dropped in place, unpinned field returned" contract.
// ---------------------------------------------------------------------------

struct Counted<'a> {
    log: &'a RefCell<Vec<&'static str>>,
    name: &'static str,
}

impl Drop for Counted<'_> {
    fn drop(&mut self) {
        self.log.borrow_mut().push(self.name);
    }
}

pin_project! {
    #[project_replace = RProjReplace]
    struct R<'a> {
        #[pin]
        first: Counted<'a>,
        middle: Counted<'a>,
        #[pin]
        last: Counted<'a>,
    }
}

#[test]
fn project_replace_drop_order_and_result() {
    let log = RefCell::new(Vec::new());
    {
        let mut r = R {
            first: Counted { log: &log, name: "first-old" },
            middle: Counted { log: &log, name: "middle-old" },
            last: Counted { log: &log, name: "last-old" },
        };
        let mut r = Pin::new(&mut r);
        {
            let moved: RProjReplace<'_> = r.as_mut().project_replace(R {
                first: Counted { log: &log, name: "first-new" },
                middle: Counted { log: &log, name: "middle-new" },
                last: Counted { log: &log, name: "last-new" },
            });
            // Pinned fields are dropped in place, in declaration order.
            assert_eq!(*log.borrow(), ["first-old", "last-old"]);
            // The unpinned field was moved out, not dropped yet.
            assert_eq!(moved.middle.name, "middle-old");
            drop(moved);
            assert_eq!(*log.borrow(), ["first-old", "last-old", "middle-old"]);
        }
    }
    // The replacement value is dropped normally, in declaration order.
    assert_eq!(*log.borrow(), [
        "first-old",
        "last-old",
        "middle-old",
        "first-new",
        "middle-new",
        "last-new"
    ]);
}

#[test]
fn plain_drop_order_is_declaration_order() {
    let log = RefCell::new(Vec::new());
    {
        let _r = R {
            first: Counted { log: &log, name: "first" },
            middle: Counted { log: &log, name: "middle" },
            last: Counted { log: &log, name: "last" },
        };
    }
    assert_eq!(*log.borrow(), ["first", "middle", "last"]);
}

// ---------------------------------------------------------------------------
// 5b. Panic while a projection is alive: unwinding must drop every field
//     exactly once (the projection only holds references, no drop glue).
// ---------------------------------------------------------------------------

#[cfg(panic = "unwind")]
#[test]
fn panic_while_projection_alive_drops_once() {
    let log = RefCell::new(Vec::new());
    let res = panic::catch_unwind(AssertUnwindSafe(|| {
        let mut r = R {
            first: Counted { log: &log, name: "first" },
            middle: Counted { log: &log, name: "middle" },
            last: Counted { log: &log, name: "last" },
        };
        let mut r = Pin::new(&mut r);
        let p = r.as_mut().project();
        // Keep a `Pin<&mut _>` from the projection alive across the panic.
        let _held: Pin<&mut Counted<'_>> = p.first;
        panic!("boom while projected");
    }));
    assert!(res.is_err());
    assert_eq!(*log.borrow(), ["first", "middle", "last"]);
}

// ---------------------------------------------------------------------------
// 6. Panic inside `PinnedDrop::drop`: must run exactly once, and must not
//    double-drop the pinned field.
// ---------------------------------------------------------------------------

struct DropOnce<'a>(&'a RefCell<u32>);

impl Drop for DropOnce<'_> {
    fn drop(&mut self) {
        *self.0.borrow_mut() += 1;
    }
}

pin_project! {
    pub struct PDrop<'a> {
        #[pin]
        field: DropOnce<'a>,
        drops: &'a RefCell<u32>,
    }

    impl PinnedDrop for PDrop<'_> {
        fn drop(this: Pin<&mut Self>) {
            let this = this.project();
            *this.drops.borrow_mut() += 100;
            panic!("pinned drop panic");
        }
    }
}

#[cfg(panic = "unwind")]
#[test]
fn panic_in_pinned_drop_runs_once() {
    let drops = RefCell::new(0);
    let res = panic::catch_unwind(AssertUnwindSafe(|| {
        let _x = PDrop { field: DropOnce(&drops), drops: &drops };
    }));
    assert!(res.is_err());
    // PinnedDrop::drop ran once (+100), and the pinned field was dropped
    // exactly once by the normal struct drop glue afterwards (+1).
    assert_eq!(*drops.borrow(), 101);
}

// ---------------------------------------------------------------------------
// 7. Panic while dropping a pinned field inside project_replace: all field
//    destructors must still run (no leaks) and none must run twice.
// ---------------------------------------------------------------------------

struct PanicOnDrop<'a>(&'a RefCell<u32>, bool);

impl Drop for PanicOnDrop<'_> {
    fn drop(&mut self) {
        *self.0.borrow_mut() += 1;
        if self.1 {
            panic!("field drop panic");
        }
    }
}

pin_project! {
    #[project_replace = PRProjReplace]
    struct PR<'a> {
        #[pin]
        pinned: PanicOnDrop<'a>,
        unpinned: PanicOnDrop<'a>,
    }
}

#[cfg(panic = "unwind")]
#[test]
fn project_replace_panic_in_pinned_field_drop() {
    let count = RefCell::new(0);
    let res = panic::catch_unwind(AssertUnwindSafe(|| {
        let mut x = PR { pinned: PanicOnDrop(&count, true), unpinned: PanicOnDrop(&count, false) };
        let mut x = Pin::new(&mut x);
        let _ = x.as_mut().project_replace(PR {
            pinned: PanicOnDrop(&count, false),
            unpinned: PanicOnDrop(&count, false),
        });
    }));
    assert!(res.is_err());
    // old pinned (1), old unpinned (1), and both replacement fields (2) are
    // all dropped exactly once: nothing is leaked and nothing is dropped
    // twice.
    assert_eq!(*count.borrow(), 4);
}

// ---------------------------------------------------------------------------
// 8. Self-referential type pinned through the macro must never be moved by
//    safe code (Miri checks the pointer stays valid).
// ---------------------------------------------------------------------------

struct SelfRef {
    // `UnsafeCell` so that the stored self-pointer stays valid across the
    // `&mut Self` borrows created by each `project()` call (Stacked Borrows).
    data: core::cell::UnsafeCell<u64>,
    ptr: *const u64,
    _pin: PhantomPinned,
}

impl SelfRef {
    fn new(data: u64) -> Self {
        Self {
            data: core::cell::UnsafeCell::new(data),
            ptr: core::ptr::null(),
            _pin: PhantomPinned,
        }
    }
    /// SAFETY: must not be moved after this call.
    fn init(&mut self) {
        self.ptr = self.data.get();
    }
    fn get(&self) -> u64 {
        // SAFETY: `init` established a pointer to `self.data`, and the pin
        // guarantee (enforced by the macro) keeps `self` in place.
        unsafe { *self.ptr }
    }
    // The usual pattern for a self-referential type: mutation only through
    // `Pin<&mut Self>`, using `get_unchecked_mut` internally.
    fn init_pinned(this: Pin<&mut Self>) {
        // SAFETY: caller has pinned `this`; we do not move out of it.
        let this = unsafe { this.get_unchecked_mut() };
        this.ptr = this.data.get();
    }
    fn set_pinned(this: Pin<&mut Self>, v: u64) {
        // SAFETY: caller has pinned `this`; we do not move out of it.
        unsafe { *this.get_unchecked_mut().data.get() = v };
    }
    fn get_pinned(this: Pin<&Self>) -> u64 {
        this.get()
    }
}

pin_project! {
    struct SelfRefHolder {
        #[pin]
        inner: SelfRef,
        _other: u64,
    }
}

impl SelfRefHolder {
    fn init_pinned(mut this: Pin<&mut Self>) {
        SelfRef::init_pinned(this.as_mut().project().inner);
    }
}

#[test]
fn self_referential_pinned_field_stays_valid() {
    // `Box::pin` performs the final move, then `init_pinned` records the
    // address; afterwards only `Pin<&mut _>` is available (the type is
    // `!Unpin`, so safe code cannot move it again).
    let mut holder = Box::pin(SelfRefHolder { inner: SelfRef::new(0xdead_beef), _other: 1 });
    SelfRefHolder::init_pinned(holder.as_mut());
    for i in 0..8_u64 {
        {
            let p = holder.as_mut().project();
            SelfRef::set_pinned(p.inner, 0xdead_beef + i);
        }
        assert_eq!(SelfRef::get_pinned(holder.as_ref().project_ref().inner), 0xdead_beef + i);
    }
}

// ---------------------------------------------------------------------------
// 9. `project_replace` on a self-referential type: the pinned SelfRef is
//    dropped in place, and the replacement must be used at the same address.
// ---------------------------------------------------------------------------

pin_project! {
    #[project_replace = SRReplace]
    struct SelfRefReplace {
        #[pin]
        inner: SelfRef,
        tag: u32,
    }
}

#[test]
fn project_replace_self_referential() {
    let mut holder = Box::pin(SelfRefReplace { inner: SelfRef::new(1234), tag: 1 });
    SelfRef::init_pinned(holder.as_mut().project().inner);
    let addr_before = (&raw const *holder.as_ref().project_ref().inner) as usize;
    {
        let r =
            holder.as_mut().project_replace(SelfRefReplace { inner: SelfRef::new(5678), tag: 2 });
        assert_eq!(r.tag, 1);
    }
    let mut p = holder.as_mut().project();
    assert_eq!((&raw const *p.inner) as usize, addr_before);
    SelfRef::init_pinned(p.inner.as_mut());
    assert_eq!(SelfRef::get_pinned(p.inner.as_ref()), 5678);
    assert_eq!(*p.tag, 2);
}

// ---------------------------------------------------------------------------
// 10. ZST / zero-sized pinned field.
// ---------------------------------------------------------------------------

pin_project! {
    struct Zst {
        #[pin]
        z: (),
        u: u8,
    }
}

#[test]
fn zero_sized_projection() {
    let mut z = Zst { z: (), u: 3 };
    let mut z = Pin::new(&mut z);
    let p = z.as_mut().project();
    assert_eq!(*p.u, 3);
    let _: Pin<&mut ()> = p.z;
}
