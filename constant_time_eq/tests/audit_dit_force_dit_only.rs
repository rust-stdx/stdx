//! Force the crate-wide feature cache to `DitOnly` *before first use* so the
//! `with_feat_dit` + `synchronization_barrier` (`dsb nsh; isb sy`) path is executed, and
//! check that DIT is restored on every return.

#![cfg(all(target_arch = "aarch64", not(miri)))]

use constant_time_eq::{constant_time_eq, dit::set_aarch64_dit_sb_features};

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

#[test]
fn forced_dit_only_restores_dit() {
    if !std::arch::is_aarch64_feature_detected!("dit") {
        eprintln!("skipping: FEAT_DIT not reported");
        return;
    }

    // FIRST use of the crate in this process. `sb = false` never claims an
    // unimplemented feature.
    // SAFETY: `FEAT_DIT` was detected above.
    unsafe { set_aarch64_dit_sb_features(true, false) };

    unsafe {
        write_dit(0);
        let zero = read_dit();
        write_dit(1 << 24);
        let one = read_dit();
        assert_ne!(zero, one, "MSR/MRS DIT do not round-trip");

        for state in [zero, one] {
            write_dit(state);
            assert!(constant_time_eq(b"0123456789abcdef", b"0123456789abcdef"));
            assert_eq!(read_dit(), state, "DIT not restored after normal return (state {state:#x})");

            write_dit(state);
            assert!(!constant_time_eq(b"0123456789abcdef", b"0123456789abcdee"));
            assert_eq!(read_dit(), state, "DIT not restored after unequal return (state {state:#x})");
        }

        write_dit(zero);
    }
}
