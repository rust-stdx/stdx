use crate::Checksum;

const PRIME32_1: u32 = 0x9E3779B1;
const PRIME32_2: u32 = 0x85EBCA77;
const PRIME32_3: u32 = 0xC2B2AE3D;
const PRIME32_4: u32 = 0x27D4EB2F;
const PRIME32_5: u32 = 0x165667B1;

/// XXH32 hash (32-bit).
///
/// The classic 32-bit xxHash algorithm. Accepts an optional `u32` seed via
/// [`with_seed`](Xxh32::with_seed) (defaults to 0).
///
/// # Example
///
/// ```rust
/// use xxhash::{Xxh32, Checksum};
///
/// let hash = Xxh32::checksum(b"hello");
/// assert_eq!(hash, 0xFB0077F9);
/// ```
#[derive(Clone)]
pub struct Xxh32 {
    seed: u32,
    v: [u32; 4],
    total_len: u64,
    has_stripes: bool,
    buf: [u8; 16],
    buf_len: u8,
}

impl Xxh32 {
    /// Create a new XXH32 hasher with the given seed.
    #[inline]
    pub const fn with_seed(seed: u32) -> Self {
        Xxh32 {
            seed,
            v: [0; 4],
            total_len: 0,
            has_stripes: false,
            buf: [0u8; 16],
            buf_len: 0,
        }
    }

    #[inline]
    fn init_stripes(&mut self) {
        self.v[0] = self.seed.wrapping_add(PRIME32_1).wrapping_add(PRIME32_2);
        self.v[1] = self.seed.wrapping_add(PRIME32_2);
        self.v[2] = self.seed;
        self.v[3] = self.seed.wrapping_sub(PRIME32_1);
        self.has_stripes = true;
    }

    #[inline]
    fn process_stripe(&mut self, stripe: &[u8]) {
        debug_assert_eq!(stripe.len(), 16);
        for i in 0..4 {
            let lane = u32::from_le_bytes(stripe[i * 4..(i + 1) * 4].try_into().unwrap());
            self.v[i] = self.v[i].wrapping_add(lane.wrapping_mul(PRIME32_2));
            self.v[i] = self.v[i].rotate_left(13);
            self.v[i] = self.v[i].wrapping_mul(PRIME32_1);
        }
    }

    #[inline]
    fn process_4bytes(acc: &mut u32, data: &[u8]) {
        let lane = u32::from_le_bytes(data.try_into().unwrap());
        *acc = acc.wrapping_add(lane.wrapping_mul(PRIME32_3));
        *acc = acc.rotate_left(17).wrapping_mul(PRIME32_4);
    }

    #[inline]
    fn process_1byte(acc: &mut u32, byte: u8) {
        *acc = acc.wrapping_add((byte as u32).wrapping_mul(PRIME32_5));
        *acc = acc.rotate_left(11).wrapping_mul(PRIME32_1);
    }
}

impl Checksum for Xxh32 {
    type Output = u32;

    #[inline]
    fn new() -> Self {
        Self::with_seed(0)
    }

    fn update(&mut self, data: &[u8]) {
        self.total_len += data.len() as u64;
        let mut data = data;

        // If we have buffered data, try to fill a full stripe
        if self.buf_len > 0 {
            let take = (16 - self.buf_len as usize).min(data.len());
            self.buf[self.buf_len as usize..self.buf_len as usize + take].copy_from_slice(&data[..take]);
            self.buf_len += take as u8;
            data = &data[take..];

            if self.buf_len == 16 {
                if !self.has_stripes {
                    self.init_stripes();
                }
                let stripe = self.buf;
                self.process_stripe(&stripe);
                self.buf_len = 0;
            }
        }

        // If we have stripes active or enough data to start, process full stripes
        if self.has_stripes {
            let chunks = data.chunks_exact(16);
            let remainder = chunks.remainder();
            for stripe in chunks {
                self.process_stripe(stripe);
            }
            data = remainder;
        } else if data.len() >= 16 {
            self.init_stripes();
            let chunks = data.chunks_exact(16);
            let remainder = chunks.remainder();
            for stripe in chunks {
                self.process_stripe(stripe);
            }
            data = remainder;
        }

        // Buffer remaining bytes
        if !data.is_empty() {
            self.buf[..data.len()].copy_from_slice(data);
            self.buf_len = data.len() as u8;
        }
    }

    fn sum(self) -> Self::Output {
        let mut h32: u32;

        if self.total_len >= 16 {
            h32 = self.v[0]
                .rotate_left(1)
                .wrapping_add(self.v[1].rotate_left(7))
                .wrapping_add(self.v[2].rotate_left(12))
                .wrapping_add(self.v[3].rotate_left(18));
        } else {
            h32 = self.seed.wrapping_add(PRIME32_5);
        }

        h32 = h32.wrapping_add(self.total_len as u32);

        let mut idx = 0usize;
        while idx + 4 <= self.buf_len as usize {
            Self::process_4bytes(&mut h32, &self.buf[idx..idx + 4]);
            idx += 4;
        }
        while idx < self.buf_len as usize {
            Self::process_1byte(&mut h32, self.buf[idx]);
            idx += 1;
        }

        avalanche(h32)
    }
}

impl core::fmt::Debug for Xxh32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Xxh32").finish()
    }
}

impl Default for Xxh32 {
    #[inline]
    fn default() -> Self {
        Self::with_seed(0)
    }
}

// ---------------------------------------------------------------------------
// Standalone const one-shot function
// ---------------------------------------------------------------------------

/// Compute the XXH32 hash of `data` in a single call.
///
/// Available as a `const fn` for compile-time hashing with seed=0.
///
/// # Example
///
/// ```rust
/// use xxhash::xxh32;
///
/// let hash: u32 = xxh32(b"hello");
/// assert_eq!(hash, 0xFB0077F9);
/// ```
#[inline]
pub const fn xxh32(data: &[u8]) -> u32 {
    let len = data.len();
    let seed: u32 = 0;
    let mut h32: u32;

    if len >= 16 {
        let mut v = [
            seed.wrapping_add(PRIME32_1).wrapping_add(PRIME32_2),
            seed.wrapping_add(PRIME32_2),
            seed,
            seed.wrapping_sub(PRIME32_1),
        ];
        let mut p = 0;
        while p + 16 <= len {
            let mut i = 0;
            while i < 4 {
                let off = p + i * 4;
                let lane = u32::from_le_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]]);
                v[i] = v[i].wrapping_add(lane.wrapping_mul(PRIME32_2));
                v[i] = v[i].rotate_left(13);
                v[i] = v[i].wrapping_mul(PRIME32_1);
                i += 1;
            }
            p += 16;
        }
        h32 = v[0]
            .rotate_left(1)
            .wrapping_add(v[1].rotate_left(7))
            .wrapping_add(v[2].rotate_left(12))
            .wrapping_add(v[3].rotate_left(18));
    } else {
        h32 = seed.wrapping_add(PRIME32_5);
    }

    h32 = h32.wrapping_add(len as u32);

    let mut p = (len / 16) * 16;
    while p + 4 <= len {
        let lane = u32::from_le_bytes([data[p], data[p + 1], data[p + 2], data[p + 3]]);
        h32 = h32.wrapping_add(lane.wrapping_mul(PRIME32_3));
        h32 = h32.rotate_left(17).wrapping_mul(PRIME32_4);
        p += 4;
    }
    while p < len {
        h32 = h32.wrapping_add((data[p] as u32).wrapping_mul(PRIME32_5));
        h32 = h32.rotate_left(11).wrapping_mul(PRIME32_1);
        p += 1;
    }

    let mut h = h32;
    h ^= h >> 15;
    h = h.wrapping_mul(PRIME32_2);
    h ^= h >> 13;
    h = h.wrapping_mul(PRIME32_3);
    h ^= h >> 16;
    h
}

#[inline]
const fn avalanche(mut h32: u32) -> u32 {
    h32 ^= h32 >> 15;
    h32 = h32.wrapping_mul(PRIME32_2);
    h32 ^= h32 >> 13;
    h32 = h32.wrapping_mul(PRIME32_3);
    h32 ^= h32 >> 16;
    h32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Checksum, test_helpers::fill_test_buffer};

    /// Canonical prime used as seed in the official C test vectors.
    const TEST_SEED: u32 = 0x9E3779B1;

    #[test]
    fn test_empty() {
        assert_eq!(Xxh32::checksum(b""), 0x02CC5D05);
    }

    #[test]
    fn test_hello() {
        assert_eq!(Xxh32::checksum(b"hello"), 0xFB0077F9);
    }

    #[test]
    fn test_fox() {
        assert_eq!(Xxh32::checksum(b"The quick brown fox jumps over the lazy dog"), 0xE85EA4DE);
    }

    #[test]
    fn test_incremental() {
        let mut h = Xxh32::new();
        h.update(b"The quick brown ");
        h.update(b"fox jumps over ");
        h.update(b"the lazy dog");
        assert_eq!(h.sum(), 0xE85EA4DE);
    }

    /// Official XXH32 test vectors from the C reference's `xsum_sanity_check.c`.
    /// The test data is generated via `XSUM_fillTestBuffer` (see `fill_test_buffer`).
    #[test]
    fn test_official_vectors() {
        let cases: &[(usize, u32, u32)] = &[
            (0, 0, 0x02CC5D05),
            (0, TEST_SEED, 0x36B78AE7),
            (1, 0, 0xCF65B03E),
            (1, TEST_SEED, 0xB4545AA4),
            (14, 0, 0x1208E7E2),
            (14, TEST_SEED, 0x6AF1D1FE),
            (222, 0, 0x5BD11DBD),
            (222, TEST_SEED, 0x58803C5F),
        ];

        for &(len, seed, expected) in cases {
            let buf = fill_test_buffer(len);
            let mut h = Xxh32::with_seed(seed);
            h.update(&buf);
            assert_eq!(h.sum(), expected, "XXH32 length {len} seed {seed:#x}");
        }
    }

    /// Byte-at-a-time incremental hashing produces the same result as one-shot.
    /// This is tested by the C reference for each test vector.
    #[test]
    fn test_byte_at_a_time() {
        let cases: &[(usize, u32, u32)] = &[
            (0, 0, 0x02CC5D05),
            (1, TEST_SEED, 0xB4545AA4),
            (14, 0, 0x1208E7E2),
            (222, TEST_SEED, 0x58803C5F),
        ];

        for &(len, seed, expected) in cases {
            let buf = fill_test_buffer(len);
            let mut h = Xxh32::with_seed(seed);
            for b in &buf {
                h.update(&[*b]);
            }
            assert_eq!(h.sum(), expected, "XXH32 byte-at-a-time length {len} seed {seed:#x}");
        }
    }

    /// The `const fn` one-shot produces the same result as the trait-based
    /// [`Checksum::checksum`] and is usable at compile time.
    #[test]
    fn test_const_fn() {
        assert_eq!(xxh32(b""), Xxh32::checksum(b""));
        assert_eq!(xxh32(b"hello"), Xxh32::checksum(b"hello"));
        assert_eq!(
            xxh32(b"The quick brown fox jumps over the lazy dog"),
            Xxh32::checksum(b"The quick brown fox jumps over the lazy dog")
        );
        let buf = &[0x42u8; 256];
        assert_eq!(xxh32(buf), Xxh32::checksum(buf));
    }
}
