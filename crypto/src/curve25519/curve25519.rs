use big_number::{Uint, adc, mac};

pub type U256 = Uint<256, 4>;

pub const P: U256 = U256::from_limbs([
    0xffff_ffff_ffff_ffed,
    0xffff_ffff_ffff_ffff,
    0xffff_ffff_ffff_ffff,
    0x7fff_ffff_ffff_ffff,
]);

const P_MINUS_TWO: U256 = U256::from_limbs([
    0xffff_ffff_ffff_ffeb,
    0xffff_ffff_ffff_ffff,
    0xffff_ffff_ffff_ffff,
    0x7fff_ffff_ffff_ffff,
]);

/// (p+3)/8 — exponent used for square root extraction when p ≡ 5 (mod 8).
/// Bit pattern: 0x0fff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_fffe
const P_PLUS_THREE_OVER_EIGHT: [u64; 4] = [
    0xffff_ffff_ffff_fffe,
    0xffff_ffff_ffff_ffff,
    0xffff_ffff_ffff_ffff,
    0x0fff_ffff_ffff_ffff,
];

/// Element of the field GF(p) where p = 2^255 - 19.
///
/// All operations are constant-time to prevent side-channel attacks.
/// The element is represented in little-endian limb form (a[0] is the
/// least significant 64-bit word).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldElement(pub U256);

impl FieldElement {
    pub const ZERO: Self = Self(U256::ZERO);
    pub const ONE: Self = Self(U256::ONE);

    /// Decodes a field element from a canonical 32-byte representation.
    ///
    /// Returns `None` if the input is not a valid field element — that is,
    /// if the value is greater than or equal to `p = 2^255 - 19`.
    ///
    /// This should be used when decoding public keys or points where
    /// non-canonical encodings MUST be rejected (e.g. Ed25519 public keys
    /// and R components of signatures, where RFC 8032 requires rejecting
    /// values ≥ p).
    pub fn from_canonical_bytes(bytes: &[u8; 32]) -> Option<Self> {
        let value = U256::from_le_slice(bytes);
        if value.ct_ge(&P) { None } else { Some(Self(value)) }
    }

    /// Decodes a field element from a relaxed 32-byte representation.
    ///
    /// Accepts any 32-byte input by:
    /// 1. Masking out the top bit (bit 255) to clear the sign/x-coordinate bit
    /// 2. Reducing the resulting value modulo `p = 2^255 - 19` if ≥ p
    ///
    /// This should be used when decoding X25519 public keys, where RFC 7748
    /// requires accepting non-canonical encodings (values ≥ p are reduced to
    /// their canonical form).
    pub fn from_relaxed_bytes(bytes: &[u8; 32]) -> Self {
        let mut masked = *bytes;
        masked[31] &= 0x7f;
        let value = U256::from_le_slice(&masked);
        let (reduced, _) = value.sub_raw(&P);
        let needs_reduction = value.ct_ge(&P);
        Self(U256::ct_select(&reduced, &value, needs_reduction))
    }

    /// Encodes this field element to 32 little-endian bytes.
    pub fn to_bytes(self) -> [u8; 32] {
        self.0.to_le_bytes_fixed::<32>()
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn is_odd(&self) -> bool {
        self.0.is_odd()
    }

    /// Constant-time addition modulo p.
    #[inline]
    pub fn add(self, rhs: Self) -> Self {
        Self(self.0.add_mod(&rhs.0, &P))
    }

    /// Constant-time subtraction modulo p.
    #[inline]
    pub fn sub(self, rhs: Self) -> Self {
        Self(self.0.sub_mod(&rhs.0, &P))
    }

    /// Constant-time multiplication modulo p (fully reduced).
    ///
    /// Uses the identity 2^256 ≡ 38 (mod p) for fast reduction:
    ///   1. Compute the 512-bit product via schoolbook multiplication
    ///   2. Forward-fold the high 256 bits: result += 38 × high_bits
    ///   3. Propagate any remaining carry through all limbs
    ///   4. Subtract p up to two times if the result is ≥ p
    #[inline]
    pub fn mul(self, rhs: Self) -> Self {
        let prod = mul_4x4(&self.0.limbs, &rhs.0.limbs);
        Self(reduce_38_full(&prod))
    }

    /// Constant-time multiplication modulo p (relaxed reduction).
    ///
    /// Same as `mul` but omits the final two conditional subtractions,
    /// returning a value in [0, 2p). Use this when the result will be
    /// consumed by further field operations, deferring full reduction.
    #[inline]
    pub fn mul_relaxed(self, rhs: Self) -> Self {
        let prod = mul_4x4(&self.0.limbs, &rhs.0.limbs);
        Self(reduce_38_relaxed(&prod))
    }

    /// Constant-time square modulo p (fully reduced).
    ///
    /// Uses a dedicated squaring routine that computes only the unique
    /// half-products (4 diagonals + 6 cross-terms, doubled) instead of
    /// the full 16-product schoolbook used by `mul`.
    #[inline]
    pub fn square(self) -> Self {
        let sq = square_4(&self.0.limbs);
        Self(reduce_38_full(&sq))
    }

    /// Constant-time square modulo p (relaxed reduction).
    ///
    /// Same as `square` but omits the final two conditional subtractions.
    #[inline]
    pub fn square_relaxed(self) -> Self {
        let sq = square_4(&self.0.limbs);
        Self(reduce_38_relaxed(&sq))
    }

    /// Constant-time negation modulo p.
    #[inline]
    pub fn negate(self) -> Self {
        let (diff, _) = P.sub_raw(&self.0);
        Self(U256::ct_select(&U256::ZERO, &diff, self.is_zero()))
    }

    /// Constant-time modular inverse via Fermat's little theorem: a^(p-2) mod p.
    pub fn invert(self) -> Option<Self> {
        if self.is_zero() {
            return None;
        }
        Some(self.pow(&P_MINUS_TWO))
    }

    /// Constant-time modular exponentiation.
    ///
    /// Scans the exponent bits from most significant to least, using
    /// square-and-conditional-multiply.
    pub fn pow(self, exponent: &U256) -> Self {
        let mut result = Self::ONE;
        let mut i = 256;
        while i > 0 {
            i -= 1;
            result = result.square();
            let product = result.mul(self);
            result = Self::select(&product, &result, exponent.bit(i));
        }
        result
    }

    /// Specialized constant-time exponentiation for the sqrt exponent
    /// (p+3)/8 = 0x0ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe.
    ///
    /// Avoids per-bit extraction via `exponent.bit(i)` by directly indexing
    /// into the pre-split limb array.
    pub fn pow_sqrt_exponent(self) -> Self {
        let mut result = Self::ONE;
        let exponent = &P_PLUS_THREE_OVER_EIGHT;

        let mut bit = 256u32;
        while bit > 0 {
            bit -= 1;
            result = result.square();
            let limb = exponent[(bit / 64) as usize];
            let mask = 1u64 << (bit % 64);
            if (limb & mask) != 0 {
                result = result.mul(self);
            }
        }

        result
    }

    pub fn ct_eq(&self, rhs: &Self) -> bool {
        self.0.ct_eq(&rhs.0)
    }

    /// Constant-time select: returns `a` if `choice` is true, `b` otherwise.
    #[inline]
    pub fn select(a: &Self, b: &Self, choice: bool) -> Self {
        Self(U256::ct_select(&a.0, &b.0, choice))
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// 4×4 -> 8-limb schoolbook product (16 multiplications).
#[inline]
fn mul_4x4(a: &[u64; 4], b: &[u64; 4]) -> [u64; 8] {
    let mut p = [0u64; 8];
    let mut i = 0usize;
    while i < 4 {
        let mut carry = 0u64;
        let mut j = 0usize;
        while j < 4 {
            let (v, c) = mac(p[i + j], a[i], b[j], carry);
            p[i + j] = v;
            carry = c;
            j += 1;
        }
        p[i + 4] = carry;
        i += 1;
    }
    p
}

/// 4-limb squaring — delegates to the generic `mul_4x4` since
/// a dedicated squaring with the same operand count (16 macs for 4-limb
/// schoolbook) does not reduce the number of multiplications.
/// The real savings come from lazy reduction in `square_relaxed`.
#[inline]
fn square_4(a: &[u64; 4]) -> [u64; 8] {
    mul_4x4(a, a)
}

/// Full reduction from 8 limbs to 4 limbs (mod p), including 2 conditional
/// subtractions to ensure the result is < p.
#[inline]
fn reduce_38_full(limbs: &[u64; 8]) -> U256 {
    let mut p = *limbs;
    fold_38(&mut p);
    let mut result = U256::from_limbs([p[0], p[1], p[2], p[3]]);
    let (sub, borrow) = result.sub_raw(&P);
    result = U256::ct_select(&sub, &result, borrow == 0);
    let (sub, borrow) = result.sub_raw(&P);
    U256::ct_select(&sub, &result, borrow == 0)
}

/// Relaxed reduction from 8 limbs to 4 limbs (mod p), WITHOUT final
/// conditional subtractions. Returns a value in [0, 2p).
#[inline]
fn reduce_38_relaxed(limbs: &[u64; 8]) -> U256 {
    let mut p = *limbs;
    fold_38(&mut p);
    U256::from_limbs([p[0], p[1], p[2], p[3]])
}

/// Forward-fold the high 4 limbs into the low 4 using 2^256 ≡ 38 (mod p).
///
/// After folding, p[0..3] + carry*2^256 represents the reduced value.
/// The carry is then resolved by adding 38·carry to p[0..3] and finally
/// 38 to p[0] if any overflow remains.
#[inline]
fn fold_38(p: &mut [u64; 8]) {
    let mut carry = 0u64;
    let mut i = 0usize;
    while i < 4 {
        let (v, c) = mac(p[i], 38, p[i + 4], carry);
        p[i] = v;
        carry = c;
        i += 1;
    }

    let mut extra = carry.wrapping_mul(38);
    i = 0;
    while i < 4 {
        let (v, c) = adc(p[i], extra, 0);
        p[i] = v;
        extra = c;
        i += 1;
    }
    if extra > 0 {
        let (v, _) = adc(p[0], 38, 0);
        p[0] = v;
    }
}
