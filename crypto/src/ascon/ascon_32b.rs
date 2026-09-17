// Ascon core: 32-bit bit-interleaved state representation and permutation.
//
// Compiled on targets with `target_pointer_width != "64"`. The 320-bit state is
// held as ten 32-bit words in bit-interleaved form so the permutation only ever
// needs 32-bit operations, which is friendlier to narrow embedded CPUs.

use super::{RC4, RC5, RC6, RC7, RC8, RC9, RC10, RC11, RC12, RC13, RC14, RC15};

// ============================================================================
// 32-bit State ([u32; 10])
// ============================================================================

/// The 320-bit Ascon state, stored as ten 32-bit words in bit-interleaved form.
///
/// Each original 64-bit word is split into two 32-bit words holding the even
/// and odd bits respectively (matching the `bi32` implementation of the
/// reference ascon-c library):
///
/// | u32 index | Content                    |
/// |-----------|----------------------------|
/// | 0         | word 0 even bits (rate)    |
/// | 1         | word 0 odd bits (rate)     |
/// | 2         | word 1 even bits (rate)    |
/// | 3         | word 1 odd bits (rate)     |
/// | 4         | word 2 even bits           |
/// | 5         | word 2 odd bits            |
/// | 6         | word 3 even bits (tag)     |
/// | 7         | word 3 odd bits (tag)      |
/// | 8         | word 4 even bits (tag)     |
/// | 9         | word 4 odd bits (tag)      |
///
/// Bit-interleaving lets the permutation's diffusion layer rotate each 32-bit
/// half independently (even rotation amounts) or with a single register swap
/// (odd rotation amounts), avoiding the cross-word shifts required by a plain
/// lo/hi split. Byte-level state I/O converts to and from the interleaved form
/// on the absorb/squeeze boundary via [`to_bi`] and [`from_bi`].
///
/// State bit 0 is the LSB of word 0; state bit 319 is the MSB of word 4.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub(crate) struct State(pub [u32; 10]);

impl State {
    #[inline]
    pub(crate) fn init_aead(key: &[u8; 16], nonce: &[u8; 16], iv: u64) -> Self {
        let k0 = u64::from_le_bytes(key[0..8].try_into().unwrap());
        let k1 = u64::from_le_bytes(key[8..16].try_into().unwrap());
        let n0 = u64::from_le_bytes(nonce[0..8].try_into().unwrap());
        let n1 = u64::from_le_bytes(nonce[8..16].try_into().unwrap());
        let iv = to_bi(iv as u32, (iv >> 32) as u32);
        let k0 = to_bi(k0 as u32, (k0 >> 32) as u32);
        let k1 = to_bi(k1 as u32, (k1 >> 32) as u32);
        let n0 = to_bi(n0 as u32, (n0 >> 32) as u32);
        let n1 = to_bi(n1 as u32, (n1 >> 32) as u32);
        State([iv.0, iv.1, k0.0, k0.1, k1.0, k1.1, n0.0, n0.1, n1.0, n1.1])
    }

    #[inline]
    pub(crate) fn init_hash(iv: u64) -> Self {
        let iv = to_bi(iv as u32, (iv >> 32) as u32);
        State([iv.0, iv.1, 0, 0, 0, 0, 0, 0, 0, 0])
    }

    #[inline]
    pub(crate) fn xor_word(&mut self, idx: usize, val: u64) {
        let (e, o) = to_bi(val as u32, (val >> 32) as u32);
        self.0[idx * 2] ^= e;
        self.0[idx * 2 + 1] ^= o;
    }

    #[inline]
    pub(crate) fn xor_rate128_bytes(&mut self, bytes: &[u8; 16]) {
        let w0 = load_to_bi(&bytes[..8]);
        let w1 = load_to_bi(&bytes[8..]);
        self.0[0] ^= w0.0;
        self.0[1] ^= w0.1;
        self.0[2] ^= w1.0;
        self.0[3] ^= w1.1;
    }

    #[inline]
    pub(crate) fn xor_partial_rate(&mut self, bytes: &[u8]) {
        debug_assert!(bytes.len() <= 16 && !bytes.is_empty());
        let n = bytes.len();
        if n >= 8 {
            let lo = u64::from_le_bytes(bytes[..8].try_into().unwrap());
            let (e, o) = to_bi(lo as u32, (lo >> 32) as u32);
            self.0[0] ^= e;
            self.0[1] ^= o;
            if n > 8 {
                let (e, o) = load_to_bi(&bytes[8..]);
                self.0[2] ^= e;
                self.0[3] ^= o;
            }
        } else {
            let (e, o) = load_to_bi(bytes);
            self.0[0] ^= e;
            self.0[1] ^= o;
        }
    }

    #[inline]
    pub(crate) fn encrypt_in_place_block(&mut self, in_out: &mut [u8; 16]) {
        let pt0 = load_to_bi(&in_out[..8]);
        let pt1 = load_to_bi(&in_out[8..]);
        self.0[0] ^= pt0.0;
        self.0[1] ^= pt0.1;
        self.0[2] ^= pt1.0;
        self.0[3] ^= pt1.1;
        let c0 = from_bi(self.0[0], self.0[1]);
        let c1 = from_bi(self.0[2], self.0[3]);
        in_out[0..8].copy_from_slice(&((c0.0 as u64) | ((c0.1 as u64) << 32)).to_le_bytes());
        in_out[8..16].copy_from_slice(&((c1.0 as u64) | ((c1.1 as u64) << 32)).to_le_bytes());
    }

    #[inline]
    pub(crate) fn decrypt_in_place_block(&mut self, in_out: &mut [u8; 16]) {
        let ct0 = load_to_bi(&in_out[..8]);
        let ct1 = load_to_bi(&in_out[8..]);
        let p0 = from_bi(self.0[0] ^ ct0.0, self.0[1] ^ ct0.1);
        let p1 = from_bi(self.0[2] ^ ct1.0, self.0[3] ^ ct1.1);
        in_out[0..8].copy_from_slice(&((p0.0 as u64) | ((p0.1 as u64) << 32)).to_le_bytes());
        in_out[8..16].copy_from_slice(&((p1.0 as u64) | ((p1.1 as u64) << 32)).to_le_bytes());
        self.0[0] = ct0.0;
        self.0[1] = ct0.1;
        self.0[2] = ct1.0;
        self.0[3] = ct1.1;
    }

    #[inline]
    pub(crate) fn squeeze_rate_u64(&self) -> u64 {
        let (lo, hi) = from_bi(self.0[0], self.0[1]);
        (lo as u64) | ((hi as u64) << 32)
    }

    #[inline]
    pub(crate) fn read_rate_bytes(&self, out: &mut [u8]) {
        debug_assert!(out.len() <= 16 && !out.is_empty());
        let n = out.len();
        let (s0_lo, s0_hi) = from_bi(self.0[0], self.0[1]);
        let s0 = ((s0_lo as u64) | ((s0_hi as u64) << 32)).to_le_bytes();
        if n <= 8 {
            out.copy_from_slice(&s0[..n]);
        } else {
            let (s1_lo, s1_hi) = from_bi(self.0[2], self.0[3]);
            let s1 = ((s1_lo as u64) | ((s1_hi as u64) << 32)).to_le_bytes();
            out[..8].copy_from_slice(&s0);
            out[8..].copy_from_slice(&s1[..n - 8]);
        }
    }

    #[inline]
    pub(crate) fn write_rate_bytes(&mut self, bytes: &[u8]) {
        debug_assert!(bytes.len() <= 16 && !bytes.is_empty());
        let n = bytes.len();
        let (s0_lo, s0_hi) = from_bi(self.0[0], self.0[1]);
        let mut s0 = ((s0_lo as u64) | ((s0_hi as u64) << 32)).to_le_bytes();
        if n <= 8 {
            s0[..n].copy_from_slice(bytes);
            let v = u64::from_le_bytes(s0);
            let (e, o) = to_bi(v as u32, (v >> 32) as u32);
            self.0[0] = e;
            self.0[1] = o;
        } else {
            let (s1_lo, s1_hi) = from_bi(self.0[2], self.0[3]);
            let mut s1 = ((s1_lo as u64) | ((s1_hi as u64) << 32)).to_le_bytes();
            s0.copy_from_slice(&bytes[..8]);
            s1[..n - 8].copy_from_slice(&bytes[8..]);
            let v0 = u64::from_le_bytes(s0);
            let v1 = u64::from_le_bytes(s1);
            let (e0, o0) = to_bi(v0 as u32, (v0 >> 32) as u32);
            let (e1, o1) = to_bi(v1 as u32, (v1 >> 32) as u32);
            self.0[0] = e0;
            self.0[1] = o0;
            self.0[2] = e1;
            self.0[3] = o1;
        }
    }

    #[inline]
    pub(crate) fn apply_domain_sep(&mut self) {
        self.0[9] ^= 0x8000_0000;
    }

    #[inline]
    pub(crate) fn apply_aead_pad(&mut self, n: usize) {
        debug_assert!(n < 16);
        if n < 8 {
            self.0[0] ^= 0x01u32 << (4 * n);
        } else {
            self.0[2] ^= 0x01u32 << (4 * (n - 8));
        }
    }

    #[inline]
    pub(crate) fn tag_bytes(&self) -> [u8; 16] {
        let mut tag = [0u8; 16];
        let a = from_bi(self.0[6], self.0[7]);
        let b = from_bi(self.0[8], self.0[9]);
        tag[..8].copy_from_slice(&((a.0 as u64) | ((a.1 as u64) << 32)).to_le_bytes());
        tag[8..].copy_from_slice(&((b.0 as u64) | ((b.1 as u64) << 32)).to_le_bytes());
        tag
    }

    #[inline]
    pub(crate) fn squeeze_byte(&self) -> [u8; 8] {
        let (lo, hi) = from_bi(self.0[0], self.0[1]);
        ((lo as u64) | ((hi as u64) << 32)).to_le_bytes()
    }

    #[inline]
    pub(crate) fn absorb_block(&mut self, block: &[u8]) {
        debug_assert_eq!(block.len(), 8);
        let v = u64::from_le_bytes(block.try_into().unwrap());
        let (e, o) = to_bi(v as u32, (v >> 32) as u32);
        self.0[0] ^= e;
        self.0[1] ^= o;
    }
}

// ============================================================================
// 32-bit Permutation
// ============================================================================

/// A single round of the Ascon permutation on 32-bit bit-interleaved state.
///
/// The 320-bit state is stored as 10 × u32 in bit-interleaved form (see
/// [`State`]): each 64-bit word is split into even and odd bits, so the S-box
/// is applied independently to both halves and every 64-bit rotation of the
/// diffusion layer reduces to a 32-bit rotation per half — with odd rotation
/// amounts expressed as a register swap — avoiding the cross-word shifts that a
/// plain lo/hi split would require.
///
/// The round constant is pre-interleaved before XORing into word 2.
/// The implementation is constant-time (no data-dependent branches or table lookups).
#[inline(always)]
pub(crate) fn round(state: &mut State, c: u8) {
    let s = &mut state.0;

    // p_C: round constant addition to word 2 (indices 4,5), pre-interleaved.
    // The low byte of the 64-bit word 2 maps to the low nibble of each half.
    let c_lo = ((c >> 0) & 0x1) | (((c >> 2) & 0x1) << 1) | (((c >> 4) & 0x1) << 2) | (((c >> 6) & 0x1) << 3);
    let c_hi = ((c >> 1) & 0x1) | (((c >> 3) & 0x1) << 1) | (((c >> 5) & 0x1) << 2) | (((c >> 7) & 0x1) << 3);
    s[4] ^= c_lo as u32;
    s[5] ^= c_hi as u32;

    // p_S: substitution layer, applied independently to even and odd halves.
    // Pre-XORs for even halves (indices 0,2,4,6,8).
    s[0] ^= s[8];
    s[8] ^= s[6];
    s[4] ^= s[2];

    // Pre-XORs for odd halves (indices 1,3,5,7,9).
    s[1] ^= s[9];
    s[9] ^= s[7];
    s[5] ^= s[3];

    // S-box for even halves.
    let t0 = s[0] ^ (!s[2] & s[4]);
    let t1 = s[2] ^ (!s[4] & s[6]);
    let t2 = s[4] ^ (!s[6] & s[8]);
    let t3 = s[6] ^ (!s[8] & s[0]);
    let t4 = s[8] ^ (!s[0] & s[2]);
    let t1 = t1 ^ t0;
    let t0 = t0 ^ t4;
    let t3 = t3 ^ t2;

    // S-box for odd halves.
    let u0 = s[1] ^ (!s[3] & s[5]);
    let u1 = s[3] ^ (!s[5] & s[7]);
    let u2 = s[5] ^ (!s[7] & s[9]);
    let u3 = s[7] ^ (!s[9] & s[1]);
    let u4 = s[9] ^ (!s[1] & s[3]);
    let u1 = u1 ^ u0;
    let u0 = u0 ^ u4;
    let u3 = u3 ^ u2;

    // p_L: linear diffusion layer, using the factored-rotation identity
    // `x ^ ROR(x, a) ^ ROR(x, b) = t ^ ROR(t ^ ROR(t, a-b), b)` with `t = x`
    // (matching the reference `bi32` implementation). Word 0: ror 19, ror 28;
    // Word 1: ror 61, ror 39; Word 2: ror 1, ror 6; Word 3: ror 10, ror 17;
    // Word 4: ror 7, ror 41. Each 64-bit rotation is two 32-bit rotations
    // (with a swap for odd amounts).

    // Word 0: delta = 28 - 19 = 9, then ror 19.
    let (e, o) = ror_pair(t0, u0, 9);
    let x0 = (t0 ^ e, u0 ^ o);
    let (e, o) = ror_pair(x0.0, x0.1, 19);
    s[0] = t0 ^ e;
    s[1] = u0 ^ o;

    // Word 1: delta = 61 - 39 = 22, then ror 39.
    let (e, o) = ror_pair(t1, u1, 22);
    let x1 = (t1 ^ e, u1 ^ o);
    let (e, o) = ror_pair(x1.0, x1.1, 39);
    s[2] = t1 ^ e;
    s[3] = u1 ^ o;

    // Word 2: delta = 6 - 1 = 5, then ror 1 (and the S-box NOT on word 2).
    let (e, o) = ror_pair(t2, u2, 5);
    let x2 = (t2 ^ e, u2 ^ o);
    let (e, o) = ror_pair(x2.0, x2.1, 1);
    s[4] = !(t2 ^ e);
    s[5] = !(u2 ^ o);

    // Word 3: delta = 17 - 10 = 7, then ror 10.
    let (e, o) = ror_pair(t3, u3, 7);
    let x3 = (t3 ^ e, u3 ^ o);
    let (e, o) = ror_pair(x3.0, x3.1, 10);
    s[6] = t3 ^ e;
    s[7] = u3 ^ o;

    // Word 4: delta = 41 - 7 = 34, then ror 7.
    let (e, o) = ror_pair(t4, u4, 34);
    let x4 = (t4 ^ e, u4 ^ o);
    let (e, o) = ror_pair(x4.0, x4.1, 7);
    s[8] = t4 ^ e;
    s[9] = u4 ^ o;
}

/// Apply 12 rounds of the Ascon permutation (initialization and finalization).
#[inline(always)]
pub(crate) fn p12(state: &mut State) {
    round(state, RC4);
    round(state, RC5);
    round(state, RC6);
    round(state, RC7);
    round(state, RC8);
    round(state, RC9);
    round(state, RC10);
    round(state, RC11);
    round(state, RC12);
    round(state, RC13);
    round(state, RC14);
    round(state, RC15);
}

/// Apply 8 rounds of the Ascon permutation (data processing).
#[inline(always)]
pub(crate) fn p8(state: &mut State) {
    round(state, RC8);
    round(state, RC9);
    round(state, RC10);
    round(state, RC11);
    round(state, RC12);
    round(state, RC13);
    round(state, RC14);
    round(state, RC15);
}

/// De-interleave a 32-bit word four bits at a time (Ascon bit-interleaving).
///
/// Returns the input with the even bits of each byte in the low nibble and the
/// odd bits in the high nibble.
#[inline(always)]
fn deinterleave16(x: u32) -> u32 {
    let mut x = x;
    let t = (x ^ (x >> 1)) & 0x2222_2222;
    x ^= t ^ (t << 1);
    let t = (x ^ (x >> 2)) & 0x0c0c_0c0c;
    x ^= t ^ (t << 2);
    let t = (x ^ (x >> 4)) & 0x00f0_00f0;
    x ^= t ^ (t << 4);
    let t = (x ^ (x >> 8)) & 0x0000_ff00;
    x ^= t ^ (t << 8);
    x
}

/// Inverse of [`deinterleave16`].
#[inline(always)]
fn interleave16(x: u32) -> u32 {
    let mut x = x;
    let t = (x ^ (x >> 8)) & 0x0000_ff00;
    x ^= t ^ (t << 8);
    let t = (x ^ (x >> 4)) & 0x00f0_00f0;
    x ^= t ^ (t << 4);
    let t = (x ^ (x >> 2)) & 0x0c0c_0c0c;
    x ^= t ^ (t << 2);
    let t = (x ^ (x >> 1)) & 0x2222_2222;
    x ^= t ^ (t << 1);
    x
}

/// Bit-interleave a 64-bit value given its `(lo, hi)` u32 halves, returning the
/// `(even, odd)` representation.
#[inline(always)]
fn to_bi(lo: u32, hi: u32) -> (u32, u32) {
    let t0 = deinterleave16(lo);
    let t1 = deinterleave16(hi);
    let e = (t1 << 16) | (t0 & 0x0000_ffff);
    let o = (t1 & 0xffff_0000) | (t0 >> 16);
    (e, o)
}

/// Inverse of [`to_bi`]: convert the `(even, odd)` representation back to the
/// `(lo, hi)` u32 halves.
#[inline(always)]
fn from_bi(e: u32, o: u32) -> (u32, u32) {
    let t0 = (o << 16) | (e & 0x0000_ffff);
    let t1 = (o & 0xffff_0000) | (e >> 16);
    (interleave16(t0), interleave16(t1))
}

/// Load a little-endian 64-bit value (padded with zeros) and bit-interleave it.
#[inline(always)]
fn load_to_bi(bytes: &[u8]) -> (u32, u32) {
    let mut tmp = [0u8; 8];
    tmp[..bytes.len()].copy_from_slice(bytes);
    let v = u64::from_le_bytes(tmp);
    to_bi(v as u32, (v >> 32) as u32)
}

/// Rotate a bit-interleaved 64-bit word right by `n` bits.
#[inline(always)]
fn ror_pair(even: u32, odd: u32, n: u32) -> (u32, u32) {
    if n & 1 == 0 {
        (even.rotate_right(n >> 1), odd.rotate_right(n >> 1))
    } else {
        (odd.rotate_right((n - 1) >> 1), even.rotate_right((n + 1) >> 1))
    }
}
