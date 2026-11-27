// SPDX-License-Identifier: Apache-2.0 OR MIT

// Local replacements for the `static_assertions` macros used by the upstream
// `_tests` suite (that dev-dependency is not vendored in this workspace).
#![allow(dead_code, unused_macros)]

macro_rules! assert_unpin {
    ($ty:ty) => {
        const _: fn() = || {
            fn assert_impl<T: ::core::marker::Unpin>() {}
            assert_impl::<$ty>();
        };
    };
}

// Same technique `static_assertions::assert_not_impl_all!` uses: if `$ty`
// implements `Unpin` both impls apply and the inference variable is ambiguous.
macro_rules! assert_not_unpin {
    ($ty:ty) => {
        const _: fn() = || {
            trait AmbiguousIfImpl<A> {
                fn some_item() {}
            }
            impl<T: ?Sized> AmbiguousIfImpl<()> for T {}
            impl<T: ?Sized + ::core::marker::Unpin> AmbiguousIfImpl<u8> for T {}
            let _ = <$ty as AmbiguousIfImpl<_>>::some_item;
        };
    };
}
