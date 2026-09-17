// Ascon core: 64-bit state representation and permutation.
//
// Compiled on targets with `target_pointer_width = "64"`. The 320-bit state is
// held as five native 64-bit words, so the permutation's rotations map directly
// to single instructions.

use super::{RC4, RC5, RC6, RC7, RC8, RC9, RC10, RC11, RC12, RC13, RC14, RC15};

// ============================================================================
// 64-bit State ([u64; 5])
// ============================================================================

/// The 320-bit Ascon state, consisting of five 64-bit words.
///
/// Words are stored in little-endian byte order per NIST SP 800-232 Appendix A.
/// State bit 0 is the LSB of S[0]; state bit 319 is the MSB of S[4].
#[derive(Clone, Debug)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub(crate) struct State(pub [u64; 5]);

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

    #[inline(always)]
    pub(crate) fn encrypt_in_place_block(&mut self, in_out: &mut [u8; 16]) {
        let pt0 = u64::from_le_bytes(in_out[0..8].try_into().unwrap());
        let pt1 = u64::from_le_bytes(in_out[8..16].try_into().unwrap());
        self.0[0] ^= pt0;
        self.0[1] ^= pt1;
        in_out[0..8].copy_from_slice(&self.0[0].to_le_bytes());
        in_out[8..16].copy_from_slice(&self.0[1].to_le_bytes());
    }

    #[inline(always)]
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
// 64-bit Permutation
// ============================================================================

/// A single round of the Ascon permutation.
///
/// Applies the constant addition `p_C`, substitution layer `p_S` (5-bit S-box applied
/// 64 times in parallel via bit-slicing), and linear diffusion layer `p_L`.
/// The implementation is constant-time (no data-dependent branches or table lookups).
#[inline(always)]
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
