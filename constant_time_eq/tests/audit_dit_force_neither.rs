//! Force the crate-wide feature cache to `Neither` *before first use* and check that the
//! comparison leaves PSTATE.DIT completely untouched (this is a separate test binary so it
//! is guaranteed to run before any other use of the crate).

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
fn forced_neither_leaves_dit_untouched() {
    if !std::arch::is_aarch64_feature_detected!("dit") {
        eprintln!("skipping: FEAT_DIT not reported");
        return;
    }

    // FIRST use of the crate in this process.
    // SAFETY: false never claims an unimplemented feature.
    unsafe { set_aarch64_dit_sb_features(false, false) };

    unsafe {
        write_dit(0);
        let zero = read_dit();
        write_dit(1 << 24);
        let one = read_dit();
        assert_ne!(zero, one, "MSR/MRS DIT do not round-trip");

        write_dit(one);
        assert!(constant_time_eq(b"abc", b"abc"));
        assert_eq!(read_dit(), one, "forced Neither modified an enabled DIT");

        write_dit(zero);
        assert!(!constant_time_eq(b"abc", b"abd"));
        assert_eq!(read_dit(), zero, "forced Neither modified a disabled DIT");

        write_dit(zero);
    }
}
