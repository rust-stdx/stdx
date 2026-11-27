//! Audit tests for `constant_time_eq::dit` on aarch64: DIT is restored after a normal
//! return, after unwinding and for several concurrent threads.
//!
//! These tests only run when the hardware/kernel reports `FEAT_DIT`; they skip otherwise.

#![cfg(target_arch = "aarch64")]

use constant_time_eq::constant_time_eq;

#[target_feature(enable = "dit")]
unsafe fn read_dit() -> u64 {
    let mut value: u64;
    // SAFETY: `FEAT_DIT` is checked by the caller (`supported()`).
    unsafe {
        core::arch::asm!("mrs {}, dit", out(reg) value, options(nomem, preserves_flags, nostack));
    }
    value
}

#[target_feature(enable = "dit")]
unsafe fn write_dit(value: u64) {
    // SAFETY: `FEAT_DIT` is checked by the caller (`supported()`).
    unsafe {
        core::arch::asm!("msr dit, {}", in(reg) value, options(nostack));
    }
}

fn supported() -> bool {
    std::arch::is_aarch64_feature_detected!("dit")
}

/// Discover the two PSTATE.DIT representations (disabled/enabled) without assuming whether
/// `MRS DIT` reports the field value or the SPSR bit position.
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

#[test]
fn dit_restored_after_normal_return_and_nesting() {
    if !supported() {
        eprintln!("skipping: FEAT_DIT not reported");
        return;
    }

    // Trigger the lazy feature detection before measuring, as the crate documents.
    let _ = constant_time_eq(b"warmup", b"warmup");

    unsafe {
        let (zero, one) = dit_states();

        for state in [zero, one] {
            // Start from a known DIT state. Externally enabling DIT before the call also
            // simulates a nested `with_dit`: the inner call must save/restore `one`.
            for _ in 0..8 {
                write_dit(state);
                assert!(constant_time_eq(
                    b"0123456789abcdef0123456789abcdef",
                    b"0123456789abcdef0123456789abcdef"
                ));
                assert_eq!(read_dit(), state, "DIT not restored after equal compare");

                write_dit(state);
                assert!(!constant_time_eq(
                    b"0123456789abcdef0123456789abcdef",
                    b"0123456789abcdef0123456789abcdee"
                ));
                assert_eq!(read_dit(), state, "DIT not restored after unequal compare");

                // The `_n` entry point as well.
                write_dit(state);
                let a = [7u8; 64];
                let mut b = [7u8; 64];
                b[63] = 8;
                assert!(!constant_time_eq::constant_time_eq_n(&a, &b));
                assert_eq!(read_dit(), state, "DIT not restored after constant_time_eq_n");
            }
        }

        // Leave the thread as we found it.
        write_dit(zero);
    }
}

#[test]
fn dit_restored_in_four_threads() {
    if !supported() {
        eprintln!("skipping: FEAT_DIT not reported");
        return;
    }

    let _ = constant_time_eq(b"warmup", b"warmup");
    let (zero, one) = unsafe { dit_states() };

    let handles: Vec<_> = (0..4u64)
        .map(|i| {
            std::thread::spawn(move || {
                let state = if i % 2 == 0 { zero } else { one };
                unsafe {
                    write_dit(state);
                    for k in 0..200u8 {
                        let a = [k; 33];
                        let mut b = [k; 33];
                        if k % 2 == 0 {
                            b[32] ^= 1;
                        }
                        let _ = constant_time_eq(&a, &b);
                        assert_eq!(read_dit(), state, "thread {i} lost DIT after call {k}");
                    }
                    write_dit(zero);
                }
            })
        })
        .collect();

    for (i, h) in handles.into_iter().enumerate() {
        h.join().unwrap_or_else(|_| panic!("thread {i} panicked"));
    }
}
