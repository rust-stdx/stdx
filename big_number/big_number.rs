#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};
use core::{
    fmt,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

pub const MAX_LIMBS: usize = 256;

const fn max_limbs<const BITS: usize, const LIMBS: usize>() -> [u64; LIMBS] {
    let mut limbs = [u64::MAX; LIMBS];
    let rem = BITS % 64;
    if rem != 0 {
        limbs[LIMBS - 1] = (1u64 << rem) - 1;
    }
    limbs
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Uint<const BITS: usize, const LIMBS: usize> {
    pub limbs: [u64; LIMBS],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Int<const BITS: usize, const LIMBS: usize>(Uint<BITS, LIMBS>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRadix,
    InvalidDigit,
    EmptyString,
    Overflow,
}

#[inline]
pub const fn adc(a: u64, b: u64, carry: u64) -> (u64, u64) {
    let sum = (a as u128) + (b as u128) + (carry as u128);
    (sum as u64, (sum >> 64) as u64)
}

#[inline]
pub const fn sbb(a: u64, b: u64, borrow: u64) -> (u64, u64) {
    let diff = (1u128 << 64) + (a as u128) - (b as u128) - (borrow as u128);
    (diff as u64, 1u64.wrapping_sub((diff >> 64) as u64))
}

#[inline]
pub const fn mac(acc: u64, a: u64, b: u64, carry: u64) -> (u64, u64) {
    let value = (acc as u128) + (a as u128) * (b as u128) + (carry as u128);
    (value as u64, (value >> 64) as u64)
}

fn mul_limbs(out: &mut [u64], a: &[u64], a_len: usize, b: &[u64], b_len: usize) {
    for i in 0..a_len {
        let mut carry = 0u64;
        for j in 0..b_len {
            let (word, next_carry) = mac(out[i + j], a[i], b[j], carry);
            out[i + j] = word;
            carry = next_carry;
        }
        let mut k = i + b_len;
        while carry != 0 {
            let (word, next_carry) = adc(out[k], 0, carry);
            out[k] = word;
            carry = next_carry;
            k += 1;
        }
    }
}

#[inline]
const fn ct_select_u64(a: u64, b: u64, choice: bool) -> u64 {
    let mask = 0u64.wrapping_sub(choice as u64);
    b ^ ((a ^ b) & mask)
}

#[inline]
const fn digit_to_char(digit: u64, upper: bool) -> char {
    match digit {
        0..=9 => (b'0' + digit as u8) as char,
        _ if upper => (b'A' + (digit as u8 - 10)) as char,
        _ => (b'a' + (digit as u8 - 10)) as char,
    }
}

#[inline]
fn char_to_digit(byte: u8) -> Option<u32> {
    match byte {
        b'0'..=b'9' => Some((byte - b'0') as u32),
        b'a'..=b'z' => Some((byte - b'a' + 10) as u32),
        b'A'..=b'Z' => Some((byte - b'A' + 10) as u32),
        _ => None,
    }
}

const fn uint_from_u128<const BITS: usize, const LIMBS: usize>(value: u128) -> Uint<BITS, LIMBS> {
    let mut limbs = [0u64; LIMBS];
    if LIMBS > 0 {
        limbs[0] = value as u64;
    }
    if LIMBS > 1 {
        limbs[1] = (value >> 64) as u64;
    }
    Uint {
        limbs,
    }
}

fn u128_to_word(value: u128) -> u64 {
    u64::try_from(value).expect("primitive operand exceeds u64")
}

fn i128_abs_to_word(value: i128) -> (u64, bool) {
    (u128_to_word(value.unsigned_abs()), value.is_negative())
}

impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
    const _LIMBS_CHECK: () = assert!(LIMBS == (BITS + 63) / 64, "LIMBS must equal ceil(BITS/64)");

    pub const ZERO: Self = Self {
        limbs: [0u64; LIMBS],
    };
    pub const ONE: Self = Self::from_u64(1);
    pub const MAX: Self = Self {
        limbs: max_limbs::<BITS, LIMBS>(),
    };

    #[inline]
    pub const fn from_limbs(limbs: [u64; LIMBS]) -> Self {
        Self {
            limbs,
        }
    }

    #[inline]
    pub const fn from_u64(v: u64) -> Self {
        let mut limbs = [0u64; LIMBS];
        if LIMBS > 0 {
            limbs[0] = v;
        }
        Self {
            limbs,
        }
    }

    #[inline]
    pub fn from_be_slice(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), BITS / 8);
        let mut limbs = [0u64; LIMBS];
        let mut i = 0;
        while i < LIMBS {
            let start = bytes.len() - ((i + 1) * 8);
            let mut limb = [0u8; 8];
            limb.copy_from_slice(&bytes[start..start + 8]);
            limbs[i] = u64::from_be_bytes(limb);
            i += 1;
        }
        Self {
            limbs,
        }
    }

    #[inline]
    pub fn from_le_slice(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), BITS / 8);
        let mut limbs = [0u64; LIMBS];
        let mut i = 0;
        while i < LIMBS {
            let start = i * 8;
            let mut limb = [0u8; 8];
            limb.copy_from_slice(&bytes[start..start + 8]);
            limbs[i] = u64::from_le_bytes(limb);
            i += 1;
        }
        Self {
            limbs,
        }
    }

    #[cfg(feature = "alloc")]
    #[inline]
    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(BITS / 8);
        let mut i = LIMBS;
        while i > 0 {
            i -= 1;
            out.extend_from_slice(&self.limbs[i].to_be_bytes());
        }
        out
    }

    #[cfg(feature = "alloc")]
    #[inline]
    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(BITS / 8);
        let mut i = 0;
        while i < LIMBS {
            out.extend_from_slice(&self.limbs[i].to_le_bytes());
            i += 1;
        }
        out
    }

    #[inline]
    pub fn to_be_bytes_fixed<const N: usize>(&self) -> [u8; N] {
        assert_eq!(N, BITS / 8);
        let mut out = [0u8; N];
        let mut i = 0;
        while i < LIMBS {
            let start = N - ((i + 1) * 8);
            out[start..start + 8].copy_from_slice(&self.limbs[i].to_be_bytes());
            i += 1;
        }
        out
    }

    #[inline]
    pub fn to_le_bytes_fixed<const N: usize>(&self) -> [u8; N] {
        assert_eq!(N, BITS / 8);
        let mut out = [0u8; N];
        let mut i = 0;
        while i < LIMBS {
            let start = i * 8;
            out[start..start + 8].copy_from_slice(&self.limbs[i].to_le_bytes());
            i += 1;
        }
        out
    }

    #[inline]
    pub fn bit(&self, index: usize) -> bool {
        if index >= BITS {
            return false;
        }
        ((self.limbs[index / 64] >> (index % 64)) & 1) == 1
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        let mut acc = 0u64;
        let mut i = 0;
        while i < LIMBS {
            acc |= self.limbs[i];
            i += 1;
        }
        acc == 0
    }

    /// Position of the highest set bit, plus one. Returns 0 for zero.
    pub fn bit_len(&self) -> usize {
        let mut i = LIMBS;
        while i > 0 {
            i -= 1;
            if self.limbs[i] != 0 {
                return i * 64 + 64 - self.limbs[i].leading_zeros() as usize;
            }
        }
        0
    }

    #[inline]
    pub fn is_odd(&self) -> bool {
        (self.limbs[0] & 1) == 1
    }

    #[inline]
    pub fn ct_ge(&self, rhs: &Self) -> bool {
        let (_, borrow) = self.sub_raw(rhs);
        borrow == 0
    }

    #[inline]
    pub fn ct_eq(&self, rhs: &Self) -> bool {
        let mut diff = 0u64;
        let mut i = 0;
        while i < LIMBS {
            diff |= self.limbs[i] ^ rhs.limbs[i];
            i += 1;
        }
        diff == 0
    }

    #[inline]
    pub fn ct_select(a: &Self, b: &Self, choice: bool) -> Self {
        let mut limbs = [0u64; LIMBS];
        let mut i = 0;
        while i < LIMBS {
            limbs[i] = ct_select_u64(a.limbs[i], b.limbs[i], choice);
            i += 1;
        }
        Self {
            limbs,
        }
    }

    #[inline]
    pub fn add_raw(&self, rhs: &Self) -> (Self, u64) {
        let mut out = [0u64; LIMBS];
        let mut carry = 0u64;
        let mut i = 0;
        while i < LIMBS {
            let (word, next_carry) = adc(self.limbs[i], rhs.limbs[i], carry);
            out[i] = word;
            carry = next_carry;
            i += 1;
        }
        (
            Self {
                limbs: out,
            },
            carry,
        )
    }

    #[inline]
    pub fn sub_raw(&self, rhs: &Self) -> (Self, u64) {
        let mut out = [0u64; LIMBS];
        let mut borrow = 0u64;
        let mut i = 0;
        while i < LIMBS {
            let (word, next_borrow) = sbb(self.limbs[i], rhs.limbs[i], borrow);
            out[i] = word;
            borrow = next_borrow;
            i += 1;
        }
        (
            Self {
                limbs: out,
            },
            borrow,
        )
    }

    #[inline]
    pub fn add_mod(&self, rhs: &Self, modulus: &Self) -> Self {
        let (sum, carry) = self.add_raw(rhs);
        let (reduced, borrow) = sum.sub_raw(modulus);
        // Use bitwise OR to avoid short-circuit branching on secret carry/borrow bits.
        // Reduce when carry==1 (overflow) OR borrow==0 (sum >= modulus).
        Self::ct_select(&reduced, &sum, (carry | (borrow ^ 1)) != 0)
    }

    #[inline]
    pub fn sub_mod(&self, rhs: &Self, modulus: &Self) -> Self {
        let (diff, borrow) = self.sub_raw(rhs);
        let (corrected, _) = diff.add_raw(modulus);
        Self::ct_select(&corrected, &diff, borrow == 1)
    }

    #[inline]
    pub fn double_mod(&self, modulus: &Self) -> Self {
        self.add_mod(self, modulus)
    }

    fn mul_wide_internal(&self, rhs: &Self) -> [u64; MAX_LIMBS] {
        assert!(LIMBS * 2 <= MAX_LIMBS);
        let mut out = [0u64; MAX_LIMBS];
        let mut i = 0;
        while i < LIMBS {
            let mut carry = 0u64;
            let mut j = 0;
            while j < LIMBS {
                let (word, next_carry) = mac(out[i + j], self.limbs[i], rhs.limbs[j], carry);
                out[i + j] = word;
                carry = next_carry;
                j += 1;
            }
            let mut k = i + LIMBS;
            while k < MAX_LIMBS {
                let (word, next_carry) = adc(out[k], 0, carry);
                out[k] = word;
                carry = next_carry;
                k += 1;
            }
            i += 1;
        }
        out
    }

    fn reduce_wide_internal(product: &[u64; MAX_LIMBS], modulus: &Self) -> Self {
        let total_bits = LIMBS * 128;
        let mut rem = Self::ZERO;
        let mut bit_index = total_bits;
        while bit_index > 0 {
            bit_index -= 1;
            let limb_idx = bit_index / 64;
            let bit_pos = bit_index % 64;
            let bit = ((product[limb_idx] >> bit_pos) & 1) as u64;

            let mut shifted = [0u64; LIMBS];
            let mut carry = bit;
            let mut i = 0;
            while i < LIMBS {
                let next = rem.limbs[i] >> 63;
                shifted[i] = (rem.limbs[i] << 1) | carry;
                carry = next;
                i += 1;
            }
            let shifted_rem = Self {
                limbs: shifted,
            };
            let (reduced, borrow) = shifted_rem.sub_raw(modulus);
            // Use bitwise OR to avoid short-circuit branching on secret carry/borrow bits.
            rem = Self::ct_select(&reduced, &shifted_rem, (carry | (borrow ^ 1)) != 0);
        }
        rem
    }

    #[inline]
    pub fn mul_mod(&self, rhs: &Self, modulus: &Self) -> Self {
        let product = self.mul_wide_internal(rhs);
        Self::reduce_wide_internal(&product, modulus)
    }

    /// Precompute mu for Barrett reduction, auto-detecting effective limb count.
    pub fn compute_mu_for_barrett(&self) -> [u64; MAX_LIMBS] {
        let eff_limbs = (self.bit_len() + 63) / 64;
        self.compute_mu_for_barrett_eff(eff_limbs)
    }

    fn compute_mu_for_barrett_eff(&self, eff_limbs: usize) -> [u64; MAX_LIMBS] {
        const {
            assert!(LIMBS + 1 <= MAX_LIMBS);
        }
        assert!(eff_limbs >= 1 && eff_limbs <= LIMBS);

        let total_bits = eff_limbs * 128;
        let mut mu = [0u64; MAX_LIMBS];
        let mut rem = Self::ZERO;

        for bit_pos in (0..=total_bits).rev() {
            let mut shifted_limbs = [0u64; LIMBS];
            let mut carry = 0u64;
            for i in 0..LIMBS {
                let next = rem.limbs[i] >> 63;
                shifted_limbs[i] = (rem.limbs[i] << 1) | carry;
                carry = next;
            }

            let bit = if bit_pos == total_bits { 1u64 } else { 0u64 };
            let mut with = shifted_limbs;
            with[0] |= bit;

            let shifted = Self {
                limbs: with,
            };
            let (reduced, borrow) = shifted.sub_raw(self);
            // Use the same overflow-aware selection as reduce_wide_internal:
            // carry == 1 → true value >= 2^(LIMBS*64) >= divisor → always subtract.
            rem = Self::ct_select(&reduced, &shifted, (carry | (borrow ^ 1)) != 0);
            // When we subtracted, the quotient bit is 1.
            if carry | (borrow ^ 1) != 0 {
                let qi = bit_pos / 64;
                let qbit = bit_pos % 64;
                if qi < eff_limbs + 1 {
                    mu[qi] |= 1 << qbit;
                }
            }
        }

        mu
    }

    /// Modular exponentiation using Barrett reduction.
    pub fn modpow_barrett(&self, exp: &Self, modulus: &Self, mu: &[u64]) -> Self {
        let eff_limbs = (modulus.bit_len() + 63) / 64;
        self.modpow_barrett_eff(exp, modulus, mu, eff_limbs)
    }

    /// Barrett modular exponentiation with explicit effective limb count.
    pub fn modpow_barrett_eff(&self, exp: &Self, modulus: &Self, mu: &[u64], eff_limbs: usize) -> Self {
        let mut base = self.mul_mod_barrett_eff(&Self::ONE, modulus, mu, eff_limbs);
        let mut result = Self::ONE;
        let bits = exp.bit_len();
        let mut i = 0;
        while i < bits {
            if exp.bit(i) {
                result = result.mul_mod_barrett_eff(&base, modulus, mu, eff_limbs);
            }
            base = base.mul_mod_barrett_eff(&base, modulus, mu, eff_limbs);
            i += 1;
        }
        result
    }

    /// Modular multiplication using Barrett reduction.
    pub fn mul_mod_barrett(&self, rhs: &Self, modulus: &Self, mu: &[u64]) -> Self {
        let product = self.mul_wide_internal(rhs);
        Self::reduce_wide_barrett(&product, modulus, mu)
    }

    /// Modular multiplication using Barrett reduction with explicit effective limb count.
    pub fn mul_mod_barrett_eff(&self, rhs: &Self, modulus: &Self, mu: &[u64], eff_limbs: usize) -> Self {
        let product = self.mul_wide_internal(rhs);
        Self::reduce_wide_barrett_eff(&product, modulus, mu, eff_limbs)
    }

    /// Barrett reduction with explicit effective limb count.
    pub fn reduce_wide_barrett_eff(product: &[u64; MAX_LIMBS], modulus: &Self, mu: &[u64], eff_limbs: usize) -> Self {
        assert!(mu.len() >= eff_limbs + 1);

        let k = eff_limbs;
        let k1 = k + 1;
        let k_minus_1 = k.saturating_sub(1);

        let mut q1 = [0u64; MAX_LIMBS];
        q1[..k1].copy_from_slice(&product[k_minus_1..k_minus_1 + k1]);

        let mut q2 = [0u64; MAX_LIMBS];
        mul_limbs(&mut q2, &q1, k1, mu, k1);

        let mut q3 = [0u64; MAX_LIMBS];
        q3[..k1].copy_from_slice(&q2[k1..k1 + k1]);

        let mut r1 = [0u64; MAX_LIMBS];
        r1[..k1].copy_from_slice(&product[..k1]);

        let mut q3m = [0u64; MAX_LIMBS];
        mul_limbs(&mut q3m, &q3[..k1], k1, &modulus.limbs, k);
        let mut r2 = [0u64; MAX_LIMBS];
        r2[..k1].copy_from_slice(&q3m[..k1]);

        let mut r = [0u64; MAX_LIMBS];
        let mut borrow = 0u64;
        let mut i = 0;
        while i < k1 {
            let (word, next_borrow) = sbb(r1[i], r2[i], borrow);
            r[i] = word;
            borrow = next_borrow;
            i += 1;
        }

        // Conditional subtract modulus up to twice, using the full k+1-limb r.
        // The Barrett remainder r = product - q3 * n can be up to 3n, requiring
        // k+1 limbs.  Truncating to k limbs before subtracting would lose the
        // top limb and give incorrect results.
        for _ in 0..2 {
            let mut r_try = r;
            let mut borrow = 0u64;
            let mut j = 0;
            while j < k1 {
                let mod_limb = if j < LIMBS { modulus.limbs[j] } else { 0u64 };
                let (word, next_borrow) = sbb(r_try[j], mod_limb, borrow);
                r_try[j] = word;
                borrow = next_borrow;
                j += 1;
            }
            let choose = borrow == 0;
            let mut j = 0;
            while j < k1 {
                r[j] = ct_select_u64(r_try[j], r[j], choose);
                j += 1;
            }
        }

        let mut limbs = [0u64; LIMBS];
        limbs[..k].copy_from_slice(&r[..k]);
        Self {
            limbs,
        }
    }

    /// Modular exponentiation: `self^exp mod modulus` using square-and-multiply,
    /// scanning only up to the exponent's bit length.
    pub fn modpow(&self, exp: &Self, modulus: &Self) -> Self {
        let mut base = self.mul_mod(&Self::ONE, modulus);
        let mut result = Self::ONE;
        let bits = exp.bit_len();
        let mut i = 0;
        while i < bits {
            if exp.bit(i) {
                result = result.mul_mod(&base, modulus);
            }
            base = base.mul_mod(&base, modulus);
            i += 1;
        }
        result
    }

    /// Barrett reduction of a 2*LIMBS-wide product modulo `modulus`.
    ///
    /// `mu` must equal `floor(2^(2*LIMBS*64) / modulus)` and have at least `LIMBS+1` entries.
    /// Prefer `reduce_wide_barrett_eff` which auto-detects the effective limb count.
    pub fn reduce_wide_barrett(product: &[u64; MAX_LIMBS], modulus: &Self, mu: &[u64]) -> Self {
        assert!(mu.len() >= LIMBS + 1);

        let k = LIMBS;
        let k1 = k + 1;
        let k_minus_1 = k - 1;

        let mut q1 = [0u64; MAX_LIMBS];
        q1[..k1].copy_from_slice(&product[k_minus_1..k_minus_1 + k1]);

        let mut q2 = [0u64; MAX_LIMBS];
        mul_limbs(&mut q2, &q1, k1, mu, k1);

        let mut q3 = [0u64; MAX_LIMBS];
        q3[..k1].copy_from_slice(&q2[k1..k1 + k1]);

        let mut r1 = [0u64; MAX_LIMBS];
        r1[..k1].copy_from_slice(&product[..k1]);

        let mut q3m = [0u64; MAX_LIMBS];
        mul_limbs(&mut q3m, &q3[..k1], k1, &modulus.limbs, k);
        let mut r2 = [0u64; MAX_LIMBS];
        r2[..k1].copy_from_slice(&q3m[..k1]);

        let mut r = [0u64; MAX_LIMBS];
        let mut borrow = 0u64;
        let mut i = 0;
        while i < k1 {
            let (word, next_borrow) = sbb(r1[i], r2[i], borrow);
            r[i] = word;
            borrow = next_borrow;
            i += 1;
        }

        // Conditional subtract modulus up to twice, using the full k+1-limb r.
        // The Barrett remainder r = product - q3 * n can be up to 3n, requiring
        // k+1 limbs.  Truncating to k limbs before subtracting would lose the
        // top limb and give incorrect results.
        for _ in 0..2 {
            let mut r_try = r;
            let mut borrow = 0u64;
            let mut j = 0;
            while j < k1 {
                let mod_limb = if j < LIMBS { modulus.limbs[j] } else { 0u64 };
                let (word, next_borrow) = sbb(r_try[j], mod_limb, borrow);
                r_try[j] = word;
                borrow = next_borrow;
                j += 1;
            }
            let choose = borrow == 0;
            let mut j = 0;
            while j < k1 {
                r[j] = ct_select_u64(r_try[j], r[j], choose);
                j += 1;
            }
        }

        let mut limbs = [0u64; LIMBS];
        limbs.copy_from_slice(&r[..LIMBS]);
        Self {
            limbs,
        }
    }

    #[inline]
    pub fn add_word(&self, word: u64) -> (Self, u64) {
        let mut out = self.limbs;
        let (first, mut carry) = adc(out[0], word, 0);
        out[0] = first;
        let mut i = 1;
        while i < LIMBS {
            let (next, next_carry) = adc(out[i], 0, carry);
            out[i] = next;
            carry = next_carry;
            i += 1;
        }
        (
            Self {
                limbs: out,
            },
            carry,
        )
    }

    #[inline]
    pub fn sub_word(&self, word: u64) -> (Self, u64) {
        let mut out = self.limbs;
        let (first, mut borrow) = sbb(out[0], word, 0);
        out[0] = first;
        let mut i = 1;
        while i < LIMBS {
            let (next, next_borrow) = sbb(out[i], 0, borrow);
            out[i] = next;
            borrow = next_borrow;
            i += 1;
        }
        (
            Self {
                limbs: out,
            },
            borrow,
        )
    }

    #[inline]
    pub fn mul_word(&self, word: u64) -> (Self, u64) {
        let mut out = [0u64; LIMBS];
        let mut carry = 0u64;
        let mut i = 0;
        while i < LIMBS {
            let (next, next_carry) = mac(0, self.limbs[i], word, carry);
            out[i] = next;
            carry = next_carry;
            i += 1;
        }
        (
            Self {
                limbs: out,
            },
            carry,
        )
    }

    #[inline]
    pub fn div_rem_word(&self, word: u64) -> (Self, u64) {
        assert!(word != 0, "division by zero");
        let mut out = [0u64; LIMBS];
        let mut rem = 0u64;
        let mut i = LIMBS;
        while i > 0 {
            i -= 1;
            let dividend = ((rem as u128) << 64) | self.limbs[i] as u128;
            out[i] = (dividend / word as u128) as u64;
            rem = (dividend % word as u128) as u64;
        }
        (
            Self {
                limbs: out,
            },
            rem,
        )
    }

    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, Error> {
        if !(2..=36).contains(&radix) {
            return Err(Error::InvalidRadix);
        }
        if src.is_empty() {
            return Err(Error::EmptyString);
        }

        let mut out = Self::ZERO;
        for byte in src.bytes() {
            let digit = char_to_digit(byte).ok_or(Error::InvalidDigit)?;
            if digit >= radix {
                return Err(Error::InvalidDigit);
            }
            let (mul, high) = out.mul_word(radix as u64);
            if high != 0 {
                return Err(Error::Overflow);
            }
            let (next, carry) = mul.add_word(digit as u64);
            if carry != 0 {
                return Err(Error::Overflow);
            }
            out = next;
        }
        Ok(out)
    }

    #[cfg(feature = "alloc")]
    pub fn to_string_radix(&self, radix: u32) -> String {
        assert!((2..=36).contains(&radix), "invalid radix");
        if self.is_zero() {
            return String::from("0");
        }

        let mut value = *self;
        let mut digits = Vec::new();
        while !value.is_zero() {
            let (quotient, remainder) = value.div_rem_word(radix as u64);
            digits.push(digit_to_char(remainder, false));
            value = quotient;
        }
        digits.into_iter().rev().collect()
    }
}

#[cfg(feature = "alloc")]
impl<const BITS: usize, const LIMBS: usize> fmt::Display for Uint<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_radix(10))
    }
}

impl<const BITS: usize, const LIMBS: usize> fmt::LowerHex for Uint<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }
        let mut started = false;
        let mut i = LIMBS;
        while i > 0 {
            i -= 1;
            let limb = self.limbs[i];
            if started {
                write!(f, "{limb:016x}")?;
            } else if limb != 0 {
                write!(f, "{limb:x}")?;
                started = true;
            }
        }
        if !started {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl<const BITS: usize, const LIMBS: usize> fmt::UpperHex for Uint<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }
        let mut started = false;
        let mut i = LIMBS;
        while i > 0 {
            i -= 1;
            let limb = self.limbs[i];
            if started {
                write!(f, "{limb:016X}")?;
            } else if limb != 0 {
                write!(f, "{limb:X}")?;
                started = true;
            }
        }
        if !started {
            f.write_str("0")?;
        }
        Ok(())
    }
}

impl<const BITS: usize, const LIMBS: usize> fmt::Debug for Uint<BITS, LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Uint(0x{self:x})")
    }
}

impl<const BITS: usize, const LIMBS: usize> Int<BITS, LIMBS> {
    const _LIMBS_CHECK: () = assert!(LIMBS == (BITS + 63) / 64, "LIMBS must equal ceil(BITS/64)");

    pub const ZERO: Self = Self(Uint::ZERO);
    pub const ONE: Self = Self(Uint::ONE);
    pub const MINUS_ONE: Self = Self(Uint::MAX);

    #[inline]
    const fn from_uint_bits(bits: Uint<BITS, LIMBS>) -> Self {
        Self(bits)
    }

    #[inline]
    fn from_u128(value: u128) -> Self {
        Self(uint_from_u128(value))
    }

    #[inline]
    fn from_i128(value: i128) -> Self {
        if value.is_negative() {
            let magnitude = uint_from_u128::<BITS, LIMBS>(value.unsigned_abs());
            Self(Self::ZERO.0.sub_raw(&magnitude).0)
        } else {
            Self(uint_from_u128(value as u128))
        }
    }

    #[inline]
    pub fn is_negative(&self) -> bool {
        (self.0.limbs[LIMBS - 1] >> 63) == 1
    }

    #[inline]
    pub fn abs(&self) -> Uint<BITS, LIMBS> {
        if self.is_negative() {
            Self::ZERO.0.sub_raw(&self.0).0
        } else {
            self.0
        }
    }
}

impl<const BITS: usize, const LIMBS: usize> Add for Uint<BITS, LIMBS> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_raw(&rhs).0
    }
}

impl<const BITS: usize, const LIMBS: usize> Sub for Uint<BITS, LIMBS> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_raw(&rhs).0
    }
}

impl<const BITS: usize, const LIMBS: usize> Mul for Uint<BITS, LIMBS> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let product = self.mul_wide_internal(&rhs);
        let mut limbs = [0u64; LIMBS];
        let mut i = 0;
        while i < LIMBS {
            limbs[i] = product[i];
            i += 1;
        }
        Self {
            limbs,
        }
    }
}

impl<const BITS: usize, const LIMBS: usize> Add for Int<BITS, LIMBS> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_uint_bits(self.0.add_raw(&rhs.0).0)
    }
}

impl<const BITS: usize, const LIMBS: usize> Sub for Int<BITS, LIMBS> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_uint_bits(self.0.sub_raw(&rhs.0).0)
    }
}

impl<const BITS: usize, const LIMBS: usize> Neg for Int<BITS, LIMBS> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_uint_bits(Int::ZERO.0.sub_raw(&self.0).0)
    }
}

macro_rules! impl_uint_ops_unsigned {
    ($($ty:ty),* $(,)?) => {
        $(
            impl<const BITS: usize, const LIMBS: usize> Add<$ty> for Uint<BITS, LIMBS> {
                type Output = Self;
                fn add(self, rhs: $ty) -> Self::Output {
                    self.add_word(u128_to_word(rhs as u128)).0
                }
            }

            impl<const BITS: usize, const LIMBS: usize> Sub<$ty> for Uint<BITS, LIMBS> {
                type Output = Self;
                fn sub(self, rhs: $ty) -> Self::Output {
                    self.sub_word(u128_to_word(rhs as u128)).0
                }
            }

            impl<const BITS: usize, const LIMBS: usize> Mul<$ty> for Uint<BITS, LIMBS> {
                type Output = Self;
                fn mul(self, rhs: $ty) -> Self::Output {
                    self.mul_word(u128_to_word(rhs as u128)).0
                }
            }

            impl<const BITS: usize, const LIMBS: usize> Div<$ty> for Uint<BITS, LIMBS> {
                type Output = Self;
                fn div(self, rhs: $ty) -> Self::Output {
                    self.div_rem_word(u128_to_word(rhs as u128)).0
                }
            }

            impl<const BITS: usize, const LIMBS: usize> Rem<$ty> for Uint<BITS, LIMBS> {
                type Output = u64;
                fn rem(self, rhs: $ty) -> Self::Output {
                    self.div_rem_word(u128_to_word(rhs as u128)).1
                }
            }
        )*
    };
}

macro_rules! impl_uint_ops_signed {
    ($($ty:ty),* $(,)?) => {
        $(
            impl<const BITS: usize, const LIMBS: usize> Add<$ty> for Uint<BITS, LIMBS> {
                type Output = Self;
                fn add(self, rhs: $ty) -> Self::Output {
                    let (word, negative) = i128_abs_to_word(rhs as i128);
                    if negative {
                        self.sub_word(word).0
                    } else {
                        self.add_word(word).0
                    }
                }
            }

            impl<const BITS: usize, const LIMBS: usize> Sub<$ty> for Uint<BITS, LIMBS> {
                type Output = Self;
                fn sub(self, rhs: $ty) -> Self::Output {
                    let (word, negative) = i128_abs_to_word(rhs as i128);
                    if negative {
                        self.add_word(word).0
                    } else {
                        self.sub_word(word).0
                    }
                }
            }

            impl<const BITS: usize, const LIMBS: usize> Mul<$ty> for Uint<BITS, LIMBS> {
                type Output = Self;
                fn mul(self, rhs: $ty) -> Self::Output {
                    self.mul_word(i128_abs_to_word(rhs as i128).0).0
                }
            }

            impl<const BITS: usize, const LIMBS: usize> Div<$ty> for Uint<BITS, LIMBS> {
                type Output = Self;
                fn div(self, rhs: $ty) -> Self::Output {
                    self.div_rem_word(i128_abs_to_word(rhs as i128).0).0
                }
            }

            impl<const BITS: usize, const LIMBS: usize> Rem<$ty> for Uint<BITS, LIMBS> {
                type Output = u64;
                fn rem(self, rhs: $ty) -> Self::Output {
                    self.div_rem_word(i128_abs_to_word(rhs as i128).0).1
                }
            }
        )*
    };
}

macro_rules! impl_int_ops_unsigned {
    ($($ty:ty),* $(,)?) => {
        $(
            impl<const BITS: usize, const LIMBS: usize> Add<$ty> for Int<BITS, LIMBS> {
                type Output = Self;
                fn add(self, rhs: $ty) -> Self::Output {
                    self + Self::from_u128(rhs as u128)
                }
            }

            impl<const BITS: usize, const LIMBS: usize> Sub<$ty> for Int<BITS, LIMBS> {
                type Output = Self;
                fn sub(self, rhs: $ty) -> Self::Output {
                    self - Self::from_u128(rhs as u128)
                }
            }

            impl<const BITS: usize, const LIMBS: usize> Mul<$ty> for Int<BITS, LIMBS> {
                type Output = Self;
                fn mul(self, rhs: $ty) -> Self::Output {
                    let (product, _) = self.abs().mul_word(u128_to_word(rhs as u128));
                    let bits = if self.is_negative() {
                        Uint::ZERO.sub_raw(&product).0
                    } else {
                        product
                    };
                    Self::from_uint_bits(bits)
                }
            }

            impl<const BITS: usize, const LIMBS: usize> Div<$ty> for Int<BITS, LIMBS> {
                type Output = Self;
                fn div(self, rhs: $ty) -> Self::Output {
                    let (quotient, _) = self.abs().div_rem_word(u128_to_word(rhs as u128));
                    let bits = if self.is_negative() {
                        Uint::ZERO.sub_raw(&quotient).0
                    } else {
                        quotient
                    };
                    Self::from_uint_bits(bits)
                }
            }

            impl<const BITS: usize, const LIMBS: usize> Rem<$ty> for Int<BITS, LIMBS> {
                type Output = Self;
                fn rem(self, rhs: $ty) -> Self::Output {
                    let (_, remainder) = self.abs().div_rem_word(u128_to_word(rhs as u128));
                    let bits = Uint::from_u64(remainder);
                    let bits = if self.is_negative() {
                        Uint::ZERO.sub_raw(&bits).0
                    } else {
                        bits
                    };
                    Self::from_uint_bits(bits)
                }
            }
        )*
    };
}

macro_rules! impl_int_ops_signed {
    ($($ty:ty),* $(,)?) => {
        $(
            impl<const BITS: usize, const LIMBS: usize> Add<$ty> for Int<BITS, LIMBS> {
                type Output = Self;
                fn add(self, rhs: $ty) -> Self::Output {
                    self + Self::from_i128(rhs as i128)
                }
            }

            impl<const BITS: usize, const LIMBS: usize> Sub<$ty> for Int<BITS, LIMBS> {
                type Output = Self;
                fn sub(self, rhs: $ty) -> Self::Output {
                    self - Self::from_i128(rhs as i128)
                }
            }

            impl<const BITS: usize, const LIMBS: usize> Mul<$ty> for Int<BITS, LIMBS> {
                type Output = Self;
                fn mul(self, rhs: $ty) -> Self::Output {
                    let (word, negative) = i128_abs_to_word(rhs as i128);
                    let (product, _) = self.abs().mul_word(word);
                    let make_negative = self.is_negative() ^ negative;
                    let bits = if make_negative {
                        Uint::ZERO.sub_raw(&product).0
                    } else {
                        product
                    };
                    Self::from_uint_bits(bits)
                }
            }

            impl<const BITS: usize, const LIMBS: usize> Div<$ty> for Int<BITS, LIMBS> {
                type Output = Self;
                fn div(self, rhs: $ty) -> Self::Output {
                    let (word, negative) = i128_abs_to_word(rhs as i128);
                    let (quotient, _) = self.abs().div_rem_word(word);
                    let make_negative = self.is_negative() ^ negative;
                    let bits = if make_negative {
                        Uint::ZERO.sub_raw(&quotient).0
                    } else {
                        quotient
                    };
                    Self::from_uint_bits(bits)
                }
            }

            impl<const BITS: usize, const LIMBS: usize> Rem<$ty> for Int<BITS, LIMBS> {
                type Output = Self;
                fn rem(self, rhs: $ty) -> Self::Output {
                    let (word, _) = i128_abs_to_word(rhs as i128);
                    let (_, remainder) = self.abs().div_rem_word(word);
                    let bits = Uint::from_u64(remainder);
                    let bits = if self.is_negative() {
                        Uint::ZERO.sub_raw(&bits).0
                    } else {
                        bits
                    };
                    Self::from_uint_bits(bits)
                }
            }
        )*
    };
}

impl_uint_ops_unsigned!(u8, u16, u32, u64, u128);
impl_uint_ops_signed!(i8, i16, i32, i64, i128);
impl_int_ops_unsigned!(u8, u16, u32, u64, u128);
impl_int_ops_signed!(i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    use super::*;
    type U256 = Uint<256, 4>;
    type U128 = Uint<128, 2>;
    type I128 = Int<128, 2>;

    const P256_MODULUS: U256 = U256::from_limbs([
        0xffff_ffff_ffff_ffff,
        0x0000_0000_ffff_ffff,
        0x0000_0000_0000_0000,
        0xffff_ffff_0000_0001,
    ]);
    const P256_ORDER: U256 = U256::from_limbs([
        0xf3b9_cac2_fc63_2551,
        0xbce6_faad_a717_9e84,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_0000_0000,
    ]);
    const P256_P_MINUS_TWO: U256 = U256::from_limbs([
        0xffff_ffff_ffff_fffd,
        0x0000_0000_ffff_ffff,
        0x0000_0000_0000_0000,
        0xffff_ffff_0000_0001,
    ]);
    const P256_P_PLUS_ONE_OVER_FOUR: U256 = U256::from_limbs([
        0x0000_0000_0000_0000,
        0x0000_0000_4000_0000,
        0x4000_0000_0000_0000,
        0x3fff_ffff_c000_0000,
    ]);
    const ED25519_P: U256 = U256::from_limbs([
        0xffff_ffff_ffff_ffed,
        0xffff_ffff_ffff_ffff,
        0xffff_ffff_ffff_ffff,
        0x7fff_ffff_ffff_ffff,
    ]);

    fn decode_hex<const N: usize>(input: &str) -> [u8; N] {
        assert_eq!(input.len(), N * 2);
        let mut out = [0u8; N];
        let bytes = input.as_bytes();
        let mut i = 0;
        while i < N {
            let hi = char_to_digit(bytes[i * 2]).unwrap() as u8;
            let lo = char_to_digit(bytes[i * 2 + 1]).unwrap() as u8;
            out[i] = (hi << 4) | lo;
            i += 1;
        }
        out
    }

    #[test]
    fn raw_arithmetic_reports_carry_and_borrow() {
        let (sum, carry) = U128::MAX.add_raw(&U128::ONE);
        assert_eq!(sum, U128::ZERO);
        assert_eq!(carry, 1);

        let (diff, borrow) = U128::ZERO.sub_raw(&U128::ONE);
        assert_eq!(diff, U128::MAX);
        assert_eq!(borrow, 1);

        let (low, high) = U128::MAX.mul_word(2);
        assert_eq!(low, U128::from_limbs([0xffff_ffff_ffff_fffe, 0xffff_ffff_ffff_ffff]));
        assert_eq!(high, 1);
    }

    #[test]
    fn modular_arithmetic_matches_known_values() {
        assert_eq!(P256_P_MINUS_TWO.add_mod(&U256::from_u64(2), &P256_MODULUS), U256::ZERO);
        assert_eq!(
            P256_MODULUS.sub_mod(&U256::from_u64(1), &P256_MODULUS),
            P256_MODULUS - U256::ONE
        );
        assert_eq!(P256_P_MINUS_TWO.double_mod(&P256_MODULUS), P256_MODULUS - U256::from_u64(4));
        assert_eq!(P256_P_PLUS_ONE_OVER_FOUR.mul_mod(&U256::from_u64(4), &P256_MODULUS), U256::ONE);
        assert_eq!(P256_ORDER.add_mod(&U256::ONE, &P256_ORDER), U256::ONE);
    }

    #[test]
    fn string_round_trips_for_common_radices() {
        let values = [
            U256::ZERO,
            U256::ONE,
            U256::MAX,
            U256::from_be_slice(&decode_hex::<32>(
                "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296",
            )),
        ];

        for value in values {
            for radix in [2, 10, 16] {
                let encoded = value.to_string_radix(radix);
                let decoded = U256::from_str_radix(&encoded, radix).unwrap();
                assert_eq!(decoded, value);
            }
        }
    }

    #[test]
    fn operator_overloads_work_with_primitives() {
        let value = U128::from_u64(10);
        assert_eq!(value + 5u64, U128::from_u64(15));
        assert_eq!(value - (-5i32), U128::from_u64(15));
        assert_eq!(value * -3i32, U128::from_u64(30));
        assert_eq!(value / -3i32, U128::from_u64(3));
        assert_eq!(value % -3i32, 1);

        let signed = I128::from_i128(-10);
        assert_eq!(signed + 3u32, I128::from_i128(-7));
        assert_eq!(signed - (-5i32), I128::from_i128(-5));
        assert_eq!(signed * -2i32, I128::from_i128(20));
        assert_eq!(signed / -4i32, I128::from_i128(2));
        assert_eq!(signed % 4u32, I128::from_i128(-2));
    }

    #[test]
    fn constant_time_helpers_select_and_compare() {
        let a = U256::from_u64(7);
        let b = U256::from_u64(11);
        assert_eq!(U256::ct_select(&a, &b, true), a);
        assert_eq!(U256::ct_select(&a, &b, false), b);
        assert!(a.ct_eq(&a));
        assert!(!a.ct_eq(&b));
        assert!(b.ct_ge(&a));
        assert!(!a.ct_ge(&b));
    }

    #[test]
    fn from_str_radix_rejects_invalid_inputs() {
        assert_eq!(U128::from_str_radix("", 10), Err(Error::EmptyString));
        assert_eq!(U128::from_str_radix("10", 1), Err(Error::InvalidRadix));
        assert_eq!(U128::from_str_radix("2", 2), Err(Error::InvalidDigit));
        assert_eq!(U128::from_str_radix("zz", 10), Err(Error::InvalidDigit));
        assert_eq!(
            U128::from_str_radix("340282366920938463463374607431768211456", 10),
            Err(Error::Overflow)
        );
    }

    #[test]
    fn byte_round_trips_match_p256_constants() {
        let modulus_hex = "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff";
        let order_hex = "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551";

        let modulus_bytes = decode_hex::<32>(modulus_hex);
        let order_bytes = decode_hex::<32>(order_hex);

        let modulus = U256::from_be_slice(&modulus_bytes);
        let order = U256::from_be_slice(&order_bytes);

        assert_eq!(modulus, P256_MODULUS);
        assert_eq!(order, P256_ORDER);
        assert_eq!(modulus.to_be_bytes_fixed::<32>(), modulus_bytes);
        assert_eq!(order.to_be_bytes_fixed::<32>(), order_bytes);
        assert_eq!(format!("{modulus:x}"), modulus_hex);
        assert_eq!(format!("{order:X}"), order_hex.to_uppercase());
    }

    #[test]
    fn little_endian_round_trip_works() {
        let value = U256::from_be_slice(&decode_hex::<32>(
            "4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5",
        ));
        let le = value.to_le_bytes_fixed::<32>();
        assert_eq!(U256::from_le_slice(&le), value);
    }

    #[test]
    fn modular_addition_vectors() {
        struct Test {
            a: U256,
            b: U256,
            m: U256,
            expected: U256,
        }
        let tests = [
            Test {
                a: U256::ZERO,
                b: U256::ZERO,
                m: P256_MODULUS,
                expected: U256::ZERO,
            },
            Test {
                a: U256::ZERO,
                b: U256::ONE,
                m: P256_MODULUS,
                expected: U256::ONE,
            },
            Test {
                a: U256::ONE,
                b: U256::ONE,
                m: P256_MODULUS,
                expected: U256::from_u64(2),
            },
            Test {
                a: P256_MODULUS - U256::ONE,
                b: U256::ONE,
                m: P256_MODULUS,
                expected: U256::ZERO,
            },
            Test {
                a: P256_MODULUS - U256::ONE,
                b: U256::from_u64(2),
                m: P256_MODULUS,
                expected: U256::ONE,
            },
            Test {
                a: U256::ZERO,
                b: U256::ZERO,
                m: P256_ORDER,
                expected: U256::ZERO,
            },
            Test {
                a: U256::ZERO,
                b: U256::ONE,
                m: P256_ORDER,
                expected: U256::ONE,
            },
            Test {
                a: U256::ONE,
                b: U256::ONE,
                m: P256_ORDER,
                expected: U256::from_u64(2),
            },
            Test {
                a: U256::MAX,
                b: U256::ONE,
                m: P256_ORDER,
                expected: U256::from_limbs([
                    0x0c46_353d_039c_daaf,
                    0x4319_0552_58e8_617b,
                    0x0000_0000_0000_0000,
                    0x0000_0000_ffff_ffff,
                ]),
            },
            Test {
                a: P256_ORDER - U256::ONE,
                b: U256::ONE,
                m: P256_ORDER,
                expected: U256::ZERO,
            },
            Test {
                a: P256_ORDER - U256::ONE,
                b: U256::from_u64(2),
                m: P256_ORDER,
                expected: U256::ONE,
            },
            Test {
                a: U256::ZERO,
                b: U256::ZERO,
                m: ED25519_P,
                expected: U256::ZERO,
            },
            Test {
                a: U256::ZERO,
                b: U256::ONE,
                m: ED25519_P,
                expected: U256::ONE,
            },
            Test {
                a: U256::ONE,
                b: U256::ONE,
                m: ED25519_P,
                expected: U256::from_u64(2),
            },
            Test {
                a: U256::from_limbs([
                    0xffff_ffff_ffff_ffff,
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0000,
                ]),
                b: U256::ONE,
                m: P256_MODULUS,
                expected: U256::from_limbs([
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0001,
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0000,
                ]),
            },
            Test {
                a: U256::from_limbs([
                    0xffff_ffff_ffff_ffff,
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0000,
                ]),
                b: U256::ONE,
                m: P256_ORDER,
                expected: U256::from_limbs([
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0001,
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0000,
                ]),
            },
            Test {
                a: U256::from_limbs([
                    0xffff_ffff_ffff_ffff,
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0000,
                ]),
                b: U256::ONE,
                m: ED25519_P,
                expected: U256::from_limbs([
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0001,
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0000,
                ]),
            },
            Test {
                a: ED25519_P - U256::ONE,
                b: ED25519_P - U256::ONE,
                m: ED25519_P,
                expected: (ED25519_P - U256::ONE).double_mod(&ED25519_P),
            },
        ];
        for t in &tests {
            assert_eq!(t.a.add_mod(&t.b, &t.m), t.expected, "add_mod({:x}, {:x}, {:x})", t.a, t.b, t.m);
        }
    }

    #[test]
    fn modular_subtraction_vectors() {
        struct Test {
            a: U256,
            b: U256,
            m: U256,
            expected: U256,
        }
        let tests = [
            Test {
                a: U256::ZERO,
                b: U256::ZERO,
                m: P256_MODULUS,
                expected: U256::ZERO,
            },
            Test {
                a: U256::ONE,
                b: U256::ZERO,
                m: P256_MODULUS,
                expected: U256::ONE,
            },
            Test {
                a: U256::ZERO,
                b: U256::ONE,
                m: P256_MODULUS,
                expected: P256_MODULUS - U256::ONE,
            },
            Test {
                a: P256_MODULUS - U256::ONE,
                b: U256::ONE,
                m: P256_MODULUS,
                expected: P256_MODULUS - U256::from_u64(2),
            },
            Test {
                a: U256::ONE,
                b: P256_MODULUS - U256::ONE,
                m: P256_MODULUS,
                expected: U256::from_u64(2),
            },
            Test {
                a: U256::ZERO,
                b: U256::ZERO,
                m: P256_ORDER,
                expected: U256::ZERO,
            },
            Test {
                a: U256::ONE,
                b: U256::ZERO,
                m: P256_ORDER,
                expected: U256::ONE,
            },
            Test {
                a: U256::ZERO,
                b: U256::ONE,
                m: P256_ORDER,
                expected: P256_ORDER - U256::ONE,
            },
            Test {
                a: P256_ORDER - U256::ONE,
                b: U256::ONE,
                m: P256_ORDER,
                expected: P256_ORDER - U256::from_u64(2),
            },
            Test {
                a: U256::ONE,
                b: P256_ORDER - U256::ONE,
                m: P256_ORDER,
                expected: U256::from_u64(2),
            },
            Test {
                a: U256::ZERO,
                b: U256::ZERO,
                m: ED25519_P,
                expected: U256::ZERO,
            },
            Test {
                a: U256::ONE,
                b: U256::ZERO,
                m: ED25519_P,
                expected: U256::ONE,
            },
            Test {
                a: U256::ZERO,
                b: U256::ONE,
                m: ED25519_P,
                expected: ED25519_P - U256::ONE,
            },
            Test {
                a: ED25519_P - U256::ONE,
                b: U256::ONE,
                m: ED25519_P,
                expected: ED25519_P - U256::from_u64(2),
            },
            Test {
                a: U256::ONE,
                b: ED25519_P - U256::ONE,
                m: ED25519_P,
                expected: U256::from_u64(2),
            },
        ];
        for t in &tests {
            assert_eq!(t.a.sub_mod(&t.b, &t.m), t.expected, "sub_mod({:x}, {:x}, {:x})", t.a, t.b, t.m);
        }
    }

    #[test]
    fn modular_multiplication_vectors() {
        struct Test {
            a: U256,
            b: U256,
            m: U256,
            expected: U256,
        }
        let tests = [
            Test {
                a: U256::ZERO,
                b: U256::ZERO,
                m: P256_MODULUS,
                expected: U256::ZERO,
            },
            Test {
                a: U256::ZERO,
                b: U256::ONE,
                m: P256_MODULUS,
                expected: U256::ZERO,
            },
            Test {
                a: U256::ONE,
                b: U256::ZERO,
                m: P256_MODULUS,
                expected: U256::ZERO,
            },
            Test {
                a: U256::ONE,
                b: U256::ONE,
                m: P256_MODULUS,
                expected: U256::ONE,
            },
            Test {
                a: U256::from_u64(2),
                b: U256::from_u64(3),
                m: P256_MODULUS,
                expected: U256::from_u64(6),
            },
            Test {
                a: U256::MAX,
                b: U256::ONE,
                m: P256_MODULUS,
                expected: U256::from_limbs([
                    0x0000_0000_0000_0000,
                    0xffff_ffff_0000_0000,
                    0xffff_ffff_ffff_ffff,
                    0x0000_0000_ffff_fffe,
                ]),
            },
            Test {
                a: P256_MODULUS - U256::ONE,
                b: U256::ONE,
                m: P256_MODULUS,
                expected: P256_MODULUS - U256::ONE,
            },
            Test {
                a: P256_MODULUS - U256::ONE,
                b: P256_MODULUS - U256::ONE,
                m: P256_MODULUS,
                expected: U256::ONE,
            },
            Test {
                a: P256_MODULUS + U256::ONE,
                b: P256_MODULUS + U256::ONE,
                m: P256_MODULUS,
                expected: U256::ONE,
            },
            Test {
                a: U256::ZERO,
                b: U256::ZERO,
                m: P256_ORDER,
                expected: U256::ZERO,
            },
            Test {
                a: U256::ZERO,
                b: U256::ONE,
                m: P256_ORDER,
                expected: U256::ZERO,
            },
            Test {
                a: U256::ONE,
                b: U256::ZERO,
                m: P256_ORDER,
                expected: U256::ZERO,
            },
            Test {
                a: U256::ONE,
                b: U256::ONE,
                m: P256_ORDER,
                expected: U256::ONE,
            },
            Test {
                a: U256::from_u64(2),
                b: U256::from_u64(3),
                m: P256_ORDER,
                expected: U256::from_u64(6),
            },
            Test {
                a: P256_ORDER - U256::ONE,
                b: U256::ONE,
                m: P256_ORDER,
                expected: P256_ORDER - U256::ONE,
            },
            Test {
                a: P256_ORDER - U256::ONE,
                b: P256_ORDER - U256::ONE,
                m: P256_ORDER,
                expected: U256::ONE,
            },
            Test {
                a: P256_ORDER + U256::ONE,
                b: P256_ORDER + U256::ONE,
                m: P256_ORDER,
                expected: U256::ONE,
            },
            Test {
                a: U256::ZERO,
                b: U256::ZERO,
                m: ED25519_P,
                expected: U256::ZERO,
            },
            Test {
                a: U256::ZERO,
                b: U256::ONE,
                m: ED25519_P,
                expected: U256::ZERO,
            },
            Test {
                a: U256::ONE,
                b: U256::ZERO,
                m: ED25519_P,
                expected: U256::ZERO,
            },
            Test {
                a: U256::ONE,
                b: U256::ONE,
                m: ED25519_P,
                expected: U256::ONE,
            },
            Test {
                a: U256::from_u64(2),
                b: U256::from_u64(3),
                m: ED25519_P,
                expected: U256::from_u64(6),
            },
            Test {
                a: ED25519_P - U256::ONE,
                b: U256::ONE,
                m: ED25519_P,
                expected: ED25519_P - U256::ONE,
            },
            Test {
                a: ED25519_P - U256::ONE,
                b: ED25519_P - U256::ONE,
                m: ED25519_P,
                expected: U256::ONE,
            },
            Test {
                a: ED25519_P + U256::ONE,
                b: ED25519_P + U256::ONE,
                m: ED25519_P,
                expected: U256::ONE,
            },
            Test {
                a: U256::from_limbs([
                    0xffff_ffff_ffff_ffff,
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0000,
                ]),
                b: U256::from_limbs([
                    0xffff_ffff_ffff_ffff,
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0000,
                ]),
                m: P256_MODULUS,
                expected: U256::from_limbs([
                    0x0000_0000_0000_0001,
                    0xffff_ffff_ffff_fffe,
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0000,
                ]),
            },
        ];
        for t in &tests {
            assert_eq!(t.a.mul_mod(&t.b, &t.m), t.expected, "mul_mod({:x}, {:x}, {:x})", t.a, t.b, t.m);
        }
    }

    #[test]
    fn raw_addition_vectors() {
        struct Test {
            a: U256,
            b: U256,
            expected_sum: U256,
            expected_carry: u64,
        }
        let tests = [
            Test {
                a: U256::ZERO,
                b: U256::ZERO,
                expected_sum: U256::ZERO,
                expected_carry: 0,
            },
            Test {
                a: U256::ZERO,
                b: U256::ONE,
                expected_sum: U256::ONE,
                expected_carry: 0,
            },
            Test {
                a: U256::ONE,
                b: U256::ONE,
                expected_sum: U256::from_u64(2),
                expected_carry: 0,
            },
            Test {
                a: U256::MAX,
                b: U256::ZERO,
                expected_sum: U256::MAX,
                expected_carry: 0,
            },
            Test {
                a: U256::MAX,
                b: U256::ONE,
                expected_sum: U256::ZERO,
                expected_carry: 1,
            },
            Test {
                a: U256::MAX,
                b: U256::MAX,
                expected_sum: U256::MAX - U256::ONE,
                expected_carry: 1,
            },
            Test {
                a: U256::MAX - U256::ONE,
                b: U256::ONE,
                expected_sum: U256::MAX,
                expected_carry: 0,
            },
            Test {
                a: U256::ONE,
                b: U256::MAX,
                expected_sum: U256::ZERO,
                expected_carry: 1,
            },
            Test {
                a: U256::from_limbs([
                    0xffff_ffff_ffff_ffff,
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0000,
                ]),
                b: U256::ONE,
                expected_sum: U256::from_limbs([
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0001,
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0000,
                ]),
                expected_carry: 0,
            },
            Test {
                a: U256::from_limbs([
                    0xffff_ffff_ffff_ffff,
                    0xffff_ffff_ffff_ffff,
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0000,
                ]),
                b: U256::ONE,
                expected_sum: U256::from_limbs([
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0000,
                    0x0000_0000_0000_0001,
                    0x0000_0000_0000_0000,
                ]),
                expected_carry: 0,
            },
        ];
        for t in &tests {
            let (sum, carry) = t.a.add_raw(&t.b);
            assert_eq!(sum, t.expected_sum, "add_raw({:x}, {:x}).sum", t.a, t.b);
            assert_eq!(carry, t.expected_carry, "add_raw({:x}, {:x}).carry", t.a, t.b);
        }
    }

    #[test]
    fn raw_subtraction_vectors() {
        struct Test {
            a: U256,
            b: U256,
            expected_diff: U256,
            expected_borrow: u64,
        }
        let tests = [
            Test {
                a: U256::ZERO,
                b: U256::ZERO,
                expected_diff: U256::ZERO,
                expected_borrow: 0,
            },
            Test {
                a: U256::ONE,
                b: U256::ZERO,
                expected_diff: U256::ONE,
                expected_borrow: 0,
            },
            Test {
                a: U256::ONE,
                b: U256::ONE,
                expected_diff: U256::ZERO,
                expected_borrow: 0,
            },
            Test {
                a: U256::ZERO,
                b: U256::ONE,
                expected_diff: U256::MAX,
                expected_borrow: 1,
            },
            Test {
                a: U256::MAX,
                b: U256::MAX,
                expected_diff: U256::ZERO,
                expected_borrow: 0,
            },
            Test {
                a: U256::MAX,
                b: U256::ZERO,
                expected_diff: U256::MAX,
                expected_borrow: 0,
            },
            Test {
                a: U256::MAX,
                b: U256::MAX - U256::ONE,
                expected_diff: U256::ONE,
                expected_borrow: 0,
            },
            Test {
                a: U256::ONE,
                b: U256::MAX,
                expected_diff: U256::from_u64(2),
                expected_borrow: 1,
            },
        ];
        for t in &tests {
            let (diff, borrow) = t.a.sub_raw(&t.b);
            assert_eq!(diff, t.expected_diff, "sub_raw({:x}, {:x}).diff", t.a, t.b);
            assert_eq!(borrow, t.expected_borrow, "sub_raw({:x}, {:x}).borrow", t.a, t.b);
        }
    }

    #[test]
    fn string_roundtrip_vectors() {
        struct Test {
            value: U256,
            radix: u32,
            expected: &'static str,
        }
        let tests = [
            Test {
                value: U256::ZERO,
                radix: 2,
                expected: "0",
            },
            Test {
                value: U256::ZERO,
                radix: 10,
                expected: "0",
            },
            Test {
                value: U256::ZERO,
                radix: 16,
                expected: "0",
            },
            Test {
                value: U256::ONE,
                radix: 2,
                expected: "1",
            },
            Test {
                value: U256::ONE,
                radix: 10,
                expected: "1",
            },
            Test {
                value: U256::ONE,
                radix: 16,
                expected: "1",
            },
            Test {
                value: U256::MAX,
                radix: 10,
                expected: "115792089237316195423570985008687907853269984665640564039457584007913129639935",
            },
            Test {
                value: U256::MAX,
                radix: 16,
                expected: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            },
            Test {
                value: P256_MODULUS,
                radix: 10,
                expected: "115792089210356248762697446949407573530086143415290314195533631308867097853951",
            },
            Test {
                value: P256_MODULUS,
                radix: 16,
                expected: "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff",
            },
            Test {
                value: P256_ORDER,
                radix: 10,
                expected: "115792089210356248762697446949407573529996955224135760342422259061068512044369",
            },
            Test {
                value: P256_ORDER,
                radix: 16,
                expected: "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551",
            },
            Test {
                value: ED25519_P,
                radix: 10,
                expected: "57896044618658097711785492504343953926634992332820282019728792003956564819949",
            },
            Test {
                value: ED25519_P,
                radix: 16,
                expected: "7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed",
            },
        ];
        for t in &tests {
            let encoded = t.value.to_string_radix(t.radix);
            assert_eq!(encoded, t.expected, "to_string_radix({:x}, {})", t.value, t.radix);
            let decoded = U256::from_str_radix(t.expected, t.radix).unwrap();
            assert_eq!(decoded, t.value, "from_str_radix round-trip");
        }
    }

    #[test]
    fn mul_mod_edge_cases() {
        assert_eq!(P256_P_PLUS_ONE_OVER_FOUR.mul_mod(&U256::from_u64(4), &P256_MODULUS), U256::ONE);

        let (a, _) = U256::MAX.mul_word(2);
        assert_eq!(a.add_mod(&U256::ZERO, &P256_MODULUS), U256::MAX.double_mod(&P256_MODULUS));
    }

    // P-256 generator point coordinates (NIST SP 800-186 / RFC 6979).
    // Test vectors generated with Python: `hex((Gx * Gy) % p256_p)` etc.
    const P256_GX: U256 = U256::from_be_slice_const(
        0x6b17d1f2, 0xe12c4247, 0xf8bce6e5, 0x63a440f2, 0x77037d81, 0x2deb33a0, 0xf4a13945, 0xd898c296,
    );
    const P256_GY: U256 = U256::from_be_slice_const(
        0x4fe342e2, 0xfe1a7f9b, 0x8ee7eb4a, 0x7c0f9e16, 0x2bce3357, 0x6b315ece, 0xcbb64068, 0x37bf51f5,
    );

    impl<const BITS: usize, const LIMBS: usize> Uint<BITS, LIMBS> {
        // Helper to build a 256-bit constant from eight 32-bit big-endian words.
        // Only valid for BITS=256, LIMBS=4; used only in test helpers.
        const fn from_be_slice_const(w7: u32, w6: u32, w5: u32, w4: u32, w3: u32, w2: u32, w1: u32, w0: u32) -> Self {
            let limb3 = ((w7 as u64) << 32) | (w6 as u64);
            let limb2 = ((w5 as u64) << 32) | (w4 as u64);
            let limb1 = ((w3 as u64) << 32) | (w2 as u64);
            let limb0 = ((w1 as u64) << 32) | (w0 as u64);
            let limbs = [0u64; LIMBS];
            if LIMBS > 0 {
                let mut l = [0u64; LIMBS];
                l[0] = limb0;
                l[1] = limb1;
                l[2] = limb2;
                l[3] = limb3;
                return Self {
                    limbs: l,
                };
            }
            Self {
                limbs,
            }
        }
    }

    #[test]
    fn p256_mul_mod_with_generator_coordinates() {
        // Vectors verified with Python's arbitrary-precision arithmetic.
        let gx_gy_mod_p = U256::from_be_slice(&decode_hex::<32>(
            "823cd15f6dd3c71933565064513a6b2bd183e554c6a08622f713ebbbface98be",
        ));
        let gx_sq_mod_p = U256::from_be_slice(&decode_hex::<32>(
            "98f6b84d29bef2b281819a5e0e3690d833b699495d694dd1002ae56c426b3f8c",
        ));
        let gy_sq_mod_p = U256::from_be_slice(&decode_hex::<32>(
            "55df5d5850f47bad82149139979369fe498a9022a412b5e0bedd2cfc21c3ed91",
        ));

        assert_eq!(P256_GX.mul_mod(&P256_GY, &P256_MODULUS), gx_gy_mod_p, "Gx*Gy mod p");
        // Commutativity
        assert_eq!(
            P256_GY.mul_mod(&P256_GX, &P256_MODULUS),
            gx_gy_mod_p,
            "Gy*Gx mod p (commutativity)"
        );
        assert_eq!(P256_GX.mul_mod(&P256_GX, &P256_MODULUS), gx_sq_mod_p, "Gx^2 mod p");
        assert_eq!(P256_GY.mul_mod(&P256_GY, &P256_MODULUS), gy_sq_mod_p, "Gy^2 mod p");
    }

    #[test]
    fn p256_add_sub_mod_with_generator_coordinates() {
        // Gx + Gy mod p256_p  (Python: hex((Gx + Gy) % p))
        let gx_plus_gy = U256::from_be_slice(&decode_hex::<32>(
            "bafb14d5df46c1e387a4d22fdfb3df08a2d1b0d8991c926fc05779ae1058148b",
        ));
        // Gx - Gy mod p256_p  (Python: hex((Gx - Gy) % p))
        let gx_minus_gy = U256::from_be_slice(&decode_hex::<32>(
            "1b348f0fe311c2ac69d4fb9ae794a2dc4b354a29c2b9d4d228eaf8dda0d970a1",
        ));
        // Gy - Gx mod p256_p
        let gy_minus_gx = U256::from_be_slice(&decode_hex::<32>(
            "e4cb70ef1cee3d54962b0465186b5d23b4cab5d73d462b2dd71507225f268f5e",
        ));

        assert_eq!(P256_GX.add_mod(&P256_GY, &P256_MODULUS), gx_plus_gy);
        assert_eq!(P256_GX.sub_mod(&P256_GY, &P256_MODULUS), gx_minus_gy);
        assert_eq!(P256_GY.sub_mod(&P256_GX, &P256_MODULUS), gy_minus_gx);
        // (a-b) + (b-a) = 0 mod p
        assert_eq!(gx_minus_gy.add_mod(&gy_minus_gx, &P256_MODULUS), U256::ZERO);
    }

    #[test]
    fn bit_access() {
        // Gx = 0x6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296
        // Bit 0: the LSB of limb[0] = 0x...96 → bit 0 = 0
        assert!(!P256_GX.bit(0), "bit 0 of Gx");
        assert!(P256_GX.bit(1), "bit 1 of Gx");
        assert!(P256_GX.bit(2), "bit 2 of Gx");
        assert!(P256_GX.bit(63), "bit 63 of Gx");
        assert!(!P256_GX.bit(64), "bit 64 of Gx");
        assert!(!P256_GX.bit(65), "bit 65 of Gx");
        assert!(!P256_GX.bit(127), "bit 127 of Gx");
        assert!(!P256_GX.bit(128), "bit 128 of Gx");
        assert!(P256_GX.bit(192), "bit 192 of Gx");
        assert!(!P256_GX.bit(255), "bit 255 of Gx");
        // Out-of-range bit is false
        assert!(!P256_GX.bit(256), "bit 256 (out of range) of Gx");

        // p256_p: all-ones in low 64 bits → bit 0 is 1
        assert!(P256_MODULUS.bit(0), "bit 0 of p256_p");
        assert!(P256_MODULUS.bit(63), "bit 63 of p256_p");
        assert!(P256_MODULUS.bit(64), "bit 64 of p256_p");
        assert!(!P256_MODULUS.bit(96), "bit 96 of p256_p");
        assert!(P256_MODULUS.bit(255), "bit 255 of p256_p");

        assert!(!U256::ZERO.bit(0));
        assert!(!U256::ZERO.bit(255));
        assert!(U256::ONE.bit(0));
        assert!(!U256::ONE.bit(1));
        assert!(U256::MAX.bit(0));
        assert!(U256::MAX.bit(255));
    }

    #[test]
    fn is_odd_flag() {
        assert!(!U256::ZERO.is_odd(), "0 is even");
        assert!(U256::ONE.is_odd(), "1 is odd");
        assert!(!U256::from_u64(2).is_odd(), "2 is even");
        assert!(U256::from_u64(3).is_odd(), "3 is odd");
        assert!(P256_MODULUS.is_odd(), "p256_p is odd");
        assert!(P256_ORDER.is_odd(), "p256_n is odd");
        assert!(!P256_GX.is_odd(), "Gx is even");
        assert!(P256_GY.is_odd(), "Gy is odd");
        assert!(U256::MAX.is_odd(), "MAX is odd");
    }

    #[test]
    fn ct_ge_comprehensive() {
        let zero = U256::ZERO;
        let one = U256::ONE;
        let max = U256::MAX;

        // Equal values
        assert!(zero.ct_ge(&zero), "0 >= 0");
        assert!(one.ct_ge(&one), "1 >= 1");
        assert!(max.ct_ge(&max), "MAX >= MAX");
        assert!(P256_MODULUS.ct_ge(&P256_MODULUS), "p >= p");

        // Strict greater
        assert!(one.ct_ge(&zero), "1 >= 0");
        assert!(max.ct_ge(&zero), "MAX >= 0");
        assert!(max.ct_ge(&one), "MAX >= 1");
        assert!(P256_GX.ct_ge(&zero), "Gx >= 0");

        // Strict less
        assert!(!zero.ct_ge(&one), "NOT 0 >= 1");
        assert!(!zero.ct_ge(&max), "NOT 0 >= MAX");
        assert!(!one.ct_ge(&max), "NOT 1 >= MAX");
        assert!(!P256_GX.ct_ge(&max), "NOT Gx >= MAX");

        // Adjacent values
        assert!((P256_MODULUS - U256::ONE).ct_ge(&(P256_MODULUS - U256::from_u64(2))));
        assert!(!P256_MODULUS.sub_mod(&U256::ONE, &P256_MODULUS).ct_ge(&P256_MODULUS));
    }

    #[test]
    fn div_rem_word_vectors() {
        // Vectors generated with Python: p256_p / divisor
        // p256_p = 0xffffffff00000001000000000000000000000000ffffffffffffffffffffffff
        struct Test {
            dividend: U256,
            divisor: u64,
            expected_quotient: U256,
            expected_rem: u64,
        }
        let tests = [
            Test {
                dividend: P256_MODULUS,
                divisor: 1,
                expected_quotient: P256_MODULUS,
                expected_rem: 0,
            },
            Test {
                dividend: P256_MODULUS,
                divisor: 2,
                expected_quotient: U256::from_be_slice(&decode_hex::<32>(
                    "7fffffff800000008000000000000000000000007fffffffffffffffffffffff",
                )),
                expected_rem: 1,
            },
            Test {
                dividend: P256_MODULUS,
                divisor: 10,
                expected_quotient: U256::from_be_slice(&decode_hex::<32>(
                    "1999999980000000199999999999999999999999b33333333333333333333333",
                )),
                expected_rem: 1,
            },
            Test {
                dividend: P256_MODULUS,
                divisor: 16,
                expected_quotient: U256::from_be_slice(&decode_hex::<32>(
                    "0ffffffff00000001000000000000000000000000fffffffffffffffffffffff",
                )),
                expected_rem: 0xf,
            },
            Test {
                dividend: P256_MODULUS,
                divisor: 0xffff_ffff,
                expected_quotient: U256::from_be_slice(&decode_hex::<32>(
                    "0000000100000000000000010000000100000001000000020000000200000002",
                )),
                expected_rem: 1,
            },
            Test {
                dividend: P256_MODULUS,
                divisor: 0x1_0000_0000,
                expected_quotient: U256::from_be_slice(&decode_hex::<32>(
                    "00000000ffffffff00000001000000000000000000000000ffffffffffffffff",
                )),
                expected_rem: 0xffff_ffff,
            },
            Test {
                dividend: U256::ZERO,
                divisor: 7,
                expected_quotient: U256::ZERO,
                expected_rem: 0,
            },
            Test {
                dividend: U256::ONE,
                divisor: 7,
                expected_quotient: U256::ZERO,
                expected_rem: 1,
            },
            Test {
                dividend: U256::from_u64(100),
                divisor: 7,
                expected_quotient: U256::from_u64(14),
                expected_rem: 2,
            },
        ];
        for t in &tests {
            let (q, r) = t.dividend.div_rem_word(t.divisor);
            assert_eq!(q, t.expected_quotient, "div_rem_word({:x}, {}).quotient", t.dividend, t.divisor);
            assert_eq!(r, t.expected_rem, "div_rem_word({:x}, {}).rem", t.dividend, t.divisor);
        }
    }

    #[test]
    fn add_word_vectors() {
        // Vectors: Python hex(Gx + k) for small k
        let gx = P256_GX;
        let gx_plus_1 = U256::from_be_slice(&decode_hex::<32>(
            "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c297",
        ));
        let gx_minus_1 = U256::from_be_slice(&decode_hex::<32>(
            "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c295",
        ));

        let (sum, carry) = gx.add_word(1);
        assert_eq!(sum, gx_plus_1, "Gx + 1");
        assert_eq!(carry, 0);

        let (diff, borrow) = gx.sub_word(1);
        assert_eq!(diff, gx_minus_1, "Gx - 1");
        assert_eq!(borrow, 0);

        // Adding to MAX wraps
        let (sum_max, carry_max) = U256::MAX.add_word(1);
        assert_eq!(sum_max, U256::ZERO);
        assert_eq!(carry_max, 1);

        // Subtracting from ZERO borrows
        let (diff_zero, borrow_zero) = U256::ZERO.sub_word(1);
        assert_eq!(diff_zero, U256::MAX);
        assert_eq!(borrow_zero, 1);

        // Word that carries across limb boundary: 2^64 - 1 + 1
        let val = U256::from_u64(u64::MAX);
        let (sum2, carry2) = val.add_word(1);
        assert_eq!(sum2, U256::from_limbs([0, 1, 0, 0]));
        assert_eq!(carry2, 0);
    }

    #[test]
    fn mul_word_vectors() {
        // Gx * 2 = 2*Gx (no overflow since Gx < 2^255)
        // Python: hex(Gx * 2)
        let gx_times_2 = U256::from_be_slice(&decode_hex::<32>(
            "d62fa3e5c258848ff179cdcac74881e4ee06fb025bd66741e942728bb131852c",
        ));
        let (prod, carry) = P256_GX.mul_word(2);
        assert_eq!(prod, gx_times_2, "Gx * 2");
        assert_eq!(carry, 0, "Gx * 2 carry");

        // MAX * 2 = 2^256 - 2: low 256 bits = all-ones XOR 1, overflow carry = 1
        let (prod_max, carry_max) = U256::MAX.mul_word(2);
        assert_eq!(prod_max, U256::from_limbs([u64::MAX - 1, u64::MAX, u64::MAX, u64::MAX]));
        assert_eq!(carry_max, 1);

        // ONE * 0 = 0
        let (zero_prod, zero_carry) = U256::ONE.mul_word(0);
        assert_eq!(zero_prod, U256::ZERO);
        assert_eq!(zero_carry, 0);

        // from_u64(10) * from_u64(10) (single-word)
        let (p, c) = U128::from_u64(u64::MAX).mul_word(u64::MAX);
        // u64::MAX * u64::MAX = 2^128 - 2^65 + 1 = (2^64-1)^2
        // low 64 bits = 1, high 64 bits = u64::MAX - 1
        assert_eq!(p, U128::from_limbs([1, u64::MAX - 1]));
        assert_eq!(c, 0);
    }

    #[test]
    fn fibonacci_number_round_trip() {
        // Fib(100) = 354224848179261915075
        // hex: 0x1333db76a7c594bfc3
        // Verified with Python: `a, b = 0, 1; [a, b = b, a+b for _ in range(100)]; print(a)`
        let fib100_dec = "354224848179261915075";
        let fib100_hex = "1333db76a7c594bfc3";

        let from_dec = U128::from_str_radix(fib100_dec, 10).unwrap();
        let from_hex = U128::from_str_radix(fib100_hex, 16).unwrap();

        assert_eq!(from_dec, from_hex, "Fib(100) from decimal == from hex");
        assert_eq!(from_dec.to_string_radix(10), fib100_dec);
        assert_eq!(from_dec.to_string_radix(16), fib100_hex);

        // Fib(100) in binary
        let fib100_bin = from_dec.to_string_radix(2);
        let from_bin = U128::from_str_radix(&fib100_bin, 2).unwrap();
        assert_eq!(from_bin, from_dec);

        // Octal round-trip
        let fib100_oct = from_dec.to_string_radix(8);
        let from_oct = U128::from_str_radix(&fib100_oct, 8).unwrap();
        assert_eq!(from_oct, from_dec);
    }

    #[test]
    fn display_and_debug_formatting() {
        // Display uses decimal
        assert_eq!(format!("{}", U256::ZERO), "0");
        assert_eq!(format!("{}", U256::ONE), "1");
        assert_eq!(
            format!("{}", U256::MAX),
            "115792089237316195423570985008687907853269984665640564039457584007913129639935"
        );
        assert_eq!(
            format!("{}", P256_MODULUS),
            "115792089210356248762697446949407573530086143415290314195533631308867097853951"
        );

        // LowerHex
        assert_eq!(format!("{:x}", U256::ZERO), "0");
        assert_eq!(format!("{:x}", U256::ONE), "1");
        assert_eq!(
            format!("{:x}", U256::MAX),
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        );
        assert_eq!(format!("{:#x}", U256::from_u64(255)), "0xff");

        // UpperHex
        assert_eq!(format!("{:X}", U256::ZERO), "0");
        assert_eq!(
            format!("{:X}", P256_MODULUS),
            "FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF"
        );
        assert_eq!(format!("{:#X}", U256::from_u64(255)), "0xFF");

        // Debug
        assert_eq!(format!("{:?}", U256::ZERO), "Uint(0x0)");
        assert_eq!(format!("{:?}", U256::ONE), "Uint(0x1)");
        assert_eq!(
            format!("{:?}", P256_MODULUS),
            "Uint(0xffffffff00000001000000000000000000000000ffffffffffffffffffffffff)"
        );
    }

    #[test]
    fn to_be_le_bytes_vec() {
        // Verify Vec<u8> output matches fixed-size output
        let values = [U256::ZERO, U256::ONE, U256::MAX, P256_MODULUS, P256_GX, P256_GY];
        for v in &values {
            let be_vec = v.to_be_bytes();
            let be_fixed = v.to_be_bytes_fixed::<32>();
            assert_eq!(be_vec.as_slice(), be_fixed.as_slice(), "BE bytes of {:x}", v);

            let le_vec = v.to_le_bytes();
            let le_fixed = v.to_le_bytes_fixed::<32>();
            assert_eq!(le_vec.as_slice(), le_fixed.as_slice(), "LE bytes of {:x}", v);

            // Round-trip through be
            let rt_be = U256::from_be_slice(&be_vec);
            assert_eq!(rt_be, *v, "BE round-trip of {:x}", v);

            // Round-trip through le
            let rt_le = U256::from_le_slice(&le_vec);
            assert_eq!(rt_le, *v, "LE round-trip of {:x}", v);
        }
    }

    #[test]
    fn max_for_non_64_multiple_bits() {
        // Uint<130, 3>: LIMBS=3, top limb should have only bits 0-1 set.
        // MAX = 2^130 - 1:
        //   limb[0] = 2^64 - 1
        //   limb[1] = 2^64 - 1
        //   limb[2] = 0b11 = 3  (only 2 bits, since 130 % 64 = 2)
        type U130 = Uint<130, 3>;
        let max = U130::MAX;
        assert_eq!(max.limbs[0], u64::MAX, "U130::MAX limb[0]");
        assert_eq!(max.limbs[1], u64::MAX, "U130::MAX limb[1]");
        assert_eq!(max.limbs[2], 0b11, "U130::MAX limb[2] should have only 2 bits set");

        // Uint<192, 3>: 192 = 3*64, all limbs should be u64::MAX
        type U192 = Uint<192, 3>;
        let max192 = U192::MAX;
        assert_eq!(max192.limbs[0], u64::MAX, "U192::MAX limb[0]");
        assert_eq!(max192.limbs[1], u64::MAX, "U192::MAX limb[1]");
        assert_eq!(max192.limbs[2], u64::MAX, "U192::MAX limb[2]");

        // Uint<65, 2>: top limb should have only 1 bit set
        type U65 = Uint<65, 2>;
        let max65 = U65::MAX;
        assert_eq!(max65.limbs[0], u64::MAX, "U65::MAX limb[0]");
        assert_eq!(max65.limbs[1], 1, "U65::MAX limb[1] should be 1 (bit 64 only)");
    }

    #[test]
    fn mul_mod_associativity_and_identity() {
        // (a * b) * c == a * (b * c) mod p
        let a = P256_GX;
        let b = P256_GY;
        let c = P256_MODULUS - U256::ONE;
        let m = P256_MODULUS;

        let ab = a.mul_mod(&b, &m);
        let bc = b.mul_mod(&c, &m);

        assert_eq!(ab.mul_mod(&c, &m), a.mul_mod(&bc, &m), "associativity");

        // Identity: a * 1 = a
        assert_eq!(a.mul_mod(&U256::ONE, &m), a, "a * 1 = a");
        assert_eq!(U256::ONE.mul_mod(&a, &m), a, "1 * a = a");

        // Zero: a * 0 = 0
        assert_eq!(a.mul_mod(&U256::ZERO, &m), U256::ZERO, "a * 0 = 0");
        assert_eq!(U256::ZERO.mul_mod(&a, &m), U256::ZERO, "0 * a = 0");

        // Distributivity: (a + b) * c == a*c + b*c mod p
        let ab_sum = a.add_mod(&b, &m);
        let lhs = ab_sum.mul_mod(&c, &m);
        let rhs = a.mul_mod(&c, &m).add_mod(&b.mul_mod(&c, &m), &m);
        assert_eq!(lhs, rhs, "distributivity");
    }

    #[test]
    fn mul_mod_barrett_agrees_with_mul_mod() {
        // Already tested in ed25519; verify single multiplication works
        type U256 = Uint<256, 4>;
        let a = U256::from_u64(7);
        let b = U256::from_u64(11);
        let m = U256::from_limbs([
            0xffff_ffff_ffff_ffff,
            0x0000_0000_ffff_ffff,
            0x0000_0000_0000_0000,
            0xffff_ffff_0000_0001,
        ]);
        let mu = m.compute_mu_for_barrett();
        let barrett = a.mul_mod_barrett(&b, &m, &mu);
        let standard = a.mul_mod(&b, &m);
        assert_eq!(barrett, standard);
    }

    #[test]
    fn modpow_barrett_agrees_with_modpow() {
        type U256 = Uint<256, 4>;
        // Use P-256 prime so eff_limbs == LIMBS (4 == 4) → takes Barrett path
        let m = U256::from_limbs([
            0xffff_ffff_ffff_ffff,
            0x0000_0000_ffff_ffff,
            0x0000_0000_0000_0000,
            0xffff_ffff_0000_0001,
        ]);
        let mu = m.compute_mu_for_barrett();

        // 3^5 mod m
        let base = U256::from_u64(3);
        let exp = U256::from_u64(5);
        let barrett = base.modpow_barrett(&exp, &m, &mu);
        let standard = base.modpow(&exp, &m);
        assert_eq!(barrett, standard, "3^5 mod P256");

        // 2^10 mod m
        let base = U256::from_u64(2);
        let exp = U256::from_u64(10);
        let barrett = base.modpow_barrett(&exp, &m, &mu);
        let standard = base.modpow(&exp, &m);
        assert_eq!(barrett, standard, "2^10 mod P256");

        // 123 ^ 456 mod m
        let base = U256::from_u64(123);
        let exp = U256::from_u64(456);
        let barrett = base.modpow_barrett(&exp, &m, &mu);
        let standard = base.modpow(&exp, &m);
        assert_eq!(barrett, standard, "123^456 mod P256");

        // base == m (should give 0)
        let base = m;
        let exp = U256::from_u64(5);
        let barrett = base.modpow_barrett(&exp, &m, &mu);
        let standard = base.modpow(&exp, &m);
        assert_eq!(barrett, U256::ZERO, "m^5 mod m should be 0");
        assert_eq!(barrett, standard);

        // exp == 0 (should give 1)
        let base = U256::from_u64(123);
        let exp = U256::from_u64(0);
        let barrett = base.modpow_barrett(&exp, &m, &mu);
        let standard = base.modpow(&exp, &m);
        assert_eq!(barrett, standard, "123^0 mod P256");
        assert_eq!(barrett, U256::ONE, "123^0 mod P256 should be 1");
    }

    #[test]
    fn barrett_reduction_comprehensive() {
        type U256 = Uint<256, 4>;
        let m = U256::from_limbs([
            0xffff_ffff_ffff_ffff,
            0x0000_0000_ffff_ffff,
            0x0000_0000_0000_0000,
            0xffff_ffff_0000_0001,
        ]);
        let mu = m.compute_mu_for_barrett();

        // mul_mod_barrett should match mul_mod for various inputs
        for i in 0..100 {
            let a = U256::from_limbs([
                (i * 1234567 + 1) as u64,
                (i * 7654321 + 2) as u64,
                (i * 1357924 + 3) as u64,
                (i * 2468013 + 4) as u64,
            ]);
            let b = U256::from_limbs([
                (i * 9876543 + 5) as u64,
                (i * 3456789 + 6) as u64,
                (i * 567899 + 7) as u64,
                (i * 1122334 + 8) as u64,
            ]);
            let expected = a.mul_mod(&b, &m);
            let barrett = a.mul_mod_barrett(&b, &m, &mu);
            assert_eq!(barrett, expected, "mul_mod_barrett vs mul_mod iteration {i}");
        }

        // Edge case: (m-1)^2 mod m should be 1
        let (am1, _) = m.sub_word(1);
        let barrett = am1.mul_mod_barrett(&am1, &m, &mu);
        let expected = am1.mul_mod(&am1, &m);
        assert_eq!(barrett, expected, "(m-1)^2 mod m");
    }

    #[test]
    fn barrett_modpow_step_trace() {
        type U256 = Uint<256, 4>;
        let m = U256::from_limbs([
            0xffff_ffff_ffff_ffff,
            0x0000_0000_ffff_ffff,
            0x0000_0000_0000_0000,
            0xffff_ffff_0000_0001,
        ]);
        let mu = m.compute_mu_for_barrett();

        // Trace through 123^4 step by step
        let a = U256::from_u64(123);

        // Step 1: base = 123 * 1 mod m = 123
        let base1 = a.mul_mod_barrett(&U256::ONE, &m, &mu);
        assert_eq!(base1, U256::from_u64(123), "step1: self*1 mod m");

        // Step 2: result = 1, base = 123
        // Iterate with exp=4 (100b), LSB first:
        // i=0: bit=0 → result unchanged, base = 123*123 mod m
        let a2 = base1.mul_mod_barrett(&base1, &m, &mu);
        let a2_expected = a.mul_mod(&a, &m);
        assert_eq!(a2, a2_expected, "123*123 mod m");

        // i=1: bit=0 → result unchanged, base = a2*a2 mod m
        let a4 = a2.mul_mod_barrett(&a2, &m, &mu);
        let a4_expected = a2.mul_mod(&a2, &m);
        assert_eq!(a4, a4_expected, "123^2*123^2 mod m");

        // i=2: bit=1 → result = 1 * base = a4, base = a4*a4 mod m
        let a8 = a4.mul_mod_barrett(&a4, &m, &mu);
        let result_bit2 = a4.mul_mod_barrett(&U256::ONE, &m, &mu);
        assert_eq!(result_bit2, a4, "result after bit 2");
        let a8_expected = a4.mul_mod(&a4, &m);
        assert_eq!(a8, a8_expected, "123^4*123^4 mod m");

        // final result should be a4 (since 4=100b, only bit 2 set)
        let final_barrett = a.modpow_barrett(&U256::from_u64(4), &m, &mu);
        let final_expected = a.modpow(&U256::from_u64(4), &m);
        assert_eq!(final_barrett, final_expected, "123^4 mod m via modpow_barrett");
    }
}
