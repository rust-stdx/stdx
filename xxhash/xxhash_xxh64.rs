use crate::Checksum;

const PRIME64_1: u64 = 0x9E3779B185EBCA87;
const PRIME64_2: u64 = 0xC2B2AE3D27D4EB4F;
const PRIME64_3: u64 = 0x165667B19E3779F9;
const PRIME64_4: u64 = 0x85EBCA77C2B2AE63;
const PRIME64_5: u64 = 0x27D4EB2F165667C5;

/// XXH64 hash (64-bit).
///
/// The classic 64-bit xxHash algorithm. Accepts an optional `u64` seed via
/// [`with_seed`](Xxh64::with_seed) (defaults to 0).
///
/// # Example
///
/// ```rust
/// use xxhash::{Xxh64, Checksum};
///
/// let hash = Xxh64::checksum(b"hello");
/// assert_eq!(hash, 0x26C7827D889F6DA3);
/// ```
#[derive(Clone)]
pub struct Xxh64 {
    seed: u64,
    v: [u64; 4],
    total_len: u64,
    has_stripes: bool,
    buf: [u8; 32],
    buf_len: u8,
}

impl Xxh64 {
    /// Create a new XXH64 hasher with the given seed.
    #[inline]
    pub fn with_seed(seed: u64) -> Self {
        Xxh64 {
            seed,
            v: [0; 4],
            total_len: 0,
            has_stripes: false,
            buf: [0u8; 32],
            buf_len: 0,
        }
    }

    #[inline]
    fn init_stripes(&mut self) {
        self.v[0] = self.seed.wrapping_add(PRIME64_1).wrapping_add(PRIME64_2);
        self.v[1] = self.seed.wrapping_add(PRIME64_2);
        self.v[2] = self.seed;
        self.v[3] = self.seed.wrapping_sub(PRIME64_1);
        self.has_stripes = true;
    }

    #[inline]
    fn process_stripe(&mut self, stripe: &[u8]) {
        debug_assert_eq!(stripe.len(), 32);
        for i in 0..4 {
            let lane = u64::from_le_bytes(stripe[i * 8..(i + 1) * 8].try_into().unwrap());
            self.v[i] = self.v[i].wrapping_add(lane.wrapping_mul(PRIME64_2));
            self.v[i] = self.v[i].rotate_left(31);
            self.v[i] = self.v[i].wrapping_mul(PRIME64_1);
        }
    }

    #[inline]
    fn process_8bytes(acc: &mut u64, data: &[u8]) {
        let lane = u64::from_le_bytes(data.try_into().unwrap());
        let k1 = lane.wrapping_mul(PRIME64_2).rotate_left(31).wrapping_mul(PRIME64_1);
        *acc ^= k1;
        *acc = acc.rotate_left(27).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
    }

    #[inline]
    fn process_4bytes(acc: &mut u64, data: &[u8]) {
        let lane = u32::from_le_bytes(data.try_into().unwrap()) as u64;
        *acc ^= lane.wrapping_mul(PRIME64_1);
        *acc = acc.rotate_left(23).wrapping_mul(PRIME64_2).wrapping_add(PRIME64_3);
    }

    #[inline]
    fn process_1byte(acc: &mut u64, byte: u8) {
        *acc ^= (byte as u64).wrapping_mul(PRIME64_5);
        *acc = acc.rotate_left(11).wrapping_mul(PRIME64_1);
    }
}

impl Checksum for Xxh64 {
    type Output = u64;

    #[inline]
    fn new() -> Self {
        Self::with_seed(0)
    }

    fn update(&mut self, data: &[u8]) {
        self.total_len += data.len() as u64;
        let mut data = data;

        if self.buf_len > 0 {
            let take = (32 - self.buf_len as usize).min(data.len());
            self.buf[self.buf_len as usize..self.buf_len as usize + take].copy_from_slice(&data[..take]);
            self.buf_len += take as u8;
            data = &data[take..];

            if self.buf_len == 32 {
                if !self.has_stripes {
                    self.init_stripes();
                }
                let stripe = self.buf;
                self.process_stripe(&stripe);
                self.buf_len = 0;
            }
        }

        if self.has_stripes {
            let chunks = data.chunks_exact(32);
            let remainder = chunks.remainder();
            for stripe in chunks {
                self.process_stripe(stripe);
            }
            data = remainder;
        } else if data.len() >= 32 {
            self.init_stripes();
            let chunks = data.chunks_exact(32);
            let remainder = chunks.remainder();
            for stripe in chunks {
                self.process_stripe(stripe);
            }
            data = remainder;
        }

        if !data.is_empty() {
            self.buf[..data.len()].copy_from_slice(data);
            self.buf_len = data.len() as u8;
        }
    }

    fn sum(self) -> Self::Output {
        let mut h64: u64;

        if self.total_len >= 32 {
            h64 = self.v[0]
                .rotate_left(1)
                .wrapping_add(self.v[1].rotate_left(7))
                .wrapping_add(self.v[2].rotate_left(12))
                .wrapping_add(self.v[3].rotate_left(18));
            h64 = xxh64_merge_round(h64, self.v[0]);
            h64 = xxh64_merge_round(h64, self.v[1]);
            h64 = xxh64_merge_round(h64, self.v[2]);
            h64 = xxh64_merge_round(h64, self.v[3]);
        } else {
            h64 = self.seed.wrapping_add(PRIME64_5);
        }

        h64 = h64.wrapping_add(self.total_len);

        let mut idx = 0usize;
        while idx + 8 <= self.buf_len as usize {
            Self::process_8bytes(&mut h64, &self.buf[idx..idx + 8]);
            idx += 8;
        }
        if idx + 4 <= self.buf_len as usize {
            Self::process_4bytes(&mut h64, &self.buf[idx..idx + 4]);
            idx += 4;
        }
        while idx < self.buf_len as usize {
            Self::process_1byte(&mut h64, self.buf[idx]);
            idx += 1;
        }

        avalanche(h64)
    }
}

#[inline]
fn xxh64_merge_round(mut acc: u64, val: u64) -> u64 {
    let mut val = val.wrapping_mul(PRIME64_2);
    val = val.rotate_left(31);
    val = val.wrapping_mul(PRIME64_1);
    acc ^= val;
    acc = acc.wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
    acc
}

#[inline]
fn avalanche(mut h64: u64) -> u64 {
    h64 ^= h64 >> 33;
    h64 = h64.wrapping_mul(PRIME64_2);
    h64 ^= h64 >> 29;
    h64 = h64.wrapping_mul(PRIME64_3);
    h64 ^= h64 >> 32;
    h64
}

impl core::fmt::Debug for Xxh64 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Xxh64").finish()
    }
}

impl Default for Xxh64 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Checksum, test_helpers::fill_test_buffer};

    /// Canonical prime used as seed in the official C test vectors.
    const TEST_SEED: u64 = 0x9E3779B1; // PRIME32 in C test file, but seed is u64

    #[test]
    fn test_empty() {
        assert_eq!(Xxh64::checksum(b""), 0xEF46DB3751D8E999);
    }

    #[test]
    fn test_hello() {
        assert_eq!(Xxh64::checksum(b"hello"), 0x26C7827D889F6DA3);
    }

    #[test]
    fn test_fox() {
        assert_eq!(
            Xxh64::checksum(b"The quick brown fox jumps over the lazy dog"),
            0x0B242D361FDA71BC
        );
    }

    #[test]
    fn test_incremental() {
        let mut h = Xxh64::new();
        h.update(b"The quick brown ");
        h.update(b"fox jumps over ");
        h.update(b"the lazy dog");
        assert_eq!(h.sum(), 0x0B242D361FDA71BC);
    }

    /// Official XXH64 test vectors from the C reference's `xsum_sanity_check.c`.
    #[test]
    fn test_official_vectors() {
        let cases: &[(usize, u64, u64)] = &[
            (0, 0, 0xEF46DB3751D8E999),
            (0, TEST_SEED, 0xAC75FDA2929B17EF),
            (1, 0, 0xE934A84ADB052768),
            (1, TEST_SEED, 0x5014607643A9B4C3),
            (4, 0, 0x9136A0DCA57457EE),
            (14, 0, 0x8282DCC4994E35C8),
            (14, TEST_SEED, 0xC3BD6BF63DEB6DF0),
            (222, 0, 0xB641AE8CB691C174),
            (222, TEST_SEED, 0x20CB8AB7AE10C14A),
        ];

        for &(len, seed, expected) in cases {
            let buf = fill_test_buffer(len);
            let mut h = Xxh64::with_seed(seed);
            h.update(&buf);
            assert_eq!(h.sum(), expected, "XXH64 length {len} seed {seed:#x}");
        }
    }

    /// Byte-at-a-time incremental hashing produces the same result as one-shot.
    #[test]
    fn test_byte_at_a_time() {
        let cases: &[(usize, u64, u64)] = &[
            (0, 0, 0xEF46DB3751D8E999),
            (1, TEST_SEED, 0x5014607643A9B4C3),
            (14, 0, 0x8282DCC4994E35C8),
            (222, TEST_SEED, 0x20CB8AB7AE10C14A),
        ];

        for &(len, seed, expected) in cases {
            let buf = fill_test_buffer(len);
            let mut h = Xxh64::with_seed(seed);
            for b in &buf {
                h.update(&[*b]);
            }
            assert_eq!(h.sum(), expected, "XXH64 byte-at-a-time length {len} seed {seed:#x}");
        }
    }
}
