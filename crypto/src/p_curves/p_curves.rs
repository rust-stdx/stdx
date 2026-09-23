//! Shared generic implementation of the NIST P-224, P-256, P-384 and P-521
//! prime-order curves.
//!
//! This module is an implementation detail of [`crate::p_curves`]; the public
//! API lives in the per-curve modules (`p224`, `p256`, `p384`, `p521`).
//!
//! The curves share the exact same algorithms (short Weierstrass arithmetic,
//! RFC 6979 deterministic ECDSA, ECDH and SEC1 encoding). Only a handful of
//! parameters differ: the prime and group order, the curve equation's `b`
//! coefficient, the generator, the digest, and the modular reduction
//! strategy. Those are supplied by a [`Curve`] implementation.
//!
//! Fixed-size byte buffers are associated *types* (`Curve::FieldBytes`, …)
//! rather than array lengths derived from associated constants: the latter is
//! a generic const expression and is not stable.

use crate::{EllipticCurveError, RandomError};

/// Maximum field size in bytes across the supported curves (P-521).
const MAX_FIELD_BYTES: usize = 66;
/// Maximum hash output size in bytes across the supported curves (SHA-512).
const MAX_DIGEST_BYTES: usize = 64;
/// Maximum number of HMAC outputs concatenated by RFC 6979 to form `T`. Only
/// P-521 needs two (its 521-bit order is wider than SHA-512's 512-bit output);
/// every other curve fits in one.
const MAX_NONCE_BLOCKS: usize = 2;
/// Maximum size of the RFC 6979 `T` buffer in bytes.
const MAX_NONCE_BYTES: usize = MAX_NONCE_BLOCKS * MAX_DIGEST_BYTES;

/// A fixed-size byte buffer usable by the curve core.
///
/// Implemented for every `[u8; N]`. It exists so that [`Curve`] can name
/// concrete array sizes without relying on generic const expressions.
pub(crate) trait Bytes: Copy + AsRef<[u8]> + AsMut<[u8]> + PartialEq + Eq + core::fmt::Debug {
    /// Returns an all-zero buffer.
    fn zeroed() -> Self;
}

impl<const N: usize> Bytes for [u8; N] {
    #[inline]
    fn zeroed() -> Self {
        [0u8; N]
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Uint operations
////////////////////////////////////////////////////////////////////////////////////////////////////

/// The big-integer operations the curve core needs from its backing type.
///
/// This exists because [`big_number::Uint`] exposes its operations as inherent
/// methods, which cannot be called through a generic type parameter. It is
/// implemented for every `Uint<BITS, LIMBS>` by forwarding to the inherent
/// methods.
pub(crate) trait UintOps: Copy + PartialEq + Eq + core::fmt::Debug {
    /// The additive identity.
    const ZERO: Self;
    /// The multiplicative identity.
    const ONE: Self;

    /// Builds a value from a big-endian byte slice of at most `LIMBS * 8`
    /// bytes, left-padded with zeros. Panics when the slice is too long.
    fn read_be(bytes: &[u8]) -> Self;
    /// Encodes the value as big-endian bytes into `out`, whose length must be
    /// at most `LIMBS * 8`. Panics otherwise.
    fn write_be(&self, out: &mut [u8]);
    /// Returns the bit at `index` (0 is the least significant).
    fn bit(&self, index: usize) -> bool;
    /// Returns `true` when the value is zero.
    fn is_zero(&self) -> bool;
    /// Returns `true` when the value is odd.
    fn is_odd(&self) -> bool;
    /// Constant-time `>=`.
    fn ct_ge(&self, rhs: &Self) -> bool;
    /// Constant-time select: returns `a` if `choice` else `b`.
    fn ct_select(a: &Self, b: &Self, choice: bool) -> Self;
    /// Subtracts with borrow, returning the result and the borrow flag.
    fn sub_raw(&self, rhs: &Self) -> (Self, u64);
    /// Addition modulo `modulus`.
    fn add_mod(&self, rhs: &Self, modulus: &Self) -> Self;
    /// Subtraction modulo `modulus`.
    fn sub_mod(&self, rhs: &Self, modulus: &Self) -> Self;
    /// Doubling modulo `modulus`.
    fn double_mod(&self, modulus: &Self) -> Self;
    /// Zeroes the value. A no-op when the `zeroize` feature is disabled.
    fn zeroize_repr(&mut self);
}

impl<const BITS: usize, const LIMBS: usize> UintOps for big_number::Uint<BITS, LIMBS> {
    const ZERO: Self = big_number::Uint::ZERO;
    const ONE: Self = big_number::Uint::ONE;

    #[inline]
    fn read_be(bytes: &[u8]) -> Self {
        assert!(bytes.len() <= LIMBS * 8);
        let mut limbs = [0u64; LIMBS];
        let mut idx = bytes.len();
        for limb in limbs.iter_mut() {
            let take = core::cmp::min(8, idx);
            idx -= take;
            let mut buf = [0u8; 8];
            buf[8 - take..].copy_from_slice(&bytes[idx..idx + take]);
            *limb = u64::from_be_bytes(buf);
        }
        big_number::Uint::from_limbs(limbs)
    }

    #[inline]
    fn write_be(&self, out: &mut [u8]) {
        assert!(out.len() <= LIMBS * 8);
        let mut idx = out.len();
        for limb in self.limbs.iter() {
            let buf = limb.to_be_bytes();
            let take = core::cmp::min(8, idx);
            idx -= take;
            out[idx..idx + take].copy_from_slice(&buf[8 - take..]);
        }
    }

    #[inline]
    fn bit(&self, index: usize) -> bool {
        big_number::Uint::bit(self, index)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        big_number::Uint::is_zero(self)
    }

    #[inline]
    fn is_odd(&self) -> bool {
        big_number::Uint::is_odd(self)
    }

    #[inline]
    fn ct_ge(&self, rhs: &Self) -> bool {
        big_number::Uint::ct_ge(self, rhs)
    }

    #[inline]
    fn ct_select(a: &Self, b: &Self, choice: bool) -> Self {
        big_number::Uint::ct_select(a, b, choice)
    }

    #[inline]
    fn sub_raw(&self, rhs: &Self) -> (Self, u64) {
        big_number::Uint::sub_raw(self, rhs)
    }

    #[inline]
    fn add_mod(&self, rhs: &Self, modulus: &Self) -> Self {
        big_number::Uint::add_mod(self, rhs, modulus)
    }

    #[inline]
    fn sub_mod(&self, rhs: &Self, modulus: &Self) -> Self {
        big_number::Uint::sub_mod(self, rhs, modulus)
    }

    #[inline]
    fn double_mod(&self, modulus: &Self) -> Self {
        big_number::Uint::double_mod(self, modulus)
    }

    #[inline]
    #[cfg(feature = "zeroize")]
    fn zeroize_repr(&mut self) {
        zeroize::Zeroize::zeroize(self);
    }

    #[inline]
    #[cfg(not(feature = "zeroize"))]
    fn zeroize_repr(&mut self) {}
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Curve parameters
////////////////////////////////////////////////////////////////////////////////////////////////////

/// The parameters and hash/reduction hooks that distinguish one prime-order
/// curve from another.
pub(crate) trait Curve: Copy + Clone + core::fmt::Debug + PartialEq + Eq {
    /// The backing unsigned integer type (`Uint<BITS, LIMBS>`).
    type U: UintOps;
    /// A field-sized buffer (`FIELD_BYTES`).
    type FieldBytes: Bytes;
    /// A digest-sized buffer (`DIGEST_BYTES`).
    type DigestBytes: Bytes;
    /// A compressed SEC1 point buffer (`COMPRESSED_BYTES`).
    type CompressedBytes: Bytes;
    /// An uncompressed SEC1 point buffer (`UNCOMPRESSED_BYTES`).
    type UncompressedBytes: Bytes;
    /// A signature buffer (`SIGNATURE_BYTES`).
    type SignatureBytes: Bytes;

    /// Size of the field modulus in bits (224/256/384).
    const FIELD_BITS: usize;
    /// Size of a field element, scalar, coordinate and secret key in bytes.
    const FIELD_BYTES: usize;
    /// Output size of the curve's digest in bytes (32 or 48).
    const DIGEST_BYTES: usize;
    /// Size of a compressed SEC1 public key in bytes.
    const COMPRESSED_BYTES: usize;
    /// Size of an uncompressed SEC1 public key in bytes.
    const UNCOMPRESSED_BYTES: usize;

    /// The field prime `p`.
    const MODULUS_P: Self::U;
    /// The group order `n`.
    const MODULUS_N: Self::U;
    /// `floor(n / 2)`, used for low-s normalization.
    const N_HALF: Self::U;
    /// `p - 2`, used to invert field elements.
    const P_MINUS_TWO: Self::U;
    /// `n - 2`, used to invert scalars.
    const N_MINUS_TWO: Self::U;
    /// The curve equation coefficient `b`.
    const CURVE_B: Self::U;
    /// The generator's affine `x` coordinate.
    const GENERATOR_X: Self::U;
    /// The generator's affine `y` coordinate.
    const GENERATOR_Y: Self::U;

    /// Multiplies two field elements modulo `p`.
    fn field_mul(a: &Self::U, b: &Self::U) -> Self::U;
    /// Multiplies two scalars modulo `n`.
    fn scalar_mul(a: &Self::U, b: &Self::U) -> Self::U;
    /// Returns a square root of `a` modulo `p`, or `None` if `a` is a
    /// quadratic non-residue.
    fn field_sqrt(a: Self::U) -> Option<Self::U>;

    /// Hashes `data` with the curve's digest.
    fn hash(data: &[u8]) -> Self::DigestBytes;
    /// Computes the curve's HMAC over `data` with `key`.
    fn hmac(key: &[u8], data: &[u8]) -> Self::DigestBytes;

    /// Reduces a message digest to a scalar. P-224 truncates the digest to the
    /// leftmost `FIELD_BYTES` first, as specified by FIPS 186-4.
    fn scalar_from_digest(digest: &Self::DigestBytes) -> Self::U;
    /// Reduces a field-sized big-endian value modulo `n`.
    fn scalar_from_field_bytes(bytes: &Self::FieldBytes) -> Self::U;

    /// Constant-time RFC 6979 `bits2int` transform.
    ///
    /// Interprets `t` as a big-endian bit string and returns its leftmost
    /// `FIELD_BITS` bits, right-aligned in a big-endian `FieldBytes` buffer.
    /// When `t` is shorter than the field, the value is left-padded with
    /// zeros. The bit indexing is independent of the data, so no timing
    /// information about the nonce is exposed.
    fn bits2int_octets(t: &[u8]) -> Self::FieldBytes {
        let qlen = Self::FIELD_BITS;
        let blen = t.len() * 8;
        let mut out = Self::FieldBytes::zeroed();
        if blen >= qlen {
            // Keep the leftmost `qlen` bits: result bit `j` comes from bit
            // `qlen - 1 - j` of `t` (both counted from the most significant
            // bit). All indices are data-independent.
            let mut j = 0;
            while j < qlen {
                let src = qlen - 1 - j;
                let bit = (t[src / 8] >> (7 - (src % 8))) & 1;
                let byte = Self::FIELD_BYTES - 1 - (j / 8);
                out.as_mut()[byte] |= bit << (j % 8);
                j += 1;
            }
        } else {
            let pad = Self::FIELD_BYTES - t.len();
            out.as_mut()[pad..].copy_from_slice(t);
        }
        out
    }

    /// Converts an RFC 6979 `T` value (one or more concatenated HMAC outputs)
    /// into a nonce in `[1, n - 1]`, if any.
    ///
    /// Applies the `bits2int` transform, then rejects zero and values greater
    /// than or equal to the group order without reducing modulo `n`, as
    /// required by RFC 6979 §3.2.
    fn scalar_from_nonce(t: &[u8]) -> Option<Self::U> {
        let bytes = Self::bits2int_octets(t);
        let value = Self::U::read_be(bytes.as_ref());
        if value.is_zero() || value.ct_ge(&Self::MODULUS_N) {
            None
        } else {
            Some(value)
        }
    }

    /// Generates fresh random secret key bytes.
    ///
    /// Returns [`RandomError`] when the operating system's random number
    /// generator is unavailable or fails.
    #[cfg(feature = "random")]
    fn random_secret_key() -> Result<Self::FieldBytes, RandomError>;
}

/// Raises a field element to `exponent` using the curve's modular
/// multiplication.
pub(crate) fn field_pow<C: Curve>(base: C::U, exponent: &C::U) -> C::U {
    let mut result = C::U::ONE;
    let mut i = C::FIELD_BITS;
    while i > 0 {
        i -= 1;
        result = C::field_mul(&result, &result);
        let product = C::field_mul(&result, &base);
        result = C::U::ct_select(&product, &result, exponent.bit(i));
    }
    result
}

// Branch-free byte-level select: returns `a` if `choice` else `b`.
fn ct_select_bytes<C: Curve>(a: &C::DigestBytes, b: &C::DigestBytes, choice: bool) -> C::DigestBytes {
    let mask = (choice as u8).wrapping_neg();
    let mut out = C::DigestBytes::zeroed();
    {
        let o = out.as_mut();
        let aa = a.as_ref();
        let bb = b.as_ref();
        for i in 0..o.len() {
            o[i] = (aa[i] & mask) | (bb[i] & !mask);
        }
    }
    out
}

// Branch-free byte-level select: returns `a` if `choice` else `b`.
#[inline]
fn ct_select_byte(a: u8, b: u8, choice: bool) -> u8 {
    let mask = (choice as u8).wrapping_neg();
    (a & mask) | (b & !mask)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Field elements
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct FieldElement<C: Curve>(pub(crate) C::U);

#[cfg(feature = "zeroize")]
impl<C: Curve> zeroize::Zeroize for FieldElement<C> {
    fn zeroize(&mut self) {
        self.0.zeroize_repr();
    }
}

impl<C: Curve> FieldElement<C> {
    pub(crate) const ZERO: Self = Self(C::U::ZERO);
    pub(crate) const ONE: Self = Self(C::U::ONE);

    #[inline]
    pub(crate) fn from_bytes(bytes: &C::FieldBytes) -> Option<Self> {
        let value = C::U::read_be(bytes.as_ref());
        if value.ct_ge(&C::MODULUS_P) {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Wraps a raw backing integer. Test-only convenience.
    #[cfg(test)]
    #[inline]
    pub(crate) fn from_uint(value: C::U) -> Self {
        Self(value)
    }

    #[inline]
    pub(crate) fn to_bytes(self) -> C::FieldBytes {
        let mut out = C::FieldBytes::zeroed();
        self.0.write_be(out.as_mut());
        out
    }

    #[inline]
    pub(crate) fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[inline]
    pub(crate) fn is_odd(&self) -> bool {
        self.0.is_odd()
    }

    #[inline]
    pub(crate) fn add(self, rhs: Self) -> Self {
        Self(self.0.add_mod(&rhs.0, &C::MODULUS_P))
    }

    #[inline]
    pub(crate) fn sub(self, rhs: Self) -> Self {
        Self(self.0.sub_mod(&rhs.0, &C::MODULUS_P))
    }

    #[inline]
    pub(crate) fn double(self) -> Self {
        Self(self.0.double_mod(&C::MODULUS_P))
    }

    #[inline]
    pub(crate) fn square(self) -> Self {
        self.mul(self)
    }

    #[inline]
    pub(crate) fn mul(self, rhs: Self) -> Self {
        Self(C::field_mul(&self.0, &rhs.0))
    }

    #[inline]
    pub(crate) fn triple(self) -> Self {
        self.double().add(self)
    }

    #[inline]
    pub(crate) fn negate(self) -> Self {
        let (diff, _) = C::MODULUS_P.sub_raw(&self.0);
        Self(C::U::ct_select(&C::U::ZERO, &diff, self.is_zero()))
    }

    #[inline]
    pub(crate) fn pow(self, exponent: &C::U) -> Self {
        Self(field_pow::<C>(self.0, exponent))
    }

    #[inline]
    pub(crate) fn invert(self) -> Option<Self> {
        Some(self.pow(&C::P_MINUS_TWO))
    }

    #[inline]
    pub(crate) fn sqrt(self) -> Option<Self> {
        C::field_sqrt(self.0).map(Self)
    }

    #[inline]
    pub(crate) fn select(a: &Self, b: &Self, choice: bool) -> Self {
        Self(C::U::ct_select(&a.0, &b.0, choice))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Scalars
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Scalar<C: Curve>(pub(crate) C::U);

#[cfg(feature = "zeroize")]
impl<C: Curve> zeroize::Zeroize for Scalar<C> {
    fn zeroize(&mut self) {
        self.0.zeroize_repr();
    }
}

impl<C: Curve> Scalar<C> {
    pub(crate) const ZERO: Self = Self(C::U::ZERO);
    #[cfg(test)]
    pub(crate) const ONE: Self = Self(C::U::ONE);

    #[inline]
    pub(crate) fn from_slice(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != C::FIELD_BYTES {
            return None;
        }
        let value = C::U::read_be(bytes);
        if value.is_zero() || value.ct_ge(&C::MODULUS_N) {
            None
        } else {
            Some(Self(value))
        }
    }

    #[inline]
    pub(crate) fn from_bytes(bytes: &C::FieldBytes) -> Option<Self> {
        Self::from_slice(bytes.as_ref())
    }

    /// Reduces a field-sized big-endian value modulo n. Unlike [`Self::from_bytes`],
    /// this accepts zero and values greater than or equal to n.
    #[inline]
    pub(crate) fn from_reduced_bytes(bytes: &C::FieldBytes) -> Self {
        Self(C::scalar_from_field_bytes(bytes))
    }

    /// Reduces a message digest to a scalar. FIPS 186-4 truncates the digest to
    /// the leftmost `FIELD_BITS` bits for curves whose digest is wider than the
    /// field (see the curve's `scalar_from_digest`).
    #[inline]
    pub(crate) fn from_hash(hash: &C::DigestBytes) -> Self {
        Self(C::scalar_from_digest(hash))
    }

    #[inline]
    pub(crate) fn to_bytes(self) -> C::FieldBytes {
        let mut out = C::FieldBytes::zeroed();
        self.0.write_be(out.as_mut());
        out
    }

    #[inline]
    pub(crate) fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[inline]
    pub(crate) fn bit(&self, index: usize) -> bool {
        self.0.bit(index)
    }

    #[inline]
    pub(crate) fn add(self, rhs: Self) -> Self {
        Self(self.0.add_mod(&rhs.0, &C::MODULUS_N))
    }

    #[inline]
    pub(crate) fn sub(self, rhs: Self) -> Self {
        Self(self.0.sub_mod(&rhs.0, &C::MODULUS_N))
    }

    /// Returns `true` when `s > n / 2`, i.e. the signature is in
    /// "high-s" form.
    #[inline]
    pub(crate) fn is_high(&self) -> bool {
        !C::N_HALF.ct_ge(&self.0)
    }

    /// Returns the canonical low-s representative: `s` if `s <= n / 2`,
    /// otherwise `n - s`. The selection is branch-free so no timing
    /// information about the pre-normalized value is exposed.
    #[inline]
    pub(crate) fn normalize_low_s(self) -> Self {
        let negated = Self::ZERO.sub(self);
        Self::select(&negated, &self, self.is_high())
    }

    #[inline]
    pub(crate) fn mul(self, rhs: Self) -> Self {
        Self(C::scalar_mul(&self.0, &rhs.0))
    }

    #[inline]
    pub(crate) fn invert(self) -> Option<Self> {
        Some(Self(self.scalar_pow(&C::N_MINUS_TWO)))
    }

    #[inline]
    pub(crate) fn scalar_pow(self, exponent: &C::U) -> C::U {
        let mut result = C::U::ONE;
        let mut i = C::FIELD_BITS;
        while i > 0 {
            i -= 1;
            result = C::scalar_mul(&result, &result);
            let product = C::scalar_mul(&result, &self.0);
            result = C::U::ct_select(&product, &result, exponent.bit(i));
        }
        result
    }

    #[inline]
    pub(crate) fn select(a: &Self, b: &Self, choice: bool) -> Self {
        Self(C::U::ct_select(&a.0, &b.0, choice))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Points
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct AffinePoint<C: Curve> {
    pub(crate) x: FieldElement<C>,
    pub(crate) y: FieldElement<C>,
    infinity: bool,
}

#[cfg(feature = "zeroize")]
impl<C: Curve> zeroize::Zeroize for AffinePoint<C> {
    fn zeroize(&mut self) {
        self.x.zeroize();
        self.y.zeroize();
        self.infinity = false;
    }
}

impl<C: Curve> AffinePoint<C> {
    pub(crate) const GENERATOR: Self = Self {
        x: FieldElement(C::GENERATOR_X),
        y: FieldElement(C::GENERATOR_Y),
        infinity: false,
    };

    #[inline]
    pub(crate) fn new(x: FieldElement<C>, y: FieldElement<C>) -> Option<Self> {
        let point = Self {
            x,
            y,
            infinity: false,
        };
        if point.is_on_curve() { Some(point) } else { None }
    }

    #[inline]
    pub(crate) fn is_on_curve(&self) -> bool {
        if self.infinity {
            return false;
        }
        let x2 = self.x.square();
        let x3 = x2.mul(self.x);
        let rhs = x3.sub(self.x.triple()).add(FieldElement(C::CURVE_B));
        self.y.square() == rhs
    }

    #[inline]
    pub(crate) fn to_uncompressed_bytes(&self) -> C::UncompressedBytes {
        let mut out = C::UncompressedBytes::zeroed();
        let o = out.as_mut();
        o[0] = 0x04;
        self.x.0.write_be(&mut o[1..1 + C::FIELD_BYTES]);
        self.y.0.write_be(&mut o[1 + C::FIELD_BYTES..1 + 2 * C::FIELD_BYTES]);
        out
    }

    /// SEC1 compressed encoding: `0x02`/`0x03 || x`, selecting the prefix from
    /// the parity of `y`.
    #[inline]
    pub(crate) fn to_compressed_bytes(&self) -> C::CompressedBytes {
        let mut out = C::CompressedBytes::zeroed();
        let o = out.as_mut();
        o[0] = if self.y.is_odd() { 0x03 } else { 0x02 };
        self.x.0.write_be(&mut o[1..1 + C::FIELD_BYTES]);
        out
    }

    pub(crate) fn from_sec1_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() == C::UNCOMPRESSED_BYTES && bytes[0] == 0x04 {
            let x = FieldElement::from_slice(&bytes[1..1 + C::FIELD_BYTES])?;
            let y = FieldElement::from_slice(&bytes[1 + C::FIELD_BYTES..1 + 2 * C::FIELD_BYTES])?;
            Self::new(x, y)
        } else if bytes.len() == C::COMPRESSED_BYTES && (bytes[0] == 0x02 || bytes[0] == 0x03) {
            let x = FieldElement::from_slice(&bytes[1..1 + C::FIELD_BYTES])?;
            let rhs = x.square().mul(x).sub(x.triple()).add(FieldElement(C::CURVE_B));
            let y = rhs.sqrt()?;
            let y_is_odd = y.is_odd();
            let select_neg = y_is_odd != (bytes[0] == 0x03);
            let y = FieldElement::select(&y.negate(), &y, select_neg);
            Self::new(x, y)
        } else {
            None
        }
    }
}

impl<C: Curve> FieldElement<C> {
    #[inline]
    fn from_slice(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != C::FIELD_BYTES {
            return None;
        }
        let value = C::U::read_be(bytes);
        if value.ct_ge(&C::MODULUS_P) {
            None
        } else {
            Some(Self(value))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ProjectivePoint<C: Curve> {
    x: FieldElement<C>,
    y: FieldElement<C>,
    z: FieldElement<C>,
}

impl<C: Curve> ProjectivePoint<C> {
    pub(crate) const IDENTITY: Self = Self {
        x: FieldElement::ZERO,
        y: FieldElement::ONE,
        z: FieldElement::ZERO,
    };

    #[cfg(test)]
    #[inline]
    pub(crate) fn from_affine(point: &AffinePoint<C>) -> Self {
        if point.infinity {
            Self::IDENTITY
        } else {
            Self {
                x: point.x,
                y: point.y,
                z: FieldElement::ONE,
            }
        }
    }

    #[inline]
    pub(crate) fn is_identity(&self) -> bool {
        self.z.is_zero()
    }

    #[inline]
    pub(crate) fn select(a: &Self, b: &Self, choice: bool) -> Self {
        Self {
            x: FieldElement::select(&a.x, &b.x, choice),
            y: FieldElement::select(&a.y, &b.y, choice),
            z: FieldElement::select(&a.z, &b.z, choice),
        }
    }

    #[inline]
    pub(crate) fn to_affine(&self) -> Option<AffinePoint<C>> {
        if self.is_identity() {
            return None;
        }
        let z_inv = self.z.invert()?;
        AffinePoint::new(self.x.mul(z_inv), self.y.mul(z_inv))
    }

    pub(crate) fn add(&self, rhs: &Self) -> Self {
        let xx = self.x.mul(rhs.x);
        let yy = self.y.mul(rhs.y);
        let zz = self.z.mul(rhs.z);
        let xy_pairs = self.x.add(self.y).mul(rhs.x.add(rhs.y)).sub(xx.add(yy));
        let yz_pairs = self.y.add(self.z).mul(rhs.y.add(rhs.z)).sub(yy.add(zz));
        let xz_pairs = self.x.add(self.z).mul(rhs.x.add(rhs.z)).sub(xx.add(zz));

        let bzz_part = xz_pairs.sub(FieldElement(C::CURVE_B).mul(zz));
        let bzz3_part = bzz_part.triple();
        let yy_m_bzz3 = yy.sub(bzz3_part);
        let yy_p_bzz3 = yy.add(bzz3_part);

        let zz3 = zz.triple();
        let bxz_part = FieldElement(C::CURVE_B).mul(xz_pairs).sub(zz3.add(xx));
        let bxz3_part = bxz_part.triple();
        let xx3_m_zz3 = xx.triple().sub(zz3);

        Self {
            x: yy_p_bzz3.mul(xy_pairs).sub(yz_pairs.mul(bxz3_part)),
            y: yy_p_bzz3.mul(yy_m_bzz3).add(xx3_m_zz3.mul(bxz3_part)),
            z: yy_m_bzz3.mul(yz_pairs).add(xy_pairs.mul(xx3_m_zz3)),
        }
    }

    pub(crate) fn add_mixed(&self, rhs: &AffinePoint<C>) -> Self {
        if rhs.infinity {
            return *self;
        }

        let xx = self.x.mul(rhs.x);
        let yy = self.y.mul(rhs.y);
        let xy_pairs = self.x.add(self.y).mul(rhs.x.add(rhs.y)).sub(xx.add(yy));
        let yz_pairs = rhs.y.mul(self.z).add(self.y);
        let xz_pairs = rhs.x.mul(self.z).add(self.x);

        let bz_part = xz_pairs.sub(FieldElement(C::CURVE_B).mul(self.z));
        let bz3_part = bz_part.triple();
        let yy_m_bzz3 = yy.sub(bz3_part);
        let yy_p_bzz3 = yy.add(bz3_part);

        let z3 = self.z.triple();
        let bxz_part = FieldElement(C::CURVE_B).mul(xz_pairs).sub(z3.add(xx));
        let bxz3_part = bxz_part.triple();
        let xx3_m_zz3 = xx.triple().sub(z3);

        Self {
            x: yy_p_bzz3.mul(xy_pairs).sub(yz_pairs.mul(bxz3_part)),
            y: yy_p_bzz3.mul(yy_m_bzz3).add(xx3_m_zz3.mul(bxz3_part)),
            z: yy_m_bzz3.mul(yz_pairs).add(xy_pairs.mul(xx3_m_zz3)),
        }
    }

    pub(crate) fn double(&self) -> Self {
        let xx = self.x.square();
        let yy = self.y.square();
        let zz = self.z.square();
        let xy2 = self.x.mul(self.y).double();
        let xz2 = self.x.mul(self.z).double();

        let bzz_part = FieldElement(C::CURVE_B).mul(zz).sub(xz2);
        let bzz3_part = bzz_part.triple();
        let yy_m_bzz3 = yy.sub(bzz3_part);
        let yy_p_bzz3 = yy.add(bzz3_part);
        let y_frag = yy_p_bzz3.mul(yy_m_bzz3);
        let x_frag = yy_m_bzz3.mul(xy2);

        let zz3 = zz.triple();
        let bxz2_part = FieldElement(C::CURVE_B).mul(xz2).sub(zz3.add(xx));
        let bxz6_part = bxz2_part.triple();
        let xx3_m_zz3 = xx.triple().sub(zz3);

        let y = y_frag.add(xx3_m_zz3.mul(bxz6_part));
        let yz2 = self.y.mul(self.z).double();
        let x = x_frag.sub(bxz6_part.mul(yz2));
        let z = yz2.mul(yy).double().double();

        Self {
            x,
            y,
            z,
        }
    }
}

pub(crate) fn scalar_mul_generator<C: Curve>(scalar: &Scalar<C>) -> ProjectivePoint<C> {
    scalar_mul_affine(&AffinePoint::<C>::GENERATOR, scalar)
}

pub(crate) fn scalar_mul_affine<C: Curve>(base: &AffinePoint<C>, scalar: &Scalar<C>) -> ProjectivePoint<C> {
    let mut acc = ProjectivePoint::<C>::IDENTITY;
    let mut bit = C::FIELD_BITS;
    while bit > 0 {
        bit -= 1;
        acc = acc.double();
        let candidate = acc.add_mixed(base);
        acc = ProjectivePoint::select(&candidate, &acc, scalar.bit(bit));
    }
    acc
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Hashing helpers
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
#[inline]
pub(crate) fn hash_message<C: Curve>(message: &[u8]) -> C::DigestBytes {
    C::hash(message)
}

#[cfg(test)]
#[inline]
pub(crate) fn hmac_digest<C: Curve>(key: &[u8], data: &[u8]) -> C::DigestBytes {
    C::hmac(key, data)
}

// FIPS 186-4 truncates a digest to the leftmost `FIELD_BITS` bits, then reduces
// modulo n. The result is the `bits2octets` value used by RFC 6979.
pub(crate) fn bits2octets<C: Curve>(hash: &C::DigestBytes) -> C::FieldBytes {
    Scalar::<C>::from_hash(hash).to_bytes()
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// RFC 6979 deterministic nonces
////////////////////////////////////////////////////////////////////////////////////////////////////

fn rfc6979_init_state<C: Curve>(
    private_key: &Scalar<C>,
    message_hash: &C::DigestBytes,
) -> (C::DigestBytes, C::DigestBytes) {
    let x = private_key.to_bytes();
    let h1 = bits2octets::<C>(message_hash);

    let mut v = C::DigestBytes::zeroed();
    v.as_mut().fill(0x01);
    let k = C::DigestBytes::zeroed();

    let mut buf = [0u8; MAX_DIGEST_BYTES + 1 + 2 * MAX_FIELD_BYTES];
    let len = C::DIGEST_BYTES + 1 + 2 * C::FIELD_BYTES;
    buf[..C::DIGEST_BYTES].copy_from_slice(v.as_ref());
    buf[C::DIGEST_BYTES] = 0x00;
    buf[C::DIGEST_BYTES + 1..C::DIGEST_BYTES + 1 + C::FIELD_BYTES].copy_from_slice(x.as_ref());
    buf[C::DIGEST_BYTES + 1 + C::FIELD_BYTES..len].copy_from_slice(h1.as_ref());
    let k = C::hmac(k.as_ref(), &buf[..len]);
    let v = C::hmac(k.as_ref(), v.as_ref());

    buf[..C::DIGEST_BYTES].copy_from_slice(v.as_ref());
    buf[C::DIGEST_BYTES] = 0x01;
    let k = C::hmac(k.as_ref(), &buf[..len]);
    let v = C::hmac(k.as_ref(), v.as_ref());

    (k, v)
}

fn rfc6979_retry<C: Curve>(k: &mut C::DigestBytes, v: &mut C::DigestBytes) {
    let mut retry_buf = [0u8; MAX_DIGEST_BYTES + 1];
    let len = C::DIGEST_BYTES + 1;
    retry_buf[..C::DIGEST_BYTES].copy_from_slice(v.as_ref());
    retry_buf[C::DIGEST_BYTES] = 0x00;
    *k = C::hmac(k.as_ref(), &retry_buf[..len]);
    *v = C::hmac(k.as_ref(), v.as_ref());
}

/// Returns the post-retry state without mutating, for constant-time selection.
fn rfc6979_retry_clone<C: Curve>(k: &C::DigestBytes, v: &C::DigestBytes) -> (C::DigestBytes, C::DigestBytes) {
    let mut retry_buf = [0u8; MAX_DIGEST_BYTES + 1];
    let len = C::DIGEST_BYTES + 1;
    retry_buf[..C::DIGEST_BYTES].copy_from_slice(v.as_ref());
    retry_buf[C::DIGEST_BYTES] = 0x00;
    let k_new = C::hmac(k.as_ref(), &retry_buf[..len]);
    let v_new = C::hmac(k_new.as_ref(), v.as_ref());
    (k_new, v_new)
}

/// RFC 6979 §3.2 HMAC_DRBG used to derive deterministic ECDSA nonces.
pub(crate) struct Rfc6979<C: Curve> {
    pub(crate) k: C::DigestBytes,
    pub(crate) v: C::DigestBytes,
}

#[cfg(feature = "zeroize")]
impl<C: Curve> zeroize::Zeroize for Rfc6979<C> {
    fn zeroize(&mut self) {
        zeroize::Zeroize::zeroize(self.k.as_mut());
        zeroize::Zeroize::zeroize(self.v.as_mut());
    }
}

#[cfg(feature = "zeroize")]
impl<C: Curve> zeroize::ZeroizeOnDrop for Rfc6979<C> {}

impl<C: Curve> Rfc6979<C> {
    pub(crate) fn new(private_key: &Scalar<C>, message_hash: &C::DigestBytes) -> Self {
        let (k, v) = rfc6979_init_state::<C>(private_key, message_hash);
        Self {
            k,
            v,
        }
    }

    // RFC 6979 §3.2 step h.3: continue the DRBG after an unsuitable candidate.
    pub(crate) fn retry(&mut self) {
        rfc6979_retry::<C>(&mut self.k, &mut self.v);
    }

    // Fills `t` with the RFC 6979 `T` value built from the current DRBG state
    // (step h.2: repeatedly `V = HMAC_K(V)` and append `V` until `tlen >=
    // qlen`). `self.v` is left holding the last generated block, so a
    // subsequent `retry()` continues the sequence.
    fn build_t(&mut self, t: &mut [u8]) -> usize {
        let blocks = (C::FIELD_BITS + C::DIGEST_BYTES * 8 - 1) / (C::DIGEST_BYTES * 8);
        let len = blocks * C::DIGEST_BYTES;
        assert!(len <= MAX_NONCE_BYTES);
        for b in 0..blocks {
            self.v = C::hmac(self.k.as_ref(), self.v.as_ref());
            t[b * C::DIGEST_BYTES..(b + 1) * C::DIGEST_BYTES].copy_from_slice(self.v.as_ref());
        }
        len
    }

    // Returns the first candidate `k` in `[1, n-1]` from the current DRBG
    // state, advancing it as specified in RFC 6979 §3.2. On return the stored
    // state is positioned immediately after the returned candidate, so a
    // subsequent `retry()` continues the sequence.
    pub(crate) fn generate(&mut self) -> Scalar<C> {
        // Fixed 3 iterations with constant-time state selection. In each
        // iteration we generate `T`, check validity, and ct_select between
        // keeping the original state (candidate valid) or replacing it with
        // the retry state (candidate invalid). The first valid candidate is
        // captured and returned after the loop.
        let mut candidate = [0u8; MAX_NONCE_BYTES];
        let mut candidate_k = C::DigestBytes::zeroed();
        let mut candidate_v = C::DigestBytes::zeroed();
        let mut found = false;

        for _ in 0..3 {
            let mut t = [0u8; MAX_NONCE_BYTES];
            let len = self.build_t(&mut t);
            let is_valid = C::scalar_from_nonce(&t[..len]).is_some();

            let take = is_valid && !found;
            for i in 0..len {
                candidate[i] = ct_select_byte(t[i], candidate[i], take);
            }
            candidate_k = ct_select_bytes::<C>(&self.k, &candidate_k, take);
            candidate_v = ct_select_bytes::<C>(&self.v, &candidate_v, take);
            found = found || is_valid;

            // Advance DRBG: if invalid, replace state with the retry state.
            let (k_retry, v_retry) = rfc6979_retry_clone::<C>(&self.k, &self.v);
            self.k = ct_select_bytes::<C>(&self.k, &k_retry, !is_valid);
            self.v = ct_select_bytes::<C>(&self.v, &v_retry, !is_valid);
        }

        if found {
            // `candidate` passed the identical range check that
            // `scalar_from_nonce` applies, so this always succeeds.
            let len = (C::FIELD_BITS + C::DIGEST_BYTES * 8 - 1) / (C::DIGEST_BYTES * 8) * C::DIGEST_BYTES;
            if let Some(value) = C::scalar_from_nonce(&candidate[..len]) {
                self.k = candidate_k;
                self.v = candidate_v;
                return Scalar(value);
            }
        }

        // Fallback (probability < 2^-96): keep advancing the DRBG until a
        // valid candidate appears. Never returns k = 0.
        loop {
            let mut t = [0u8; MAX_NONCE_BYTES];
            let len = self.build_t(&mut t);
            if let Some(value) = C::scalar_from_nonce(&t[..len]) {
                return Scalar(value);
            }
            rfc6979_retry::<C>(&mut self.k, &mut self.v);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// ECDSA and ECDH
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
pub(crate) fn parse_secret_key<C: Curve>(private_key: &C::FieldBytes) -> Result<Scalar<C>, EllipticCurveError> {
    Scalar::from_bytes(private_key).ok_or(EllipticCurveError::InvalidKey)
}

#[cfg(test)]
pub(crate) fn parse_public_key<C: Curve>(public_key: &[u8]) -> Result<AffinePoint<C>, EllipticCurveError> {
    AffinePoint::<C>::from_sec1_bytes(public_key).ok_or(EllipticCurveError::InvalidKey)
}

#[cfg(test)]
pub(crate) fn derive_public_key_uncompressed<C: Curve>(
    private_key: &C::FieldBytes,
) -> Result<C::UncompressedBytes, EllipticCurveError> {
    let scalar = parse_secret_key::<C>(private_key)?;
    let point = scalar_mul_generator(&scalar)
        .to_affine()
        .ok_or(EllipticCurveError::Unspecified)?;
    Ok(point.to_uncompressed_bytes())
}

#[cfg(test)]
pub(crate) fn derive_public_key_compressed<C: Curve>(
    private_key: &C::FieldBytes,
) -> Result<C::CompressedBytes, EllipticCurveError> {
    let scalar = parse_secret_key::<C>(private_key)?;
    let point = scalar_mul_generator(&scalar)
        .to_affine()
        .ok_or(EllipticCurveError::Unspecified)?;
    Ok(point.to_compressed_bytes())
}

fn ecdh_inner<C: Curve>(scalar: &Scalar<C>, peer_point: &AffinePoint<C>) -> Result<C::FieldBytes, EllipticCurveError> {
    let shared_point = scalar_mul_affine(peer_point, scalar)
        .to_affine()
        .ok_or(EllipticCurveError::Unspecified)?;
    Ok(shared_point.x.to_bytes())
}

fn ecdsa_sign_inner<C: Curve>(scalar: &Scalar<C>, message: &[u8]) -> Result<C::SignatureBytes, EllipticCurveError> {
    ecdsa_sign_inner_impl(scalar, message, false)
}

// `force_first_retry` exercises the RFC 6979 §3.2 h.3 continuation path in
// tests; it is always `false` in production.
pub(crate) fn ecdsa_sign_inner_impl<C: Curve>(
    scalar: &Scalar<C>,
    message: &[u8],
    force_first_retry: bool,
) -> Result<C::SignatureBytes, EllipticCurveError> {
    let message_hash = C::hash(message);
    let z = Scalar::<C>::from_hash(&message_hash);

    let mut drbg = Rfc6979::<C>::new(scalar, &message_hash);
    let mut k = drbg.generate();

    // The first candidate is used with overwhelming probability. A retry only
    // occurs if it yields r = 0 or s = 0 (probability ≈ 2^-256), in which case
    // RFC 6979 §3.2 requires continuing the DRBG for a fresh nonce.
    for i in 0..2 {
        if force_first_retry && i == 0 {
            drbg.retry();
            k = drbg.generate();
            continue;
        }

        let r_point = scalar_mul_generator(&k)
            .to_affine()
            .ok_or(EllipticCurveError::Unspecified)?;
        let r = Scalar::<C>::from_reduced_bytes(&r_point.x.to_bytes());
        if r.is_zero() {
            drbg.retry();
            k = drbg.generate();
            continue;
        }

        let kinv = k.invert().ok_or(EllipticCurveError::Unspecified)?;
        let s = kinv.mul(z.add(r.mul(*scalar)));
        if s.is_zero() {
            drbg.retry();
            k = drbg.generate();
            continue;
        }

        // Emit only the canonical low-s representative so signatures are
        // non-malleable and byte-for-byte deterministic.
        let s = s.normalize_low_s();

        let mut out = C::SignatureBytes::zeroed();
        let o = out.as_mut();
        r.0.write_be(&mut o[..C::FIELD_BYTES]);
        s.0.write_be(&mut o[C::FIELD_BYTES..]);
        return Ok(out);
    }

    Err(EllipticCurveError::Unspecified)
}

pub(crate) fn ecdsa_verify_inner<C: Curve>(
    public_point: &AffinePoint<C>,
    message: &[u8],
    signature: &C::SignatureBytes,
) -> Result<(), EllipticCurveError> {
    ecdsa_verify_inner_with(public_point, message, signature, false)
}

pub(crate) fn ecdsa_verify_inner_with<C: Curve>(
    public_point: &AffinePoint<C>,
    message: &[u8],
    signature: &C::SignatureBytes,
    require_low_s: bool,
) -> Result<(), EllipticCurveError> {
    let sig = signature.as_ref();
    let r = Scalar::<C>::from_slice(&sig[..C::FIELD_BYTES]).ok_or(EllipticCurveError::Unspecified)?;
    let s = Scalar::<C>::from_slice(&sig[C::FIELD_BYTES..]).ok_or(EllipticCurveError::Unspecified)?;
    if require_low_s && s.is_high() {
        return Err(EllipticCurveError::Unspecified);
    }
    let z = Scalar::<C>::from_hash(&C::hash(message));

    let w = s.invert().ok_or(EllipticCurveError::Unspecified)?;
    let u1 = z.mul(w);
    let u2 = r.mul(w);

    let point = scalar_mul_generator(&u1).add(&scalar_mul_affine(public_point, &u2));
    let affine = point.to_affine().ok_or(EllipticCurveError::Unspecified)?;
    let x_mod_n = Scalar::<C>::from_reduced_bytes(&affine.x.to_bytes());

    if x_mod_n == r {
        Ok(())
    } else {
        Err(EllipticCurveError::Unspecified)
    }
}

pub(crate) fn is_valid_public_key<C: Curve>(public_key: &[u8]) -> bool {
    AffinePoint::<C>::from_sec1_bytes(public_key).is_some()
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Public API
////////////////////////////////////////////////////////////////////////////////////////////////////

/// ECDSA secret key.
///
/// Supports signing with deterministic (RFC 6979) nonces and ECDH key
/// agreement.
///
/// # Signing
///
/// ```ignore
/// use crypto::p256::SecretKey;
///
/// let key = SecretKey::generate().unwrap();
/// let signature = key.sign(b"message").unwrap();
/// ```
///
/// # ECDH key exchange
///
/// ```ignore
/// use crypto::p256::SecretKey;
///
/// let alice = SecretKey::generate().unwrap();
/// let bob = SecretKey::generate().unwrap();
/// let alice_shared = alice.ecdh(&bob.public_key()).unwrap();
/// let bob_shared = bob.ecdh(&alice.public_key()).unwrap();
/// assert_eq!(alice_shared, bob_shared);
/// ```
///
/// # Security
///
/// The raw shared secret from [`ecdh`](Self::ecdh) **must not** be used
/// directly as an encryption key. Apply a KDF (e.g. HKDF) first.
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct SecretKey<C: Curve> {
    scalar: Scalar<C>,
    public_point: AffinePoint<C>,
}

#[cfg(feature = "zeroize")]
impl<C: Curve> zeroize::Zeroize for SecretKey<C> {
    fn zeroize(&mut self) {
        self.scalar.zeroize();
        self.public_point.zeroize();
    }
}

#[cfg(feature = "zeroize")]
impl<C: Curve> zeroize::ZeroizeOnDrop for SecretKey<C> {}

impl<C: Curve> SecretKey<C> {
    #[cfg(feature = "random")]
    pub fn generate() -> Result<SecretKey<C>, EllipticCurveError> {
        let key = C::random_secret_key()?;
        Self::from_bytes(&key)
    }

    pub fn from_bytes(key: &C::FieldBytes) -> Result<SecretKey<C>, EllipticCurveError> {
        let scalar = Scalar::from_bytes(key).ok_or(EllipticCurveError::InvalidKey)?;
        let public_point = scalar_mul_generator(&scalar)
            .to_affine()
            .ok_or(EllipticCurveError::Unspecified)?;
        Ok(SecretKey {
            scalar,
            public_point,
        })
    }

    pub fn public_key(&self) -> PublicKey<C> {
        PublicKey {
            point: self.public_point,
        }
    }

    /// Signs `message` with ECDSA using the curve's digest and deterministic
    /// (RFC 6979) nonces.
    ///
    /// The emitted signature is always in canonical low-s form (`s <= n / 2`),
    /// so a given key and message always produce the exact same signature
    /// bytes. This prevents signature malleability for systems that hash the
    /// signature bytes (e.g. transaction ids, replay caches).
    ///
    /// Verification remains permissive by default (both `s` and `n - s` are
    /// accepted); use [`PublicKey::verify_strict`] if high-s signatures must be
    /// rejected.
    pub fn sign(&self, message: &[u8]) -> Result<C::SignatureBytes, EllipticCurveError> {
        ecdsa_sign_inner(&self.scalar, message)
    }

    pub fn ecdh(&self, peer_public: &PublicKey<C>) -> Result<C::FieldBytes, EllipticCurveError> {
        ecdh_inner(&self.scalar, &peer_public.point)
    }

    pub fn to_bytes(&self) -> C::FieldBytes {
        self.scalar.to_bytes()
    }
}

/// ECDSA public key.
///
/// Supports signature verification and ECDH key agreement. Both compressed and
/// uncompressed SEC1 encodings are accepted on input.
///
/// # Verification
///
/// ```ignore
/// use crypto::p256::SecretKey;
///
/// let key = SecretKey::generate().unwrap();
/// let signature = key.sign(b"message").unwrap();
/// assert!(key.public_key().verify(b"message", &signature).is_ok());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PublicKey<C: Curve> {
    point: AffinePoint<C>,
}

impl<C: Curve> PublicKey<C> {
    #[inline]
    pub fn from_bytes(key: &[u8]) -> Result<PublicKey<C>, EllipticCurveError> {
        let point = AffinePoint::from_sec1_bytes(key).ok_or(EllipticCurveError::InvalidKey)?;
        Ok(PublicKey {
            point,
        })
    }

    /// Build a public key from raw affine x and y coordinates (both
    /// big-endian, `FIELD_BYTES` each). Returns `InvalidKey` if the coordinates
    /// are not a valid point on the curve.
    ///
    /// This is useful when importing keys from formats like JWK where `x`
    /// and `y` are available directly.
    #[inline]
    pub fn from_x_y(x_bytes: &C::FieldBytes, y_bytes: &C::FieldBytes) -> Result<PublicKey<C>, EllipticCurveError> {
        let x = FieldElement::from_bytes(x_bytes).ok_or(EllipticCurveError::InvalidKey)?;
        let y = FieldElement::from_bytes(y_bytes).ok_or(EllipticCurveError::InvalidKey)?;
        let point = AffinePoint::new(x, y).ok_or(EllipticCurveError::InvalidKey)?;
        Ok(PublicKey {
            point,
        })
    }

    /// Verifies an ECDSA signature over `message` using the curve's digest.
    ///
    /// Both canonical low-s and non-canonical high-s signatures are accepted,
    /// matching typical ECDSA interoperability. Use [`Self::verify_strict`] to
    /// additionally reject high-s signatures.
    pub fn verify(&self, message: &[u8], signature: &C::SignatureBytes) -> Result<(), EllipticCurveError> {
        ecdsa_verify_inner(&self.point, message, signature)
    }

    /// Verifies an ECDSA signature over `message`, additionally rejecting
    /// non-canonical high-s signatures (`s > n / 2`).
    ///
    /// Use this when signature malleability must be excluded (Bitcoin/EIP-2
    /// style). Note that some third-party signers emit high-s signatures, which
    /// this method will reject; [`Self::verify`] accepts both forms.
    pub fn verify_strict(&self, message: &[u8], signature: &C::SignatureBytes) -> Result<(), EllipticCurveError> {
        ecdsa_verify_inner_with(&self.point, message, signature, true)
    }

    #[inline]
    pub fn to_bytes(&self) -> C::UncompressedBytes {
        self.point.to_uncompressed_bytes()
    }

    /// Returns the compressed SEC1 encoding (`0x02`/`0x03 || x`).
    #[inline]
    pub fn to_compressed_bytes(&self) -> C::CompressedBytes {
        self.point.to_compressed_bytes()
    }

    /// Returns the `X` and `Y` points as big-endian arrays.
    #[inline]
    pub fn x_y(&self) -> (C::FieldBytes, C::FieldBytes) {
        (self.point.x.to_bytes(), self.point.y.to_bytes())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Curve-independent tests
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;

    // Raw-byte ECDH built from the public method, used to exercise peer-key
    // parsing in the tests below.
    fn ecdh<C: Curve>(secret_key: &C::FieldBytes, peer: &[u8]) -> Result<C::FieldBytes, EllipticCurveError> {
        SecretKey::<C>::from_bytes(secret_key)?.ecdh(&PublicKey::<C>::from_bytes(peer)?)
    }

    pub(crate) fn generator_point_is_on_curve<C: Curve>() {
        assert!(AffinePoint::<C>::GENERATOR.is_on_curve());
    }

    pub(crate) fn from_x_y_rejects_off_curve<C: Curve>() {
        assert!(PublicKey::<C>::from_x_y(&C::FieldBytes::zeroed(), &C::FieldBytes::zeroed()).is_err());
    }

    #[cfg(feature = "random")]
    pub(crate) fn ecdh_round_trip_alice_bob<C: Curve>() {
        // Full round-trip ECDH key exchange with randomly generated keys
        let alice = SecretKey::<C>::generate().unwrap();
        let bob = SecretKey::<C>::generate().unwrap();

        let alice_shared = alice.ecdh(&bob.public_key()).unwrap();
        let bob_shared = bob.ecdh(&alice.public_key()).unwrap();

        assert_eq!(alice_shared, bob_shared);
        assert_eq!(alice_shared.as_ref().len(), C::FIELD_BYTES);
    }

    #[cfg(feature = "random")]
    pub(crate) fn ecdh_rejects_off_curve_peer_public_key<C: Curve>() {
        let alice = SecretKey::<C>::generate().unwrap();
        let mut bad_pub = alice.public_key().to_bytes().as_ref().to_vec();
        // Flip a bit in y to take it off the curve
        bad_pub[1 + C::FIELD_BYTES] ^= 0x01;
        assert!(!is_valid_public_key::<C>(&bad_pub));
        assert!(ecdh::<C>(&alice.to_bytes(), &bad_pub).is_err());
    }

    #[cfg(feature = "random")]
    pub(crate) fn ecdh_rejects_infinity_peer_public_key<C: Curve>() {
        let alice = SecretKey::<C>::generate().unwrap();
        // Infinity encoding (0x00) should be rejected
        let infinity = [0x00u8];
        assert!(ecdh::<C>(&alice.to_bytes(), &infinity).is_err());
    }

    #[cfg(feature = "random")]
    pub(crate) fn ecdh_rejects_bad_length_peer_public_key<C: Curve>() {
        let alice = SecretKey::<C>::generate().unwrap();
        // Empty key
        assert!(ecdh::<C>(&alice.to_bytes(), &[]).is_err());
        // Truncated key
        assert!(ecdh::<C>(&alice.to_bytes(), &[0x04, 0x00]).is_err());
        // Too long
        let long = [0x04u8; 200];
        assert!(ecdh::<C>(&alice.to_bytes(), &long).is_err());
    }

    #[cfg(feature = "random")]
    pub(crate) fn ecdh_multiple_exchanges_consistency<C: Curve>() {
        // Verify ECDH commutativity across multiple key pairs
        let alice = SecretKey::<C>::generate().unwrap();
        let bob = SecretKey::<C>::generate().unwrap();
        let charlie = SecretKey::<C>::generate().unwrap();

        let alice_bob = alice.ecdh(&bob.public_key()).unwrap();
        let bob_alice = bob.ecdh(&alice.public_key()).unwrap();
        assert_eq!(alice_bob, bob_alice);

        let alice_charlie = alice.ecdh(&charlie.public_key()).unwrap();
        let charlie_alice = charlie.ecdh(&alice.public_key()).unwrap();
        assert_eq!(alice_charlie, charlie_alice);

        let bob_charlie = bob.ecdh(&charlie.public_key()).unwrap();
        let charlie_bob = charlie.ecdh(&bob.public_key()).unwrap();
        assert_eq!(bob_charlie, charlie_bob);

        // All three should be different
        assert_ne!(alice_bob, alice_charlie);
        assert_ne!(alice_bob, bob_charlie);
        assert_ne!(alice_charlie, bob_charlie);
    }

    #[cfg(feature = "random")]
    pub(crate) fn private_key_round_trip_bytes<C: Curve>() {
        let key = SecretKey::<C>::generate().unwrap();
        let bytes = key.to_bytes();
        let key2 = SecretKey::<C>::from_bytes(&bytes).unwrap();
        assert_eq!(key.to_bytes(), key2.to_bytes());
        assert_eq!(key.public_key().to_bytes(), key2.public_key().to_bytes());
    }

    #[cfg(feature = "random")]
    pub(crate) fn public_key_round_trip_bytes<C: Curve>() {
        let key = SecretKey::<C>::generate().unwrap();
        let pub_key = key.public_key();
        let bytes = pub_key.to_bytes();
        let pub_key2 = PublicKey::<C>::from_bytes(bytes.as_ref()).unwrap();
        assert_eq!(pub_key.to_bytes(), pub_key2.to_bytes());
    }

    pub(crate) fn point_double_and_add_consistency<C: Curve>() {
        // 2*G = G + G
        let g = AffinePoint::<C>::GENERATOR;
        let proj_g = ProjectivePoint::<C>::from_affine(&g);
        let doubled = proj_g.double();
        let added = proj_g.add(&proj_g);
        assert_eq!(
            doubled.to_affine().unwrap().to_uncompressed_bytes().as_ref(),
            added.to_affine().unwrap().to_uncompressed_bytes().as_ref(),
        );
    }

    #[cfg(feature = "random")]
    pub(crate) fn compressed_public_key_has_correct_prefix<C: Curve>() {
        for _ in 0..5 {
            let key = SecretKey::<C>::generate().unwrap();
            let compressed = derive_public_key_compressed::<C>(&key.to_bytes()).unwrap();
            let prefix = compressed.as_ref()[0];
            assert!(prefix == 0x02 || prefix == 0x03, "invalid compressed prefix: {prefix:#x}");
        }
    }

    #[cfg(feature = "zeroize")]
    pub(crate) fn rfc6979_state_zeroize_clears_drbg<C: Curve>() {
        use zeroize::Zeroize;

        let mut key = C::FieldBytes::zeroed();
        key.as_mut().fill(1);
        let scalar = Scalar::<C>::from_bytes(&key).unwrap();
        let mut hash = C::DigestBytes::zeroed();
        hash.as_mut().fill(0xab);
        let mut drbg = Rfc6979::<C>::new(&scalar, &hash);
        assert_ne!(drbg.k, C::DigestBytes::zeroed());
        assert_ne!(drbg.v, C::DigestBytes::zeroed());
        drbg.zeroize();
        assert_eq!(drbg.k, C::DigestBytes::zeroed());
        assert_eq!(drbg.v, C::DigestBytes::zeroed());
    }
}
