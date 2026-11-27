// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Macro hygiene / parsing edge cases that must keep compiling.
//!
//! These are not new upstream tests for features that already work; they are
//! audit probes for `$crate` hygiene and for generated local identifiers
//! (`result`, `__self_ptr`, `__guard`, ...) colliding with user fields.

#![allow(dead_code)]
// Intentionally places `?Sized` inline and other bounds in the `where` clause
// to exercise both parsing paths.
#![allow(clippy::multiple_bound_locations)]

use core::pin::Pin;

use pin_project_lite::pin_project;

// -- hostile names ----------------------------------------------------------

mod shadowed {
    // The expansion must not rely on `Pin`/`PhantomData` being in scope; it
    // refers to them through `$crate::__private`.
    #[allow(dead_code)]
    pub struct Pin;
    #[allow(dead_code)]
    pub struct PhantomData<T>(T);
    #[allow(dead_code)]
    pub struct ManuallyDrop<T>(T);
    #[allow(dead_code)]
    pub struct UnsafeOverwriteGuard<T>(T);

    pin_project_lite::pin_project! {
        pub struct Shadowed<T> {
            #[pin]
            pub result: T,
            pub __self_ptr: u8,
            pub __guard: u16,
            pub pinned_self: u32,
        }
    }

    pin_project_lite::pin_project! {
        #[project = ShadowedProj]
        pub enum ShadowedEnum<T> {
            V { #[pin] result: T, __guard: u8 },
        }
    }
}

#[test]
fn shadowed_core_names() {
    let mut s = shadowed::Shadowed { result: 1_u64, __self_ptr: 2, __guard: 3, pinned_self: 4 };
    let mut s = Pin::new(&mut s);
    let p = s.as_mut().project();
    assert_eq!(*p.result, 1);
    *p.__self_ptr = 20;
    *p.__guard = 30;
    *p.pinned_self = 40;
}

// -- visibility forms -------------------------------------------------------

mod vis {
    use core::pin::Pin;

    use pin_project_lite::pin_project;

    pin_project! {
        pub(super) struct PubSuper {
            #[pin]
            pub f: u8,
        }
    }

    pin_project! {
        pub(in crate::vis) struct PubIn {
            #[pin]
            pub f: u8,
        }
    }

    pin_project! {
        struct Priv {
            pub f: u8,
        }
    }

    pub fn use_them() {
        let mut a = PubSuper { f: 1 };
        let mut b = PubIn { f: 2 };
        let mut c = Priv { f: 3 };
        let _ = Pin::new(&mut a).project().f;
        let _ = Pin::new(&mut b).project().f;
        let _ = Pin::new(&mut c).project().f;
    }
}

#[test]
fn visibility_forms() {
    vis::use_them();
}

// -- raw identifiers --------------------------------------------------------

pin_project! {
    struct r#Raw {
        #[pin]
        r#type: u8,
        r#match: u16,
    }
}

#[test]
fn raw_identifiers() {
    let mut x = r#Raw { r#type: 1, r#match: 2 };
    let mut x = Pin::new(&mut x);
    let p = x.as_mut().project();
    assert_eq!(*p.r#type, 1);
    assert_eq!(*p.r#match, 2);
}

// -- generics: defaults, bounds, where clauses, trailing commas -------------

pin_project! {
    pub struct Generics<
        'a,
        T: ?Sized,
        U: Clone = (),
    >
    where
        T: 'a,
    {
        #[pin]
        t: &'a T,
        u: U,
    }
}

trait Assoc {
    type Out;
}
impl Assoc for u8 {
    type Out = u16;
}

pin_project! {
    struct WithWhere<I>
    where
        I: Iterator,
        I::Item: Clone,
    {
        #[pin]
        it: I,
        item: Option<I::Item>,
    }
}

// -- PinnedDrop with attributes on the impl and the method (0.2.13) ---------

pin_project! {
    pub struct AttrDrop<'a> {
        flag: &'a core::cell::Cell<bool>,
        #[pin]
        field: u8,
    }

    /// doc on impl
    #[allow(clippy::needless_pass_by_value)]
    impl<'a> PinnedDrop for AttrDrop<'a> {
        /// doc on method
        #[inline]
        fn drop(this: Pin<&mut Self>) {
            this.project().flag.set(true);
        }
    }
}

#[test]
fn pinned_drop_attributes() {
    let flag = core::cell::Cell::new(false);
    drop(AttrDrop { flag: &flag, field: 1 });
    assert!(flag.get());
}

// -- naming options in any order, plus other type attributes ----------------

pin_project! {
    /// doc1
    #[derive(Debug)]
    #[project_ref = NamingProjRef]
    #[project = NamingProj]
    #[project_replace = NamingProjReplace]
    /// doc2
    pub struct Naming<T> {
        #[pin]
        a: T,
        b: u8,
    }
}

#[test]
fn naming_any_order() {
    let mut x = Naming { a: 1_u8, b: 2 };
    let mut x = Pin::new(&mut x);
    assert_eq!(*x.as_mut().project().a, 1);
    assert_eq!(*x.as_ref().project_ref().b, 2);
    let r = x.as_mut().project_replace(Naming { a: 3, b: 4 });
    assert_eq!(r.b, 2);
    assert_eq!(x.a, 3);
}

// -- single pinned field only / single unpinned field only ------------------

pin_project! {
    #[project_replace = OnlyPinnedReplace]
    struct OnlyPinned {
        #[pin]
        f: u8,
    }
}

pin_project! {
    #[project_replace = OnlyUnpinnedReplace]
    struct OnlyUnpinned {
        f: u8,
    }
}

#[test]
fn only_one_field() {
    let mut a = OnlyPinned { f: 1 };
    let mut a = Pin::new(&mut a);
    let _ = a.as_mut().project_replace(OnlyPinned { f: 2 });
    assert_eq!(a.f, 2);

    let mut b = OnlyUnpinned { f: 1 };
    let mut b = Pin::new(&mut b);
    let r = b.as_mut().project_replace(OnlyUnpinned { f: 2 });
    assert_eq!(r.f, 1);
    assert_eq!(b.f, 2);
}

// -- cfg on the whole invocation (the supported form) -----------------------

#[cfg(any())]
pin_project! {
    struct NeverIncluded {
        #[pin]
        f: u8,
    }
}

#[test]
fn cfg_outside_invocation() {
    // `#[cfg]` must be placed on the `pin_project!` invocation, not on the
    // item inside it; see the note in the audit report (the latter generates
    // impls for a cfg'd-out type and fails to compile).
    let mut x = OnlyPinned { f: 1 };
    let mut x = Pin::new(&mut x);
    let _ = x.as_mut().project().f;
}
