//! Constant-time portable backend.
//!
//! There is no AES instruction on many embedded targets (and no AES instruction
//! in WebAssembly without the `simd128` swizzle fallback). This backend reuses
//! the bitsliced S-box circuit from [`crate::aes::aes_ct`] and processes two
//! lanes per pass, so it is free of secret-dependent table lookups and branches
//! like the rest of the crate's software AES.

use crate::{aegis::aegis_core::BlockOps, aes::round_pair};

/// A portable block vector: `D` independent 128-bit blocks.
#[derive(Clone, Copy)]
pub(crate) struct Soft<const D: usize> {
    v: [[u8; 16]; D],
}

impl<const D: usize> BlockOps for Soft<D> {
    const D: usize = D;

    #[inline(always)]
    fn zero() -> Self {
        Self {
            v: [[0u8; 16]; D],
        }
    }

    #[inline(always)]
    fn xor(self, rhs: Self) -> Self {
        let mut v = [[0u8; 16]; D];
        for lane in 0..D {
            for i in 0..16 {
                v[lane][i] = self.v[lane][i] ^ rhs.v[lane][i];
            }
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    fn and(self, rhs: Self) -> Self {
        let mut v = [[0u8; 16]; D];
        for lane in 0..D {
            for i in 0..16 {
                v[lane][i] = self.v[lane][i] & rhs.v[lane][i];
            }
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    fn round(self, rk: Self) -> Self {
        let mut v = self.v;
        let mut lane = 0;
        while lane + 1 < D {
            let (left, right) = v.split_at_mut(lane + 1);
            round_pair(&mut left[lane], &mut right[0], &rk.v[lane], &rk.v[lane + 1]);
            lane += 2;
        }
        if D % 2 == 1 {
            let mut dummy = [0u8; 16];
            let dummy_rk = [0u8; 16];
            round_pair(&mut v[D - 1], &mut dummy, &rk.v[D - 1], &dummy_rk);
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    unsafe fn load(ptr: *const u8) -> Self {
        let mut v = [[0u8; 16]; D];
        for lane in 0..D {
            // SAFETY: the caller guarantees `D * 16` readable bytes.
            unsafe { core::ptr::copy_nonoverlapping(ptr.add(lane * 16), v[lane].as_mut_ptr(), 16) };
        }
        Self {
            v,
        }
    }

    #[inline(always)]
    unsafe fn store(self, ptr: *mut u8) {
        for lane in 0..D {
            // SAFETY: the caller guarantees `D * 16` writable bytes.
            unsafe { core::ptr::copy_nonoverlapping(self.v[lane].as_ptr(), ptr.add(lane * 16), 16) };
        }
    }

    #[inline(always)]
    fn broadcast(block: &[u8; 16]) -> Self {
        Self {
            v: [*block; D],
        }
    }

    #[inline(always)]
    fn ctx_all() -> Self {
        let mut v = [[0u8; 16]; D];
        for lane in 0..D {
            v[lane][0] = lane as u8;
            v[lane][1] = (D - 1) as u8;
        }
        Self {
            v,
        }
    }
}
