// Ascon core: state representation and permutation.
//
// Uses the 64-bit representation when target_pointer_width = "64".
// Falls back to the 32-bit representation on all other targets.

// Round constants for the Ascon permutation (only the low byte of word 2 is touched).
const RC4: u8 = 0xf0;
const RC5: u8 = 0xe1;
const RC6: u8 = 0xd2;
const RC7: u8 = 0xc3;
const RC8: u8 = 0xb4;
const RC9: u8 = 0xa5;
const RC10: u8 = 0x96;
const RC11: u8 = 0x87;
const RC12: u8 = 0x78;
const RC13: u8 = 0x69;
const RC14: u8 = 0x5a;
const RC15: u8 = 0x4b;

// ============================================================================
// 64-bit State ([u64; 5])
// ============================================================================

/// The 320-bit Ascon state, consisting of five 64-bit words.
///
/// Words are stored in little-endian byte order per NIST SP 800-232 Appendix A.
/// State bit 0 is the LSB of S[0]; state bit 319 is the MSB of S[4].
#[derive(Clone, Debug)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
#[cfg(target_pointer_width = "64")]
pub(crate) struct State(pub [u64; 5]);

#[cfg(target_pointer_width = "64")]
impl State {
    #[inline]
    pub(crate) fn init_aead(key: &[u8; 16], nonce: &[u8; 16], iv: u64) -> Self {
        let k0 = u64::from_le_bytes(key[0..8].try_into().unwrap());
        let k1 = u64::from_le_bytes(key[8..16].try_into().unwrap());
        let n0 = u64::from_le_bytes(nonce[0..8].try_into().unwrap());
        let n1 = u64::from_le_bytes(nonce[8..16].try_into().unwrap());
        State([iv, k0, k1, n0, n1])
    }

    #[inline]
    pub(crate) fn init_hash(iv: u64) -> Self {
        State([iv, 0, 0, 0, 0])
    }

    #[inline]
    pub(crate) fn xor_word(&mut self, idx: usize, val: u64) {
        self.0[idx] ^= val;
    }

    #[inline]
    pub(crate) fn xor_rate128_bytes(&mut self, bytes: &[u8; 16]) {
        self.0[0] ^= u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        self.0[1] ^= u64::from_le_bytes(bytes[8..16].try_into().unwrap());
    }

    #[inline]
    pub(crate) fn xor_partial_rate(&mut self, bytes: &[u8]) {
        debug_assert!(bytes.len() <= 16 && !bytes.is_empty());
        if bytes.len() >= 8 {
            if bytes.len() > 8 {
                let mut tmp = [0u8; 8];
                let hi = bytes.len() - 8;
                tmp[..hi].copy_from_slice(&bytes[8..]);
                self.0[1] ^= u64::from_le_bytes(tmp);
            }
            self.0[0] ^= u64::from_le_bytes(bytes[..8].try_into().unwrap());
        } else {
            let mut tmp = [0u8; 8];
            tmp[..bytes.len()].copy_from_slice(bytes);
            self.0[0] ^= u64::from_le_bytes(tmp);
        }
    }

    #[inline]
    pub(crate) fn encrypt_in_place_block(&mut self, in_out: &mut [u8; 16]) {
        let pt0 = u64::from_le_bytes(in_out[0..8].try_into().unwrap());
        let pt1 = u64::from_le_bytes(in_out[8..16].try_into().unwrap());
        self.0[0] ^= pt0;
        self.0[1] ^= pt1;
        in_out[0..8].copy_from_slice(&self.0[0].to_le_bytes());
        in_out[8..16].copy_from_slice(&self.0[1].to_le_bytes());
    }

    #[inline]
    pub(crate) fn decrypt_in_place_block(&mut self, in_out: &mut [u8; 16]) {
        let ct0 = u64::from_le_bytes(in_out[0..8].try_into().unwrap());
        let ct1 = u64::from_le_bytes(in_out[8..16].try_into().unwrap());
        in_out[0..8].copy_from_slice(&(self.0[0] ^ ct0).to_le_bytes());
        in_out[8..16].copy_from_slice(&(self.0[1] ^ ct1).to_le_bytes());
        self.0[0] = ct0;
        self.0[1] = ct1;
    }

    #[inline]
    pub(crate) fn squeeze_rate_u64(&self) -> u64 {
        self.0[0]
    }

    #[inline]
    pub(crate) fn read_rate_bytes(&self, out: &mut [u8]) {
        debug_assert!(out.len() <= 16 && !out.is_empty());
        let s0 = self.0[0].to_le_bytes();
        let s1 = self.0[1].to_le_bytes();
        if out.len() <= 8 {
            out.copy_from_slice(&s0[..out.len()]);
        } else {
            let hi = out.len() - 8;
            out[..8].copy_from_slice(&s0);
            out[8..].copy_from_slice(&s1[..hi]);
        }
    }

    #[inline]
    pub(crate) fn write_rate_bytes(&mut self, bytes: &[u8]) {
        debug_assert!(bytes.len() <= 16 && !bytes.is_empty());
        let mut s0 = self.0[0].to_le_bytes();
        let n = bytes.len();
        if n <= 8 {
            s0[..n].copy_from_slice(bytes);
            self.0[0] = u64::from_le_bytes(s0);
        } else {
            let mut s1 = self.0[1].to_le_bytes();
            s0.copy_from_slice(&bytes[..8]);
            s1[..n - 8].copy_from_slice(&bytes[8..]);
            self.0[0] = u64::from_le_bytes(s0);
            self.0[1] = u64::from_le_bytes(s1);
        }
    }

    #[inline]
    pub(crate) fn apply_domain_sep(&mut self) {
        self.0[4] ^= 0x8000_0000_0000_0000;
    }

    #[inline]
    pub(crate) fn apply_aead_pad(&mut self, n: usize) {
        debug_assert!(n < 16);
        if n < 8 {
            self.0[0] ^= 0x01u64 << (8 * n);
        } else {
            self.0[1] ^= 0x01u64 << (8 * (n - 8));
        }
    }

    #[inline]
    pub(crate) fn tag_bytes(&self) -> [u8; 16] {
        let mut tag = [0u8; 16];
        tag[..8].copy_from_slice(&self.0[3].to_le_bytes());
        tag[8..].copy_from_slice(&self.0[4].to_le_bytes());
        tag
    }

    #[inline]
    pub(crate) fn squeeze_byte(&self) -> [u8; 8] {
        self.0[0].to_le_bytes()
    }

    #[inline]
    pub(crate) fn absorb_block(&mut self, block: &[u8]) {
        debug_assert_eq!(block.len(), 8);
        self.0[0] ^= u64::from_le_bytes(block.try_into().unwrap());
    }
}

// ============================================================================
// 32-bit State ([u32; 10]) — default for non-64-bit targets
// ============================================================================

/// The 320-bit Ascon state, stored as ten 32-bit words.
///
/// Each original 64-bit word is split into a low u32 (bytes 0-3) and a high u32
/// (bytes 4-7) in little-endian order:
///
/// | u32 index | Content              |
/// |-----------|----------------------|
/// | 0         | word 0 low (rate)    |
/// | 1         | word 0 high (rate)   |
/// | 2         | word 1 low (rate)    |
/// | 3         | word 1 high (rate)   |
/// | 4         | word 2 low           |
/// | 5         | word 2 high          |
/// | 6         | word 3 low (tag)     |
/// | 7         | word 3 high (tag)    |
/// | 8         | word 4 low (tag)     |
/// | 9         | word 4 high (tag)    |
///
/// State bit 0 is the LSB of word 0; state bit 319 is the MSB of word 4.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
#[cfg(not(target_pointer_width = "64"))]
pub(crate) struct State(pub [u32; 10]);

#[cfg(not(target_pointer_width = "64"))]
impl State {
    #[inline]
    pub(crate) fn init_aead(key: &[u8; 16], nonce: &[u8; 16], iv: u64) -> Self {
        let k0_lo = u32::from_le_bytes(key[0..4].try_into().unwrap());
        let k0_hi = u32::from_le_bytes(key[4..8].try_into().unwrap());
        let k1_lo = u32::from_le_bytes(key[8..12].try_into().unwrap());
        let k1_hi = u32::from_le_bytes(key[12..16].try_into().unwrap());
        let n0_lo = u32::from_le_bytes(nonce[0..4].try_into().unwrap());
        let n0_hi = u32::from_le_bytes(nonce[4..8].try_into().unwrap());
        let n1_lo = u32::from_le_bytes(nonce[8..12].try_into().unwrap());
        let n1_hi = u32::from_le_bytes(nonce[12..16].try_into().unwrap());
        let iv_lo = iv as u32;
        let iv_hi = (iv >> 32) as u32;
        State([iv_lo, iv_hi, k0_lo, k0_hi, k1_lo, k1_hi, n0_lo, n0_hi, n1_lo, n1_hi])
    }

    #[inline]
    pub(crate) fn init_hash(iv: u64) -> Self {
        State([iv as u32, (iv >> 32) as u32, 0, 0, 0, 0, 0, 0, 0, 0])
    }

    #[inline]
    pub(crate) fn xor_word(&mut self, idx: usize, val: u64) {
        self.0[idx * 2] ^= val as u32;
        self.0[idx * 2 + 1] ^= (val >> 32) as u32;
    }

    #[inline]
    pub(crate) fn xor_rate128_bytes(&mut self, bytes: &[u8; 16]) {
        self.0[0] ^= u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        self.0[1] ^= u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        self.0[2] ^= u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        self.0[3] ^= u32::from_le_bytes(bytes[12..16].try_into().unwrap());
    }

    #[inline]
    pub(crate) fn xor_partial_rate(&mut self, bytes: &[u8]) {
        debug_assert!(bytes.len() <= 16 && !bytes.is_empty());
        let n = bytes.len();
        if n >= 8 {
            if n > 8 {
                let mut tmp = [0u8; 4];
                tmp.copy_from_slice(&bytes[..4]);
                self.0[0] ^= u32::from_le_bytes(tmp);
                tmp.copy_from_slice(&bytes[4..8]);
                self.0[1] ^= u32::from_le_bytes(tmp);
                let rem = n - 8;
                let mut tmp = [0u8; 4];
                if rem <= 4 {
                    tmp[..rem].copy_from_slice(&bytes[8..8 + rem]);
                    self.0[2] ^= u32::from_le_bytes(tmp);
                } else {
                    self.0[2] ^= u32::from_le_bytes(bytes[8..12].try_into().unwrap());
                    tmp[..rem - 4].copy_from_slice(&bytes[12..]);
                    self.0[3] ^= u32::from_le_bytes(tmp);
                }
            } else {
                let mut tmp = [0u8; 4];
                tmp.copy_from_slice(&bytes[..4]);
                self.0[0] ^= u32::from_le_bytes(tmp);
                tmp.copy_from_slice(&bytes[4..8]);
                self.0[1] ^= u32::from_le_bytes(tmp);
            }
        } else if n <= 4 {
            let mut tmp = [0u8; 4];
            tmp[..n].copy_from_slice(bytes);
            self.0[0] ^= u32::from_le_bytes(tmp);
        } else {
            self.0[0] ^= u32::from_le_bytes(bytes[..4].try_into().unwrap());
            let mut tmp = [0u8; 4];
            tmp[..n - 4].copy_from_slice(&bytes[4..]);
            self.0[1] ^= u32::from_le_bytes(tmp);
        }
    }

    #[inline]
    pub(crate) fn encrypt_in_place_block(&mut self, in_out: &mut [u8; 16]) {
        let pt0_lo = u32::from_le_bytes(in_out[0..4].try_into().unwrap());
        let pt0_hi = u32::from_le_bytes(in_out[4..8].try_into().unwrap());
        let pt1_lo = u32::from_le_bytes(in_out[8..12].try_into().unwrap());
        let pt1_hi = u32::from_le_bytes(in_out[12..16].try_into().unwrap());
        self.0[0] ^= pt0_lo;
        self.0[1] ^= pt0_hi;
        self.0[2] ^= pt1_lo;
        self.0[3] ^= pt1_hi;
        in_out[0..4].copy_from_slice(&self.0[0].to_le_bytes());
        in_out[4..8].copy_from_slice(&self.0[1].to_le_bytes());
        in_out[8..12].copy_from_slice(&self.0[2].to_le_bytes());
        in_out[12..16].copy_from_slice(&self.0[3].to_le_bytes());
    }

    #[inline]
    pub(crate) fn decrypt_in_place_block(&mut self, in_out: &mut [u8; 16]) {
        let ct0_lo = u32::from_le_bytes(in_out[0..4].try_into().unwrap());
        let ct0_hi = u32::from_le_bytes(in_out[4..8].try_into().unwrap());
        let ct1_lo = u32::from_le_bytes(in_out[8..12].try_into().unwrap());
        let ct1_hi = u32::from_le_bytes(in_out[12..16].try_into().unwrap());
        in_out[0..4].copy_from_slice(&(self.0[0] ^ ct0_lo).to_le_bytes());
        in_out[4..8].copy_from_slice(&(self.0[1] ^ ct0_hi).to_le_bytes());
        in_out[8..12].copy_from_slice(&(self.0[2] ^ ct1_lo).to_le_bytes());
        in_out[12..16].copy_from_slice(&(self.0[3] ^ ct1_hi).to_le_bytes());
        self.0[0] = ct0_lo;
        self.0[1] = ct0_hi;
        self.0[2] = ct1_lo;
        self.0[3] = ct1_hi;
    }

    #[inline]
    pub(crate) fn squeeze_rate_u64(&self) -> u64 {
        (self.0[0] as u64) | ((self.0[1] as u64) << 32)
    }

    #[inline]
    pub(crate) fn read_rate_bytes(&self, out: &mut [u8]) {
        debug_assert!(out.len() <= 16 && !out.is_empty());
        let n = out.len();
        let s0_lo = self.0[0].to_le_bytes();
        let s0_hi = self.0[1].to_le_bytes();
        if n <= 4 {
            out.copy_from_slice(&s0_lo[..n]);
        } else if n <= 8 {
            out[..4].copy_from_slice(&s0_lo);
            out[4..n].copy_from_slice(&s0_hi[..n - 4]);
        } else {
            let s1_lo = self.0[2].to_le_bytes();
            let s1_hi = self.0[3].to_le_bytes();
            out[..4].copy_from_slice(&s0_lo);
            out[4..8].copy_from_slice(&s0_hi);
            let hi = n - 8;
            if hi <= 4 {
                out[8..n].copy_from_slice(&s1_lo[..hi]);
            } else {
                out[8..12].copy_from_slice(&s1_lo);
                out[12..n].copy_from_slice(&s1_hi[..hi - 4]);
            }
        }
    }

    #[inline]
    pub(crate) fn write_rate_bytes(&mut self, bytes: &[u8]) {
        debug_assert!(bytes.len() <= 16 && !bytes.is_empty());
        let n = bytes.len();
        if n <= 4 {
            let mut s0 = self.0[0].to_le_bytes();
            s0[..n].copy_from_slice(bytes);
            self.0[0] = u32::from_le_bytes(s0);
        } else if n <= 8 {
            self.0[0] = u32::from_le_bytes(bytes[..4].try_into().unwrap());
            let mut s1 = self.0[1].to_le_bytes();
            s1[..n - 4].copy_from_slice(&bytes[4..]);
            self.0[1] = u32::from_le_bytes(s1);
        } else if n <= 12 {
            self.0[0] = u32::from_le_bytes(bytes[..4].try_into().unwrap());
            self.0[1] = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
            let mut s2 = self.0[2].to_le_bytes();
            s2[..n - 8].copy_from_slice(&bytes[8..]);
            self.0[2] = u32::from_le_bytes(s2);
        } else {
            self.0[0] = u32::from_le_bytes(bytes[..4].try_into().unwrap());
            self.0[1] = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
            self.0[2] = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
            if n < 16 {
                let mut s3 = self.0[3].to_le_bytes();
                s3[..n - 12].copy_from_slice(&bytes[12..]);
                self.0[3] = u32::from_le_bytes(s3);
            } else {
                self.0[3] = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
            }
        }
    }

    #[inline]
    pub(crate) fn apply_domain_sep(&mut self) {
        self.0[9] ^= 0x8000_0000;
    }

    #[inline]
    pub(crate) fn apply_aead_pad(&mut self, n: usize) {
        debug_assert!(n < 16);
        if n < 4 {
            self.0[0] ^= 0x01u32 << (8 * n);
        } else if n < 8 {
            self.0[1] ^= 0x01u32 << (8 * (n - 4));
        } else if n < 12 {
            self.0[2] ^= 0x01u32 << (8 * (n - 8));
        } else {
            self.0[3] ^= 0x01u32 << (8 * (n - 12));
        }
    }

    #[inline]
    pub(crate) fn tag_bytes(&self) -> [u8; 16] {
        let mut tag = [0u8; 16];
        tag[0..4].copy_from_slice(&self.0[6].to_le_bytes());
        tag[4..8].copy_from_slice(&self.0[7].to_le_bytes());
        tag[8..12].copy_from_slice(&self.0[8].to_le_bytes());
        tag[12..16].copy_from_slice(&self.0[9].to_le_bytes());
        tag
    }

    #[inline]
    pub(crate) fn squeeze_byte(&self) -> [u8; 8] {
        let mut out = [0u8; 8];
        out[0..4].copy_from_slice(&self.0[0].to_le_bytes());
        out[4..8].copy_from_slice(&self.0[1].to_le_bytes());
        out
    }

    #[inline]
    pub(crate) fn absorb_block(&mut self, block: &[u8]) {
        debug_assert_eq!(block.len(), 8);
        self.0[0] ^= u32::from_le_bytes(block[0..4].try_into().unwrap());
        self.0[1] ^= u32::from_le_bytes(block[4..8].try_into().unwrap());
    }
}

// ============================================================================
// 64-bit Permutation
// ============================================================================

/// A single round of the Ascon permutation.
///
/// Applies the constant addition `p_C`, substitution layer `p_S` (5-bit S-box applied
/// 64 times in parallel via bit-slicing), and linear diffusion layer `p_L`.
/// The implementation is constant-time (no data-dependent branches or table lookups).
#[inline(always)]
#[cfg(target_pointer_width = "64")]
pub(crate) fn round(state: &mut State, c: u8) {
    // p_C: constant addition to word 2
    state.0[2] ^= c as u64;

    // p_S: substitution layer (bit-sliced 5-bit S-box)
    let s = &mut state.0;
    s[0] ^= s[4];
    s[4] ^= s[3];
    s[2] ^= s[1];

    let t0 = s[0] ^ (!s[1] & s[2]);
    let t1 = s[1] ^ (!s[2] & s[3]);
    let t2 = s[2] ^ (!s[3] & s[4]);
    let t3 = s[3] ^ (!s[4] & s[0]);
    let t4 = s[4] ^ (!s[0] & s[1]);

    let t1 = t1 ^ t0;
    let t0 = t0 ^ t4;
    let t3 = t3 ^ t2;
    let t2 = !t2;

    // p_L: linear diffusion layer
    s[0] = t0 ^ t0.rotate_right(19) ^ t0.rotate_right(28);
    s[1] = t1 ^ t1.rotate_right(61) ^ t1.rotate_right(39);
    s[2] = t2 ^ t2.rotate_right(1) ^ t2.rotate_right(6);
    s[3] = t3 ^ t3.rotate_right(10) ^ t3.rotate_right(17);
    s[4] = t4 ^ t4.rotate_right(7) ^ t4.rotate_right(41);
}

/// Apply 12 rounds of the Ascon permutation (initialization and finalization).
#[inline]
#[cfg(target_pointer_width = "64")]
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
#[inline]
#[cfg(target_pointer_width = "64")]
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

// ============================================================================
// 32-bit Permutation — default for non-64-bit targets
// ============================================================================

/// A single round of the Ascon permutation on 32-bit state.
///
/// The 320-bit state is stored as 10 × u32 (each 64-bit word split into lo/hi halves).
/// The S-box is applied independently to both the low and high halves.
/// The linear diffusion layer uses explicit cross-32-bit rotations.
#[inline(always)]
#[cfg(not(target_pointer_width = "64"))]
pub(crate) fn round(state: &mut State, c: u8) {
    let s = &mut state.0;

    // p_C: constant addition to word 2 (low byte -> low u32)
    s[4] ^= c as u32;

    // p_S: substitution layer (bit-sliced 5-bit S-box)
    // Pre-XORs for low halves (indices 0,2,4,6,8)
    s[0] ^= s[8];
    s[8] ^= s[6];
    s[4] ^= s[2];

    // Pre-XORs for high halves (indices 1,3,5,7,9)
    s[1] ^= s[9];
    s[9] ^= s[7];
    s[5] ^= s[3];

    // S-box for low halves
    let t0 = s[0] ^ (!s[2] & s[4]);
    let t1 = s[2] ^ (!s[4] & s[6]);
    let t2 = s[4] ^ (!s[6] & s[8]);
    let t3 = s[6] ^ (!s[8] & s[0]);
    let t4 = s[8] ^ (!s[0] & s[2]);
    let t1 = t1 ^ t0;
    let t0 = t0 ^ t4;
    let t3 = t3 ^ t2;
    let t2 = !t2;

    // S-box for high halves
    let u0 = s[1] ^ (!s[3] & s[5]);
    let u1 = s[3] ^ (!s[5] & s[7]);
    let u2 = s[5] ^ (!s[7] & s[9]);
    let u3 = s[7] ^ (!s[9] & s[1]);
    let u4 = s[9] ^ (!s[1] & s[3]);
    let u1 = u1 ^ u0;
    let u0 = u0 ^ u4;
    let u3 = u3 ^ u2;
    let u2 = !u2;

    // p_L: linear diffusion layer
    // Word 0: ror 19 (n<32), ror 28 (n<32)
    let r19_lo = (t0 >> 19) | (u0 << 13);
    let r19_hi = (u0 >> 19) | (t0 << 13);
    let r28_lo = (t0 >> 28) | (u0 << 4);
    let r28_hi = (u0 >> 28) | (t0 << 4);
    s[0] = t0 ^ r19_lo ^ r28_lo;
    s[1] = u0 ^ r19_hi ^ r28_hi;

    // Word 1: ror 61 (n>32), ror 39 (n>32)
    let r61_lo = (u1 >> 29) | (t1 << 3);
    let r61_hi = (t1 >> 29) | (u1 << 3);
    let r39_lo = (u1 >> 7) | (t1 << 25);
    let r39_hi = (t1 >> 7) | (u1 << 25);
    s[2] = t1 ^ r61_lo ^ r39_lo;
    s[3] = u1 ^ r61_hi ^ r39_hi;

    // Word 2: ror 1 (n<32), ror 6 (n<32)
    let r1_lo = (t2 >> 1) | (u2 << 31);
    let r1_hi = (u2 >> 1) | (t2 << 31);
    let r6_lo = (t2 >> 6) | (u2 << 26);
    let r6_hi = (u2 >> 6) | (t2 << 26);
    s[4] = t2 ^ r1_lo ^ r6_lo;
    s[5] = u2 ^ r1_hi ^ r6_hi;

    // Word 3: ror 10 (n<32), ror 17 (n<32)
    let r10_lo = (t3 >> 10) | (u3 << 22);
    let r10_hi = (u3 >> 10) | (t3 << 22);
    let r17_lo = (t3 >> 17) | (u3 << 15);
    let r17_hi = (u3 >> 17) | (t3 << 15);
    s[6] = t3 ^ r10_lo ^ r17_lo;
    s[7] = u3 ^ r10_hi ^ r17_hi;

    // Word 4: ror 7 (n<32), ror 41 (n>32)
    let r7_lo = (t4 >> 7) | (u4 << 25);
    let r7_hi = (u4 >> 7) | (t4 << 25);
    let r41_lo = (u4 >> 9) | (t4 << 23);
    let r41_hi = (t4 >> 9) | (u4 << 23);
    s[8] = t4 ^ r7_lo ^ r41_lo;
    s[9] = u4 ^ r7_hi ^ r41_hi;
}

/// Apply 12 rounds of the Ascon permutation (initialization and finalization).
#[inline]
#[cfg(not(target_pointer_width = "64"))]
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
#[inline]
#[cfg(not(target_pointer_width = "64"))]
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
