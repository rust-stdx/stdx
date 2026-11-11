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
        let mut raw = [0u8; 64];
        for lane in 0..D {
            raw[lane * 16] = lane as u8;
            raw[lane * 16 + 1] = (D - 1) as u8;
        }
        // SAFETY: `raw` is at least `D * 16` bytes.
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
        let mut raw = [0u8; 64];
        for lane in 0..2 * L {
            raw[lane * 16] = lane as u8;
            raw[lane * 16 + 1] = (2 * L - 1) as u8;
        }
        // SAFETY: `raw` is at least `2 * L * 16` bytes.
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
        let mut raw = [0u8; 64];
        for lane in 0..4 * L {
            raw[lane * 16] = lane as u8;
            raw[lane * 16 + 1] = (4 * L - 1) as u8;
        }
        // SAFETY: `raw` is at least `4 * L * 16` bytes.
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
