//! Shared SIMD helpers adapted from simdutf8 (MIT OR Apache-2.0).
#![allow(unsafe_op_in_unsafe_fn)]

#[allow(dead_code)]
pub(crate) unsafe fn memcpy_unaligned_nonoverlapping_inline_opt_lt_64(
    mut src: *const u8,
    mut dest: *mut u8,
    mut len: usize,
) {
    // This gets properly auto-vectorized on AVX 2 and SSE 4.2
    #[inline]
    unsafe fn memcpy_u64(src: &mut *const u8, dest: &mut *mut u8) {
        #[allow(clippy::cast_ptr_alignment)]
        dest.cast::<u64>().write_unaligned(src.cast::<u64>().read_unaligned());
        *src = src.offset(8);
        *dest = dest.offset(8);
    }
    if len >= 32 {
        memcpy_u64(&mut src, &mut dest);
        memcpy_u64(&mut src, &mut dest);
        memcpy_u64(&mut src, &mut dest);
        memcpy_u64(&mut src, &mut dest);
        len -= 32;
    }
    if len >= 16 {
        memcpy_u64(&mut src, &mut dest);
        memcpy_u64(&mut src, &mut dest);
        len -= 16;
    }
    if len >= 8 {
        memcpy_u64(&mut src, &mut dest);
        len -= 8;
    }
    while len > 0 {
        *dest = *src;
        src = src.offset(1);
        dest = dest.offset(1);
        len -= 1;
    }
}

pub(crate) const SIMD_CHUNK_SIZE: usize = 64;

#[repr(C, align(32))]
#[allow(dead_code)]
pub(crate) struct Utf8CheckAlgorithm<T> {
    pub(crate) prev: T,
    pub(crate) incomplete: T,
    pub(crate) error: T,
}

#[repr(C, align(16))]
#[allow(dead_code)]
pub(crate) struct TempSimdChunkA16(pub(crate) [u8; SIMD_CHUNK_SIZE]);

#[allow(dead_code)]
impl TempSimdChunkA16 {
    #[inline]
    pub(crate) const fn new() -> Self {
        Self([0; SIMD_CHUNK_SIZE])
    }
}

#[repr(C, align(32))]
#[allow(dead_code)]
pub(crate) struct TempSimdChunkA32(pub(crate) [u8; SIMD_CHUNK_SIZE]);

#[allow(dead_code)]
impl TempSimdChunkA32 {
    #[inline]
    pub(crate) const fn new() -> Self {
        Self([0; SIMD_CHUNK_SIZE])
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(crate) struct SimdU8Value<T>(pub(crate) T)
where
    T: Copy;
