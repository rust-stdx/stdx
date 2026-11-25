//! ARMv8 Crypto Extensions backend.
//!
//! AEGIS is built from the AES encryption round, so the `aes` extension is all
//! that is needed. On aarch64 the hardware registers are 128 bits wide, so
//! AEGIS-256X2/X4 use two/four independent `uint8x16_t` registers. The
//! `#[target_feature]` wrappers around the whole operation are required: the
//! shared core is `#[inline(always)]`, so it is compiled and scheduled with the
//! AES feature enabled.

use core::arch::aarch64::*;

use crate::aegis::aegis_core::{BlockOps, Core};

/// A vector of `D` 128-bit AES lanes.
///
/// # Target features
///
/// Although every method of its [`BlockOps`] implementation is a safe function,
/// each one executes `aes`/`neon` instructions. The type is crate-private and is
/// only instantiated from the `#[target_feature(enable = "aes,neon")]` wrappers
/// at the bottom of this file, which are called after runtime feature detection
/// (or from a `cfg!(target_feature = "aes")` build). Do not use it from any other
/// context, or the instructions may be illegal on the current CPU.
#[derive(Clone, Copy)]
pub(crate) struct Ne<const D: usize> {
    v: [uint8x16_t; D],
}

impl<const D: usize> BlockOps for Ne<D> {
    const D: usize = D;

    // The split loop needs 12 live vectors. That fits the 32 NEON registers
    // for D = 1; for D = 2/4 it spills and is measurably slower.
    const SPLIT_BULK: bool = D == 1;

    #[inline(always)]
    fn zero() -> Self {
        Self {
            v: [unsafe { vdupq_n_u8(0) }; D],
        }
    }

    #[inline(always)]
    fn xor(self, rhs: Self) -> Self {
        let mut v = [unsafe { vdupq_n_u8(0) }; D];
        for lane in 0..D {
            v[lane] = unsafe { veorq_u8(self.v[lane], rhs.v[lane]) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    fn and(self, rhs: Self) -> Self {
        let mut v = [unsafe { vdupq_n_u8(0) }; D];
        for lane in 0..D {
            v[lane] = unsafe { vandq_u8(self.v[lane], rhs.v[lane]) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    fn round(self, rk: Self) -> Self {
        let zero = unsafe { vdupq_n_u8(0) };
        let mut v = [zero; D];
        for lane in 0..D {
            v[lane] = unsafe { veorq_u8(vaesmcq_u8(vaeseq_u8(self.v[lane], zero)), rk.v[lane]) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    fn round_nc_with(self, key: Self) -> Self {
        let mut v = [unsafe { vdupq_n_u8(0) }; D];
        for lane in 0..D {
            // AESE performs AddRoundKey first, so this is round_nc(self ^ key).
            v[lane] = unsafe { vaesmcq_u8(vaeseq_u8(self.v[lane], key.v[lane])) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    unsafe fn load(ptr: *const u8) -> Self {
        let mut v = [unsafe { vdupq_n_u8(0) }; D];
        for lane in 0..D {
            // SAFETY: the caller guarantees `D * 16` readable bytes.
            v[lane] = unsafe { vld1q_u8(ptr.add(lane * 16)) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    unsafe fn store(self, ptr: *mut u8) {
        for lane in 0..D {
            // SAFETY: the caller guarantees `D * 16` writable bytes.
            unsafe { vst1q_u8(ptr.add(lane * 16), self.v[lane]) };
        }
    }

    #[inline(always)]
    fn broadcast(block: &[u8; 16]) -> Self {
        let b = unsafe { vld1q_u8(block.as_ptr()) };
        Self {
            v: [b; D],
        }
    }

    #[inline(always)]
    fn ctx_all() -> Self {
        let mut v = [unsafe { vdupq_n_u8(0) }; D];
        for lane in 0..D {
            let mut ctx = [0u8; 16];
            ctx[0] = lane as u8;
            ctx[1] = (D - 1) as u8;
            v[lane] = unsafe { vld1q_u8(ctx.as_ptr()) };
        }
        Self {
            v,
        }
    }
}

/// SHA3 variant of [`Ne`], using `eor3` for three-input XORs.
///
/// This is used only for AEGIS-256 and AEGIS-256X2. `eor3` mainly helps the
/// two-lane mode; for AEGIS-256 it is neutral. AEGIS-256X4 keeps the plain
/// NEON path: for the four-lane AEAD loop `eor3` was about 7% slower in
/// measurements because of register pressure.
///
/// # Target features
///
/// As with [`Ne`], the methods of the [`BlockOps`] implementation are safe
/// functions that execute `aes` and, for [`Self::xor3`]/[`Self::round_xor`],
/// `sha3` instructions. Only the `*_sha3` wrappers at the bottom of this file
/// (compiled with `#[target_feature(enable = "aes,neon,sha3")` and selected by
/// runtime detection) may instantiate it.
#[derive(Clone, Copy)]
pub(crate) struct Ne3<const D: usize> {
    v: [uint8x16_t; D],
}

impl<const D: usize> Ne3<D> {
    #[inline(always)]
    fn round_nc(self) -> Self {
        let zero = unsafe { vdupq_n_u8(0) };
        let mut v = [zero; D];
        for lane in 0..D {
            v[lane] = unsafe { vaesmcq_u8(vaeseq_u8(self.v[lane], zero)) };
        }
        Self {
            v,
        }
    }
}

impl<const D: usize> BlockOps for Ne3<D> {
    const D: usize = D;

    // Same register-pressure argument as `Ne`.
    const SPLIT_BULK: bool = D == 1;

    #[inline(always)]
    fn zero() -> Self {
        Self {
            v: [unsafe { vdupq_n_u8(0) }; D],
        }
    }

    #[inline(always)]
    fn xor(self, rhs: Self) -> Self {
        let mut v = [unsafe { vdupq_n_u8(0) }; D];
        for lane in 0..D {
            v[lane] = unsafe { veorq_u8(self.v[lane], rhs.v[lane]) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    fn and(self, rhs: Self) -> Self {
        let mut v = [unsafe { vdupq_n_u8(0) }; D];
        for lane in 0..D {
            v[lane] = unsafe { vandq_u8(self.v[lane], rhs.v[lane]) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    fn xor3(self, b: Self, c: Self) -> Self {
        let mut v = [unsafe { vdupq_n_u8(0) }; D];
        for lane in 0..D {
            v[lane] = unsafe { veor3q_u8(self.v[lane], b.v[lane], c.v[lane]) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    fn round(self, rk: Self) -> Self {
        let nc = self.round_nc();
        let mut v = [unsafe { vdupq_n_u8(0) }; D];
        for lane in 0..D {
            v[lane] = unsafe { veorq_u8(nc.v[lane], rk.v[lane]) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    fn round_nc_with(self, key: Self) -> Self {
        let mut v = [unsafe { vdupq_n_u8(0) }; D];
        for lane in 0..D {
            v[lane] = unsafe { vaesmcq_u8(vaeseq_u8(self.v[lane], key.v[lane])) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    fn round_xor(self, rk: Self, m: Self) -> Self {
        let nc = self.round_nc();
        let mut v = [unsafe { vdupq_n_u8(0) }; D];
        for lane in 0..D {
            v[lane] = unsafe { veor3q_u8(nc.v[lane], rk.v[lane], m.v[lane]) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    unsafe fn load(ptr: *const u8) -> Self {
        let mut v = [unsafe { vdupq_n_u8(0) }; D];
        for lane in 0..D {
            // SAFETY: the caller guarantees `D * 16` readable bytes.
            v[lane] = unsafe { vld1q_u8(ptr.add(lane * 16)) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    unsafe fn store(self, ptr: *mut u8) {
        for lane in 0..D {
            // SAFETY: the caller guarantees `D * 16` writable bytes.
            unsafe { vst1q_u8(ptr.add(lane * 16), self.v[lane]) };
        }
    }

    #[inline(always)]
    fn broadcast(block: &[u8; 16]) -> Self {
        let b = unsafe { vld1q_u8(block.as_ptr()) };
        Self {
            v: [b; D],
        }
    }

    #[inline(always)]
    fn ctx_all() -> Self {
        let mut v = [unsafe { vdupq_n_u8(0) }; D];
        for lane in 0..D {
            let mut ctx = [0u8; 16];
            ctx[0] = lane as u8;
            ctx[1] = (D - 1) as u8;
            v[lane] = unsafe { vld1q_u8(ctx.as_ptr()) };
        }
        Self {
            v,
        }
    }
}

#[target_feature(enable = "aes,neon")]
pub(crate) unsafe fn init_state<const D: usize>(key: &[u8; 32], nonce: &[u8; 32], out: &mut [u8]) {
    crate::aegis::aegis_core::init_state::<Ne<D>>(key, nonce, out)
}

#[target_feature(enable = "aes,neon")]
#[inline(never)]
pub(crate) unsafe fn init_core<const D: usize>(key: &[u8; 32], nonce: &[u8; 32]) -> Core<Ne<D>> {
    Core::new(key, nonce)
}

#[target_feature(enable = "aes,neon")]
pub(crate) unsafe fn encrypt<const D: usize>(
    key: &[u8; 32],
    nonce: &[u8; 32],
    ad: &[u8],
    msg: &mut [u8],
    tag: &mut [u8],
) {
    let core = unsafe { init_core::<D>(key, nonce) };
    crate::aegis::aegis_core::encrypt_with_core(core, ad, msg, tag)
}

#[target_feature(enable = "aes,neon")]
pub(crate) unsafe fn decrypt<const D: usize>(
    key: &[u8; 32],
    nonce: &[u8; 32],
    ad: &[u8],
    msg: &mut [u8],
    tag: &[u8],
) -> bool {
    let core = unsafe { init_core::<D>(key, nonce) };
    crate::aegis::aegis_core::decrypt_with_core(core, ad, msg, tag)
}

#[target_feature(enable = "aes,neon")]
pub(crate) unsafe fn absorb_state<const D: usize>(bytes: &mut [u8], ad: &[u8]) {
    crate::aegis::aegis_core::absorb_state::<Ne<D>>(bytes, ad)
}

#[target_feature(enable = "aes,neon")]
pub(crate) unsafe fn finalize_mac_state<const D: usize>(bytes: &[u8], data_len: u64, tag: &mut [u8]) {
    crate::aegis::aegis_core::finalize_mac_state::<Ne<D>>(bytes, data_len, tag)
}

#[target_feature(enable = "aes,neon")]
pub(crate) unsafe fn next_keystream_state<const D: usize>(bytes: &mut [u8], out: &mut [u8]) {
    crate::aegis::aegis_core::next_keystream_state::<Ne<D>>(bytes, out)
}

#[target_feature(enable = "aes,neon")]
pub(crate) unsafe fn stream_state<const D: usize>(bytes: &mut [u8], in_out: &mut [u8]) {
    crate::aegis::aegis_core::stream_xor_state::<Ne<D>>(bytes, in_out)
}

#[target_feature(enable = "aes,neon")]
pub(crate) unsafe fn fill_state<const D: usize>(bytes: &mut [u8], out: &mut [u8]) {
    crate::aegis::aegis_core::stream_fill_state::<Ne<D>>(bytes, out)
}

#[target_feature(enable = "aes,neon,sha3")]
pub(crate) unsafe fn init_state_sha3<const D: usize>(key: &[u8; 32], nonce: &[u8; 32], out: &mut [u8]) {
    crate::aegis::aegis_core::init_state::<Ne3<D>>(key, nonce, out)
}

#[target_feature(enable = "aes,neon,sha3")]
#[inline(never)]
pub(crate) unsafe fn init_core_sha3<const D: usize>(key: &[u8; 32], nonce: &[u8; 32]) -> Core<Ne3<D>> {
    Core::new(key, nonce)
}

#[target_feature(enable = "aes,neon,sha3")]
pub(crate) unsafe fn encrypt_sha3<const D: usize>(
    key: &[u8; 32],
    nonce: &[u8; 32],
    ad: &[u8],
    msg: &mut [u8],
    tag: &mut [u8],
) {
    let core = unsafe { init_core_sha3::<D>(key, nonce) };
    crate::aegis::aegis_core::encrypt_with_core(core, ad, msg, tag)
}

#[target_feature(enable = "aes,neon,sha3")]
pub(crate) unsafe fn decrypt_sha3<const D: usize>(
    key: &[u8; 32],
    nonce: &[u8; 32],
    ad: &[u8],
    msg: &mut [u8],
    tag: &[u8],
) -> bool {
    let core = unsafe { init_core_sha3::<D>(key, nonce) };
    crate::aegis::aegis_core::decrypt_with_core(core, ad, msg, tag)
}

#[target_feature(enable = "aes,neon,sha3")]
pub(crate) unsafe fn absorb_state_sha3<const D: usize>(bytes: &mut [u8], ad: &[u8]) {
    crate::aegis::aegis_core::absorb_state::<Ne3<D>>(bytes, ad)
}

#[target_feature(enable = "aes,neon,sha3")]
pub(crate) unsafe fn finalize_mac_state_sha3<const D: usize>(bytes: &[u8], data_len: u64, tag: &mut [u8]) {
    crate::aegis::aegis_core::finalize_mac_state::<Ne3<D>>(bytes, data_len, tag)
}

#[target_feature(enable = "aes,neon,sha3")]
pub(crate) unsafe fn next_keystream_state_sha3<const D: usize>(bytes: &mut [u8], out: &mut [u8]) {
    crate::aegis::aegis_core::next_keystream_state::<Ne3<D>>(bytes, out)
}

#[target_feature(enable = "aes,neon,sha3")]
pub(crate) unsafe fn stream_state_sha3<const D: usize>(bytes: &mut [u8], in_out: &mut [u8]) {
    crate::aegis::aegis_core::stream_xor_state::<Ne3<D>>(bytes, in_out)
}

#[target_feature(enable = "aes,neon,sha3")]
pub(crate) unsafe fn fill_state_sha3<const D: usize>(bytes: &mut [u8], out: &mut [u8]) {
    crate::aegis::aegis_core::stream_fill_state::<Ne3<D>>(bytes, out)
}

/// Differential tests for the NEON / AES / SHA3 kernels.
///
/// Everything here compares a hardware kernel against the portable constant-time
/// backend ([`Soft`]), which is itself pinned by the RFC 10032 vectors in
/// `aegis::tests`. These tests reach the kernels directly, which matters because
/// public dispatch selects only *one* hardware backend per degree of
/// parallelism: on a CPU with AES and SHA3 the `Ne` kernels for `D = 1` and
/// `D = 2` are never selected, so the public API alone does not execute them.
#[cfg(all(test, feature = "alloc"))]
mod kernel_tests {
    use alloc::{vec, vec::Vec};

    use random::{RngExt, SeedableRng, rngs::ChaChaRng};

    use super::{
        Ne, Ne3, absorb_state, absorb_state_sha3, decrypt, decrypt_sha3, encrypt, encrypt_sha3, fill_state,
        fill_state_sha3, finalize_mac_state, finalize_mac_state_sha3, init_state, init_state_sha3,
        next_keystream_state, next_keystream_state_sha3, stream_state, stream_state_sha3,
    };
    use crate::aegis::{
        aegis_core::{self, BlockOps},
        aegis_soft::Soft,
    };

    const KEY: [u8; 32] = [0x42; 32];
    const NONCE: [u8; 32] = [0x24; 32];

    #[cfg(feature = "std")]
    fn has_aes() -> bool {
        std::arch::is_aarch64_feature_detected!("aes")
    }

    #[cfg(not(feature = "std"))]
    fn has_aes() -> bool {
        cfg!(target_feature = "aes")
    }

    #[cfg(feature = "std")]
    fn has_sha3() -> bool {
        std::arch::is_aarch64_feature_detected!("sha3")
    }

    #[cfg(not(feature = "std"))]
    fn has_sha3() -> bool {
        cfg!(target_feature = "sha3")
    }

    /// Deterministic seeded random byte pattern; the seed is forced odd so that no
    /// seed degenerates to the all-zero sequence.
    fn pattern(len: usize, seed: u8) -> Vec<u8> {
        let mut rng = ChaChaRng::seed_from_u64(u64::from(seed));
        let mut out = vec![0u8; len];
        rng.fill(&mut out);
        out
    }

    /// Store a block vector into a fresh buffer and return exactly `D * 16`
    /// bytes. Only NEON is required, which is baseline on aarch64.
    fn lanes<B: BlockOps>(block: B) -> Vec<u8> {
        let mut raw = vec![0u8; 4 * 16];
        // SAFETY: `raw` has room for the maximum of four lanes.
        unsafe { block.store(raw.as_mut_ptr()) };
        raw.truncate(B::D * 16);
        raw
    }

    fn soft_init<const D: usize>() -> Vec<u8> {
        let mut state = vec![0u8; 96 * D];
        aegis_core::init_state::<Soft<D>>(&KEY, &NONCE, &mut state);
        state
    }

    /// `load`/`store` must touch exactly `D * 16` bytes and must not require
    /// alignment (the pointer offsets used here are odd).
    fn load_store_exact<const D: usize, B: BlockOps>() {
        let len = D * 16;
        let payload = pattern(len, 0x21);
        for &in_off in &[0usize, 1, 3, 8, 15] {
            let mut src = vec![0xC3u8; in_off + len + 24];
            src[in_off..in_off + len].copy_from_slice(&payload);
            let before = src.clone();

            // SAFETY: `src` contains `len` readable bytes at `in_off`.
            let v = unsafe { B::load(src.as_ptr().add(in_off)) };
            // A load must not write anything, including the canaries.
            assert_eq!(src, before, "load modified its source (D={D})");

            for &out_off in &[0usize, 1, 5, 17] {
                let mut dst = vec![0xE7u8; out_off + len + 16];
                // SAFETY: `dst` contains `len` writable bytes at `out_off`.
                unsafe { v.store(dst.as_mut_ptr().add(out_off)) };
                assert!(dst[..out_off].iter().all(|&b| b == 0xE7), "pre-canary (D={D})");
                assert!(dst[out_off + len..].iter().all(|&b| b == 0xE7), "post-canary (D={D})");
                assert_eq!(&dst[out_off..out_off + len], &payload[..], "payload (D={D})");
            }
        }
    }

    #[test]
    fn load_store_touches_exact_bytes() {
        load_store_exact::<1, Ne<1>>();
        load_store_exact::<2, Ne<2>>();
        load_store_exact::<4, Ne<4>>();
        // `Ne3::load`/`store` only use NEON, no `eor3`.
        load_store_exact::<1, Ne3<1>>();
        load_store_exact::<2, Ne3<2>>();
        load_store_exact::<4, Ne3<4>>();
    }

    #[test]
    fn lane_layout_matches_core_contract() {
        fn check<const D: usize>() {
            // `ctx_all`: byte 0 is the lane index, byte 1 is `D - 1`, rest zero.
            let raw = lanes(Ne::<D>::ctx_all());
            for lane in 0..D {
                assert_eq!(raw[lane * 16], lane as u8);
                assert_eq!(raw[lane * 16 + 1], (D - 1) as u8);
                assert!(raw[lane * 16 + 2..(lane + 1) * 16].iter().all(|&b| b == 0));
            }
            assert_eq!(raw, lanes(Ne3::<D>::ctx_all()));

            // `broadcast`: every lane holds the same block.
            let block = [0x5Au8; 16];
            let raw = lanes(Ne::<D>::broadcast(&block));
            assert_eq!(raw.len(), D * 16);
            for lane in 0..D {
                assert_eq!(&raw[lane * 16..(lane + 1) * 16], &block);
            }
            assert_eq!(raw, lanes(Ne3::<D>::broadcast(&block)));

            // `zero` must not leak stack garbage into any lane.
            assert!(lanes(Ne::<D>::zero()).iter().all(|&b| b == 0));
            assert!(lanes(Ne3::<D>::zero()).iter().all(|&b| b == 0));
        }
        check::<1>();
        check::<2>();
        check::<4>();
    }

    /// AES round semantics, including the fused `round_nc_with` and the
    /// `eor3`-based `xor3`/`round_xor`, checked lane by lane against `Soft`.
    #[test]
    fn aes_ops_match_soft() {
        if !has_aes() {
            return;
        }
        fn check_ne<const D: usize>() {
            let a = pattern(D * 16, 0x11);
            let b = pattern(D * 16, 0x77);
            let c = pattern(D * 16, 0x7d);
            let va = unsafe { Ne::<D>::load(a.as_ptr()) };
            let vb = unsafe { Ne::<D>::load(b.as_ptr()) };
            let vc = unsafe { Ne::<D>::load(c.as_ptr()) };
            let sa = unsafe { Soft::<D>::load(a.as_ptr()) };
            let sb = unsafe { Soft::<D>::load(b.as_ptr()) };
            let sc = unsafe { Soft::<D>::load(c.as_ptr()) };

            assert_eq!(lanes(va.xor(vb)), lanes(sa.xor(sb)));
            assert_eq!(lanes(va.and(vb)), lanes(sa.and(sb)));
            assert_eq!(lanes(va.round(vb)), lanes(sa.round(sb)));
            assert_eq!(lanes(va.round_nc()), lanes(sa.round_nc()));
            assert_eq!(lanes(va.round_nc_with(vb)), lanes(sa.round_nc_with(sb)));
            assert_eq!(lanes(va.xor3(vb, vc)), lanes(sa.xor3(sb, sc)));
            assert_eq!(lanes(va.round_xor(vb, vc)), lanes(sa.round_xor(sb, sc)));
        }
        fn check_ne3<const D: usize>() {
            let a = pattern(D * 16, 0x11);
            let b = pattern(D * 16, 0x77);
            let c = pattern(D * 16, 0x7d);
            let va = unsafe { Ne3::<D>::load(a.as_ptr()) };
            let vb = unsafe { Ne3::<D>::load(b.as_ptr()) };
            let vc = unsafe { Ne3::<D>::load(c.as_ptr()) };
            let sa = unsafe { Soft::<D>::load(a.as_ptr()) };
            let sb = unsafe { Soft::<D>::load(b.as_ptr()) };
            let sc = unsafe { Soft::<D>::load(c.as_ptr()) };

            assert_eq!(lanes(va.xor(vb)), lanes(sa.xor(sb)));
            assert_eq!(lanes(va.and(vb)), lanes(sa.and(sb)));
            assert_eq!(lanes(va.round(vb)), lanes(sa.round(sb)));
            assert_eq!(lanes(va.round_nc()), lanes(sa.round_nc()));
            assert_eq!(lanes(va.round_nc_with(vb)), lanes(sa.round_nc_with(sb)));
            assert_eq!(lanes(va.xor3(vb, vc)), lanes(sa.xor3(sb, sc)));
            assert_eq!(lanes(va.round_xor(vb, vc)), lanes(sa.round_xor(sb, sc)));
        }
        check_ne::<1>();
        check_ne::<2>();
        check_ne::<4>();
        if has_sha3() {
            // `eor3` is the only SHA3 instruction used; it must be a plain
            // three-input XOR on every lane.
            check_ne3::<1>();
            check_ne3::<2>();
            check_ne3::<4>();
        }
    }

    fn check_init<const D: usize>() {
        if !has_aes() {
            return;
        }
        let len = 96 * D;
        let expected = soft_init::<D>();

        // Odd output offset: the state functions must not assume alignment and
        // must write exactly `96 * D` bytes.
        let mut hw = vec![0xA5u8; 3 + len + 5];
        unsafe { init_state::<D>(&KEY, &NONCE, &mut hw[3..3 + len]) };
        assert_eq!(&hw[3..3 + len], &expected[..], "init_state D={D}");
        assert!(hw[..3].iter().all(|&b| b == 0xA5));
        assert!(hw[3 + len..].iter().all(|&b| b == 0xA5));

        if has_sha3() {
            let mut hw3 = vec![0xFFu8; len];
            unsafe { init_state_sha3::<D>(&KEY, &NONCE, &mut hw3) };
            assert_eq!(hw3, expected, "init_state_sha3 D={D}");
        }
    }

    #[test]
    fn init_state_matches_soft() {
        check_init::<1>();
        check_init::<2>();
        check_init::<4>();
    }

    fn check_mac<const D: usize>(ad_len: usize, tag_len: usize) {
        if !has_aes() {
            return;
        }
        let ad = pattern(ad_len, 0x33);
        let expected_state = {
            let mut s = soft_init::<D>();
            aegis_core::absorb_state::<Soft<D>>(&mut s, &ad);
            s
        };

        let len = 96 * D;
        let mut hw = vec![0xA5u8; len + 2];
        // Start from the same serialized state the soft reference uses; this
        // isolates `absorb_state` from `init_state` (tested separately).
        hw[1..1 + len].copy_from_slice(&soft_init::<D>());
        unsafe { absorb_state::<D>(&mut hw[1..1 + len], &ad) };
        assert_eq!(&hw[1..1 + len], &expected_state[..], "absorb D={D} ad={ad_len}");
        assert_eq!(hw[0], 0xA5);
        assert_eq!(hw[len + 1], 0xA5);

        for &data_len in &[0u64, 1, 7, 1000, (ad_len as u64).wrapping_mul(8)] {
            let mut hw_tag = vec![0u8; tag_len];
            let mut sw_tag = vec![0u8; tag_len];
            unsafe { finalize_mac_state::<D>(&hw[1..1 + len], data_len, &mut hw_tag) };
            aegis_core::finalize_mac_state::<Soft<D>>(&expected_state, data_len, &mut sw_tag);
            assert_eq!(hw_tag, sw_tag, "finalize_mac D={D} ad={ad_len} tag={tag_len} len={data_len}");
        }

        if has_sha3() {
            let mut hw3 = soft_init::<D>();
            unsafe { absorb_state_sha3::<D>(&mut hw3, &ad) };
            assert_eq!(hw3, expected_state, "absorb_sha3 D={D} ad={ad_len}");
            for &data_len in &[0u64, 1, 7, 1000, (ad_len as u64).wrapping_mul(8)] {
                let mut hw_tag = vec![0u8; tag_len];
                let mut sw_tag = vec![0u8; tag_len];
                unsafe { finalize_mac_state_sha3::<D>(&hw3, data_len, &mut hw_tag) };
                aegis_core::finalize_mac_state::<Soft<D>>(&expected_state, data_len, &mut sw_tag);
                assert_eq!(hw_tag, sw_tag, "finalize_mac_sha3 D={D} ad={ad_len} tag={tag_len}");
            }
        }
    }

    #[test]
    fn mac_matches_soft_at_boundaries() {
        let mut ad_lens: Vec<usize> = (0..=64).collect();
        ad_lens.extend([127, 128, 129, 255, 256, 257, 1023, 1024, 1025, 1039, 1040, 1041, 4096]);
        for &ad_len in &ad_lens {
            check_mac::<1>(ad_len, 16);
            check_mac::<1>(ad_len, 32);
            check_mac::<2>(ad_len, 16);
            check_mac::<2>(ad_len, 32);
            check_mac::<4>(ad_len, 16);
            check_mac::<4>(ad_len, 32);
        }
    }

    /// One AEAD case through a concrete backend, compared against `Soft<D>`.
    fn check_aead<const D: usize>(
        hw_encrypt: unsafe fn(&[u8; 32], &[u8; 32], &[u8], &mut [u8], &mut [u8]),
        hw_decrypt: unsafe fn(&[u8; 32], &[u8; 32], &[u8], &mut [u8], &[u8]) -> bool,
        msg_len: usize,
        ad_len: usize,
        tag_len: usize,
        off: usize,
        label: &str,
    ) {
        if !has_aes() {
            return;
        }
        let ad = pattern(ad_len, 0x33);
        let plain = pattern(msg_len, 0x11);

        let mut expected = plain.clone();
        let mut expected_tag = vec![0u8; tag_len];
        aegis_core::encrypt::<Soft<D>>(&KEY, &NONCE, &ad, &mut expected, &mut expected_tag);

        // Guarded, non-zero-offset message buffer.
        let mut buf = vec![0xA5u8; off + msg_len + 11];
        buf[off..off + msg_len].copy_from_slice(&plain);
        let mut tag = vec![0u8; tag_len];
        unsafe { hw_encrypt(&KEY, &NONCE, &ad, &mut buf[off..off + msg_len], &mut tag) };
        assert_eq!(
            &buf[off..off + msg_len],
            &expected[..],
            "{label} D={D} msg={msg_len} ad={ad_len} tag={tag_len}"
        );
        assert_eq!(tag, expected_tag, "{label} tag D={D} msg={msg_len} ad={ad_len} tag={tag_len}");
        assert!(buf[..off].iter().all(|&b| b == 0xA5), "{label} pre-canary");
        assert!(buf[off + msg_len..].iter().all(|&b| b == 0xA5), "{label} post-canary");

        // Correct tag: decrypt must restore the plaintext in place.
        let mut dec = buf.clone();
        let ok = unsafe { hw_decrypt(&KEY, &NONCE, &ad, &mut dec[off..off + msg_len], &tag) };
        assert!(ok, "{label} decrypt rejected a valid tag (D={D} msg={msg_len})");
        assert_eq!(&dec[off..off + msg_len], &plain[..], "{label} plaintext (D={D} msg={msg_len})");
        assert!(dec[..off].iter().all(|&b| b == 0xA5));
        assert!(dec[off + msg_len..].iter().all(|&b| b == 0xA5));

        // Corrupt tag: must fail and zero the whole message buffer.
        let mut bad_tag = tag.clone();
        bad_tag[tag_len - 1] ^= 0x01;
        let mut dec = buf.clone();
        let ok = unsafe { hw_decrypt(&KEY, &NONCE, &ad, &mut dec[off..off + msg_len], &bad_tag) };
        assert!(!ok, "{label} accepted a corrupt tag (D={D} msg={msg_len})");
        assert!(
            dec[off..off + msg_len].iter().all(|&b| b == 0),
            "{label} did not zero plaintext on failure (D={D} msg={msg_len})"
        );
        assert!(dec[..off].iter().all(|&b| b == 0xA5), "{label} zeroing crossed the pre-canary");
        assert!(
            dec[off + msg_len..].iter().all(|&b| b == 0xA5),
            "{label} zeroing crossed the post-canary"
        );
    }

    fn aead_sweep<F: FnMut(usize, usize)>(mut f: F) {
        let mut msg_lens: Vec<usize> = (0..=64).collect();
        msg_lens.extend([127, 128, 129, 255, 256, 257, 1023, 1024, 1025, 4096]);
        let small_aad = [0usize, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65];
        for &msg in &msg_lens {
            for &ad in &small_aad {
                f(msg, ad);
            }
        }
        // AAD sweep around every block boundary while the message stays short.
        for &msg in &[0usize, 1, 15, 16, 17, 32, 33, 63, 64, 65, 127, 128, 129] {
            for ad in 0..=64 {
                f(msg, ad);
            }
        }
        // Large/large combinations.
        for &msg in &[255usize, 256, 257, 1023, 1024, 1025, 4096] {
            for &ad in &[0usize, 1, 64, 65, 127, 128, 129, 255, 256, 257, 1023, 1024, 1025, 4096] {
                f(msg, ad);
            }
        }
    }

    #[test]
    fn aead_ne_matches_soft() {
        aead_sweep(|msg, ad| {
            let off = (msg * 7 + ad * 3) % 9;
            check_aead::<1>(encrypt::<1>, decrypt::<1>, msg, ad, 16, off, "Ne");
            check_aead::<1>(encrypt::<1>, decrypt::<1>, msg, ad, 32, off, "Ne");
        });
        aead_sweep(|msg, ad| {
            let off = (msg * 5 + ad * 11) % 7;
            check_aead::<2>(encrypt::<2>, decrypt::<2>, msg, ad, 16, off, "Ne");
            check_aead::<2>(encrypt::<2>, decrypt::<2>, msg, ad, 32, off, "Ne");
        });
        aead_sweep(|msg, ad| {
            let off = (msg * 3 + ad * 13) % 5;
            check_aead::<4>(encrypt::<4>, decrypt::<4>, msg, ad, 16, off, "Ne");
            check_aead::<4>(encrypt::<4>, decrypt::<4>, msg, ad, 32, off, "Ne");
        });
    }

    #[test]
    fn aead_ne3_matches_soft() {
        if !has_sha3() {
            return;
        }
        aead_sweep(|msg, ad| {
            let off = (msg * 7 + ad * 3) % 9;
            check_aead::<1>(encrypt_sha3::<1>, decrypt_sha3::<1>, msg, ad, 16, off, "Ne3");
            check_aead::<1>(encrypt_sha3::<1>, decrypt_sha3::<1>, msg, ad, 32, off, "Ne3");
        });
        aead_sweep(|msg, ad| {
            let off = (msg * 5 + ad * 11) % 7;
            check_aead::<2>(encrypt_sha3::<2>, decrypt_sha3::<2>, msg, ad, 16, off, "Ne3");
            check_aead::<2>(encrypt_sha3::<2>, decrypt_sha3::<2>, msg, ad, 32, off, "Ne3");
        });
        // `Ne3<4>` is compiled by the generic dispatch arm but never selected at
        // runtime; still exercise it so `eor3` stays correct for all degrees.
        aead_sweep(|msg, ad| {
            let off = (msg * 3 + ad * 13) % 5;
            check_aead::<4>(encrypt_sha3::<4>, decrypt_sha3::<4>, msg, ad, 16, off, "Ne3");
            check_aead::<4>(encrypt_sha3::<4>, decrypt_sha3::<4>, msg, ad, 32, off, "Ne3");
        });
    }

    fn check_next_keystream<const D: usize>(out_len: usize) {
        if !has_aes() {
            return;
        }
        let expected_state = soft_init::<D>();
        let mut hw_state = expected_state.clone();
        let mut sw_state = expected_state.clone();
        let mut hw_out = vec![0xA5u8; out_len + 4];
        let mut sw_out = vec![0u8; out_len];
        unsafe { next_keystream_state::<D>(&mut hw_state, &mut hw_out[2..2 + out_len]) };
        aegis_core::next_keystream_state::<Soft<D>>(&mut sw_state, &mut sw_out[..out_len]);
        assert_eq!(&hw_out[2..2 + out_len], &sw_out[..], "next_keystream D={D} out={out_len}");
        assert!(hw_out[..2].iter().all(|&b| b == 0xA5));
        assert!(hw_out[2 + out_len..].iter().all(|&b| b == 0xA5));
        assert_eq!(hw_state, sw_state, "next_keystream state D={D} out={out_len}");

        if has_sha3() {
            let mut hw3 = expected_state.clone();
            let mut out3 = vec![0u8; out_len];
            unsafe { next_keystream_state_sha3::<D>(&mut hw3, &mut out3[..out_len]) };
            assert_eq!(out3, sw_out, "next_keystream_sha3 D={D} out={out_len}");
            assert_eq!(hw3, sw_state, "next_keystream_sha3 state D={D} out={out_len}");
        }
    }

    #[test]
    fn next_keystream_matches_soft() {
        fn all<const D: usize>() {
            for out_len in 1..=16 * D {
                check_next_keystream::<D>(out_len);
            }
            check_next_keystream::<D>(0);
        }
        all::<1>();
        all::<2>();
        all::<4>();
    }

    fn check_stream<const D: usize>(blocks: usize) {
        if !has_aes() {
            return;
        }
        let rate = 16 * D;
        let len = blocks * rate;
        let initial = soft_init::<D>();

        // `stream_state` XORs the keystream into a whole number of blocks.
        let mut hw_state = initial.clone();
        let mut sw_state = initial.clone();
        let input = pattern(len, 0x51);
        let mut hw = vec![0xA5u8; len + 3];
        hw[1..1 + len].copy_from_slice(&input);
        let mut sw = input.clone();
        unsafe { stream_state::<D>(&mut hw_state, &mut hw[1..1 + len]) };
        aegis_core::stream_xor_state::<Soft<D>>(&mut sw_state, &mut sw[..]);
        assert_eq!(&hw[1..1 + len], &sw[..], "stream_state D={D} blocks={blocks}");
        assert_eq!(hw[0], 0xA5);
        assert_eq!(hw[len + 2], 0xA5);
        assert_eq!(hw_state, sw_state, "stream_state state D={D} blocks={blocks}");

        // `fill_state` overwrites with the raw keystream.
        let mut hw_state = initial.clone();
        let mut sw_state = initial.clone();
        let mut hw = vec![0xE7u8; len + 5];
        let mut sw = vec![0u8; len];
        unsafe { fill_state::<D>(&mut hw_state, &mut hw[2..2 + len]) };
        aegis_core::stream_fill_state::<Soft<D>>(&mut sw_state, &mut sw[..]);
        assert_eq!(&hw[2..2 + len], &sw[..], "fill_state D={D} blocks={blocks}");
        assert!(hw[..2].iter().all(|&b| b == 0xE7));
        assert!(hw[2 + len..].iter().all(|&b| b == 0xE7));
        assert_eq!(hw_state, sw_state, "fill_state state D={D} blocks={blocks}");

        // The XOR of the fill output with the input must reproduce `stream_state`.
        let mut expected = vec![0u8; len];
        let mut fill_state_soft = initial.clone();
        aegis_core::stream_fill_state::<Soft<D>>(&mut fill_state_soft, &mut expected);
        let mut fed = pattern(len, 0x51);
        let mut xor_state = initial.clone();
        unsafe { stream_state::<D>(&mut xor_state, &mut fed) };
        for i in 0..len {
            assert_eq!(fed[i], input[i] ^ expected[i], "stream xor D={D} len={len}");
        }
        assert_eq!(xor_state, fill_state_soft, "stream_state/fill_state states disagree D={D}");

        if has_sha3() {
            let mut hw3_state = initial.clone();
            let mut hw3 = vec![0u8; len];
            unsafe { fill_state_sha3::<D>(&mut hw3_state, &mut hw3[..]) };
            assert_eq!(hw3, expected, "fill_state_sha3 D={D} blocks={blocks}");
            assert_eq!(hw3_state, fill_state_soft, "fill_state_sha3 state D={D} blocks={blocks}");

            let mut hw3_state = initial.clone();
            let mut xor3 = pattern(len, 0x51);
            unsafe { stream_state_sha3::<D>(&mut hw3_state, &mut xor3[..]) };
            let mut soft_state = initial.clone();
            let mut soft_xor = pattern(len, 0x51);
            aegis_core::stream_xor_state::<Soft<D>>(&mut soft_state, &mut soft_xor[..]);
            assert_eq!(xor3, soft_xor, "stream_state_sha3 D={D} blocks={blocks}");
            assert_eq!(hw3_state, soft_state, "stream_state_sha3 state D={D} blocks={blocks}");
        }
    }

    #[test]
    fn stream_state_matches_soft() {
        fn all<const D: usize>() {
            // 0, one rate, two rates, then the split-bulk threshold (1024) for
            // `Ne<1>` and around it, plus a long input.
            let rate = 16 * D;
            for &blocks in &[0usize, 1, 2, 1024 / rate, 1024 / rate + 1, 4096 / rate] {
                check_stream::<D>(blocks);
            }
            check_stream::<D>(1024_usize.div_ceil(rate));
            check_stream::<D>((1024 + rate) / rate);
        }
        all::<1>();
        all::<2>();
        all::<4>();
    }

    /// A stray 16-byte load past the end of a slice is invisible to guard bytes
    /// inside the same allocation, so this test places messages, AAD and stream
    /// buffers in exactly-sized heap allocations. Under ASan
    /// (`-Zsanitizer=address`) any over-read or over-write becomes a hard error;
    /// without ASan it still pins the exact-length results.
    #[test]
    fn exact_sized_inputs_have_no_slack() {
        if !has_aes() {
            return;
        }
        let mut lens: Vec<usize> = (0..=80).collect();
        lens.extend([127, 128, 129, 255, 256, 257, 1023, 1024, 1025]);

        fn check_ne<const D: usize>(lens: &[usize]) {
            for &len in lens {
                for &ad_len in &[0usize, 1, 15, 16, 17, 63, 64, 65] {
                    let ad = pattern(ad_len, 0x33);
                    let plain = pattern(len, 0x11);
                    let mut msg = plain.clone();
                    let mut tag = vec![0u8; 16];
                    unsafe { encrypt::<D>(&KEY, &NONCE, &ad[..], &mut msg[..], &mut tag) };
                    assert!(unsafe { decrypt::<D>(&KEY, &NONCE, &ad[..], &mut msg[..], &tag) });
                    assert_eq!(msg, plain, "D={D} len={len} ad={ad_len}");
                }
            }
            let rate = 16 * D;
            for blocks in [1usize, 2, 64] {
                let len = blocks * rate;
                let mut state = soft_init::<D>();
                let mut io = pattern(len, 0x51);
                unsafe { stream_state::<D>(&mut state, &mut io[..]) };
                let mut state = soft_init::<D>();
                let mut out = vec![0u8; len];
                unsafe { fill_state::<D>(&mut state, &mut out[..]) };
            }
        }

        fn check_ne3<const D: usize>(lens: &[usize]) {
            for &len in lens {
                for &ad_len in &[0usize, 1, 15, 16, 17, 63, 64, 65] {
                    let ad = pattern(ad_len, 0x33);
                    let plain = pattern(len, 0x11);
                    let mut msg = plain.clone();
                    let mut tag = vec![0u8; 16];
                    unsafe { encrypt_sha3::<D>(&KEY, &NONCE, &ad[..], &mut msg[..], &mut tag) };
                    assert!(unsafe { decrypt_sha3::<D>(&KEY, &NONCE, &ad[..], &mut msg[..], &tag) });
                    assert_eq!(msg, plain, "D={D} len={len} ad={ad_len}");
                }
            }
            let rate = 16 * D;
            for blocks in [1usize, 2, 64] {
                let len = blocks * rate;
                let mut state = soft_init::<D>();
                let mut io = pattern(len, 0x51);
                unsafe { stream_state_sha3::<D>(&mut state, &mut io[..]) };
                let mut state = soft_init::<D>();
                let mut out = vec![0u8; len];
                unsafe { fill_state_sha3::<D>(&mut state, &mut out[..]) };
            }
        }

        check_ne::<1>(&lens);
        check_ne::<2>(&lens);
        check_ne::<4>(&lens);
        if has_sha3() {
            check_ne3::<1>(&lens);
            check_ne3::<2>(&lens);
            check_ne3::<4>(&lens);
        }
    }

    /// The serialized-state contract is `assert_eq!`, not a silent truncation:
    /// a short buffer must panic before any kernel writes to it.
    #[cfg(feature = "std")]
    #[test]
    fn init_state_rejects_short_output() {
        if !has_aes() {
            return;
        }
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let mut short = [0xA5u8; 95];
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            init_state::<1>(&KEY, &NONCE, &mut short[..])
        }));
        std::panic::set_hook(prev);
        assert!(result.is_err(), "init_state accepted a 95-byte state buffer");
        assert!(
            short.iter().all(|&b| b == 0xA5),
            "init_state wrote before validating the length"
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn state_functions_reject_short_input() {
        if !has_aes() {
            return;
        }
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let mut short = [0xA5u8; 6 * 16 - 1];
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            absorb_state::<1>(&mut short[..], b"aad")
        }));
        std::panic::set_hook(prev);
        assert!(result.is_err(), "absorb_state accepted a short state buffer");
        assert!(
            short.iter().all(|&b| b == 0xA5),
            "absorb_state wrote before validating the length"
        );
    }

    /// Dispatch must never hand `D > 2` to the `eor3` backend, and may only pick
    /// a hardware backend when its features are present.
    #[cfg(feature = "std")]
    #[test]
    fn dispatch_matches_runtime_features() {
        use crate::aegis::{Backend, detect};

        let expected = |d: usize| {
            if !has_aes() {
                Backend::Soft
            } else if d <= 2 && has_sha3() {
                Backend::ArmSha3
            } else {
                Backend::Arm
            }
        };
        assert_eq!(detect::<1>(), expected(1));
        assert_eq!(detect::<2>(), expected(2));
        assert_eq!(detect::<4>(), expected(4));
        // On a CPU with AES but without SHA3, D = 1/2 must use plain NEON+AES.
        if has_aes() && !has_sha3() {
            assert_eq!(detect::<1>(), Backend::Arm);
            assert_eq!(detect::<2>(), Backend::Arm);
        }
    }

    #[cfg(not(feature = "std"))]
    #[test]
    fn dispatch_matches_compile_time_features() {
        use crate::aegis::{Backend, detect};

        let expected = |d: usize| {
            if !cfg!(target_feature = "aes") {
                Backend::Soft
            } else if d <= 2 && cfg!(target_feature = "sha3") {
                Backend::ArmSha3
            } else {
                Backend::Arm
            }
        };
        assert_eq!(detect::<1>(), expected(1));
        assert_eq!(detect::<2>(), expected(2));
        assert_eq!(detect::<4>(), expected(4));
    }
}
