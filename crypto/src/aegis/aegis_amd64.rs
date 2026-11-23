//! x86-64 backends: AES-NI, VAES-256 and VAES-512.
//!
//! AEGIS-256 (D = 1) is inherently serial, so it uses plain AES-NI. AEGIS-256X2
//! (D = 2) stores two independent AEGIS states in the 128-bit lanes of a
//! `__m256i` when VAES is available, and AEGIS-256X4 (D = 4) uses four lanes in
//! a `__m512i` (or two `__m256i` registers when AVX-512 is unavailable).
//!
//! The whole operation is wrapped in `#[target_feature]` functions: the shared
//! core is `#[inline(always)]`, so this is what lets LLVM schedule the AES
//! instructions instead of emitting out-of-line calls.

use core::arch::x86_64::*;

use crate::aegis::aegis_core::{BlockOps, Core};

#[derive(Clone, Copy)]
pub(crate) struct AesNi<const D: usize> {
    v: [__m128i; D],
}

impl<const D: usize> BlockOps for AesNi<D> {
    const D: usize = D;

    // SPLIT_BULK stays disabled: x86-64 has 16 xmm registers and the split
    // loop needs 12 live state vectors plus temporaries. The packed VAES
    // implementations use one register per state block and can enable it.
    const SPLIT_BULK: bool = false;

    #[inline(always)]
    fn zero() -> Self {
        Self {
            v: [unsafe { _mm_setzero_si128() }; D],
        }
    }

    #[inline(always)]
    fn xor(self, rhs: Self) -> Self {
        let mut v = [unsafe { _mm_setzero_si128() }; D];
        for lane in 0..D {
            v[lane] = unsafe { _mm_xor_si128(self.v[lane], rhs.v[lane]) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    fn and(self, rhs: Self) -> Self {
        let mut v = [unsafe { _mm_setzero_si128() }; D];
        for lane in 0..D {
            v[lane] = unsafe { _mm_and_si128(self.v[lane], rhs.v[lane]) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    fn round(self, rk: Self) -> Self {
        let mut v = [unsafe { _mm_setzero_si128() }; D];
        for lane in 0..D {
            v[lane] = unsafe { _mm_aesenc_si128(self.v[lane], rk.v[lane]) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    unsafe fn load(ptr: *const u8) -> Self {
        let mut v = [unsafe { _mm_setzero_si128() }; D];
        for lane in 0..D {
            // SAFETY: the caller guarantees `D * 16` readable bytes.
            v[lane] = unsafe { _mm_loadu_si128(ptr.add(lane * 16).cast()) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    unsafe fn store(self, ptr: *mut u8) {
        for lane in 0..D {
            // SAFETY: the caller guarantees `D * 16` writable bytes.
            unsafe { _mm_storeu_si128(ptr.add(lane * 16).cast(), self.v[lane]) };
        }
    }

    #[inline(always)]
    fn broadcast(block: &[u8; 16]) -> Self {
        let b = unsafe { _mm_loadu_si128(block.as_ptr().cast()) };
        Self {
            v: [b; D],
        }
    }

    #[inline(always)]
    fn ctx_all() -> Self {
        const {
            assert!(D <= 4, "AEGIS degree must be at most 4");
        }
        let mut raw = [0u8; 64];
        for lane in 0..D {
            raw[lane * 16] = lane as u8;
            raw[lane * 16 + 1] = (D - 1) as u8;
        }
        // SAFETY: `raw` is at least `D * 16` bytes, and `D <= 4`.
        unsafe { Self::load(raw.as_ptr()) }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Vaes256<const L: usize> {
    v: [__m256i; L],
}

impl<const L: usize> BlockOps for Vaes256<L> {
    const D: usize = 2 * L;

    // The split loop fits in registers for L = 1 (12 ymm), but the x86
    // backend does not fuse the add-round-key into the AESENC input, so the
    // benefit must be measured on native AVX2 hardware before enabling it.
    // The generic split code is ready; flip this after benchmarking.
    const SPLIT_BULK: bool = false;

    #[inline(always)]
    fn zero() -> Self {
        Self {
            v: [unsafe { _mm256_setzero_si256() }; L],
        }
    }

    #[inline(always)]
    fn xor(self, rhs: Self) -> Self {
        let mut v = [unsafe { _mm256_setzero_si256() }; L];
        for lane in 0..L {
            v[lane] = unsafe { _mm256_xor_si256(self.v[lane], rhs.v[lane]) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    fn and(self, rhs: Self) -> Self {
        let mut v = [unsafe { _mm256_setzero_si256() }; L];
        for lane in 0..L {
            v[lane] = unsafe { _mm256_and_si256(self.v[lane], rhs.v[lane]) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    fn round(self, rk: Self) -> Self {
        let mut v = [unsafe { _mm256_setzero_si256() }; L];
        for lane in 0..L {
            v[lane] = unsafe { _mm256_aesenc_epi128(self.v[lane], rk.v[lane]) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    unsafe fn load(ptr: *const u8) -> Self {
        let mut v = [unsafe { _mm256_setzero_si256() }; L];
        for lane in 0..L {
            // SAFETY: the caller guarantees `2 * L * 16` readable bytes.
            v[lane] = unsafe { _mm256_loadu_si256(ptr.add(lane * 32).cast()) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    unsafe fn store(self, ptr: *mut u8) {
        for lane in 0..L {
            // SAFETY: the caller guarantees `2 * L * 16` writable bytes.
            unsafe { _mm256_storeu_si256(ptr.add(lane * 32).cast(), self.v[lane]) };
        }
    }

    #[inline(always)]
    fn broadcast(block: &[u8; 16]) -> Self {
        let b = unsafe { _mm256_broadcastsi128_si256(_mm_loadu_si128(block.as_ptr().cast())) };
        Self {
            v: [b; L],
        }
    }

    #[inline(always)]
    fn ctx_all() -> Self {
        const {
            assert!(2 * L <= 4, "AEGIS degree must be at most 4");
        }
        let mut raw = [0u8; 64];
        for lane in 0..2 * L {
            raw[lane * 16] = lane as u8;
            raw[lane * 16 + 1] = (2 * L - 1) as u8;
        }
        // SAFETY: `raw` is at least `2 * L * 16` bytes, and `2 * L <= 4`.
        unsafe { Self::load(raw.as_ptr()) }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Vaes512<const L: usize> {
    v: [__m512i; L],
}

impl<const L: usize> BlockOps for Vaes512<L> {
    const D: usize = 4 * L;

    // Current measurements on Zen 5 (VAES-512) indicates that enabling `SPLIT_BULK` leads to a ~10%
    // regression in performance.
    const SPLIT_BULK: bool = false;

    #[inline(always)]
    fn zero() -> Self {
        Self {
            v: [unsafe { _mm512_setzero_si512() }; L],
        }
    }

    #[inline(always)]
    fn xor(self, rhs: Self) -> Self {
        let mut v = [unsafe { _mm512_setzero_si512() }; L];
        for lane in 0..L {
            v[lane] = unsafe { _mm512_xor_si512(self.v[lane], rhs.v[lane]) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    fn and(self, rhs: Self) -> Self {
        let mut v = [unsafe { _mm512_setzero_si512() }; L];
        for lane in 0..L {
            v[lane] = unsafe { _mm512_and_si512(self.v[lane], rhs.v[lane]) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    fn round(self, rk: Self) -> Self {
        let mut v = [unsafe { _mm512_setzero_si512() }; L];
        for lane in 0..L {
            v[lane] = unsafe { _mm512_aesenc_epi128(self.v[lane], rk.v[lane]) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    unsafe fn load(ptr: *const u8) -> Self {
        let mut v = [unsafe { _mm512_setzero_si512() }; L];
        for lane in 0..L {
            // SAFETY: the caller guarantees `4 * L * 16` readable bytes.
            v[lane] = unsafe { _mm512_loadu_si512(ptr.add(lane * 64).cast()) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    unsafe fn store(self, ptr: *mut u8) {
        for lane in 0..L {
            // SAFETY: the caller guarantees `4 * L * 16` writable bytes.
            unsafe { _mm512_storeu_si512(ptr.add(lane * 64).cast(), self.v[lane]) };
        }
    }

    #[inline(always)]
    fn broadcast(block: &[u8; 16]) -> Self {
        let b = unsafe { _mm512_broadcast_i32x4(_mm_loadu_si128(block.as_ptr().cast())) };
        Self {
            v: [b; L],
        }
    }

    #[inline(always)]
    fn ctx_all() -> Self {
        const {
            assert!(4 * L <= 4, "AEGIS degree must be at most 4");
        }
        let mut raw = [0u8; 64];
        for lane in 0..4 * L {
            raw[lane * 16] = lane as u8;
            raw[lane * 16 + 1] = (4 * L - 1) as u8;
        }
        // SAFETY: `raw` is at least `4 * L * 16` bytes, and `4 * L <= 4`.
        unsafe { Self::load(raw.as_ptr()) }
    }
}

macro_rules! aead_impl {
    ($init_core:ident, $encrypt:ident, $decrypt:ident, $feature:literal, $block:ty) => {
        #[target_feature(enable = $feature)]
        #[inline(never)]
        pub(crate) unsafe fn $init_core(key: &[u8; 32], nonce: &[u8; 32]) -> Core<$block> {
            Core::new(key, nonce)
        }

        #[target_feature(enable = $feature)]
        pub(crate) unsafe fn $encrypt(key: &[u8; 32], nonce: &[u8; 32], ad: &[u8], msg: &mut [u8], tag: &mut [u8]) {
            let core = unsafe { $init_core(key, nonce) };
            crate::aegis::aegis_core::encrypt_with_core(core, ad, msg, tag)
        }

        #[target_feature(enable = $feature)]
        pub(crate) unsafe fn $decrypt(key: &[u8; 32], nonce: &[u8; 32], ad: &[u8], msg: &mut [u8], tag: &[u8]) -> bool {
            let core = unsafe { $init_core(key, nonce) };
            crate::aegis::aegis_core::decrypt_with_core(core, ad, msg, tag)
        }
    };
}

macro_rules! state_impl {
    (
        $init:ident,
        $absorb:ident,
        $finalize:ident,
        $next:ident,
        $stream:ident,
        $fill:ident,
        $feature:literal,
        $block:ty
    ) => {
        #[target_feature(enable = $feature)]
        pub(crate) unsafe fn $init(key: &[u8; 32], nonce: &[u8; 32], out: &mut [u8]) {
            crate::aegis::aegis_core::init_state::<$block>(key, nonce, out)
        }

        #[target_feature(enable = $feature)]
        pub(crate) unsafe fn $absorb(bytes: &mut [u8], ad: &[u8]) {
            crate::aegis::aegis_core::absorb_state::<$block>(bytes, ad)
        }

        #[target_feature(enable = $feature)]
        pub(crate) unsafe fn $finalize(bytes: &[u8], data_len: u64, tag: &mut [u8]) {
            crate::aegis::aegis_core::finalize_mac_state::<$block>(bytes, data_len, tag)
        }

        #[target_feature(enable = $feature)]
        pub(crate) unsafe fn $next(bytes: &mut [u8], out: &mut [u8]) {
            crate::aegis::aegis_core::next_keystream_state::<$block>(bytes, out)
        }

        #[target_feature(enable = $feature)]
        pub(crate) unsafe fn $stream(bytes: &mut [u8], in_out: &mut [u8]) {
            crate::aegis::aegis_core::stream_xor_state::<$block>(bytes, in_out)
        }

        #[target_feature(enable = $feature)]
        pub(crate) unsafe fn $fill(bytes: &mut [u8], out: &mut [u8]) {
            crate::aegis::aegis_core::stream_fill_state::<$block>(bytes, out)
        }
    };
}

aead_impl!(init_core_aesni_1, encrypt_aesni_1, decrypt_aesni_1, "aes", AesNi<1>);
aead_impl!(init_core_aesni_2, encrypt_aesni_2, decrypt_aesni_2, "aes", AesNi<2>);
aead_impl!(init_core_aesni_4, encrypt_aesni_4, decrypt_aesni_4, "aes", AesNi<4>);
aead_impl!(
    init_core_vaes256_2,
    encrypt_vaes256_2,
    decrypt_vaes256_2,
    "vaes,avx2",
    Vaes256<1>
);
aead_impl!(
    init_core_vaes256_4,
    encrypt_vaes256_4,
    decrypt_vaes256_4,
    "vaes,avx2",
    Vaes256<2>
);
aead_impl!(
    init_core_vaes512_4,
    encrypt_vaes512_4,
    decrypt_vaes512_4,
    "vaes,avx512f,avx512bw,avx2",
    Vaes512<1>
);

state_impl!(
    init_aesni_1,
    absorb_aesni_1,
    finalize_aesni_1,
    next_aesni_1,
    stream_aesni_1,
    fill_aesni_1,
    "aes",
    AesNi<1>
);
state_impl!(
    init_aesni_2,
    absorb_aesni_2,
    finalize_aesni_2,
    next_aesni_2,
    stream_aesni_2,
    fill_aesni_2,
    "aes",
    AesNi<2>
);
state_impl!(
    init_aesni_4,
    absorb_aesni_4,
    finalize_aesni_4,
    next_aesni_4,
    stream_aesni_4,
    fill_aesni_4,
    "aes",
    AesNi<4>
);
state_impl!(
    init_vaes256_2,
    absorb_vaes256_2,
    finalize_vaes256_2,
    next_vaes256_2,
    stream_vaes256_2,
    fill_vaes256_2,
    "vaes,avx2",
    Vaes256<1>
);
state_impl!(
    init_vaes256_4,
    absorb_vaes256_4,
    finalize_vaes256_4,
    next_vaes256_4,
    stream_vaes256_4,
    fill_vaes256_4,
    "vaes,avx2",
    Vaes256<2>
);
state_impl!(
    init_vaes512_4,
    absorb_vaes512_4,
    finalize_vaes512_4,
    next_vaes512_4,
    stream_vaes512_4,
    fill_vaes512_4,
    "vaes,avx512f,avx512bw,avx2",
    Vaes512<1>
);

#[cfg(all(test, feature = "std"))]
mod kernel_tests {
    //! Independent checks for the x86-64 AEGIS kernels.
    //!
    //! Each hardware backends is compared against the portable `aegis_soft`
    //! backend, both at the `BlockOps` level (load/store/round/xor/and/fold/
    //! broadcast/ctx_all) and through every macro-generated wrapper that
    //! `aegis/mod.rs` dispatches to. Buffers are placed at non-zero offsets
    //! inside canary allocations so an over-read or over-write is visible.
    //!
    //! These tests need a CPU (or emulator) with the corresponding features;
    //! they return early when detection says the feature is absent.

    use alloc::{vec, vec::Vec};

    use super::*;
    use crate::aegis::{
        aegis_core::{
            absorb_state, finalize_mac_state, init_state, next_keystream_state, stream_fill_state, stream_xor_state,
        },
        aegis_soft::Soft,
    };

    const CANARY: u8 = 0xa5;

    fn bytes_of<B: BlockOps>(b: B) -> Vec<u8> {
        let mut out = vec![0u8; 16 * B::D];
        // SAFETY: `out` is exactly `D * 16` bytes.
        unsafe { b.store(out.as_mut_ptr()) };
        out
    }

    fn pat(len: usize, mul: u8, add: u8) -> Vec<u8> {
        (0..len)
            .map(|i| (i as u8).wrapping_mul(mul).wrapping_add(add))
            .collect()
    }

    fn rand(st: &mut u32, out: &mut [u8]) {
        for b in out.iter_mut() {
            *st ^= *st << 13;
            *st ^= *st >> 17;
            *st ^= *st << 5;
            *b = *st as u8;
        }
    }

    macro_rules! block_ops_check {
        ($t:ty) => {{
            const D: usize = <$t as BlockOps>::D;
            let mut st = 0x9e37_79b9u32;
            for _ in 0..128 {
                let mut a = vec![0u8; 16 * D];
                let mut k = vec![0u8; 16 * D];
                rand(&mut st, &mut a);
                rand(&mut st, &mut k);

                // Unaligned load/store at a non-zero offset with canaries.
                let pad = 3;
                let mut src = vec![CANARY; 16 * D + 2 * pad];
                src[pad..pad + 16 * D].copy_from_slice(&a);
                // SAFETY: the slice holds `16 * D` readable bytes.
                let x = unsafe { <$t as BlockOps>::load(src[pad..].as_ptr()) };
                let mut dst = vec![CANARY; 16 * D + 2 * pad];
                // SAFETY: the slice holds `16 * D` writable bytes.
                unsafe { x.store(dst[pad..].as_mut_ptr()) };
                assert_eq!(&dst[pad..pad + 16 * D], &a[..]);
                assert!(dst[..pad].iter().all(|&b| b == CANARY));
                assert!(dst[pad + 16 * D..].iter().all(|&b| b == CANARY));

                // round / xor / and must match lane-wise software AES.
                // SAFETY: 16 * D readable bytes in `k`.
                let khw = unsafe { <$t as BlockOps>::load(k.as_ptr()) };
                // SAFETY: 16 * D readable bytes in `a`.
                let asoft = unsafe { Soft::<D>::load(a.as_ptr()) };
                // SAFETY: likewise.
                let ksoft = unsafe { Soft::<D>::load(k.as_ptr()) };
                assert_eq!(bytes_of(x.round(khw)), bytes_of(asoft.round(ksoft)), "round D={D}");
                assert_eq!(bytes_of(x.xor(khw)), bytes_of(asoft.xor(ksoft)), "xor D={D}");
                assert_eq!(bytes_of(x.and(khw)), bytes_of(asoft.and(ksoft)), "and D={D}");
                assert_eq!(bytes_of(x), bytes_of(asoft), "load D={D}");

                // fold XORs all lanes.
                let folded = x.fold();
                let mut want = [0u8; 16];
                for lane in 0..D {
                    for i in 0..16 {
                        want[i] ^= a[lane * 16 + i];
                    }
                }
                assert_eq!(folded, want, "fold D={D}");

                // broadcast repeats one block in every lane.
                let block = [0x5au8; 16];
                let mut want = vec![0u8; 16 * D];
                for lane in 0..D {
                    want[lane * 16..(lane + 1) * 16].copy_from_slice(&block);
                }
                assert_eq!(bytes_of(<$t as BlockOps>::broadcast(&block)), want, "broadcast D={D}");

                // ctx_all must be { lane, D - 1, 0, ... } per lane.
                let mut want = vec![0u8; 16 * D];
                for lane in 0..D {
                    want[lane * 16] = lane as u8;
                    want[lane * 16 + 1] = (D - 1) as u8;
                }
                assert_eq!(bytes_of(<$t as BlockOps>::ctx_all()), want, "ctx_all D={D}");
                assert!(bytes_of(<$t as BlockOps>::zero()).iter().all(|&b| b == 0), "zero D={D}");
            }
        }};
    }

    macro_rules! wrapper_check {
        ($d:expr, $enc:path, $dec:path, $init:path, $absorb:path, $fin:path, $next:path, $stream:path, $fill:path) => {{
            const D: usize = $d;
            let key = [0x42u8; 32];
            let nonce = [0x24u8; 32];
            let rate = 16 * D;

            for &len in &[
                0usize, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 1023, 1024, 1025,
            ] {
                for &ad_len in &[0usize, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 1023, 1024, 1025] {
                    let ad = pat(ad_len, 13, 9);
                    let pt = pat(len, 17, 5);
                    for tag_len in [16usize, 32] {
                        let mut want_ct = pt.clone();
                        let mut core = Core::<Soft<D>>::new(&key, &nonce);
                        core.absorb(&ad);
                        core.encrypt_msg(&mut want_ct);
                        let mut want_tag = vec![0u8; tag_len];
                        core.finalize(ad.len(), len, &mut want_tag);

                        let mut got_ct = pt.clone();
                        let mut got_tag = vec![0u8; tag_len];
                        // SAFETY: features checked by the caller.
                        unsafe { $enc(&key, &nonce, &ad, &mut got_ct, &mut got_tag) };
                        assert_eq!(got_ct, want_ct, "enc D={D} len={len} ad={ad_len} tag={tag_len}");
                        assert_eq!(got_tag, want_tag, "tag D={D} len={len} ad={ad_len} tag={tag_len}");

                        let mut dec = got_ct.clone();
                        // SAFETY: features checked by the caller.
                        let ok = unsafe { $dec(&key, &nonce, &ad, &mut dec, &got_tag) };
                        assert!(ok, "decrypt rejected valid tag D={D} len={len} ad={ad_len} tag={tag_len}");
                        assert_eq!(dec, pt, "dec D={D} len={len} ad={ad_len}");

                        let mut bad = got_tag.clone();
                        bad[0] ^= 1;
                        let mut tampered = got_ct;
                        // SAFETY: features checked by the caller.
                        let ok = unsafe { $dec(&key, &nonce, &ad, &mut tampered, &bad) };
                        assert!(!ok, "tampered tag accepted D={D} len={len}");
                        assert!(tampered.iter().all(|&b| b == 0), "tampered output not zeroed D={D}");
                    }
                }
            }

            // Serialized-state entry points, including canaries around the
            // caller-owned state buffer.
            let size = 96 * D;
            let pad = 5;
            let mut hw = vec![CANARY; size + 2 * pad];
            let mut sw = vec![0u8; size];
            // SAFETY: features checked by the caller; slice is `96 * D` bytes.
            unsafe { $init(&key, &nonce, &mut hw[pad..pad + size]) };
            init_state::<Soft<D>>(&key, &nonce, &mut sw);
            assert_eq!(&hw[pad..pad + size], &sw[..], "init D={D}");
            assert!(hw[..pad].iter().all(|&b| b == CANARY));
            assert!(hw[pad + size..].iter().all(|&b| b == CANARY));

            let ad = pat(37, 3, 1);
            // SAFETY: features checked by the caller.
            unsafe { $absorb(&mut hw[pad..pad + size], &ad) };
            absorb_state::<Soft<D>>(&mut sw, &ad);
            assert_eq!(&hw[pad..pad + size], &sw[..], "absorb_state D={D}");

            let mut tag_hw = vec![0u8; 32];
            let mut tag_sw = vec![0u8; 32];
            // SAFETY: features checked by the caller.
            unsafe { $fin(&hw[pad..pad + size], 1234, &mut tag_hw) };
            finalize_mac_state::<Soft<D>>(&sw, 1234, &mut tag_sw);
            assert_eq!(tag_hw, tag_sw, "finalize_mac_state D={D}");

            let mut state_hw = hw[pad..pad + size].to_vec();
            let mut state_sw = sw.clone();
            let mut out_hw = vec![0u8; rate];
            let mut out_sw = vec![0u8; rate];
            // SAFETY: features checked by the caller.
            unsafe { $next(&mut state_hw, &mut out_hw) };
            next_keystream_state::<Soft<D>>(&mut state_sw, &mut out_sw);
            assert_eq!(out_hw, out_sw, "next_keystream_state D={D}");
            assert_eq!(state_hw, state_sw, "next_keystream_state state D={D}");

            let mut stream_hw = pat(3 * rate, 11, 2);
            let mut stream_sw = stream_hw.clone();
            // SAFETY: features checked by the caller; length is 3 * rate.
            unsafe { $stream(&mut state_hw, &mut stream_hw) };
            stream_xor_state::<Soft<D>>(&mut state_sw, &mut stream_sw);
            assert_eq!(stream_hw, stream_sw, "stream_state D={D}");

            let mut fill_hw = vec![0u8; 2 * rate];
            let mut fill_sw = vec![0u8; 2 * rate];
            // SAFETY: features checked by the caller; length is 2 * rate.
            unsafe { $fill(&mut state_hw, &mut fill_hw) };
            stream_fill_state::<Soft<D>>(&mut state_sw, &mut fill_sw);
            assert_eq!(fill_hw, fill_sw, "fill_state D={D}");
        }};
    }

    #[target_feature(enable = "aes")]
    unsafe fn aesni_checks() {
        block_ops_check!(AesNi<1>);
        block_ops_check!(AesNi<2>);
        block_ops_check!(AesNi<4>);
        wrapper_check!(
            1,
            encrypt_aesni_1,
            decrypt_aesni_1,
            init_aesni_1,
            absorb_aesni_1,
            finalize_aesni_1,
            next_aesni_1,
            stream_aesni_1,
            fill_aesni_1
        );
        wrapper_check!(
            2,
            encrypt_aesni_2,
            decrypt_aesni_2,
            init_aesni_2,
            absorb_aesni_2,
            finalize_aesni_2,
            next_aesni_2,
            stream_aesni_2,
            fill_aesni_2
        );
        wrapper_check!(
            4,
            encrypt_aesni_4,
            decrypt_aesni_4,
            init_aesni_4,
            absorb_aesni_4,
            finalize_aesni_4,
            next_aesni_4,
            stream_aesni_4,
            fill_aesni_4
        );
    }

    #[target_feature(enable = "vaes,avx2")]
    unsafe fn vaes256_checks() {
        block_ops_check!(Vaes256<1>);
        block_ops_check!(Vaes256<2>);
        wrapper_check!(
            2,
            encrypt_vaes256_2,
            decrypt_vaes256_2,
            init_vaes256_2,
            absorb_vaes256_2,
            finalize_vaes256_2,
            next_vaes256_2,
            stream_vaes256_2,
            fill_vaes256_2
        );
        wrapper_check!(
            4,
            encrypt_vaes256_4,
            decrypt_vaes256_4,
            init_vaes256_4,
            absorb_vaes256_4,
            finalize_vaes256_4,
            next_vaes256_4,
            stream_vaes256_4,
            fill_vaes256_4
        );
    }

    #[target_feature(enable = "vaes,avx512f,avx512bw,avx2")]
    unsafe fn vaes512_checks() {
        block_ops_check!(Vaes512<1>);
        wrapper_check!(
            4,
            encrypt_vaes512_4,
            decrypt_vaes512_4,
            init_vaes512_4,
            absorb_vaes512_4,
            finalize_vaes512_4,
            next_vaes512_4,
            stream_vaes512_4,
            fill_vaes512_4
        );
    }

    #[test]
    fn aesni_matches_soft() {
        if !std::arch::is_x86_feature_detected!("aes") {
            eprintln!("skipping: AES-NI not available");
            return;
        }
        // SAFETY: AES-NI was detected.
        unsafe { aesni_checks() };
    }

    #[test]
    fn vaes256_matches_soft() {
        if !(std::arch::is_x86_feature_detected!("aes")
            && std::arch::is_x86_feature_detected!("vaes")
            && std::arch::is_x86_feature_detected!("avx2"))
        {
            eprintln!("skipping: VAES + AVX2 not available");
            return;
        }
        // SAFETY: VAES and AVX2 were detected.
        unsafe { vaes256_checks() };
    }

    #[test]
    fn vaes512_matches_soft() {
        if !(std::arch::is_x86_feature_detected!("vaes")
            && std::arch::is_x86_feature_detected!("avx2")
            && std::arch::is_x86_feature_detected!("avx512f")
            && std::arch::is_x86_feature_detected!("avx512bw"))
        {
            eprintln!("skipping: VAES-512 not available");
            return;
        }
        // SAFETY: VAES, AVX2, AVX-512F and AVX-512BW were detected.
        unsafe { vaes512_checks() };
    }
}
