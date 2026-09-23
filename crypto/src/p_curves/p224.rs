//! P-224 (secp224r1) ECDSA and ECDH.
//!
//! Messages are hashed with SHA-256, truncated to the leftmost 224 bits as
//! specified by FIPS 186-4. See [`SecretKey`] and [`PublicKey`] for the
//! signing, verification and key agreement APIs.

use big_number::Uint;

use super::p_curves::{self, Curve, UintOps, field_pow};
use crate::{EllipticCurveError, Hasher, hmac::Hmac, sha2::Sha256};

/// Size of a P-224 secret key in bytes (28 bytes).
pub const SECRET_KEY_SIZE: usize = 28;
/// Size of a compressed P-224 public key in bytes (29 bytes, includes 0x02/0x03 prefix).
pub const PUBLIC_KEY_COMPRESSED_SIZE: usize = 29;
/// Size of an uncompressed P-224 public key in bytes (57 bytes, includes 0x04 prefix).
pub const PUBLIC_KEY_UNCOMPRESSED_SIZE: usize = 57;
/// Size of a P-224 ECDSA signature in bytes (56 bytes, r || s).
pub const SIGNATURE_SIZE: usize = 56;
/// Size of the raw ECDH shared secret in bytes (28 bytes). **Must not** be used directly
/// as an encryption key; apply a KDF first.
pub const ECDH_SHARED_SECRET_SIZE: usize = 28;

/// P-224 (secp224r1) ECDSA secret key.
///
/// Supports signing and ECDH key agreement. Messages are hashed with SHA-256,
/// truncated to the leftmost 224 bits as specified by FIPS 186-4.
///
/// # Signing
///
/// ```ignore
/// use crypto::p224::SecretKey;
///
/// let key = SecretKey::generate().unwrap();
/// let signature = key.sign(b"message").unwrap();
/// ```
///
/// # ECDH key exchange
///
/// ```ignore
/// use crypto::p224::SecretKey;
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
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct SecretKey(p_curves::SecretKey<P224>);

impl SecretKey {
    /// Generates a fresh random secret key.
    #[cfg(feature = "random")]
    pub fn generate() -> Result<SecretKey, EllipticCurveError> {
        Ok(SecretKey(p_curves::SecretKey::generate()?))
    }

    /// Builds a secret key from its 28-byte big-endian scalar.
    ///
    /// Returns [`EllipticCurveError::InvalidKey`] when the scalar is zero or
    /// greater than or equal to the group order.
    pub fn from_bytes(key: &[u8; SECRET_KEY_SIZE]) -> Result<SecretKey, EllipticCurveError> {
        Ok(SecretKey(p_curves::SecretKey::from_bytes(key)?))
    }

    /// Returns the corresponding public key.
    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.public_key())
    }

    /// Signs `message` with ECDSA using SHA-256 (truncated to the leftmost 224
    /// bits) and deterministic (RFC 6979) nonces.
    ///
    /// The emitted signature is always in canonical low-s form (`s <= n / 2`),
    /// so a given key and message always produce the exact same signature
    /// bytes. This prevents signature malleability for systems that hash the
    /// signature bytes (e.g. transaction ids, replay caches).
    ///
    /// Verification remains permissive by default (both `s` and `n - s` are
    /// accepted); use [`PublicKey::verify_strict`] if high-s signatures must be
    /// rejected.
    pub fn sign(&self, message: &[u8]) -> Result<[u8; SIGNATURE_SIZE], EllipticCurveError> {
        self.0.sign(message)
    }

    /// Computes the ECDH shared secret with `peer_public`.
    ///
    /// # Security
    ///
    /// The raw shared secret **must not** be used directly as an encryption
    /// key. Apply a KDF (e.g. HKDF) first.
    pub fn ecdh(&self, peer_public: &PublicKey) -> Result<[u8; ECDH_SHARED_SECRET_SIZE], EllipticCurveError> {
        self.0.ecdh(&peer_public.0)
    }

    /// Returns the secret scalar as 28 big-endian bytes.
    pub fn to_bytes(&self) -> [u8; SECRET_KEY_SIZE] {
        self.0.to_bytes()
    }
}

/// P-224 (secp224r1) ECDSA public key.
///
/// Supports signature verification and ECDH key agreement. Both compressed
/// (29-byte) and uncompressed (57-byte) SEC1 encodings are accepted on input;
/// use [`to_compressed_bytes`](Self::to_compressed_bytes) to export compressed.
///
/// # Verification
///
/// ```ignore
/// use crypto::p224::SecretKey;
///
/// let key = SecretKey::generate().unwrap();
/// let signature = key.sign(b"message").unwrap();
/// assert!(key.public_key().verify(b"message", &signature).is_ok());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PublicKey(p_curves::PublicKey<P224>);

impl PublicKey {
    /// Parses a compressed (29-byte) or uncompressed (57-byte) SEC1 encoding.
    ///
    /// Returns [`EllipticCurveError::InvalidKey`] when the encoding is not a
    /// valid point on the P-224 curve.
    #[inline]
    pub fn from_bytes(key: &[u8]) -> Result<PublicKey, EllipticCurveError> {
        Ok(PublicKey(p_curves::PublicKey::from_bytes(key)?))
    }

    /// Builds a public key from raw affine x and y coordinates (both
    /// big-endian, 28 bytes each). Returns `InvalidKey` if the coordinates
    /// are not a valid point on the P-224 curve.
    ///
    /// This is useful when importing keys from formats like JWK where `x`
    /// and `y` are available directly.
    #[inline]
    pub fn from_x_y(x_bytes: &[u8; 28], y_bytes: &[u8; 28]) -> Result<PublicKey, EllipticCurveError> {
        Ok(PublicKey(p_curves::PublicKey::from_x_y(x_bytes, y_bytes)?))
    }

    /// Verifies an ECDSA signature over `message` using SHA-256 (truncated to
    /// the leftmost 224 bits).
    ///
    /// Both canonical low-s and non-canonical high-s signatures are accepted,
    /// matching typical ECDSA interoperability. Use [`Self::verify_strict`] to
    /// additionally reject high-s signatures.
    pub fn verify(&self, message: &[u8], signature: &[u8; SIGNATURE_SIZE]) -> Result<(), EllipticCurveError> {
        self.0.verify(message, signature)
    }

    /// Verifies an ECDSA signature over `message` using SHA-256, additionally
    /// rejecting non-canonical high-s signatures (`s > n / 2`).
    ///
    /// Use this when signature malleability must be excluded (Bitcoin/EIP-2
    /// style). Note that some third-party signers emit high-s signatures, which
    /// this method will reject; [`Self::verify`] accepts both forms.
    pub fn verify_strict(&self, message: &[u8], signature: &[u8; SIGNATURE_SIZE]) -> Result<(), EllipticCurveError> {
        self.0.verify_strict(message, signature)
    }

    /// Returns the 57-byte uncompressed SEC1 encoding (`0x04 || x || y`).
    #[inline]
    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_UNCOMPRESSED_SIZE] {
        self.0.to_bytes()
    }

    /// Returns the 29-byte compressed SEC1 encoding (`0x02`/`0x03 || x`).
    #[inline]
    pub fn to_compressed_bytes(&self) -> [u8; PUBLIC_KEY_COMPRESSED_SIZE] {
        self.0.to_compressed_bytes()
    }

    /// Returns the `X` and `Y` points as big-endian arrays.
    #[inline]
    pub fn x_y(&self) -> ([u8; 28], [u8; 28]) {
        self.0.x_y()
    }
}

/// Marker type carrying the P-224 curve parameters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct P224;

type U224 = Uint<224, 4>;
type CurveUint = U224;

const MODULUS_P: CurveUint = CurveUint::from_limbs([
    0x0000_0000_0000_0001,
    0xffff_ffff_0000_0000,
    0xffff_ffff_ffff_ffff,
    0x0000_0000_ffff_ffff,
]);

const MODULUS_N: CurveUint = CurveUint::from_limbs([
    0x13dd_2945_5c5c_2a3d,
    0xffff_16a2_e0b8_f03e,
    0xffff_ffff_ffff_ffff,
    0x0000_0000_ffff_ffff,
]);

// floor(n / 2). A signature scalar `s` is "high" when `s > N_HALF`, in which
// case it is normalized to `n - s` so that signatures are non-malleable.
const N_HALF: CurveUint = CurveUint::from_limbs([
    0x09ee_94a2_ae2e_151e,
    0xffff_8b51_705c_781f,
    0xffff_ffff_ffff_ffff,
    0x0000_0000_7fff_ffff,
]);

const P_MINUS_TWO: CurveUint = CurveUint::from_limbs([
    0xffff_ffff_ffff_ffff,
    0xffff_fffe_ffff_ffff,
    0xffff_ffff_ffff_ffff,
    0x0000_0000_ffff_ffff,
]);

const N_MINUS_TWO: CurveUint = CurveUint::from_limbs([
    0x13dd_2945_5c5c_2a3b,
    0xffff_16a2_e0b8_f03e,
    0xffff_ffff_ffff_ffff,
    0x0000_0000_ffff_ffff,
]);

// P-224 has p = 2^224 - 2^96 + 1, so p - 1 = 2^S * Q with S = 96 and
// Q = 2^128 - 1 odd. These constants drive the Tonelli-Shanks square-root
// algorithm (see `p224_field_sqrt`).
const TONELLI_S: usize = 96;
// (Q - 1) / 2.
const TONELLI_Q_MINUS_ONE_OVER_TWO: CurveUint = CurveUint::from_limbs([
    0xffff_ffff_ffff_ffff,
    0x7fff_ffff_ffff_ffff,
    0x0000_0000_0000_0000,
    0x0000_0000_0000_0000,
]);
// Generator of the order-2^S subgroup: `11^Q`, where 11 is the smallest
// quadratic non-residue modulo p (verified: 11^((p-1)/2) = -1). This is the
// fixed `z` used by the constant-time Tonelli-Shanks variant.
const TONELLI_ROOT_OF_UNITY: CurveUint = CurveUint::from_limbs([
    0xf3fb_3632_dc69_1b74,
    0x0b2d_6ffb_bea3_d8ce,
    0x8598_a792_0c55_b2d4,
    0x0000_0000_6a0f_ec67,
]);

// Barrett reduction constants: mu = floor(2^(2 * 4 * 64) / modulus), as
// computed by big_number's `compute_mu_for_barrett`.
const P_MU: [u64; 5] = [
    0x0000_0000_ffff_ffff,
    0xffff_ffff_ffff_ffff,
    0x0000_0000_ffff_ffff,
    0x0000_0000_0000_0000,
    0x0000_0001_0000_0000,
];

const N_MU: [u64; 5] = [
    0xd4ba_a4cf_1822_bc47,
    0xec22_d6ba_a3a3_d5c3,
    0x0000_e95d_1f47_0fc1,
    0x0000_0000_0000_0000,
    0x0000_0001_0000_0000,
];

const CURVE_B: CurveUint = CurveUint::from_limbs([
    0x270b_3943_2355_ffb4,
    0x5044_b0b7_d7bf_d8ba,
    0x0c04_b3ab_f541_3256,
    0x0000_0000_b405_0a85,
]);

const GENERATOR_X: CurveUint = CurveUint::from_limbs([
    0x3432_80d6_115c_1d21,
    0x4a03_c1d3_56c2_1122,
    0x6bb4_bf7f_3213_90b9,
    0x0000_0000_b70e_0cbd,
]);

const GENERATOR_Y: CurveUint = CurveUint::from_limbs([
    0x44d5_8199_8500_7e34,
    0xcd43_75a0_5a07_4764,
    0xb5f7_23fb_4c22_dfe6,
    0x0000_0000_bd37_6388,
]);

// Reduce a big-endian value modulo n, accepting zero and values >= n.
#[inline]
fn reduce_mod(value: CurveUint) -> CurveUint {
    let (sub_value, _) = value.sub_raw(&MODULUS_N);
    CurveUint::ct_select(&sub_value, &value, value.ct_ge(&MODULUS_N))
}

/// Square root modulo p. Returns `None` when `a` is a quadratic non-residue.
///
/// P-224 has p ≡ 1 (mod 4), so the simple `a^((p+1)/4)` formula used by
/// P-256/P-384 is unavailable. Since `p - 1 = 2^96 * Q`, this uses the
/// constant-time Tonelli-Shanks variant from "Square root computation over even
/// extension fields" (eprint 2012/685, Algorithm 5): every loop has a fixed
/// iteration count and all state transitions are constant-time selects, so the
/// running time does not depend on `a`. (The returned `Option` still reveals
/// whether `a` is a quadratic residue, as the signature implies.)
fn p224_field_sqrt(a: CurveUint) -> Option<CurveUint> {
    // w = a^((Q-1)/2); then x = a*w and b = x*w = a^Q.
    let w = field_pow::<P224>(a, &TONELLI_Q_MINUS_ONE_OVER_TWO);
    let mut v = TONELLI_S;
    let mut x = curve_field_mul(&w, &a);
    let mut b = curve_field_mul(&x, &w);

    // z starts as the generator of the order-2^S subgroup.
    let mut z = TONELLI_ROOT_OF_UNITY;

    for max_v in (1..=TONELLI_S).rev() {
        let mut k = 1usize;
        let mut b2k = curve_field_mul(&b, &b);
        let mut j_less_than_v = true;

        for j in 2..max_v {
            let b2k_is_one = b2k.ct_eq(&CurveUint::ONE);
            // Square `z` when `b^(2^k) == 1`, otherwise keep squaring `b2k`.
            let squared = curve_field_mul(
                &CurveUint::ct_select(&z, &b2k, b2k_is_one),
                &CurveUint::ct_select(&z, &b2k, b2k_is_one),
            );
            b2k = CurveUint::ct_select(&b2k, &squared, b2k_is_one);
            let new_z = CurveUint::ct_select(&squared, &z, b2k_is_one);
            j_less_than_v &= j != v;
            k = ct_select_usize(k, j, b2k_is_one);
            z = CurveUint::ct_select(&new_z, &z, j_less_than_v);
        }

        let result = curve_field_mul(&x, &z);
        x = CurveUint::ct_select(&x, &result, b.ct_eq(&CurveUint::ONE));
        z = curve_field_mul(&z, &z);
        b = curve_field_mul(&b, &z);
        v = k;
    }

    if curve_field_mul(&x, &x).ct_eq(&a) {
        Some(x)
    } else {
        None
    }
}

/// Branch-free select for `usize` indices: returns `a` if `choice` else `b`.
#[inline]
fn ct_select_usize(a: usize, b: usize, choice: bool) -> usize {
    let mask = (choice as usize).wrapping_neg();
    (a & mask) | (b & !mask)
}

#[inline]
fn curve_field_mul(a: &CurveUint, b: &CurveUint) -> CurveUint {
    a.mul_mod_barrett(b, &MODULUS_P, &P_MU)
}

impl Curve for P224 {
    type U = CurveUint;
    type FieldBytes = [u8; SECRET_KEY_SIZE];
    type DigestBytes = [u8; 32];
    type CompressedBytes = [u8; PUBLIC_KEY_COMPRESSED_SIZE];
    type UncompressedBytes = [u8; PUBLIC_KEY_UNCOMPRESSED_SIZE];
    type SignatureBytes = [u8; SIGNATURE_SIZE];

    const FIELD_BITS: usize = 224;
    const FIELD_BYTES: usize = SECRET_KEY_SIZE;
    const DIGEST_BYTES: usize = 32;
    const COMPRESSED_BYTES: usize = PUBLIC_KEY_COMPRESSED_SIZE;
    const UNCOMPRESSED_BYTES: usize = PUBLIC_KEY_UNCOMPRESSED_SIZE;

    const MODULUS_P: CurveUint = MODULUS_P;
    const MODULUS_N: CurveUint = MODULUS_N;
    const N_HALF: CurveUint = N_HALF;
    const P_MINUS_TWO: CurveUint = P_MINUS_TWO;
    const N_MINUS_TWO: CurveUint = N_MINUS_TWO;
    const CURVE_B: CurveUint = CURVE_B;
    const GENERATOR_X: CurveUint = GENERATOR_X;
    const GENERATOR_Y: CurveUint = GENERATOR_Y;

    #[inline]
    fn field_mul(a: &Self::U, b: &Self::U) -> Self::U {
        a.mul_mod_barrett(b, &MODULUS_P, &P_MU)
    }

    #[inline]
    fn scalar_mul(a: &Self::U, b: &Self::U) -> Self::U {
        a.mul_mod_barrett(b, &MODULUS_N, &N_MU)
    }

    #[inline]
    fn field_sqrt(a: Self::U) -> Option<Self::U> {
        p224_field_sqrt(a)
    }

    #[inline]
    fn hash(data: &[u8]) -> Self::DigestBytes {
        Sha256::hash(data).as_ref().try_into().unwrap()
    }

    #[inline]
    fn hmac(key: &[u8], data: &[u8]) -> Self::DigestBytes {
        Hmac::<Sha256>::mac(key, data).as_ref().try_into().unwrap()
    }

    /// Reduce a SHA-256 digest to a scalar. FIPS 186-4 truncates the digest to
    /// the leftmost 224 bits (the first 28 bytes) before reduction.
    #[inline]
    fn scalar_from_digest(digest: &Self::DigestBytes) -> Self::U {
        let truncated: [u8; SECRET_KEY_SIZE] = digest.as_ref()[..SECRET_KEY_SIZE].try_into().unwrap();
        reduce_mod(CurveUint::read_be(&truncated))
    }

    #[inline]
    fn scalar_from_field_bytes(bytes: &Self::FieldBytes) -> Self::U {
        reduce_mod(CurveUint::read_be(bytes.as_ref()))
    }

    #[cfg(feature = "random")]
    #[inline]
    fn random_secret_key() -> Self::FieldBytes {
        crate::random::random_bytes()
    }
}

/// Returns `true` when `public_key` is a valid SEC1 encoding of a point on the
/// P-224 curve.
pub fn is_valid_public_key(public_key: &[u8]) -> bool {
    p_curves::is_valid_public_key::<P224>(public_key)
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    use super::*;
    use crate::p_curves::p_curves;

    type FieldElement = p_curves::FieldElement<P224>;
    type Scalar = p_curves::Scalar<P224>;
    type AffinePoint = p_curves::AffinePoint<P224>;
    type ProjectivePoint = p_curves::ProjectivePoint<P224>;
    type Rfc6979 = p_curves::Rfc6979<P224>;

    // Concrete wrappers around the generic core helpers.
    fn hash_message(message: &[u8]) -> [u8; 32] {
        p_curves::hash_message::<P224>(message)
    }

    fn hmac_digest(key: &[u8], data: &[u8]) -> [u8; 32] {
        p_curves::hmac_digest::<P224>(key, data)
    }

    fn bits2octets(hash: &[u8; 32]) -> [u8; SECRET_KEY_SIZE] {
        p_curves::bits2octets::<P224>(hash)
    }

    fn derive_public_key_uncompressed(
        private_key: &[u8; SECRET_KEY_SIZE],
    ) -> Result<[u8; PUBLIC_KEY_UNCOMPRESSED_SIZE], crate::EllipticCurveError> {
        p_curves::derive_public_key_uncompressed::<P224>(private_key)
    }

    fn derive_public_key_compressed(
        private_key: &[u8; SECRET_KEY_SIZE],
    ) -> Result<[u8; PUBLIC_KEY_COMPRESSED_SIZE], crate::EllipticCurveError> {
        p_curves::derive_public_key_compressed::<P224>(private_key)
    }

    fn ecdsa_sign_inner_impl(
        scalar: &Scalar,
        message: &[u8],
        force_first_retry: bool,
    ) -> Result<[u8; SIGNATURE_SIZE], crate::EllipticCurveError> {
        p_curves::ecdsa_sign_inner_impl::<P224>(scalar, message, force_first_retry)
    }

    fn ecdsa_verify_inner(
        public_point: &AffinePoint,
        message: &[u8],
        signature: &[u8; SIGNATURE_SIZE],
    ) -> Result<(), crate::EllipticCurveError> {
        p_curves::ecdsa_verify_inner::<P224>(public_point, message, signature)
    }

    fn parse_public_key(public_key: &[u8]) -> Result<AffinePoint, crate::EllipticCurveError> {
        p_curves::parse_public_key::<P224>(public_key)
    }

    fn scalar_mul_generator(scalar: &Scalar) -> ProjectivePoint {
        p_curves::scalar_mul_generator::<P224>(scalar)
    }

    fn scalar_mul_affine(base: &AffinePoint, scalar: &Scalar) -> ProjectivePoint {
        p_curves::scalar_mul_affine::<P224>(base, scalar)
    }

    // Raw-byte ECDH built from the public method, for the vector tests.
    fn ecdh(
        secret_key: &[u8; SECRET_KEY_SIZE],
        peer_public_key: &[u8],
    ) -> Result<[u8; ECDH_SHARED_SECRET_SIZE], crate::EllipticCurveError> {
        SecretKey::from_bytes(secret_key)?.ecdh(&PublicKey::from_bytes(peer_public_key)?)
    }

    #[cfg(feature = "zeroize")]
    #[test]
    fn secret_key_zeroize_clears_scalar() {
        use zeroize::Zeroize;

        let mut key = SecretKey::from_bytes(&[1u8; SECRET_KEY_SIZE]).unwrap();
        assert_ne!(key.to_bytes(), [0u8; SECRET_KEY_SIZE]);
        key.zeroize();
        assert_eq!(key.to_bytes(), [0u8; SECRET_KEY_SIZE]);
    }

    fn decode_hex<const N: usize>(hex_bytes: &str) -> [u8; N] {
        let bytes = hex::decode(hex_bytes).unwrap();
        assert_eq!(bytes.len(), N);
        let mut out = [0u8; N];
        out.copy_from_slice(&bytes);
        out
    }

    // Read a DER TLV (tag-length-value) item.
    fn der_read_tlv<'a>(data: &'a [u8], offset: &mut usize) -> Option<(u8, &'a [u8])> {
        if *offset >= data.len() {
            return None;
        }
        let tag = data[*offset];
        *offset += 1;
        if *offset >= data.len() {
            return None;
        }
        let len_byte = data[*offset];
        *offset += 1;
        let (len, _) = if len_byte & 0x80 != 0 {
            let num_bytes = (len_byte & 0x7f) as usize;
            if num_bytes == 0 || num_bytes > core::mem::size_of::<usize>() || *offset + num_bytes > data.len() {
                return None;
            }
            if num_bytes > 1 && data[*offset] == 0 {
                return None;
            }
            let mut l = 0usize;
            for i in 0..num_bytes {
                l = (l << 8) | data[*offset + i] as usize;
            }
            if l < 128 {
                return None;
            }
            *offset += num_bytes;
            (l, num_bytes + 1)
        } else {
            (len_byte as usize, 1)
        };
        if (*offset).checked_add(len).map_or(true, |sum| sum > data.len()) {
            return None;
        }
        let value = &data[*offset..*offset + len];
        *offset = (*offset).checked_add(len)?;
        Some((tag, value))
    }

    // Convert a DER-encoded ECDSA signature (SEQUENCE { INTEGER r, INTEGER s })
    // to P1363 format (r || s, each 28 bytes).
    fn der_ecdsa_sig_to_p1363(der: &[u8]) -> Option<[u8; 56]> {
        let mut offset = 0;
        let (tag, inner) = der_read_tlv(der, &mut offset)?;
        if tag != 0x30 {
            return None;
        }
        if offset != der.len() {
            return None;
        }
        let mut inner_offset = 0;
        let (rtag, rval) = der_read_tlv(inner, &mut inner_offset)?;
        if rtag != 0x02 || rval.is_empty() || rval.len() > 29 {
            return None;
        }
        let (stag, sval) = der_read_tlv(inner, &mut inner_offset)?;
        if stag != 0x02 || sval.is_empty() || sval.len() > 29 {
            return None;
        }
        if inner_offset != inner.len() {
            return None;
        }
        let r_valid = if rval.len() == 28 && rval[0] >= 0x80 {
            false
        } else if rval.len() == 29 && rval[0] != 0 {
            false
        } else if rval.len() == 29 && rval[0] == 0 && rval[1] < 0x80 {
            false
        } else {
            rval.len() <= 29
        };
        let s_valid = if sval.len() == 28 && sval[0] >= 0x80 {
            false
        } else if sval.len() == 29 && sval[0] != 0 {
            false
        } else if sval.len() == 29 && sval[0] == 0 && sval[1] < 0x80 {
            false
        } else {
            sval.len() <= 29
        };
        if !r_valid || !s_valid {
            return None;
        }

        let r_trimmed = if rval.len() == 29 && rval[0] == 0 {
            &rval[1..]
        } else {
            rval
        };
        let s_trimmed = if sval.len() == 29 && sval[0] == 0 {
            &sval[1..]
        } else {
            sval
        };
        if r_trimmed.len() > 28 || s_trimmed.len() > 28 {
            return None;
        }
        let mut sig = [0u8; 56];
        sig[28 - r_trimmed.len()..28].copy_from_slice(r_trimmed);
        sig[56 - s_trimmed.len()..56].copy_from_slice(s_trimmed);
        Some(sig)
    }

    // Extract the raw SEC1 point from a DER SubjectPublicKeyInfo using the
    // named secp224r1 curve (explicit parameters are rejected).
    fn spki_to_sec1_point(spki: &[u8]) -> Option<Vec<u8>> {
        let ec_public_key_oid: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01];
        let secp224r1_oid: &[u8] = &[0x2b, 0x81, 0x04, 0x00, 0x21];
        let mut offset = 0;
        let (_tag, outer) = der_read_tlv(spki, &mut offset)?;
        let mut inner = 0;
        let (_alg_tag, alg_content) = der_read_tlv(outer, &mut inner)?;
        if _alg_tag != 0x30 {
            return None;
        }
        let mut ai = 0;
        let (oid1_tag, oid1) = der_read_tlv(alg_content, &mut ai)?;
        if oid1_tag != 0x06 || oid1 != ec_public_key_oid {
            return None;
        }
        let (oid2_tag, oid2) = der_read_tlv(alg_content, &mut ai)?;
        if oid2_tag != 0x06 || oid2 != secp224r1_oid {
            return None;
        }
        let (_bs_tag, bs_val) = der_read_tlv(outer, &mut inner)?;
        if _bs_tag != 0x03 || bs_val.is_empty() {
            return None;
        }
        Some(bs_val[1..].to_vec())
    }

    // RFC 6979 A.2.4: P-224 test key pair.
    const RFC6979_PRIVATE_KEY: &str = "f220266e1105bfe3083e03ec7a3a654651f45e37167e88600bf257c1";
    const RFC6979_PUBLIC_X: &str = "00cf08da5ad719e42707fa431292dea11244d64fc51610d94b130d6c";
    const RFC6979_PUBLIC_Y: &str = "eeab6f3debe455e3dbf85416f7030cbd94f34f2d6f232c69f3c1385a";

    #[test]
    fn derive_public_key_generator_matches_sec1_base_point() {
        let mut private_key = [0u8; SECRET_KEY_SIZE];
        private_key[27] = 1;
        let derived = derive_public_key_uncompressed(&private_key).unwrap();
        let expected = decode_hex::<57>(
            "04b70e0cbd6bb4bf7f321390b94a03c1d356c21122343280d6115c1d21\
             bd376388b5f723fb4c22dfe6cd4375a05a07476444d5819985007e34",
        );
        assert_eq!(derived, expected);
    }

    #[test]
    fn derive_public_key_matches_rfc6979_vector() {
        let private_key = decode_hex::<28>(RFC6979_PRIVATE_KEY);
        let expected = decode_hex::<57>(
            "0400cf08da5ad719e42707fa431292dea11244d64fc51610d94b130d6c\
             eeab6f3debe455e3dbf85416f7030cbd94f34f2d6f232c69f3c1385a",
        );
        assert_eq!(derive_public_key_uncompressed(&private_key).unwrap(), expected);
        assert_eq!(
            PublicKey::from_x_y(&decode_hex::<28>(RFC6979_PUBLIC_X), &decode_hex::<28>(RFC6979_PUBLIC_Y),)
                .unwrap()
                .to_bytes(),
            expected,
        );

        // Compressed form derives from the same point; prefix encodes y parity.
        let compressed = derive_public_key_compressed(&private_key).unwrap();
        assert_eq!(&compressed[1..], &decode_hex::<28>(RFC6979_PUBLIC_X));
    }

    #[test]
    fn ecdsa_sign_matches_rfc6979_vectors() {
        let private_key = decode_hex::<28>(RFC6979_PRIVATE_KEY);
        let key = SecretKey::from_bytes(&private_key).unwrap();

        // The raw RFC 6979 "sample" signature has high s (bc81…0101); the
        // expected value below is its canonical low-s form (n - s).
        let sample_signature = key.sign(b"sample").unwrap();
        let expected_sample = decode_hex::<56>(
            "61aa3da010e8e8406c656bc477a7a7189895e7e840cdfe8ff42307ba\
             437ebfaf254a2dc88f786b6b061e7022049df927fa4b6b5ec9ab293c",
        );
        assert_eq!(sample_signature, expected_sample);

        let test_signature = key.sign(b"test").unwrap();
        let expected_test = decode_hex::<56>(
            "ad04dde87b84747a243a631ea47a1ba6d1faa059149ad2440de6fba6\
             178d49b1ae90e3d8b629be3db5683915f4e8c99fdf6e666cf37adcfd",
        );
        assert_eq!(test_signature, expected_test);
    }

    #[test]
    fn ecdsa_sign_emits_canonical_low_s() {
        let private_key = decode_hex::<28>(RFC6979_PRIVATE_KEY);
        let key = SecretKey::from_bytes(&private_key).unwrap();
        for msg in [&b"sample"[..], b"test", b"", b"another message"] {
            let sig = key.sign(msg).unwrap();
            let s = Scalar::from_bytes(sig[28..].try_into().unwrap()).unwrap();
            assert!(!s.is_high(), "sign produced a high-s signature for {:?}", msg);
        }
    }

    #[test]
    fn ecdsa_verify_strict_rejects_high_s() {
        let private_key = decode_hex::<28>(RFC6979_PRIVATE_KEY);
        let key = SecretKey::from_bytes(&private_key).unwrap();
        let public_key = key.public_key();
        let signature = key.sign(b"sample").unwrap();

        // Canonical low-s signature passes both permissive and strict verification.
        assert!(public_key.verify(b"sample", &signature).is_ok());
        assert!(public_key.verify_strict(b"sample", &signature).is_ok());

        // The malleable counterpart (r, n - s) is rejected only by strict verify.
        let r = Scalar::from_bytes(signature[..28].try_into().unwrap()).unwrap();
        let s = Scalar::from_bytes(signature[28..].try_into().unwrap()).unwrap();
        let high_s = Scalar::ZERO.sub(s);
        assert!(high_s.is_high());

        let mut malleable = [0u8; SIGNATURE_SIZE];
        malleable[..28].copy_from_slice(&r.to_bytes());
        malleable[28..].copy_from_slice(&high_s.to_bytes());

        assert!(public_key.verify(b"sample", &malleable).is_ok());
        assert!(public_key.verify_strict(b"sample", &malleable).is_err());
    }

    #[test]
    fn rfc6979_nonce_generation_matches_known_value() {
        let private_key = Scalar::from_bytes(&decode_hex::<28>(RFC6979_PRIVATE_KEY)).unwrap();
        let hash = hash_message(b"sample");
        assert_eq!(
            Rfc6979::new(&private_key, &hash).generate().to_bytes(),
            decode_hex::<28>("ad3029e0278f80643de33917ce6908c70a8ff50a411f06e41dedfcdc")
        );
    }

    #[test]
    fn rfc6979_test_message_nonce_matches_known_value() {
        let private_key = Scalar::from_bytes(&decode_hex::<28>(RFC6979_PRIVATE_KEY)).unwrap();
        let hash = hash_message(b"test");
        assert_eq!(
            Rfc6979::new(&private_key, &hash).generate().to_bytes(),
            decode_hex::<28>("ff86f57924da248d6e44e8154eb69f0ae2aebaee9931d0b5a969f904")
        );
    }

    #[test]
    fn rfc6979_retry_advances_drbg() {
        let private_key = Scalar::from_bytes(&decode_hex::<28>(RFC6979_PRIVATE_KEY)).unwrap();
        let hash = hash_message(b"sample");

        let mut drbg = Rfc6979::new(&private_key, &hash);
        let k1 = drbg.generate();
        assert_eq!(
            k1.to_bytes(),
            decode_hex::<28>("ad3029e0278f80643de33917ce6908c70a8ff50a411f06e41dedfcdc")
        );

        // RFC 6979 §3.2 h.3: an r=0/s=0 retry must continue the DRBG, not
        // reproduce the same nonce.
        drbg.retry();
        let k2 = drbg.generate();
        assert_ne!(k1.to_bytes(), k2.to_bytes());
        assert_eq!(
            k2.to_bytes(),
            decode_hex::<28>("e651536d86136a3b2a48606e067796dd9b8586698a271d594aeb0255")
        );
    }

    #[test]
    fn ecdsa_forced_retry_uses_fresh_nonce() {
        let private_key = decode_hex::<28>(RFC6979_PRIVATE_KEY);
        let key = SecretKey::from_bytes(&private_key).unwrap();
        let scalar = Scalar::from_bytes(&private_key).unwrap();

        let normal = key.sign(b"sample").unwrap();
        // Force the r=0/s=0 retry path. The signature must be produced with the
        // next DRBG nonce; repeating the first nonce would yield `normal`.
        let retried = ecdsa_sign_inner_impl(&scalar, b"sample", true).unwrap();

        assert_ne!(normal, retried);
        assert!(key.public_key().verify(b"sample", &retried).is_ok());
    }

    #[test]
    fn rfc6979_bits2octets_truncates_and_reduces() {
        // SHA-256("sample") truncated to the leftmost 224 bits.
        let hash = hash_message(b"sample");
        let expected = decode_hex::<28>("af2bdbe1aa9b6ec1e2ade1d694f41fc71a831d0268e9891562113d8a");
        assert_eq!(bits2octets(&hash), expected);

        // A value exactly equal to n maps to zero; n+1 maps to one.
        // n in the top 224 bits maps to zero; n+1 maps to one.
        let n_times = decode_hex::<32>("ffffffffffffffffffffffffffff16a2e0b8f03e13dd29455c5c2a3d00000000");
        assert_eq!(bits2octets(&n_times), [0u8; 28]);
        let n_plus_one = decode_hex::<32>("ffffffffffffffffffffffffffff16a2e0b8f03e13dd29455c5c2a3e00000000");
        assert_eq!(
            bits2octets(&n_plus_one),
            decode_hex::<28>("00000000000000000000000000000000000000000000000000000001")
        );
    }

    #[test]
    fn ecdsa_verify_accepts_compressed_and_uncompressed_public_keys() {
        let private_key = decode_hex::<28>(RFC6979_PRIVATE_KEY);
        let key = SecretKey::from_bytes(&private_key).unwrap();
        let uncompressed = key.public_key();
        let compressed = derive_public_key_compressed(&private_key).unwrap();
        let signature = key.sign(b"sample").unwrap();

        assert!(uncompressed.verify(b"sample", &signature).is_ok());
        let point = AffinePoint::from_sec1_bytes(&compressed).unwrap();
        assert!(ecdsa_verify_inner(&point, b"sample", &signature).is_ok());
    }

    #[test]
    fn verify_rejects_tampering_and_invalid_points() {
        let private_key = decode_hex::<28>(RFC6979_PRIVATE_KEY);
        let key = SecretKey::from_bytes(&private_key).unwrap();
        let pub_key = key.public_key();
        let mut off_curve = [0u8; 57];
        off_curve.copy_from_slice(&pub_key.to_bytes());
        let signature = key.sign(b"sample").unwrap();

        assert!(pub_key.verify(b"tampered", &signature).is_err());

        let mut bad_signature = signature;
        bad_signature[10] ^= 0x80;
        assert!(pub_key.verify(b"sample", &bad_signature).is_err());

        off_curve[56] ^= 0x01;
        assert!(!is_valid_public_key(&off_curve));
        assert!(PublicKey::from_bytes(&off_curve).is_err());
    }

    #[test]
    fn invalid_inputs_are_rejected() {
        let invalid_private_key = [0u8; SECRET_KEY_SIZE];
        assert!(SecretKey::from_bytes(&invalid_private_key).is_err());
        assert!(derive_public_key_uncompressed(&invalid_private_key).is_err());
        assert!(derive_public_key_compressed(&invalid_private_key).is_err());

        let private_key = decode_hex::<28>(RFC6979_PRIVATE_KEY);
        let key = SecretKey::from_bytes(&private_key).unwrap();
        let signature = key.sign(b"msg").unwrap();
        let mut zero_r = signature;
        zero_r[..28].fill(0);
        assert!(key.public_key().verify(b"msg", &zero_r).is_err());
    }

    #[test]
    fn public_key_validation_accepts_known_good_points() {
        assert!(is_valid_public_key(&decode_hex::<57>(
            "04b70e0cbd6bb4bf7f321390b94a03c1d356c21122343280d6115c1d21\
             bd376388b5f723fb4c22dfe6cd4375a05a07476444d5819985007e34"
        )));
    }

    #[test]
    fn scalar_from_bytes_rejects_boundary_values() {
        assert!(Scalar::from_bytes(&[0u8; 28]).is_none());

        // n is rejected (must be strictly less than n)
        let n_bytes = decode_hex::<28>("ffffffffffffffffffffffffffff16a2e0b8f03e13dd29455c5c2a3d");
        assert!(Scalar::from_bytes(&n_bytes).is_none());

        // n-1 is accepted
        let n_minus_1 = decode_hex::<28>("ffffffffffffffffffffffffffff16a2e0b8f03e13dd29455c5c2a3c");
        assert!(Scalar::from_bytes(&n_minus_1).is_some());

        let one = decode_hex::<28>("00000000000000000000000000000000000000000000000000000001");
        assert!(Scalar::from_bytes(&one).is_some());
    }

    #[test]
    fn field_element_from_bytes_rejects_boundary_values() {
        // p is rejected (must be strictly less than p)
        let p_bytes = decode_hex::<28>("ffffffffffffffffffffffffffffffff000000000000000000000001");
        assert!(FieldElement::from_bytes(&p_bytes).is_none());

        // p-1 is accepted
        let p_minus_1 = decode_hex::<28>("ffffffffffffffffffffffffffffffff000000000000000000000000");
        assert!(FieldElement::from_bytes(&p_minus_1).is_some());

        assert!(FieldElement::from_bytes(&[0u8; 28]).is_some());
    }

    #[test]
    fn point_decompression_round_trip() {
        let keys: &[&str] = &[
            "00000000000000000000000000000000000000000000000000000001",
            "00000000000000000000000000000000000000000000000000000002",
            RFC6979_PRIVATE_KEY,
            "a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7",
        ];

        for key_hex in keys {
            let private_key = decode_hex::<28>(key_hex);
            let key = SecretKey::from_bytes(&private_key).unwrap();
            let uncompressed = key.public_key();
            let compressed = derive_public_key_compressed(&private_key).unwrap();

            let sig = key.sign(b"round-trip").unwrap();
            assert!(uncompressed.verify(b"round-trip", &sig).is_ok());
            let point = AffinePoint::from_sec1_bytes(&compressed).unwrap();
            assert!(ecdsa_verify_inner(&point, b"round-trip", &sig).is_ok());

            let point = AffinePoint::from_sec1_bytes(&compressed).unwrap();
            assert_eq!(point.to_uncompressed_bytes(), uncompressed.to_bytes());
        }
    }

    #[test]
    fn public_key_compressed_round_trip() {
        for _ in 0..5 {
            let key = SecretKey::generate().unwrap();
            let pub_key = key.public_key();
            let compressed = pub_key.to_compressed_bytes();
            assert_eq!(compressed.len(), PUBLIC_KEY_COMPRESSED_SIZE);

            let decoded = PublicKey::from_bytes(&compressed).unwrap();
            assert_eq!(decoded, pub_key);
            assert_eq!(decoded.to_bytes(), pub_key.to_bytes());
        }
    }

    #[test]
    fn sqrt_matches_squares_and_rejects_non_residues() {
        // Squares always have a root that squares back to the input, and that
        // root is one of the two square roots (±x).
        for _ in 0..500 {
            let bytes: [u8; 28] = rand::random();
            let Some(x) = FieldElement::from_bytes(&bytes) else {
                continue;
            };
            let square = x.square();
            let root = square.sqrt().expect("square should have a square root");
            assert_eq!(root.square(), square);
            assert!(root == x || root == x.negate());

            // The computation is deterministic.
            assert_eq!(root, square.sqrt().unwrap());
        }

        // Zero is its own square root.
        let zero = FieldElement::from_uint(U224::from_u64(0));
        assert_eq!(zero.sqrt().unwrap(), zero);

        // 11 is the smallest quadratic non-residue modulo p.
        let non_residue = FieldElement::from_uint(U224::from_u64(11));
        assert!(non_residue.sqrt().is_none());
    }

    #[test]
    fn scalar_inversion_correctness() {
        let k =
            Scalar::from_bytes(&decode_hex::<28>("ad3029e0278f80643de33917ce6908c70a8ff50a411f06e41dedfcdc")).unwrap();
        let k_inv = k.invert().unwrap();
        assert_eq!(k.mul(k_inv), Scalar::ONE);
    }

    #[test]
    fn field_element_inversion_correctness() {
        let x = FieldElement::from_bytes(&decode_hex::<28>("b70e0cbd6bb4bf7f321390b94a03c1d356c21122343280d6115c1d21"))
            .unwrap();
        let x_inv = x.invert().unwrap();
        assert_eq!(x.mul(x_inv), FieldElement::ONE);
    }

    #[test]
    fn barrett_mul_matches_generic() {
        // Verify the Barrett-based field and scalar multiplications match the
        // generic (bit-serial) Uint::mul_mod.
        for _ in 0..1000 {
            let a_bytes: [u8; 28] = rand::random();
            let b_bytes: [u8; 28] = rand::random();
            let (Some(a), Some(b)) = (FieldElement::from_bytes(&a_bytes), FieldElement::from_bytes(&b_bytes)) else {
                continue;
            };
            assert_eq!(a.mul(b).0, a.0.mul_mod(&b.0, &MODULUS_P), "field mul mismatch");

            let (Some(c), Some(d)) = (Scalar::from_bytes(&a_bytes), Scalar::from_bytes(&b_bytes)) else {
                continue;
            };
            assert_eq!(c.mul(d).0, c.0.mul_mod(&d.0, &MODULUS_N), "scalar mul mismatch");
        }
    }

    #[test]
    fn scalar_mul_generator_n_gives_identity() {
        // (n-1)*G = -G, so the x coordinate matches and y is negated.
        let n_minus_1 =
            Scalar::from_bytes(&decode_hex::<28>("ffffffffffffffffffffffffffff16a2e0b8f03e13dd29455c5c2a3c")).unwrap();
        let result = scalar_mul_generator(&n_minus_1).to_affine().unwrap();
        assert_eq!(result.x, FieldElement::from_uint(GENERATOR_X));
        assert_eq!(result.y, FieldElement::from_uint(GENERATOR_Y).negate());
    }

    #[test]
    fn ecdh_deterministic_vector_against_generator() {
        // ECDH between the RFC 6979 private key and the generator: the shared
        // secret is the x-coordinate of the derived public key.
        let private_key = decode_hex::<28>(RFC6979_PRIVATE_KEY);
        let generator = decode_hex::<57>(
            "04b70e0cbd6bb4bf7f321390b94a03c1d356c21122343280d6115c1d21\
             bd376388b5f723fb4c22dfe6cd4375a05a07476444d5819985007e34",
        );

        let key = SecretKey::from_bytes(&private_key).unwrap();
        let shared = ecdh(&private_key, &generator).unwrap();
        assert_eq!(shared, key.public_key().x_y().0);
        assert_eq!(shared, decode_hex::<28>(RFC6979_PUBLIC_X));
    }

    #[test]
    fn ecdh_with_compressed_public_key() {
        let alice = SecretKey::generate().unwrap();
        let bob = SecretKey::generate().unwrap();
        let bob_compressed = derive_public_key_compressed(&bob.to_bytes()).unwrap();

        assert!(is_valid_public_key(&bob_compressed));
        let shared = alice.ecdh(&PublicKey::from_bytes(&bob_compressed).unwrap()).unwrap();
        let expected = bob.ecdh(&alice.public_key()).unwrap();
        assert_eq!(shared, expected);
    }

    #[test]
    fn ecdh_rejects_invalid_private_key_zero_and_order() {
        let zero_key = [0u8; 28];
        assert!(SecretKey::from_bytes(&zero_key).is_err());
        let bob = SecretKey::generate().unwrap();
        assert!(ecdh(&zero_key, &bob.public_key().to_bytes()).is_err());

        let n_bytes = decode_hex::<28>("ffffffffffffffffffffffffffff16a2e0b8f03e13dd29455c5c2a3d");
        assert!(SecretKey::from_bytes(&n_bytes).is_err());
    }

    #[test]
    fn ecdh_standalone_function_matches_method() {
        let alice = SecretKey::generate().unwrap();
        let bob = SecretKey::generate().unwrap();

        let method_result = alice.ecdh(&bob.public_key()).unwrap();
        let standalone_result = ecdh(&alice.to_bytes(), &bob.public_key().to_bytes()).unwrap();
        assert_eq!(method_result, standalone_result);
    }

    #[test]
    fn ecdsa_sign_verify_round_trip_multiple_messages() {
        let key = SecretKey::generate().unwrap();
        let pub_key = key.public_key();

        let messages: &[&[u8]] = &[
            b"",
            b"hello world",
            b"The quick brown fox jumps over the lazy dog",
            &[0xffu8; 100],
            b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f",
        ];

        for msg in messages {
            let sig = key.sign(msg).unwrap();
            assert!(pub_key.verify(msg, &sig).is_ok(), "round-trip failed for message {msg:?}");
            let mut wrong_msg = msg.to_vec();
            wrong_msg.push(0x42);
            assert!(pub_key.verify(&wrong_msg, &sig).is_err());
        }
    }

    #[test]
    fn ecdsa_verify_wrong_public_key_rejects() {
        let key1 = SecretKey::generate().unwrap();
        let key2 = SecretKey::generate().unwrap();

        let sig = key1.sign(b"message").unwrap();
        assert!(key2.public_key().verify(b"message", &sig).is_err());
    }

    #[test]
    fn ecdsa_rejects_non_canonical_r_and_s() {
        let key = SecretKey::generate().unwrap();
        let mut bad_r = key.sign(b"msg").unwrap();
        bad_r[..28].copy_from_slice(&decode_hex::<28>("ffffffffffffffffffffffffffff16a2e0b8f03e13dd29455c5c2a3e"));
        assert!(key.public_key().verify(b"msg", &bad_r).is_err());

        let mut bad_s = key.sign(b"msg").unwrap();
        bad_s[28..].copy_from_slice(&decode_hex::<28>("ffffffffffffffffffffffffffff16a2e0b8f03e13dd29455c5c2a3e"));
        assert!(key.public_key().verify(b"msg", &bad_s).is_err());
    }

    #[test]
    fn from_x_y_matches_generator() {
        let key = PublicKey::from_x_y(
            &decode_hex::<28>("b70e0cbd6bb4bf7f321390b94a03c1d356c21122343280d6115c1d21"),
            &decode_hex::<28>("bd376388b5f723fb4c22dfe6cd4375a05a07476444d5819985007e34"),
        )
        .unwrap();
        let from_sec1 = PublicKey::from_bytes(&key.to_bytes()).unwrap();
        assert_eq!(key, from_sec1);
    }

    #[test]
    fn x_y_round_trip() {
        let key = SecretKey::generate().unwrap();
        let pub_key = key.public_key();
        let (x, y) = pub_key.x_y();
        let pub_key2 = PublicKey::from_x_y(&x, &y).unwrap();
        assert_eq!(pub_key.to_bytes(), pub_key2.to_bytes());
    }

    #[test]
    fn field_element_add_sub_mul_consistency() {
        let a = FieldElement::from_bytes(&decode_hex::<28>("b70e0cbd6bb4bf7f321390b94a03c1d356c21122343280d6115c1d21"))
            .unwrap();
        let b = FieldElement::from_bytes(&decode_hex::<28>("bd376388b5f723fb4c22dfe6cd4375a05a07476444d5819985007e34"))
            .unwrap();

        assert_eq!(a.add(b).sub(b), a);
        assert_eq!(a.add(b), b.add(a));
        assert_eq!(a.mul(b), b.mul(a));

        let c = FieldElement::from_bytes(&decode_hex::<28>("270b39432355ffb45044b0b7d7bfd8ba0c04b3abf5413256b4050a85"))
            .unwrap();
        assert_eq!(a.add(b).mul(c), a.mul(c).add(b.mul(c)));
    }

    #[test]
    fn scalar_add_sub_mul_consistency() {
        let a =
            Scalar::from_bytes(&decode_hex::<28>("ad3029e0278f80643de33917ce6908c70a8ff50a411f06e41dedfcdc")).unwrap();
        let one =
            Scalar::from_bytes(&decode_hex::<28>("00000000000000000000000000000000000000000000000000000001")).unwrap();

        assert_eq!(a.add(one).sub(one), a);
        assert_eq!(a.mul(one), a);

        let b =
            Scalar::from_bytes(&decode_hex::<28>("178d49b1ae90e3d8b629be3db5683915f4e8c99fdf6e666cf37adcfd")).unwrap();
        assert_eq!(a.mul(b), b.mul(a));
        assert_eq!(a.add(b), b.add(a));
    }

    #[test]
    fn field_element_negate_round_trip() {
        let x = FieldElement::from_bytes(&decode_hex::<28>("b70e0cbd6bb4bf7f321390b94a03c1d356c21122343280d6115c1d21"))
            .unwrap();
        let neg = x.negate();
        assert_eq!(neg.negate(), x);
        assert_eq!(x.add(neg), FieldElement::ZERO);
    }

    #[test]
    fn field_element_pow_correctness() {
        let x = FieldElement::from_bytes(&decode_hex::<28>("00000000000000000000000000000000000000000000000000000002"))
            .unwrap();
        assert_eq!(x.pow(&U224::from_u64(3)), x.mul(x).mul(x));
        assert_eq!(x.pow(&U224::ZERO), FieldElement::ONE);
    }

    #[test]
    fn wycheproof_ecdsa_p224_sha256_p1363() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/ecdsa_secp224r1_sha256_p1363_test.json"
        ))
        .unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            let uncompressed_hex = group["publicKey"]["uncompressed"].as_str().unwrap();
            let pubkey_bytes = hex::decode(uncompressed_hex).unwrap();
            let pk = PublicKey::from_bytes(&pubkey_bytes).unwrap();

            for test in group["tests"].as_array().unwrap() {
                let msg = hex::decode(test["msg"].as_str().unwrap()).unwrap();
                let sig_hex = test["sig"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                if sig_hex.len() != SIGNATURE_SIZE * 2 {
                    continue;
                }
                let sig = decode_hex::<SIGNATURE_SIZE>(sig_hex);
                let verify_result = pk.verify(&msg, &sig);

                if result == "valid" {
                    assert!(verify_result.is_ok(), "wycheproof ECDSA P1363 tcId={}", test["tcId"]);
                    valid_tested += 1;
                } else {
                    assert!(verify_result.is_err(), "wycheproof ECDSA P1363 tcId={}", test["tcId"]);
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid ECDSA P1363 wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid ECDSA P1363 wycheproof tests were run");
    }

    #[test]
    fn wycheproof_ecdsa_p224_sha256_der() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/ecdsa_secp224r1_sha256_test.json"
        ))
        .unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            let uncompressed_hex = group["publicKey"]["uncompressed"].as_str().unwrap();
            let pubkey_bytes = hex::decode(uncompressed_hex).unwrap();
            let pk = PublicKey::from_bytes(&pubkey_bytes).unwrap();

            for test in group["tests"].as_array().unwrap() {
                let msg = hex::decode(test["msg"].as_str().unwrap()).unwrap();
                let der_sig = hex::decode(test["sig"].as_str().unwrap()).unwrap();
                let result = test["result"].as_str().unwrap();

                let Some(sig) = der_ecdsa_sig_to_p1363(&der_sig) else {
                    continue;
                };

                let verify_result = pk.verify(&msg, &sig);
                if result == "valid" {
                    assert!(
                        verify_result.is_ok(),
                        "wycheproof ECDSA DER tcId={} expected valid but failed",
                        test["tcId"]
                    );
                    valid_tested += 1;
                } else {
                    assert!(
                        verify_result.is_err(),
                        "wycheproof ECDSA DER tcId={} expected invalid but passed",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid ECDSA DER wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid ECDSA DER wycheproof tests were run");
    }

    fn wycheproof_ecdh_case(file: &str, asn: bool) {
        let data: serde_json::Value = serde_json::from_str(file).unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            for test in group["tests"].as_array().unwrap() {
                let public_hex = test["public"].as_str().unwrap();
                let private_hex = test["private"].as_str().unwrap();
                let expected_shared_hex = test["shared"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let public_bytes = hex::decode(public_hex).unwrap();
                let public_key = if asn {
                    match spki_to_sec1_point(&public_bytes) {
                        Some(point) => point,
                        None => {
                            if result == "valid" {
                                panic!("wycheproof ECDH ASN tcId={}: failed to parse valid SPKI", test["tcId"]);
                            }
                            invalid_tested += 1;
                            continue;
                        }
                    }
                } else {
                    public_bytes
                };

                // Private key hex is a bigint and may be shorter than 28 bytes.
                let private_bytes = hex::decode(private_hex).unwrap();
                let mut private_key = [0u8; SECRET_KEY_SIZE];
                let effective_len = private_bytes.len().min(SECRET_KEY_SIZE);
                let skip = private_bytes.len().saturating_sub(SECRET_KEY_SIZE);
                private_key[SECRET_KEY_SIZE - effective_len..]
                    .copy_from_slice(&private_bytes[skip..skip + effective_len]);

                let shared = ecdh(&private_key, &public_key);

                if result == "valid" {
                    let shared = shared.unwrap();
                    assert_eq!(
                        hex::encode(shared),
                        expected_shared_hex,
                        "wycheproof ECDH tcId={}",
                        test["tcId"]
                    );
                    valid_tested += 1;
                } else if result == "invalid" {
                    assert!(
                        shared.is_err(),
                        "wycheproof ECDH tcId={} expected invalid but passed",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid ECDH wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid ECDH wycheproof tests were run");
    }

    #[test]
    fn wycheproof_ecdh_p224_ecpoint() {
        wycheproof_ecdh_case(
            include_str!("../../testdata/wycheproof/testvectors_v1/ecdh_secp224r1_ecpoint_test.json"),
            false,
        );
    }

    #[test]
    fn wycheproof_ecdh_p224_asn() {
        wycheproof_ecdh_case(
            include_str!("../../testdata/wycheproof/testvectors_v1/ecdh_secp224r1_test.json"),
            true,
        );
    }

    // Shared curve-independent tests, instantiated for P224.
    #[test]
    fn generator_point_is_on_curve() {
        p_curves::test_support::generator_point_is_on_curve::<P224>();
    }

    #[test]
    fn from_x_y_rejects_off_curve() {
        p_curves::test_support::from_x_y_rejects_off_curve::<P224>();
    }

    #[test]
    fn ecdh_round_trip_alice_bob() {
        p_curves::test_support::ecdh_round_trip_alice_bob::<P224>();
    }

    #[test]
    fn ecdh_rejects_off_curve_peer_public_key() {
        p_curves::test_support::ecdh_rejects_off_curve_peer_public_key::<P224>();
    }

    #[test]
    fn ecdh_rejects_infinity_peer_public_key() {
        p_curves::test_support::ecdh_rejects_infinity_peer_public_key::<P224>();
    }

    #[test]
    fn ecdh_rejects_bad_length_peer_public_key() {
        p_curves::test_support::ecdh_rejects_bad_length_peer_public_key::<P224>();
    }

    #[test]
    fn ecdh_multiple_exchanges_consistency() {
        p_curves::test_support::ecdh_multiple_exchanges_consistency::<P224>();
    }

    #[test]
    fn private_key_round_trip_bytes() {
        p_curves::test_support::private_key_round_trip_bytes::<P224>();
    }

    #[test]
    fn public_key_round_trip_bytes() {
        p_curves::test_support::public_key_round_trip_bytes::<P224>();
    }

    #[test]
    fn point_double_and_add_consistency() {
        p_curves::test_support::point_double_and_add_consistency::<P224>();
    }

    #[test]
    fn compressed_public_key_has_correct_prefix() {
        p_curves::test_support::compressed_public_key_has_correct_prefix::<P224>();
    }

    #[cfg(feature = "zeroize")]
    #[test]
    fn rfc6979_state_zeroize_clears_drbg() {
        p_curves::test_support::rfc6979_state_zeroize_clears_drbg::<P224>();
    }
}
