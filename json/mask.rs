#![allow(unsafe_code)]
#![allow(unsafe_op_in_unsafe_fn)]
//! Allocation-free SIMD stage-1 masks used by the parser.
//!
//! Stage 1 processes the input in 64-byte blocks and records which bytes are
//! string delimiters, structural characters, whitespace, and backslashes. The
//! parser consumes a small fixed-size ring of these masks, so no allocation is
//! required and memory use is independent of input size.
//!
//! The masks are built only after the whole input has been validated as UTF-8
//! by the owned entry points.

use core::fmt;

use crate::error::{Error, ErrorKind, Result};

/// Number of 64-byte blocks kept in the ring.
pub(crate) const RING: usize = 8;

const BLOCK: usize = 64;

/// Inputs below this size use the scalar parser directly.
const MASK_THRESHOLD: usize = 256;

/// Bit masks for one 64-byte block.
#[derive(Clone, Copy)]
pub(crate) struct MaskBlock {
    /// Unescaped string delimiters.
    pub(crate) quotes: u64,
    /// `{ } [ ] : ,` outside strings.
    #[allow(dead_code)]
    pub(crate) structurals: u64,
    /// JSON whitespace outside strings.
    pub(crate) whitespace: u64,
    /// Backslash bytes.
    pub(crate) backslashes: u64,
    /// Unescaped control characters inside strings.
    pub(crate) control: u64,
    /// Bits that correspond to real input, not tail padding.
    pub(crate) valid: u64,
}

impl MaskBlock {
    const EMPTY: Self = Self {
        quotes: 0,
        structurals: 0,
        whitespace: 0,
        backslashes: 0,
        control: 0,
        valid: 0,
    };
}

/// A fixed-capacity ring of stage-1 masks.
pub(crate) struct MaskRing {
    blocks: [MaskBlock; RING],
    /// Absolute index of the next block to produce.
    head: u64,
    /// Absolute index of the oldest live block.
    tail: u64,
    /// Carry for the in-string mask.
    prev_in_string: u64,
    /// Carry for odd-length backslash runs.
    prev_odd_backslash: u64,
    /// Whether the ring is active for this decoder.
    pub(crate) active: bool,
}

impl fmt::Debug for MaskRing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MaskRing")
            .field("head", &self.head)
            .field("tail", &self.tail)
            .field("active", &self.active)
            .finish()
    }
}

impl MaskRing {
    pub(crate) fn new(input_len: usize, enabled: bool) -> Self {
        Self {
            blocks: [MaskBlock::EMPTY; RING],
            head: 0,
            tail: 0,
            prev_in_string: 0,
            prev_odd_backslash: 0,
            active: enabled && input_len >= MASK_THRESHOLD && mask_available(),
        }
    }

    /// Disables masks and falls back to the scalar parser.
    #[inline]
    pub(crate) fn disable(&mut self) {
        self.active = false;
    }

    /// Drops blocks that end at or before `position`.
    #[inline]
    pub(crate) fn advance(&mut self, position: usize) {
        if !self.active {
            return;
        }
        let block = (position / BLOCK) as u64;
        if block > self.tail {
            self.tail = block.min(self.head);
        }
    }

    #[inline]
    fn get(&self, block: u64) -> &MaskBlock {
        &self.blocks[(block as usize) & (RING - 1)]
    }

    /// Ensures that `block` has been produced.
    fn ensure(&mut self, input: &[u8], block: u64) -> Result<()> {
        if !self.active {
            return Ok(());
        }
        while self.head <= block {
            if self.head < self.tail {
                // The forward-only parser should never request an old block.
                self.active = false;
                return Ok(());
            }
            if self.head - self.tail >= RING as u64 {
                self.tail = self.head - RING as u64 + 1;
            }
            self.produce(input, self.head)?;
            self.head += 1;
        }
        Ok(())
    }

    fn produce(&mut self, input: &[u8], block: u64) -> Result<()> {
        let start = (block as usize) * BLOCK;
        if start >= input.len() {
            self.active = false;
            return Ok(());
        }
        let len = (input.len() - start).min(BLOCK);
        let mut tail = [0u8; BLOCK];
        let ptr = if len == BLOCK {
            // SAFETY: the slice contains at least `BLOCK` bytes.
            unsafe { input.as_ptr().add(start) }
        } else {
            tail[..len].copy_from_slice(&input[start..start + len]);
            tail.as_ptr()
        };

        // SAFETY: `ptr` points to `BLOCK` readable bytes, either in `input` or
        // in the zero-padded stack buffer above.
        let raw = unsafe { raw_masks(ptr) };
        let valid = if len == BLOCK { u64::MAX } else { (1u64 << len) - 1 };

        let escaped = odd_backslash_ends(raw.backslashes & valid, &mut self.prev_odd_backslash);
        let quotes = raw.quotes & !escaped & valid;
        let quote_mask = prefix_xor(quotes) ^ self.prev_in_string;
        self.prev_in_string = ((quote_mask as i64) >> 63) as u64;

        let structurals = raw.structurals & !quote_mask & valid;
        let whitespace = raw.whitespace & !quote_mask & valid;
        let backslashes = raw.backslashes & valid;

        // Raw control characters inside strings are invalid. Outside strings
        // the grammar parser reports them naturally as unexpected tokens.
        let control = raw.control & quote_mask & valid & !escaped;

        self.blocks[(block as usize) & (RING - 1)] = MaskBlock {
            quotes,
            structurals,
            whitespace,
            backslashes,
            control,
            valid,
        };

        if start + len == input.len() && self.prev_in_string != 0 {
            return Err(Error::new(ErrorKind::UnexpectedEof, input.len()));
        }
        Ok(())
    }

    /// Skips JSON whitespace using the ring.
    pub(crate) fn skip_ws(&mut self, input: &[u8], position: &mut usize) -> Result<()> {
        while *position < input.len() {
            let block = (*position / BLOCK) as u64;
            self.ensure(input, block)?;
            if !self.active {
                return Ok(());
            }
            let mask = self.get(block);
            let offset = (*position % BLOCK) as u32;
            let remaining = mask.valid & !mask.whitespace & below(offset);
            if remaining != 0 {
                *position = (block as usize) * BLOCK + remaining.trailing_zeros() as usize;
                return Ok(());
            }
            *position = ((block + 1) as usize) * BLOCK;
        }
        *position = input.len();
        Ok(())
    }

    /// Returns the next real quote at or after `position`.
    #[allow(dead_code)]
    pub(crate) fn next_quote(&mut self, input: &[u8], position: usize) -> Result<Option<usize>> {
        let mut block = (position / BLOCK) as u64;
        while block * BLOCK as u64 <= input.len() as u64 {
            self.ensure(input, block)?;
            if !self.active {
                return Ok(None);
            }
            let mask = self.get(block);
            let offset = if position / BLOCK == block as usize {
                (position % BLOCK) as u32
            } else {
                0
            };
            let remaining = mask.valid & mask.quotes & below(offset);
            if remaining != 0 {
                return Ok(Some((block as usize) * BLOCK + remaining.trailing_zeros() as usize));
            }
            block += 1;
        }
        Ok(None)
    }

    /// Returns the next non-whitespace byte at or after `position`.
    #[allow(dead_code)]
    pub(crate) fn next_non_ws(&mut self, input: &[u8], position: usize) -> Result<Option<usize>> {
        let mut block = (position / BLOCK) as u64;
        while block * BLOCK as u64 <= input.len() as u64 {
            self.ensure(input, block)?;
            if !self.active {
                return Ok(None);
            }
            let mask = self.get(block);
            let offset = if position / BLOCK == block as usize {
                (position % BLOCK) as u32
            } else {
                0
            };
            let remaining = mask.valid & !mask.whitespace & below(offset);
            if remaining != 0 {
                return Ok(Some((block as usize) * BLOCK + remaining.trailing_zeros() as usize));
            }
            block += 1;
        }
        Ok(None)
    }

    /// Returns the next structural character at or after `position`.
    #[allow(dead_code)]
    pub(crate) fn next_structural(&mut self, input: &[u8], position: usize) -> Result<Option<usize>> {
        let mut block = (position / BLOCK) as u64;
        while block * BLOCK as u64 <= input.len() as u64 {
            self.ensure(input, block)?;
            if !self.active {
                return Ok(None);
            }
            let mask = self.get(block);
            let offset = if position / BLOCK == block as usize {
                (position % BLOCK) as u32
            } else {
                0
            };
            let remaining = mask.valid & mask.structurals & below(offset);
            if remaining != 0 {
                return Ok(Some((block as usize) * BLOCK + remaining.trailing_zeros() as usize));
            }
            block += 1;
        }
        Ok(None)
    }

    /// Finds the closing quote for a string starting at `start` (the byte
    /// after the opening quote), and reports whether the body contains a
    /// backslash.
    pub(crate) fn find_string_end(&mut self, input: &[u8], start: usize) -> Result<Option<(usize, bool)>> {
        let first_block = (start / BLOCK) as u64;
        let mut block = first_block;
        let mut escaped = false;
        while block * BLOCK as u64 <= input.len() as u64 {
            self.ensure(input, block)?;
            if !self.active {
                return Ok(None);
            }
            let mask = self.get(block);
            let lo = if block == first_block {
                (start % BLOCK) as u32
            } else {
                0
            };
            let quotes = mask.quotes & mask.valid & below(lo);
            let backslashes = mask.backslashes & mask.valid & below(lo);
            let control = mask.control & mask.valid & below(lo);
            if quotes != 0 {
                let bit = quotes.trailing_zeros();
                let bad_control = control & upto(bit);
                if bad_control != 0 {
                    return Err(Error::new(
                        ErrorKind::ControlCharacter,
                        (block as usize) * BLOCK + bad_control.trailing_zeros() as usize,
                    ));
                }
                let before_quote = backslashes & upto(bit);
                return Ok(Some(((block as usize) * BLOCK + bit as usize, escaped || before_quote != 0)));
            }
            if control != 0 {
                return Err(Error::new(
                    ErrorKind::ControlCharacter,
                    (block as usize) * BLOCK + control.trailing_zeros() as usize,
                ));
            }
            if backslashes != 0 {
                escaped = true;
            }
            block += 1;
        }
        Ok(None)
    }

    /// Returns whether any backslash occurs in `[start, end)`.
    #[allow(dead_code)]
    pub(crate) fn has_backslash(&mut self, input: &[u8], start: usize, end: usize) -> Result<bool> {
        let mut block = (start / BLOCK) as u64;
        while block * BLOCK as u64 <= end as u64 {
            self.ensure(input, block)?;
            if !self.active {
                return Ok(true);
            }
            let mask = self.get(block);
            let lo = if start / BLOCK == block as usize {
                (start % BLOCK) as u32
            } else {
                0
            };
            let hi = if end / BLOCK == block as usize {
                (end % BLOCK) as u32
            } else {
                64
            };
            let range = below(lo) & upto(hi);
            if mask.backslashes & range & mask.valid != 0 {
                return Ok(true);
            }
            block += 1;
        }
        Ok(false)
    }
}

#[inline]
fn below(offset: u32) -> u64 {
    if offset >= 64 { 0 } else { u64::MAX << offset }
}

#[inline]
fn upto(offset: u32) -> u64 {
    if offset >= 64 { u64::MAX } else { (1u64 << offset) - 1 }
}

#[inline]
fn prefix_xor(mut value: u64) -> u64 {
    value ^= value << 1;
    value ^= value << 2;
    value ^= value << 4;
    value ^= value << 8;
    value ^= value << 16;
    value ^= value << 32;
    value
}

/// Returns a bit vector whose set bits end odd-length backslash runs.
fn odd_backslash_ends(bs_bits: u64, prev: &mut u64) -> u64 {
    const EVEN_BITS: u64 = 0x5555_5555_5555_5555;
    const ODD_BITS: u64 = !EVEN_BITS;

    let start_edges = bs_bits & !(bs_bits << 1);
    let even_start_mask = EVEN_BITS ^ *prev;
    let even_starts = start_edges & even_start_mask;
    let odd_starts = start_edges & !even_start_mask;
    let even_carries = bs_bits.wrapping_add(even_starts);
    let (mut odd_carries, overflow) = bs_bits.overflowing_add(odd_starts);
    odd_carries |= *prev;
    *prev = u64::from(overflow);
    let even_carry_ends = even_carries & !bs_bits;
    let odd_carry_ends = odd_carries & !bs_bits;
    (even_carry_ends & ODD_BITS) | (odd_carry_ends & EVEN_BITS)
}

#[derive(Clone, Copy)]
struct RawMasks {
    quotes: u64,
    structurals: u64,
    whitespace: u64,
    backslashes: u64,
    control: u64,
}

#[inline]
fn mask_available() -> bool {
    #[cfg(target_arch = "aarch64")]
    {
        return true;
    }
    #[cfg(all(target_arch = "x86_64", feature = "std"))]
    {
        return std::arch::is_x86_feature_detected!("avx2");
    }
    #[cfg(all(not(feature = "std"), target_arch = "x86_64", target_feature = "avx2"))]
    {
        return true;
    }
    #[allow(unreachable_code)]
    false
}

#[allow(unsafe_code, unreachable_code)]
unsafe fn raw_masks(ptr: *const u8) -> RawMasks {
    #[cfg(target_arch = "aarch64")]
    {
        return unsafe { neon_masks(ptr) };
    }
    #[cfg(all(target_arch = "x86_64", feature = "std"))]
    {
        if std::arch::is_x86_feature_detected!("avx2") {
            return unsafe { avx2_masks(ptr) };
        }
    }
    #[cfg(all(not(feature = "std"), target_arch = "x86_64", target_feature = "avx2"))]
    {
        return unsafe { avx2_masks(ptr) };
    }
    scalar_masks(ptr)
}

fn scalar_masks(ptr: *const u8) -> RawMasks {
    let mut quotes = 0u64;
    let mut structurals = 0u64;
    let mut whitespace = 0u64;
    let mut backslashes = 0u64;
    let mut control = 0u64;
    for index in 0..BLOCK {
        // SAFETY: callers guarantee that 64 bytes are readable.
        let byte = unsafe { *ptr.add(index) };
        let bit = 1u64 << index;
        match byte {
            b'"' => quotes |= bit,
            b'\\' => backslashes |= bit,
            b'{' | b'}' | b'[' | b']' | b':' | b',' => structurals |= bit,
            b' ' | b'\t' | b'\n' | b'\r' => whitespace |= bit,
            _ => {}
        }
        if byte < 0x20 {
            control |= bit;
        }
    }
    RawMasks {
        quotes,
        structurals,
        whitespace,
        backslashes,
        control,
    }
}

#[cfg(target_arch = "aarch64")]
#[allow(unsafe_code, unsafe_op_in_unsafe_fn)]
unsafe fn neon_masks(ptr: *const u8) -> RawMasks {
    use core::arch::aarch64::*;

    let block = vld4q_u8(ptr);
    let quote = vdupq_n_u8(b'"');
    let backslash = vdupq_n_u8(b'\\');
    let control = vdupq_n_u8(0x1f);

    let quotes = neon_movemask(
        vceqq_u8(block.0, quote),
        vceqq_u8(block.1, quote),
        vceqq_u8(block.2, quote),
        vceqq_u8(block.3, quote),
    );
    let backslashes = neon_movemask(
        vceqq_u8(block.0, backslash),
        vceqq_u8(block.1, backslash),
        vceqq_u8(block.2, backslash),
        vceqq_u8(block.3, backslash),
    );
    let control = neon_movemask(
        vcleq_u8(block.0, control),
        vcleq_u8(block.1, control),
        vcleq_u8(block.2, control),
        vcleq_u8(block.3, control),
    );

    // Shufti classification for structural characters and whitespace.
    const LOW_NIBBLE_MASK: [u8; 16] = [16, 0, 0, 0, 0, 0, 0, 0, 0, 8, 12, 1, 2, 9, 0, 0];
    const HIGH_NIBBLE_MASK: [u8; 16] = [8, 0, 18, 4, 0, 1, 0, 1, 0, 0, 0, 3, 2, 1, 0, 0];
    let low = vld1q_u8(LOW_NIBBLE_MASK.as_ptr());
    let high = vld1q_u8(HIGH_NIBBLE_MASK.as_ptr());
    let structural_bucket = vdupq_n_u8(0x7);
    let whitespace_bucket = vdupq_n_u8(0x18);
    let nibble_mask = vdupq_n_u8(0x0f);

    let classify = |value: uint8x16_t| -> (uint8x16_t, uint8x16_t) {
        let lo = vqtbl1q_u8(low, vandq_u8(value, nibble_mask));
        let hi = vqtbl1q_u8(high, vshrq_n_u8(value, 4));
        let buckets = vandq_u8(lo, hi);
        (vtstq_u8(buckets, structural_bucket), vtstq_u8(buckets, whitespace_bucket))
    };
    let (s0, w0) = classify(block.0);
    let (s1, w1) = classify(block.1);
    let (s2, w2) = classify(block.2);
    let (s3, w3) = classify(block.3);

    RawMasks {
        quotes,
        structurals: neon_movemask(s0, s1, s2, s3),
        whitespace: neon_movemask(w0, w1, w2, w3),
        backslashes,
        control,
    }
}

#[cfg(target_arch = "aarch64")]
#[allow(unsafe_code, unsafe_op_in_unsafe_fn)]
#[inline]
unsafe fn neon_movemask(
    p0: core::arch::aarch64::uint8x16_t,
    p1: core::arch::aarch64::uint8x16_t,
    p2: core::arch::aarch64::uint8x16_t,
    p3: core::arch::aarch64::uint8x16_t,
) -> u64 {
    use core::arch::aarch64::*;
    let t0 = vsriq_n_u8::<1>(p1, p0);
    let t1 = vsriq_n_u8::<1>(p3, p2);
    let t2 = vsriq_n_u8::<2>(t1, t0);
    let t3 = vsriq_n_u8::<4>(t2, t2);
    let t4 = vshrn_n_u16::<4>(vreinterpretq_u16_u8(t3));
    vget_lane_u64::<0>(vreinterpret_u64_u8(t4))
}

#[cfg(target_arch = "x86_64")]
#[allow(unsafe_code, unsafe_op_in_unsafe_fn)]
#[target_feature(enable = "avx2")]
unsafe fn avx2_masks(ptr: *const u8) -> RawMasks {
    use core::arch::x86_64::*;

    let a = _mm256_loadu_si256(ptr.cast());
    let b = _mm256_loadu_si256(ptr.add(32).cast());

    let quote = _mm256_set1_epi8(b'"' as i8);
    let backslash = _mm256_set1_epi8(b'\\' as i8);
    let control_limit = _mm256_set1_epi8(0x1f);

    let quotes = movemask(_mm256_cmpeq_epi8(a, quote), _mm256_cmpeq_epi8(b, quote));
    let backslashes = movemask(_mm256_cmpeq_epi8(a, backslash), _mm256_cmpeq_epi8(b, backslash));
    let control = movemask(
        _mm256_cmpeq_epi8(_mm256_min_epu8(a, control_limit), a),
        _mm256_cmpeq_epi8(_mm256_min_epu8(b, control_limit), b),
    );

    // Shufti classification for structural characters and whitespace.
    let low = _mm256_setr_epi8(
        16, 0, 0, 0, 0, 0, 0, 0, 0, 8, 12, 1, 2, 9, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 8, 12, 1, 2, 9, 0, 0,
    );
    let high = _mm256_setr_epi8(
        8, 0, 18, 4, 0, 1, 0, 1, 0, 0, 0, 3, 2, 1, 0, 0, 8, 0, 18, 4, 0, 1, 0, 1, 0, 0, 0, 3, 2, 1, 0, 0,
    );
    let structural_bucket = _mm256_set1_epi8(0x7);
    let whitespace_bucket = _mm256_set1_epi8(0x18);
    let nibble_mask = _mm256_set1_epi8(0x0f);

    let classify = |value: __m256i| -> (__m256i, __m256i) {
        let lo = _mm256_shuffle_epi8(low, _mm256_and_si256(value, nibble_mask));
        let hi = _mm256_shuffle_epi8(high, _mm256_and_si256(_mm256_srli_epi16(value, 4), nibble_mask));
        let buckets = _mm256_and_si256(lo, hi);
        (
            _mm256_cmpeq_epi8(_mm256_and_si256(buckets, structural_bucket), _mm256_setzero_si256()),
            _mm256_cmpeq_epi8(_mm256_and_si256(buckets, whitespace_bucket), _mm256_setzero_si256()),
        )
    };
    let (sa, wa) = classify(a);
    let (sb, wb) = classify(b);

    RawMasks {
        quotes,
        structurals: movemask(sa, sb),
        whitespace: movemask(wa, wb),
        backslashes,
        control,
    }
}

#[cfg(target_arch = "x86_64")]
#[allow(unsafe_code, unsafe_op_in_unsafe_fn)]
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn movemask(a: core::arch::x86_64::__m256i, b: core::arch::x86_64::__m256i) -> u64 {
    use core::arch::x86_64::*;
    (u64::from(_mm256_movemask_epi8(a) as u32)) | ((_mm256_movemask_epi8(b) as u32 as u64) << 32)
}
