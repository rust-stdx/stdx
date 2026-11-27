//! Audit test for the `dit.rs` guard, compiled from the *actual production source* so that
//! the private `with_dit` entry point can be driven directly with a non-trivial closure.
//!
//! It verifies that DIT is enabled while the closure runs, restored after a normal return,
//! restored after an unwind, and restored after a nested call. This is the only way to
//! exercise the panic path, because the public `constant_time_eq` entry points never panic.

#![cfg(all(target_arch = "aarch64", not(miri)))]

use std::panic::{AssertUnwindSafe, catch_unwind};

#[path = "../src/dit.rs"]
#[allow(dead_code)]
mod dit_src;

#[target_feature(enable = "dit")]
unsafe fn read_dit() -> u64 {
    let mut value: u64;
    // SAFETY: `FEAT_DIT` is checked by the caller.
    unsafe {
        core::arch::asm!("mrs {}, dit", out(reg) value, options(nomem, preserves_flags, nostack));
    }
    value
}

#[target_feature(enable = "dit")]
unsafe fn write_dit(value: u64) {
    // SAFETY: `FEAT_DIT` is checked by the caller.
    unsafe {
        core::arch::asm!("msr dit, {}", in(reg) value, options(nostack));
    }
}

unsafe fn dit_states() -> (u64, u64) {
    // SAFETY: caller checked `supported()`
    unsafe {
        write_dit(0);
        let zero = read_dit();
        write_dit(1 << 24);
        let one = read_dit();
        assert_ne!(zero, one, "MSR/MRS DIT do not round-trip");
        write_dit(zero);
        (zero, one)
    }
}

fn supported() -> bool {
    std::arch::is_aarch64_feature_detected!("dit")
}

#[test]
fn dit_enabled_inside_and_restored_after_normal_panic_and_nesting() {
    if !supported() {
        eprintln!("skipping: FEAT_DIT not reported");
        return;
    }

    unsafe {
        let (zero, one) = dit_states();

        // 1. DIT is actually enabled while the closure body runs.
        write_dit(zero);
        let inside = dit_src::with_dit(|| read_dit());
        assert_eq!(inside, one, "DIT was not enabled inside with_dit");
        assert_eq!(read_dit(), zero, "DIT not restored after normal return");

        // 2. Restored after unwinding, starting disabled.
        write_dit(zero);
        let res = catch_unwind(AssertUnwindSafe(|| {
            dit_src::with_dit(|| -> () { panic!("boom") });
        }));
        assert!(res.is_err());
        assert_eq!(read_dit(), zero, "DIT not restored after unwind (starting disabled)");

        // 3. Restored after unwinding, starting enabled.
        write_dit(one);
        let res = catch_unwind(AssertUnwindSafe(|| {
            dit_src::with_dit(|| -> () { panic!("boom2") });
        }));
        assert!(res.is_err());
        assert_eq!(read_dit(), one, "DIT not restored after unwind (starting enabled)");

        // 4. Nested calls: inner enable/restore must keep the outer state.
        write_dit(zero);
        let nested = dit_src::with_dit(|| {
            let outer = read_dit();
            let inner = dit_src::with_dit(|| read_dit());
            assert_eq!(outer, one, "outer with_dit did not enable DIT");
            assert_eq!(inner, one, "inner with_dit did not enable DIT");
            read_dit()
        });
        assert_eq!(nested, one);
        assert_eq!(read_dit(), zero, "DIT not restored after nested with_dit");

        // Leave the thread disabled.
        write_dit(zero);
    }
}
