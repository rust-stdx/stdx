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
