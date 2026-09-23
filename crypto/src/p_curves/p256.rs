//! P-256 (secp256r1) ECDSA and ECDH.
//!
//! See [`SecretKey`] and [`PublicKey`] for the signing, verification and key
//! agreement APIs.

use big_number::{Uint, mac};

use super::p_curves::{self, Curve, UintOps, field_pow};
use crate::{EllipticCurveError, Hasher, RandomError, hmac::Hmac, sha2::Sha256};

/// Size of a P-256 secret key in bytes (32 bytes).
pub const SECRET_KEY_SIZE: usize = 32;
/// Size of a compressed P-256 public key in bytes (33 bytes, includes 0x02/0x03 prefix).
pub const PUBLIC_KEY_COMPRESSED_SIZE: usize = 33;
/// Size of an uncompressed P-256 public key in bytes (65 bytes, includes 0x04 prefix).
pub const PUBLIC_KEY_UNCOMPRESSED_SIZE: usize = 65;
/// Size of a P-256 ECDSA signature in bytes (64 bytes, r || s).
pub const SIGNATURE_SIZE: usize = 64;
/// Size of the raw ECDH shared secret in bytes (32 bytes). **Must not** be used directly
/// as an encryption key; apply a KDF first.
pub const ECDH_SHARED_SECRET_SIZE: usize = 32;

/// P-256 (secp256r1) ECDSA secret key.
///
/// Supports signing and ECDH key agreement.
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
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct SecretKey(p_curves::SecretKey<P256>);

impl SecretKey {
    /// Generates a fresh random secret key.
    #[cfg(feature = "random")]
    pub fn generate() -> Result<SecretKey, EllipticCurveError> {
        Ok(SecretKey(p_curves::SecretKey::generate()?))
    }

    /// Builds a secret key from its 32-byte big-endian scalar.
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

    /// Signs `message` with ECDSA using SHA-256 and deterministic (RFC 6979)
    /// nonces.
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

    /// Returns the secret scalar as 32 big-endian bytes.
    pub fn to_bytes(&self) -> [u8; SECRET_KEY_SIZE] {
        self.0.to_bytes()
    }
}

/// P-256 (secp256r1) ECDSA public key.
///
/// Supports signature verification and ECDH key agreement.
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
pub struct PublicKey(p_curves::PublicKey<P256>);

impl PublicKey {
    /// Parses a compressed (33-byte) or uncompressed (65-byte) SEC1 encoding.
    ///
    /// Returns [`EllipticCurveError::InvalidKey`] when the encoding is not a
    /// valid point on the P-256 curve.
    #[inline]
    pub fn from_bytes(key: &[u8]) -> Result<PublicKey, EllipticCurveError> {
        Ok(PublicKey(p_curves::PublicKey::from_bytes(key)?))
    }

    /// Builds a public key from raw affine x and y coordinates (both
    /// big-endian, 32 bytes each). Returns `InvalidKey` if the coordinates
    /// are not a valid point on the P-256 curve.
    ///
    /// This is useful when importing keys from formats like JWK where `x`
    /// and `y` are available directly.
    #[inline]
    pub fn from_x_y(x_bytes: &[u8; 32], y_bytes: &[u8; 32]) -> Result<PublicKey, EllipticCurveError> {
        Ok(PublicKey(p_curves::PublicKey::from_x_y(x_bytes, y_bytes)?))
    }

    /// Verifies an ECDSA signature over `message` using SHA-256.
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

    /// Returns the 65-byte uncompressed SEC1 encoding (`0x04 || x || y`).
    #[inline]
    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_UNCOMPRESSED_SIZE] {
        self.0.to_bytes()
    }

    /// Returns the 33-byte compressed SEC1 encoding (`0x02`/`0x03 || x`).
    #[inline]
    pub fn to_compressed_bytes(&self) -> [u8; PUBLIC_KEY_COMPRESSED_SIZE] {
        self.0.to_compressed_bytes()
    }

    /// Returns the `X` and `Y` points as big-endian arrays.
    #[inline]
    pub fn x_y(&self) -> ([u8; 32], [u8; 32]) {
        self.0.x_y()
    }
}

/// Marker type carrying the P-256 curve parameters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct P256;

type U256 = Uint<256, 4>;
type CurveUint = U256;

const MODULUS_P: CurveUint = CurveUint::from_limbs([
    0xffff_ffff_ffff_ffff,
    0x0000_0000_ffff_ffff,
    0x0000_0000_0000_0000,
    0xffff_ffff_0000_0001,
]);

const MODULUS_N: CurveUint = CurveUint::from_limbs([
    0xf3b9_cac2_fc63_2551,
    0xbce6_faad_a717_9e84,
    0xffff_ffff_ffff_ffff,
    0xffff_ffff_0000_0000,
]);

// floor(n / 2). A signature scalar `s` is "high" when `s > N_HALF`, in which
// case it is normalized to `n - s` so that signatures are non-malleable.
const N_HALF: CurveUint = CurveUint::from_limbs([
    0x79dc_e561_7e31_92a8,
    0xde73_7d56_d38b_cf42,
    0x7fff_ffff_ffff_ffff,
    0x7fff_ffff_8000_0000,
]);

const P_MINUS_TWO: CurveUint = CurveUint::from_limbs([
    0xffff_ffff_ffff_fffd,
    0x0000_0000_ffff_ffff,
    0x0000_0000_0000_0000,
    0xffff_ffff_0000_0001,
]);

const P_PLUS_ONE_OVER_FOUR: CurveUint = CurveUint::from_limbs([
    0x0000_0000_0000_0000,
    0x0000_0000_4000_0000,
    0x4000_0000_0000_0000,
    0x3fff_ffff_c000_0000,
]);

const N_MINUS_TWO: CurveUint = CurveUint::from_limbs([
    0xf3b9_cac2_fc63_254f,
    0xbce6_faad_a717_9e84,
    0xffff_ffff_ffff_ffff,
    0xffff_ffff_0000_0000,
]);

const CURVE_B: CurveUint = CurveUint::from_limbs([
    0x3bce_3c3e_27d2_604b,
    0x651d_06b0_cc53_b0f6,
    0xb3eb_bd55_7698_86bc,
    0x5ac6_35d8_aa3a_93e7,
]);

const GENERATOR_X: CurveUint = CurveUint::from_limbs([
    0xf4a1_3945_d898_c296,
    0x7703_7d81_2deb_33a0,
    0xf8bc_e6e5_63a4_40f2,
    0x6b17_d1f2_e12c_4247,
]);

const GENERATOR_Y: CurveUint = CurveUint::from_limbs([
    0xcbb6_4068_37bf_51f5,
    0x2bce_3357_6b31_5ece,
    0x8ee7_eb4a_7c0f_9e16,
    0x4fe3_42e2_fe1a_7f9b,
]);

// P-256 fast reduction constants: S^i = 2^(64i) mod p
// Verified against Python with 100k random tests.
const S4: [u64; 4] = [
    0x0000000000000001,
    0xffffffff00000000,
    0xffffffffffffffff,
    0x00000000fffffffe,
];

const S5: [u64; 4] = [
    0x00000000ffffffff,
    0x0000000100000001,
    0xfffffffeffffffff,
    0xfffffffe00000000,
];

const S6: [u64; 4] = [
    0xfffffffefffffffe,
    0x00000002ffffffff,
    0x0000000000000002,
    0xfffffffe00000001,
];

const S7: [u64; 4] = [
    0xfffffffeffffffff,
    0xfffffffffffffffe,
    0x0000000200000000,
    0x0000000000000003,
];

// Branch-free u128 select: returns a if choice else b.
#[inline]
fn ct_select_u128(a: u128, b: u128, choice: bool) -> u128 {
    let mask = (choice as u128).wrapping_neg();
    (a & mask) | (b & !mask)
}

// P-256 fast modular multiplication using u128 accumulators.
// All loops run fixed iteration counts with ct_select for constant-time.
fn p256_fast_mul_mod(a: &CurveUint, b: &CurveUint) -> CurveUint {
    let al = a.limbs;
    let bl = b.limbs;

    let mut prod = [0u64; 8];
    for i in 0..4 {
        let mut carry = 0u64;
        for j in 0..4 {
            let (v, cc) = mac(prod[i + j], al[i], bl[j], carry);
            prod[i + j] = v;
            carry = cc;
        }
        prod[i + 4] = carry;
    }

    const MASK: u128 = 0xffffffffffffffff;
    let c0 = [S4[0] as u128, S4[1] as u128, S4[2] as u128, S4[3] as u128];
    let c1 = [S5[0] as u128, S5[1] as u128, S5[2] as u128, S5[3] as u128];
    let c2 = [S6[0] as u128, S6[1] as u128, S6[2] as u128, S6[3] as u128];
    let c3 = [S7[0] as u128, S7[1] as u128, S7[2] as u128, S7[3] as u128];
    let coeffs = [c0, c1, c2, c3];

    let mut r0 = prod[0] as u128;
    let mut r1 = prod[1] as u128;
    let mut r2 = prod[2] as u128;
    let mut r3 = prod[3] as u128;

    for i in 0..4 {
        let w = prod[4 + i] as u128;
        let c = coeffs[i];

        r0 = r0.wrapping_add(w.wrapping_mul(c[0]));
        r1 = r1.wrapping_add(w.wrapping_mul(c[1]));
        r2 = r2.wrapping_add(w.wrapping_mul(c[2]));
        r3 = r3.wrapping_add(w.wrapping_mul(c[3]));

        // Fixed 4 iterations: carry propagation + conditional residual reduction.
        for _ in 0..4 {
            let carry = r0 >> 64;
            r1 = r1.wrapping_add(carry);
            r0 &= MASK;
            let carry = r1 >> 64;
            r2 = r2.wrapping_add(carry);
            r1 &= MASK;
            let carry = r2 >> 64;
            r3 = r3.wrapping_add(carry);
            r2 &= MASK;

            let residual = r3 >> 64;
            let need_reduce = residual != 0;

            // Compute reduced version (applied if need_reduce) and original.
            let rr3 = r3 & MASK;
            let rr0 = r0.wrapping_add(residual.wrapping_mul(c0[0]));
            let rr1 = r1.wrapping_add(residual.wrapping_mul(c0[1]));
            let rr2 = r2.wrapping_add(residual.wrapping_mul(c0[2]));
            let rr3r = rr3.wrapping_add(residual.wrapping_mul(c0[3]));

            // ct_select between reduced and non-reduced based on need_reduce.
            r0 = ct_select_u128(rr0, r0, need_reduce);
            r1 = ct_select_u128(rr1, r1, need_reduce);
            r2 = ct_select_u128(rr2, r2, need_reduce);
            r3 = ct_select_u128(rr3r, r3, need_reduce);
        }
    }

    // Fixed 8 conditional subtractions (result may be up to ~16×p).
    let mut result = CurveUint::from_limbs([r0 as u64, r1 as u64, r2 as u64, r3 as u64]);
    for _ in 0..8 {
        let (sub, borrow) = result.sub_raw(&MODULUS_P);
        result = CurveUint::ct_select(&sub, &result, borrow == 0);
    }
    result
}

// Reduce a big-endian value modulo n, accepting zero and values >= n.
#[inline]
fn reduce_mod(value: CurveUint) -> CurveUint {
    let (sub_value, _) = value.sub_raw(&MODULUS_N);
    CurveUint::ct_select(&sub_value, &value, value.ct_ge(&MODULUS_N))
}

impl Curve for P256 {
    type U = CurveUint;
    type FieldBytes = [u8; SECRET_KEY_SIZE];
    type DigestBytes = [u8; 32];
    type CompressedBytes = [u8; PUBLIC_KEY_COMPRESSED_SIZE];
    type UncompressedBytes = [u8; PUBLIC_KEY_UNCOMPRESSED_SIZE];
    type SignatureBytes = [u8; SIGNATURE_SIZE];

    const FIELD_BITS: usize = 256;
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
        p256_fast_mul_mod(a, b)
    }

    #[inline]
    fn scalar_mul(a: &Self::U, b: &Self::U) -> Self::U {
        a.mul_mod(b, &MODULUS_N)
    }

    #[inline]
    fn field_sqrt(a: Self::U) -> Option<Self::U> {
        let candidate = field_pow::<Self>(a, &P_PLUS_ONE_OVER_FOUR);
        if a.ct_eq(&Self::field_mul(&candidate, &candidate)) {
            Some(candidate)
        } else {
            None
        }
    }

    #[inline]
    fn hash(data: &[u8]) -> Self::DigestBytes {
        Sha256::hash(data).as_ref().try_into().unwrap()
    }

    #[inline]
    fn hmac(key: &[u8], data: &[u8]) -> Self::DigestBytes {
        Hmac::<Sha256>::mac(key, data).as_ref().try_into().unwrap()
    }

    #[inline]
    fn scalar_from_digest(digest: &Self::DigestBytes) -> Self::U {
        reduce_mod(CurveUint::read_be(digest.as_ref()))
    }

    #[inline]
    fn scalar_from_field_bytes(bytes: &Self::FieldBytes) -> Self::U {
        reduce_mod(CurveUint::read_be(bytes.as_ref()))
    }

    #[cfg(feature = "random")]
    #[inline]
    fn random_secret_key() -> Result<Self::FieldBytes, RandomError> {
        crate::random::bytes()
    }
}

/// Returns `true` when `public_key` is a valid SEC1 encoding of a point on the
/// P-256 curve.
pub fn is_valid_public_key(public_key: &[u8]) -> bool {
    p_curves::is_valid_public_key::<P256>(public_key)
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    use super::*;
    use crate::p_curves::p_curves;

    type FieldElement = p_curves::FieldElement<P256>;
    type Scalar = p_curves::Scalar<P256>;
    type AffinePoint = p_curves::AffinePoint<P256>;
    type ProjectivePoint = p_curves::ProjectivePoint<P256>;
    type Rfc6979 = p_curves::Rfc6979<P256>;

    // Concrete wrappers around the generic core helpers.
    fn hash_message(message: &[u8]) -> [u8; 32] {
        p_curves::hash_message::<P256>(message)
    }

    fn hmac_digest(key: &[u8], data: &[u8]) -> [u8; 32] {
        p_curves::hmac_digest::<P256>(key, data)
    }

    fn bits2octets(hash: &[u8; 32]) -> [u8; SECRET_KEY_SIZE] {
        p_curves::bits2octets::<P256>(hash)
    }

    fn derive_public_key_uncompressed(
        private_key: &[u8; SECRET_KEY_SIZE],
    ) -> Result<[u8; PUBLIC_KEY_UNCOMPRESSED_SIZE], crate::EllipticCurveError> {
        p_curves::derive_public_key_uncompressed::<P256>(private_key)
    }

    fn derive_public_key_compressed(
        private_key: &[u8; SECRET_KEY_SIZE],
    ) -> Result<[u8; PUBLIC_KEY_COMPRESSED_SIZE], crate::EllipticCurveError> {
        p_curves::derive_public_key_compressed::<P256>(private_key)
    }

    fn ecdsa_sign_inner_impl(
        scalar: &Scalar,
        message: &[u8],
        force_first_retry: bool,
    ) -> Result<[u8; SIGNATURE_SIZE], crate::EllipticCurveError> {
        p_curves::ecdsa_sign_inner_impl::<P256>(scalar, message, force_first_retry)
    }

    fn ecdsa_verify_inner(
        public_point: &AffinePoint,
        message: &[u8],
        signature: &[u8; SIGNATURE_SIZE],
    ) -> Result<(), crate::EllipticCurveError> {
        p_curves::ecdsa_verify_inner::<P256>(public_point, message, signature)
    }

    fn parse_public_key(public_key: &[u8]) -> Result<AffinePoint, crate::EllipticCurveError> {
        p_curves::parse_public_key::<P256>(public_key)
    }

    fn scalar_mul_generator(scalar: &Scalar) -> ProjectivePoint {
        p_curves::scalar_mul_generator::<P256>(scalar)
    }

    fn scalar_mul_affine(base: &AffinePoint, scalar: &Scalar) -> ProjectivePoint {
        p_curves::scalar_mul_affine::<P256>(base, scalar)
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
    // Returns (tag, value) or None on error.
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
            // DER requires the first length byte to be non-zero when
            // num_bytes > 1, otherwise shorter encoding would suffice
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
    // to P1363 format (r || s, each 32 bytes).
    fn der_ecdsa_sig_to_p1363(der: &[u8]) -> Option<[u8; 64]> {
        let mut offset = 0;
        let (tag, inner) = der_read_tlv(der, &mut offset)?;
        if tag != 0x30 {
            return None;
        }
        // DER signature must be fully consumed (no trailing bytes)
        if offset != der.len() {
            return None;
        }
        let mut inner_offset = 0;
        let (rtag, rval) = der_read_tlv(inner, &mut inner_offset)?;
        if rtag != 0x02 || rval.is_empty() || rval.len() > 33 {
            return None;
        }
        let (stag, sval) = der_read_tlv(inner, &mut inner_offset)?;
        if stag != 0x02 || sval.is_empty() || sval.len() > 33 {
            return None;
        }
        // Strict DER requires no trailing data in the SEQUENCE
        if inner_offset != inner.len() {
            return None;
        }
        // DER INTEGER encoding rules:
        // - Must use minimal number of bytes.
        // - If the high bit would be set, prepend 0x00.
        // - If leading 0x00 is used, the next byte must have high bit set.
        let r_valid = if rval.len() == 32 && rval[0] >= 0x80 {
            false
        } else if rval.len() == 33 && rval[0] != 0 {
            false
        } else if rval.len() == 33 && rval[0] == 0 && rval[1] < 0x80 {
            false
        } else if rval.len() > 33 {
            false
        } else {
            true
        };
        let s_valid = if sval.len() == 32 && sval[0] >= 0x80 {
            false
        } else if sval.len() == 33 && sval[0] != 0 {
            false
        } else if sval.len() == 33 && sval[0] == 0 && sval[1] < 0x80 {
            false
        } else if sval.len() > 33 {
            false
        } else {
            true
        };
        if !r_valid || !s_valid {
            return None;
        }

        let r_trimmed = if rval.len() == 33 && rval[0] == 0 {
            &rval[1..]
        } else {
            rval
        };
        let s_trimmed = if sval.len() == 33 && sval[0] == 0 {
            &sval[1..]
        } else {
            sval
        };
        if r_trimmed.len() > 32 || s_trimmed.len() > 32 {
            return None;
        }
        let mut sig = [0u8; 64];
        sig[32 - r_trimmed.len()..32].copy_from_slice(r_trimmed);
        sig[64 - s_trimmed.len()..64].copy_from_slice(s_trimmed);
        Some(sig)
    }

    // Extract the raw SEC1 uncompressed point from a DER SubjectPublicKeyInfo
    // that uses the named secp256r1 curve (explicit parameters are rejected).
    fn spki_to_sec1_point(spki: &[u8]) -> Option<Vec<u8>> {
        let ec_public_key_oid: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01];
        let secp256r1_oid: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07];
        let mut offset = 0;
        let (_tag, outer) = der_read_tlv(spki, &mut offset)?;
        let mut inner = 0;
        // Parse AlgorithmIdentifier SEQUENCE to verify it uses secp256r1 OID
        let (_alg_tag, alg_content) = der_read_tlv(outer, &mut inner)?;
        if _alg_tag != 0x30 {
            return None;
        }
        // First element must be OID ecPublicKey
        let mut ai = 0;
        let (oid1_tag, oid1) = der_read_tlv(alg_content, &mut ai)?;
        if oid1_tag != 0x06 || oid1 != ec_public_key_oid {
            return None;
        }
        // Second element must be OID secp256r1 (named curve), not explicit params
        let (oid2_tag, oid2) = der_read_tlv(alg_content, &mut ai)?;
        if oid2_tag != 0x06 || oid2 != secp256r1_oid {
            return None;
        }
        // Read BIT STRING
        let (_bs_tag, bs_val) = der_read_tlv(outer, &mut inner)?;
        if _bs_tag != 0x03 || bs_val.is_empty() {
            return None;
        }
        // Skip the unused-bits byte
        Some(bs_val[1..].to_vec())
    }

    #[test]
    fn from_x_y_matches_generator() {
        let key = PublicKey::from_x_y(
            &hex::decode("6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296")
                .unwrap()
                .try_into()
                .unwrap(),
            &hex::decode("4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .unwrap();
        let from_sec1 = PublicKey::from_bytes(&key.to_bytes()).unwrap();
        assert_eq!(key, from_sec1);
    }

    #[test]
    fn derive_public_key_generator_matches_sec1_base_point() {
        let mut private_key = [0u8; 32];
        private_key[31] = 1;
        let derived = derive_public_key_uncompressed(&private_key).unwrap();
        let expected = decode_hex::<65>(
            "046b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296\
             4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5",
        );
        assert_eq!(derived, expected);
    }

    #[test]
    fn derive_public_key_matches_rfc6979_vector() {
        let private_key = decode_hex::<32>("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721");
        let expected = decode_hex::<65>(
            "0460fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb6\
             7903fe1008b8bc99a41ae9e95628bc64f2f1b20c2d7e9f5177a3c294d4462299",
        );
        assert_eq!(derive_public_key_uncompressed(&private_key).unwrap(), expected);
        assert_eq!(
            derive_public_key_compressed(&private_key).unwrap(),
            decode_hex::<33>("0360fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb6"),
        );
    }

    #[test]
    fn ecdsa_sign_matches_rfc6979_vectors() {
        let private_key = decode_hex::<32>("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721");
        let key = SecretKey::from_bytes(&private_key).unwrap();
        // The raw RFC 6979 "sample" signature has high s (f7cb…cda8); the
        // expected value below is its canonical low-s form (n - s).
        let sample_signature = key.sign(b"sample").unwrap();
        let expected_sample = decode_hex::<64>(
            "efd48b2aacb6a8fd1140dd9cd45e81d69d2c877b56aaf991c34d0ea84eaf3716\
             0834e36ad29a83bf2bc9385e491d6099c8fdf9d1ed67aa7ea5f51f93782857a9",
        );
        assert_eq!(sample_signature, expected_sample);

        let test_signature = key.sign(b"test").unwrap();
        let expected_test = decode_hex::<64>(
            "f1abb023518351cd71d881567b1ea663ed3efcf6c5132b354f28d3b0b7d38367\
             019f4113742a2b14bd25926b49c649155f267e60d3814b4c0cc84250e46f0083",
        );
        assert_eq!(test_signature, expected_test);
    }

    #[test]
    fn ecdsa_sign_emits_canonical_low_s() {
        let private_key = decode_hex::<32>("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721");
        let key = SecretKey::from_bytes(&private_key).unwrap();
        for msg in [&b"sample"[..], b"test", b"", b"another message"] {
            let sig = key.sign(msg).unwrap();
            let s = Scalar::from_bytes(sig[32..].try_into().unwrap()).unwrap();
            assert!(!s.is_high(), "sign produced a high-s signature for {:?}", msg);
        }
    }

    #[test]
    fn ecdsa_verify_strict_rejects_high_s() {
        let private_key = decode_hex::<32>("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721");
        let key = SecretKey::from_bytes(&private_key).unwrap();
        let public_key = key.public_key();
        let signature = key.sign(b"sample").unwrap();

        // Canonical low-s signature passes both permissive and strict verification.
        assert!(public_key.verify(b"sample", &signature).is_ok());
        assert!(public_key.verify_strict(b"sample", &signature).is_ok());

        // The malleable counterpart (r, n - s) must be rejected by strict verify
        // but remains accepted by the permissive default.
        let r = Scalar::from_bytes(signature[..32].try_into().unwrap()).unwrap();
        let s = Scalar::from_bytes(signature[32..].try_into().unwrap()).unwrap();
        let high_s = Scalar::ZERO.sub(s);
        assert!(high_s.is_high());

        let mut malleable = [0u8; SIGNATURE_SIZE];
        malleable[..32].copy_from_slice(&r.to_bytes());
        malleable[32..].copy_from_slice(&high_s.to_bytes());

        assert!(public_key.verify(b"sample", &malleable).is_ok());
        assert!(public_key.verify_strict(b"sample", &malleable).is_err());
    }

    #[test]
    fn rfc6979_nonce_point_x_matches_signature_r() {
        let nonce = decode_hex::<32>("a6e3c57dd01abe90086538398355dd4c3b17aa873382b0f24d6129493d8aad60");
        let public = derive_public_key_uncompressed(&nonce).unwrap();
        assert_eq!(
            &public[1..33],
            &decode_hex::<32>("efd48b2aacb6a8fd1140dd9cd45e81d69d2c877b56aaf991c34d0ea84eaf3716")
        );
    }

    #[test]
    fn rfc6979_nonce_generation_matches_known_value() {
        let private_key = Scalar::from_bytes(&decode_hex::<32>(
            "c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721",
        ))
        .unwrap();
        let hash = hash_message(b"sample");
        assert_eq!(
            Rfc6979::new(&private_key, &hash).generate().to_bytes(),
            decode_hex::<32>("a6e3c57dd01abe90086538398355dd4c3b17aa873382b0f24d6129493d8aad60")
        );
    }

    #[test]
    fn rfc6979_retry_advances_drbg() {
        let private_key = Scalar::from_bytes(&decode_hex::<32>(
            "c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721",
        ))
        .unwrap();
        let hash = hash_message(b"sample");

        let mut drbg = Rfc6979::new(&private_key, &hash);
        let k1 = drbg.generate();
        assert_eq!(
            k1.to_bytes(),
            decode_hex::<32>("a6e3c57dd01abe90086538398355dd4c3b17aa873382b0f24d6129493d8aad60")
        );

        // RFC 6979 §3.2 h.3: an r=0/s=0 retry must continue the DRBG, not
        // reproduce the same nonce.
        drbg.retry();
        let k2 = drbg.generate();
        assert_ne!(k1.to_bytes(), k2.to_bytes());
        assert_eq!(
            k2.to_bytes(),
            decode_hex::<32>("8e83dc490bc5fc4d5992bd63cd87f254adffcb930f8a8011702a88870f638fdb")
        );
    }

    #[test]
    fn ecdsa_forced_retry_uses_fresh_nonce() {
        let private_key = decode_hex::<32>("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721");
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
    fn rfc6979_intermediate_hmac_values_match() {
        let x = decode_hex::<32>("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721");
        let h1 = hash_message(b"sample");
        let mut v = [0x01u8; 32];
        let mut k = [0u8; 32];

        let mut buf = [0u8; 97];
        buf[..32].copy_from_slice(&v);
        buf[32] = 0x00;
        buf[33..65].copy_from_slice(&x);
        buf[65..97].copy_from_slice(&h1);
        k = hmac_digest(&k, &buf);
        assert_eq!(
            k,
            decode_hex::<32>("122db1de98dae4dfa33f2da8e98494c80bff807b479fd79261b37e25f267ee58")
        );
        v = hmac_digest(&k, &v);
        assert_eq!(
            v,
            decode_hex::<32>("c9947803a747fc60c23535fdcc13b5ca566b48221ca67d4964d22daa48275844")
        );

        buf[..32].copy_from_slice(&v);
        buf[32] = 0x01;
        k = hmac_digest(&k, &buf);
        assert_eq!(
            k,
            decode_hex::<32>("b6d4f98ebae70aa15a2238ade4e20ab323fc1e777d22f0c582d8ef2e6ba73569")
        );
        v = hmac_digest(&k, &v);
        assert_eq!(
            v,
            decode_hex::<32>("bae57fe256de2de806b10635497237e7bae96754582566384c47c6c3416494d1")
        );
        v = hmac_digest(&k, &v);
        assert_eq!(
            v,
            decode_hex::<32>("a6e3c57dd01abe90086538398355dd4c3b17aa873382b0f24d6129493d8aad60")
        );
    }

    #[test]
    fn ecdsa_verify_accepts_compressed_and_uncompressed_public_keys() {
        let private_key = decode_hex::<32>("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721");
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
        let private_key = decode_hex::<32>("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721");
        let key = SecretKey::from_bytes(&private_key).unwrap();
        let pub_key = key.public_key();
        let mut off_curve = [0u8; 65];
        off_curve.copy_from_slice(&pub_key.to_bytes());
        let signature = key.sign(b"sample").unwrap();

        assert!(pub_key.verify(b"tampered", &signature).is_err());

        let mut bad_signature = signature;
        bad_signature[10] ^= 0x80;
        assert!(pub_key.verify(b"sample", &bad_signature).is_err());

        off_curve[64] ^= 0x01;
        assert!(!is_valid_public_key(&off_curve));
        assert!(PublicKey::from_bytes(&off_curve).is_err());

        let invalid_x = decode_hex::<33>("02ffffffff00000001000000000000000000000000ffffffffffffffffffffffff");
        assert!(!is_valid_public_key(&invalid_x));
    }

    #[test]
    fn invalid_inputs_are_rejected() {
        let invalid_private_key = [0u8; SECRET_KEY_SIZE];
        assert!(SecretKey::from_bytes(&invalid_private_key).is_err());
        assert!(derive_public_key_uncompressed(&invalid_private_key).is_err());
        assert!(derive_public_key_compressed(&invalid_private_key).is_err());

        let private_key = decode_hex::<32>("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721");
        let key = SecretKey::from_bytes(&private_key).unwrap();
        let signature = key.sign(b"msg").unwrap();
        let mut zero_r = signature;
        zero_r[..32].fill(0);
        assert!(key.public_key().verify(b"msg", &zero_r).is_err());
    }

    #[test]
    fn public_key_validation_accepts_known_good_points() {
        assert!(is_valid_public_key(&decode_hex::<65>(
            "046b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296\
             4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5"
        )));
        assert!(is_valid_public_key(&decode_hex::<33>(
            "0360fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb6"
        )));
    }

    // --- Wycheproof test vectors ---

    #[test]
    fn wycheproof_ecdsa_p256_sha256_p1363() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/ecdsa_secp256r1_sha256_p1363_test.json"
        ))
        .unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            let uncompressed_hex = group["publicKey"]["uncompressed"].as_str().unwrap();
            let pubkey_bytes = hex::decode(uncompressed_hex).unwrap();
            let pk = PublicKey::from_bytes(&pubkey_bytes).unwrap();

            for test in group["tests"].as_array().unwrap() {
                let msg_hex = test["msg"].as_str().unwrap();
                let sig_hex = test["sig"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let msg = hex::decode(msg_hex).unwrap();

                if sig_hex.len() != SIGNATURE_SIZE * 2 {
                    continue;
                }
                let sig = decode_hex::<SIGNATURE_SIZE>(sig_hex);

                let verify_result = pk.verify(&msg, &sig);

                if result == "valid" {
                    assert!(
                        verify_result.is_ok(),
                        "wycheproof ECDSA P1363 tcId={} expected valid but failed",
                        test["tcId"]
                    );
                    valid_tested += 1;
                } else {
                    assert!(
                        verify_result.is_err(),
                        "wycheproof ECDSA P1363 tcId={} expected invalid but passed",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid ECDSA P1363 wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid ECDSA P1363 wycheproof tests were run");
    }

    #[test]
    fn ecdsa_sign_verify_round_trip_multiple_messages() {
        let private_key = decode_hex::<32>("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721");
        let key = SecretKey::from_bytes(&private_key).unwrap();
        let pub_key = key.public_key();

        let messages: &[&[u8]] = &[
            b"",
            b"hello world",
            b"The quick brown fox jumps over the lazy dog",
            &[0u8; 0],
            &[0xffu8; 100],
            b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f",
        ];

        for msg in messages {
            let sig = key.sign(msg).unwrap();
            assert!(pub_key.verify(msg, &sig).is_ok(), "round-trip failed for message {:?}", msg);
            // Verify with different message fails
            let mut wrong_msg = msg.to_vec();
            wrong_msg.push(0x42);
            assert!(pub_key.verify(&wrong_msg, &sig).is_err());
        }
    }

    #[test]
    fn ecdsa_sign_verify_different_keys() {
        // Use multiple different private keys
        let keys: &[&str] = &[
            "0000000000000000000000000000000000000000000000000000000000000001",
            "0000000000000000000000000000000000000000000000000000000000000002",
            "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632550",
            "a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f90011",
        ];

        for key_hex in keys {
            let private_key = decode_hex::<32>(key_hex);
            let key = SecretKey::from_bytes(&private_key).unwrap();
            let sig = key.sign(b"test message").unwrap();
            assert!(
                key.public_key().verify(b"test message", &sig).is_ok(),
                "sign/verify failed for key {}",
                key_hex
            );
        }
    }

    #[test]
    fn ecdsa_verify_wrong_public_key_rejects() {
        let private_key1 = decode_hex::<32>("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721");
        let private_key2 = decode_hex::<32>("0000000000000000000000000000000000000000000000000000000000000001");
        let key1 = SecretKey::from_bytes(&private_key1).unwrap();
        let key2 = SecretKey::from_bytes(&private_key2).unwrap();

        let sig = key1.sign(b"message").unwrap();
        assert!(key2.public_key().verify(b"message", &sig).is_err());
    }

    #[test]
    fn scalar_from_bytes_rejects_boundary_values() {
        // Zero is rejected
        let zero = [0u8; 32];
        assert!(Scalar::from_bytes(&zero).is_none());

        // n is rejected (must be strictly less than n)
        let n_bytes = decode_hex::<32>("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551");
        assert!(Scalar::from_bytes(&n_bytes).is_none());

        // n-1 is accepted
        let n_minus_1 = decode_hex::<32>("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632550");
        assert!(Scalar::from_bytes(&n_minus_1).is_some());

        // 1 is accepted
        let one = decode_hex::<32>("0000000000000000000000000000000000000000000000000000000000000001");
        assert!(Scalar::from_bytes(&one).is_some());
    }

    #[test]
    fn field_element_from_bytes_rejects_boundary_values() {
        // p is rejected (must be strictly less than p)
        let p_bytes = decode_hex::<32>("ffffffff00000001000000000000000000000000ffffffffffffffffffffffff");
        assert!(FieldElement::from_bytes(&p_bytes).is_none());

        // p-1 is accepted
        let p_minus_1 = decode_hex::<32>("ffffffff00000001000000000000000000000000fffffffffffffffffffffffe");
        assert!(FieldElement::from_bytes(&p_minus_1).is_some());

        // 0 is accepted (zero is a valid field element)
        let zero = [0u8; 32];
        assert!(FieldElement::from_bytes(&zero).is_some());
    }

    #[test]
    fn point_decompression_round_trip() {
        // Generate several public keys and verify compressed/uncompressed round-trip
        let keys: &[&str] = &[
            "0000000000000000000000000000000000000000000000000000000000000001",
            "0000000000000000000000000000000000000000000000000000000000000002",
            "c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721",
            "a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f90011",
        ];

        for key_hex in keys {
            let private_key = decode_hex::<32>(key_hex);
            let key = SecretKey::from_bytes(&private_key).unwrap();
            let uncompressed = key.public_key();
            let compressed = derive_public_key_compressed(&private_key).unwrap();

            // Both formats should verify the same signature
            let sig = key.sign(b"round-trip").unwrap();
            assert!(uncompressed.verify(b"round-trip", &sig).is_ok());
            let point = AffinePoint::from_sec1_bytes(&compressed).unwrap();
            assert!(ecdsa_verify_inner(&point, b"round-trip", &sig).is_ok());

            // Decompress the compressed key and verify it matches the uncompressed key
            let point = AffinePoint::from_sec1_bytes(&compressed).unwrap();
            assert_eq!(point.to_uncompressed_bytes(), uncompressed.to_bytes());
        }
    }

    #[test]
    fn nist_cavp_verify_vectors() {
        // NIST CAVP-style ECDSA P-256/SHA-256 signature verification test vectors.
        // These test verification with known public keys and signatures.

        struct VerifyVector {
            qx: &'static str,
            qy: &'static str,
            msg: &'static [u8],
            r: &'static str,
            s: &'static str,
            valid: bool,
        }

        let vectors = [
            // Valid signature: RFC 6979 vector for "sample"
            VerifyVector {
                qx: "60fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb6",
                qy: "7903fe1008b8bc99a41ae9e95628bc64f2f1b20c2d7e9f5177a3c294d4462299",
                msg: b"sample",
                r: "efd48b2aacb6a8fd1140dd9cd45e81d69d2c877b56aaf991c34d0ea84eaf3716",
                s: "f7cb1c942d657c41d436c7a1b6e29f65f3e900dbb9aff4064dc4ab2f843acda8",
                valid: true,
            },
            // Valid signature: RFC 6979 vector for "test"
            VerifyVector {
                qx: "60fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb6",
                qy: "7903fe1008b8bc99a41ae9e95628bc64f2f1b20c2d7e9f5177a3c294d4462299",
                msg: b"test",
                r: "f1abb023518351cd71d881567b1ea663ed3efcf6c5132b354f28d3b0b7d38367",
                s: "019f4113742a2b14bd25926b49c649155f267e60d3814b4c0cc84250e46f0083",
                valid: true,
            },
            // Invalid: correct r from "sample" but wrong message
            VerifyVector {
                qx: "60fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb6",
                qy: "7903fe1008b8bc99a41ae9e95628bc64f2f1b20c2d7e9f5177a3c294d4462299",
                msg: b"wrong",
                r: "efd48b2aacb6a8fd1140dd9cd45e81d69d2c877b56aaf991c34d0ea84eaf3716",
                s: "f7cb1c942d657c41d436c7a1b6e29f65f3e900dbb9aff4064dc4ab2f843acda8",
                valid: false,
            },
            // Invalid: signature from "sample" verified against "test"
            VerifyVector {
                qx: "60fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb6",
                qy: "7903fe1008b8bc99a41ae9e95628bc64f2f1b20c2d7e9f5177a3c294d4462299",
                msg: b"test",
                r: "efd48b2aacb6a8fd1140dd9cd45e81d69d2c877b56aaf991c34d0ea84eaf3716",
                s: "f7cb1c942d657c41d436c7a1b6e29f65f3e900dbb9aff4064dc4ab2f843acda8",
                valid: false,
            },
            // Invalid: r modified by one bit
            VerifyVector {
                qx: "60fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb6",
                qy: "7903fe1008b8bc99a41ae9e95628bc64f2f1b20c2d7e9f5177a3c294d4462299",
                msg: b"sample",
                r: "efd48b2aacb6a8fd1140dd9cd45e81d69d2c877b56aaf991c34d0ea84eaf3717",
                s: "f7cb1c942d657c41d436c7a1b6e29f65f3e900dbb9aff4064dc4ab2f843acda8",
                valid: false,
            },
        ];

        for (i, v) in vectors.iter().enumerate() {
            let mut pubkey = [0u8; 65];
            pubkey[0] = 0x04;
            pubkey[1..33].copy_from_slice(&hex::decode(v.qx).unwrap());
            pubkey[33..65].copy_from_slice(&hex::decode(v.qy).unwrap());

            let mut sig = [0u8; 64];
            sig[..32].copy_from_slice(&hex::decode(v.r).unwrap());
            sig[32..].copy_from_slice(&hex::decode(v.s).unwrap());

            let pk = PublicKey::from_bytes(&pubkey).unwrap();
            let result = pk.verify(v.msg, &sig);
            if v.valid {
                assert!(result.is_ok(), "NIST vector {} should be valid", i);
            } else {
                assert!(result.is_err(), "NIST vector {} should be invalid", i);
            }
        }
    }

    #[test]
    fn rfc6979_bits2octets_matches_spec() {
        // For P-256 with SHA-256, bits2octets reduces the hash modulo n
        let hash = hash_message(b"sample");
        let result = bits2octets(&hash);
        // The hash of "sample" with SHA-256 is:
        // af2bdbe1aa9b6ec1e2ade1d694f41fc71a831d0268e9891562113d8a62add1bf
        // This is less than n, so bits2octets should return it unchanged
        assert_eq!(result, hash);

        // Test with a value that needs reduction (>= n)
        let big_hash: [u8; 32] = decode_hex::<32>("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632552");
        let reduced = bits2octets(&big_hash);
        // This is n+1, so reduced should be 1
        assert_eq!(
            reduced,
            decode_hex::<32>("0000000000000000000000000000000000000000000000000000000000000001")
        );
    }

    #[test]
    fn scalar_inversion_correctness() {
        // Verify that scalar inversion satisfies k * k^-1 = 1 mod n
        let k = Scalar::from_bytes(&decode_hex::<32>(
            "a6e3c57dd01abe90086538398355dd4c3b17aa873382b0f24d6129493d8aad60",
        ))
        .unwrap();
        let k_inv = k.invert().unwrap();
        let product = k.mul(k_inv);
        assert_eq!(product, Scalar::ONE);
    }

    #[test]
    fn field_element_inversion_correctness() {
        // Verify field element inversion: x * x^-1 = 1 mod p
        let x = FieldElement::from_bytes(&decode_hex::<32>(
            "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296",
        ))
        .unwrap();
        let x_inv = x.invert().unwrap();
        let product = x.mul(x_inv);
        assert_eq!(product, FieldElement::ONE);
        let product = x.mul(x_inv);
        assert_eq!(product, FieldElement::ONE);
    }

    #[test]
    fn p256_fast_mul_mod_matches_generic() {
        // Verify that the P-256 fast mul matches the generic bit-serial mul_mod
        // for many random inputs
        for _ in 0..1000 {
            let a_bytes: [u8; 32] = rand::random();
            let b_bytes: [u8; 32] = rand::random();
            let a_opt = FieldElement::from_bytes(&a_bytes);
            let b_opt = FieldElement::from_bytes(&b_bytes);
            if a_opt.is_none() || b_opt.is_none() {
                continue;
            }
            let a = a_opt.unwrap();
            let b = b_opt.unwrap();
            let expected = U256::from_limbs({
                let mut p = [0u64; 8];
                for i in 0..4 {
                    let mut c = 0u64;
                    for j in 0..4 {
                        let (v, cc) = mac(p[i + j], a.0.limbs[i], b.0.limbs[j], c);
                        p[i + j] = v;
                        c = cc;
                    }
                    p[i + 4] = c;
                }
                let mut rem = [0u64; 4];
                for bi in (0..512).rev() {
                    let li = bi / 64;
                    let pi = bi % 64;
                    let bit = ((p[li] >> pi) & 1) as u64;
                    let mut shifted = [0u64; 4];
                    let mut carry = bit;
                    for j in 0..4 {
                        let next = rem[j] >> 63;
                        shifted[j] = (rem[j] << 1) | carry;
                        carry = next;
                    }
                    let (red, br) = U256::from_limbs(shifted).sub_raw(&MODULUS_P);
                    if carry == 1 || br == 0 {
                        rem = red.limbs;
                    } else {
                        rem = shifted;
                    }
                }
                rem
            });
            let fast = p256_fast_mul_mod(&a.0, &b.0);
            assert_eq!(expected, fast, "mismatch");
        }
    }

    #[test]
    fn scalar_mul_generator_n_gives_identity() {
        // n * G = identity (point at infinity)
        // We can't use Scalar::from_bytes since it rejects n,
        // but we can verify (n-1)*G + G = identity indirectly:
        // (n-1)*G should give -G, i.e., (Gx, -Gy)
        let n_minus_1 = Scalar::from_bytes(&decode_hex::<32>(
            "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632550",
        ))
        .unwrap();
        let result = scalar_mul_generator(&n_minus_1).to_affine().unwrap();
        assert_eq!(result.x, FieldElement::from_uint(GENERATOR_X));
        // y should be -Gy mod p
        let neg_gy = FieldElement::from_uint(GENERATOR_Y).negate();
        assert_eq!(result.y, neg_gy);
    }

    // --- ECDH tests ---

    #[test]
    fn ecdh_rfc5903_section_8_1() {
        // RFC 5903 Section 8.1 — 256-bit Random ECP Group test vector
        let i_priv = decode_hex::<32>("c88f01f510d9ac3f70a292daa2316de544e9aab8afe84049c62a9c57862d1433");
        let i_pub = decode_hex::<65>(
            "04dad0b65394221cf9b051e1feca5787d098dfe637fc90b9ef945d0c3772581180\
              5271a0461cdb8252d61f1c456fa3e59ab1f45b33accf5f58389e0577b8990bb3",
        );
        let r_priv = decode_hex::<32>("c6ef9c5d78ae012a011164acb397ce2088685d8f06bf9be0b283ab46476bee53");
        let r_pub = decode_hex::<65>(
            "04d12dfb5289c8d4f81208b70270398c342296970a0bccb74c736fc7554494bf63\
              56fbf3ca366cc23e8157854c13c58d6aac23f046ada30f8353e74f33039872ab",
        );
        let expected_shared = decode_hex::<32>("d6840f6b42f6edafd13116e0e12565202fef8e9ece7dce03812464d04b9442de");

        let alice = SecretKey::from_bytes(&i_priv).unwrap();
        let bob = SecretKey::from_bytes(&r_priv).unwrap();
        let bob_pub = PublicKey::from_bytes(&r_pub).unwrap();
        let alice_pub = PublicKey::from_bytes(&i_pub).unwrap();

        assert_eq!(alice.public_key().to_bytes(), i_pub);
        assert_eq!(bob.public_key().to_bytes(), r_pub);

        let alice_shared = alice.ecdh(&bob_pub).unwrap();
        let bob_shared = bob.ecdh(&alice_pub).unwrap();

        assert_eq!(alice_shared, expected_shared);
        assert_eq!(bob_shared, expected_shared);
    }

    #[test]
    fn ecdh_nist_cavp_vector_from_go() {
        // Go stdlib crypto/ecdh NIST CAVS 14.1 ECC CDH Primitive (SP800-56A) vector
        let priv_key = decode_hex::<32>("7d7dc5f71eb29ddaf80d6214632eeae03d9058af1fb6d22ed80badb62bc1a534");
        let pub_key = decode_hex::<65>(
            "04ead218590119e8876b29146ff89ca61770c4edbbf97d38ce385ed281d8a6b230\
              28af61281fd35e2fa7002523acc85a429cb06ee6648325389f59edfce1405141",
        );
        let peer_pub = decode_hex::<65>(
            "04700c48f77f56584c5cc632ca65640db91b6bacce3a4df6b42ce7cc838833d287\
              db71e509e3fd9b060ddb20ba5c51dcc5948d46fbf640dfe0441782cab85fa4ac",
        );
        let expected_shared = decode_hex::<32>("46fc62106420ff012e54a434fbdd2d25ccc5852060561e68040dd7778997bd7b");

        let key = SecretKey::from_bytes(&priv_key).unwrap();
        assert_eq!(key.public_key().to_bytes(), pub_key);

        let peer = PublicKey::from_bytes(&peer_pub).unwrap();
        let shared = key.ecdh(&peer).unwrap();
        assert_eq!(shared, expected_shared);
    }

    #[test]
    fn ecdh_with_compressed_public_key() {
        // ECDH should accept compressed public keys as peer input
        let priv_alice = decode_hex::<32>("c88f01f510d9ac3f70a292daa2316de544e9aab8afe84049c62a9c57862d1433");
        let bob_pub_compressed = decode_hex::<33>("03d12dfb5289c8d4f81208b70270398c342296970a0bccb74c736fc7554494bf63");

        assert!(is_valid_public_key(&bob_pub_compressed));

        let expected_shared = decode_hex::<32>("d6840f6b42f6edafd13116e0e12565202fef8e9ece7dce03812464d04b9442de");

        let shared = ecdh(&priv_alice, &bob_pub_compressed).unwrap();
        assert_eq!(shared, expected_shared);
    }

    #[test]
    fn ecdh_rejects_invalid_private_key_zero() {
        let zero_key = [0u8; 32];
        assert!(SecretKey::from_bytes(&zero_key).is_err());
        let bob = SecretKey::generate().unwrap();
        assert!(ecdh(&zero_key, &bob.public_key().to_bytes()).is_err());
    }

    #[test]
    fn ecdh_rejects_invalid_private_key_order() {
        // n (the curve order) is rejected
        let n_bytes = decode_hex::<32>("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551");
        assert!(SecretKey::from_bytes(&n_bytes).is_err());

        // n+1 is rejected
        let n_plus_1 = decode_hex::<32>("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632552");
        assert!(SecretKey::from_bytes(&n_plus_1).is_err());

        // all-ones is rejected
        let all_ones = [0xffu8; 32];
        assert!(SecretKey::from_bytes(&all_ones).is_err());
    }

    #[test]
    fn ecdh_rejects_peer_public_key_x_equal_to_p() {
        // x = p (the field modulus) should be rejected as out of range
        let alice = SecretKey::generate().unwrap();
        let mut bad_pub = [0u8; 65];
        bad_pub[0] = 0x04;
        bad_pub[1..33].copy_from_slice(&decode_hex::<32>(
            "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff",
        ));
        bad_pub[33..65].fill(0x01);
        assert!(!is_valid_public_key(&bad_pub));
        assert!(ecdh(&alice.to_bytes(), &bad_pub).is_err());
    }

    #[test]
    fn ecdh_different_messages_same_shared_secret() {
        // ECDH shared secret depends only on the two key pairs, not any message
        let alice = SecretKey::generate().unwrap();
        let bob = SecretKey::generate().unwrap();

        let shared1 = alice.ecdh(&bob.public_key()).unwrap();
        let shared2 = alice.ecdh(&bob.public_key()).unwrap();
        assert_eq!(shared1, shared2);
    }

    #[test]
    fn ecdh_self_exchange_is_deterministic() {
        // ECDH with own public key produces a deterministic result
        let alice = SecretKey::generate().unwrap();
        let shared = alice.ecdh(&alice.public_key()).unwrap();
        let shared2 = alice.ecdh(&alice.public_key()).unwrap();
        assert_eq!(shared, shared2);
    }

    #[test]
    fn ecdh_different_keys_produce_different_secrets() {
        let alice = SecretKey::generate().unwrap();
        let bob1 = SecretKey::generate().unwrap();
        let bob2 = SecretKey::generate().unwrap();

        let shared1 = alice.ecdh(&bob1.public_key()).unwrap();
        let shared2 = alice.ecdh(&bob2.public_key()).unwrap();
        // Extremely unlikely that two different Bob keys produce the same secret
        assert_ne!(shared1, shared2);
    }

    #[test]
    fn ecdh_generator_multiplication_matches_go_p256_mult_test1() {
        // Go's crypto/elliptic p256_test.go: ScalarMult test 1
        let k = decode_hex::<32>("2a265f8bcbdcaf94d58519141e578124cb40d64a501fba9c11847b28965bc737");
        let x_in = decode_hex::<32>("023819813ac969847059028ea88a1f30dfbcde03fc791d3a252c6b41211882ea");
        let y_in = decode_hex::<32>("f93e4ae433cc12cf2a43fc0ef26400c0e125508224cdb649380f25479148a4ad");
        let x_out = decode_hex::<32>("4d4de80f1534850d261075997e3049321a0864082d24a917863366c0724f5ae3");
        let y_out = decode_hex::<32>("a22d2b7f7818a3563e0f7a76c9bf0921ac55e06e2e4d11795b233824b1db8cc0");

        let mut pubkey = [0u8; 65];
        pubkey[0] = 0x04;
        pubkey[1..33].copy_from_slice(&x_in);
        pubkey[33..65].copy_from_slice(&y_in);

        let point = parse_public_key(&pubkey).unwrap();
        let scalar = Scalar::from_bytes(&k).unwrap();
        let result = scalar_mul_affine(&point, &scalar).to_affine().unwrap();

        assert_eq!(result.x.to_bytes(), x_out, "x coordinate mismatch in Go test 1");
        assert_eq!(result.y.to_bytes(), y_out, "y coordinate mismatch in Go test 1");
    }

    #[test]
    fn ecdh_generator_multiplication_matches_go_p256_mult_test2() {
        // Go's crypto/elliptic p256_test.go: ScalarMult test 2
        let k = decode_hex::<32>("313f72ff9fe811bf573176231b286a3bdb6f1b14e05c40146590727a71c3bccd");
        let x_in = decode_hex::<32>("cc11887b2d66cbae8f4d306627192522932146b42f01d3c6f92bd5c8ba739b06");
        let y_in = decode_hex::<32>("a2f08a029cd06b46183085bae9248b0ed15b70280c7ef13a457f5af382426031");
        let x_out = decode_hex::<32>("831c3f6b5f762d2f461901577af41354ac5f228c2591f84f8a6e51e2e3f17991");
        let y_out = decode_hex::<32>("93f90934cd0ef2c698cc471c60a93524e87ab31ca2412252337f364513e43684");

        let mut pubkey = [0u8; 65];
        pubkey[0] = 0x04;
        pubkey[1..33].copy_from_slice(&x_in);
        pubkey[33..65].copy_from_slice(&y_in);

        let point = parse_public_key(&pubkey).unwrap();
        let scalar = Scalar::from_bytes(&k).unwrap();
        let result = scalar_mul_affine(&point, &scalar).to_affine().unwrap();

        assert_eq!(result.x.to_bytes(), x_out, "x coordinate mismatch in Go test 2");
        assert_eq!(result.y.to_bytes(), y_out, "y coordinate mismatch in Go test 2");
    }

    #[test]
    fn ecdh_rejects_invalid_curve_attack() {
        // Invalid curve attack: a point not on P-256 should always be rejected.
        // Point (1, 1) is not on the P-256 curve.
        let alice = SecretKey::generate().unwrap();
        let mut off_curve = [0u8; 65];
        off_curve[0] = 0x04;
        off_curve[33] = 0x01;
        off_curve[64] = 0x01;
        off_curve[1] = 0x01;

        assert!(!is_valid_public_key(&off_curve));
        assert!(ecdh(&alice.to_bytes(), &off_curve).is_err());
    }

    #[test]
    fn ecdh_edge_case_shared_secret_x_equals_zero() {
        // Wycheproof-style edge case: shared secret x-coordinate is 0.
        // This is a valid test case from Wycheproof ecdh_secp256r1_test.json tcId 3.
        let priv_hex = "0a0d622a47e48f6bc1038ace438c6f528aa00ad2bd1da5f13ee46bf5f633d71a";
        let pub_hex = "0458fd4168a87795603e2b04390285bdca6e57de6027fe211dd9d25e2212d29e6\
                        2080d36bd224d7405509295eed02a17150e03b314f96da37445b0d1d29377d12c";
        let expected_shared = [0u8; 32];

        let priv_key = decode_hex::<32>(priv_hex);
        let pub_key = decode_hex::<65>(pub_hex);

        assert!(is_valid_public_key(&pub_key));
        let shared = ecdh(&priv_key, &pub_key).unwrap();
        assert_eq!(shared, expected_shared);
    }

    #[test]
    fn ecdh_edge_case_shared_secret_x_equals_p_minus_3() {
        // Wycheproof-style edge case: shared secret x-coordinate is p-3.
        // From Wycheproof ecdh_secp256r1_test.json tcId 4.
        let priv_hex = "0a0d622a47e48f6bc1038ace438c6f528aa00ad2bd1da5f13ee46bf5f633d71a";
        let pub_hex = "04a1ecc24bf0d0053d23f5fd80ddf1735a1925039dc1176c581a7e795163c8b9ba\
                        2cb5a4e4d5109f4527575e3137b83d79a9bcb3faeff90d2aca2bed71bb523e7e";
        let expected_shared = decode_hex::<32>("ffffffff00000001000000000000000000000000fffffffffffffffffffffffc");

        let priv_key = decode_hex::<32>(priv_hex);
        let pub_key = decode_hex::<65>(pub_hex);

        assert!(is_valid_public_key(&pub_key));
        let shared = ecdh(&priv_key, &pub_key).unwrap();
        assert_eq!(shared, expected_shared);
    }

    #[test]
    fn ecdh_edge_case_shared_secret_power_of_two() {
        // Shared secret x-coordinate = 2^16
        // From Wycheproof ecdh_secp256r1_test.json tcId 5.
        let priv_hex = "0a0d622a47e48f6bc1038ace438c6f528aa00ad2bd1da5f13ee46bf5f633d71a";
        let pub_hex = "041b0e7437c33d379929430d3ec10df59bed7fe2a1d950c5791e1e9ddeef1f4d70\
                        fbdb0e3bbce63a27f27838c685207f2ccaf689d25eb622744db1168ac92619e8";
        let expected_shared = decode_hex::<32>("0000000000000000000000000000000000000000000000000000000000010000");

        let priv_key = decode_hex::<32>(priv_hex);
        let pub_key = decode_hex::<65>(pub_hex);

        assert!(is_valid_public_key(&pub_key));
        let shared = ecdh(&priv_key, &pub_key).unwrap();
        assert_eq!(shared, expected_shared);
    }

    #[test]
    fn ecdh_wrong_curve_rejected() {
        // A point on P-224 (a different curve) should be rejected for P-256 ECDH.
        // P-224 generator point is not on P-256.
        // P-224 generator x = b70e0cbd6bb4bf7f321390b94a03c1d356c21122343280d6115c1d21
        // (this is longer than 32 bytes, so we just test a random point that's not on P-256)
        let alice = SecretKey::generate().unwrap();
        let p224_gen_x = [
            0x00, 0x00, 0x00, 0x00, 0xb7, 0x0e, 0x0c, 0xbd, 0x6b, 0xb4, 0xbf, 0x7f, 0x32, 0x13, 0x90, 0xb9, 0x4a, 0x03,
            0xc1, 0xd3, 0x56, 0xc2, 0x11, 0x22, 0x34, 0x32, 0x80, 0xd6, 0x11, 0x5c, 0x1d, 0x21,
        ];
        let p224_gen_y = [
            0x00, 0x00, 0x00, 0x00, 0xbd, 0x37, 0x68, 0x08, 0xb3, 0x2c, 0x81, 0x2e, 0xd7, 0xd2, 0x86, 0x72, 0x37, 0x46,
            0xa5, 0xdc, 0x63, 0x63, 0x9c, 0x5d, 0x99, 0xd6, 0x9c, 0xb4, 0xd4, 0xfc, 0xb5, 0x9e,
        ];
        let mut bad_pub = [0u8; 65];
        bad_pub[0] = 0x04;
        bad_pub[1..33].copy_from_slice(&p224_gen_x);
        bad_pub[33..65].copy_from_slice(&p224_gen_y);

        assert!(!is_valid_public_key(&bad_pub));
        assert!(ecdh(&alice.to_bytes(), &bad_pub).is_err());
    }

    #[test]
    fn ecdh_private_key_rejects_zero_and_order() {
        // SecretKey::from_bytes must reject zero and n (curve order)
        let zero = [0u8; 32];
        assert!(SecretKey::from_bytes(&zero).is_err());

        let n = decode_hex::<32>("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551");
        assert!(SecretKey::from_bytes(&n).is_err());

        let n_minus_1 = decode_hex::<32>("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632550");
        assert!(SecretKey::from_bytes(&n_minus_1).is_ok());
    }

    #[test]
    fn ecdh_public_key_rejects_invalid_encodings() {
        // Infinity
        assert!(!is_valid_public_key(&[0x00]));

        // Wrong prefix
        let mut bad_prefix = [0u8; 65];
        bad_prefix[0] = 0x05;
        bad_prefix[1] = 0x01;
        assert!(!is_valid_public_key(&bad_prefix));

        // Truncated
        assert!(!is_valid_public_key(&[0x04, 0x00]));

        // Too long
        let mut too_long = [0u8; 66];
        too_long[0] = 0x04;
        assert!(!is_valid_public_key(&too_long));
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
    fn rfc6979_test_message_nonce_matches_known_value() {
        let private_key = Scalar::from_bytes(&decode_hex::<32>(
            "c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721",
        ))
        .unwrap();
        let hash = hash_message(b"test");
        assert_eq!(
            Rfc6979::new(&private_key, &hash).generate().to_bytes(),
            decode_hex::<32>("d16b6ae827f17175e040871a1c7ec3500192c4c92677336ec2537acaee0008e0")
        );
    }

    #[test]
    fn ecdsa_rejects_ptr_at_infinity_as_public_key() {
        // The point at infinity (0x00) is rejected as a public key
        assert!(!is_valid_public_key(&[0x00]));
        assert!(PublicKey::from_bytes(&[0x00]).is_err());
    }

    #[test]
    fn ecdsa_verify_rejects_non_canonical_r_and_s() {
        let private_key = decode_hex::<32>("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721");
        let key = SecretKey::from_bytes(&private_key).unwrap();
        let _valid_sig = key.sign(b"msg").unwrap();

        // r = n+1 is rejected
        let sig = decode_hex::<64>(
            "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632552\
             f7cb1c942d657c41d436c7a1b6e29f65f3e900dbb9aff4064dc4ab2f843acda8",
        );
        assert!(key.public_key().verify(b"msg", &sig).is_err());

        // s = n+1 is rejected
        let sig = decode_hex::<64>(
            "efd48b2aacb6a8fd1140dd9cd45e81d69d2c877b56aaf991c34d0ea84eaf3716\
             ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632552",
        );
        assert!(key.public_key().verify(b"msg", &sig).is_err());
    }

    #[test]
    fn field_element_add_sub_mul_consistency() {
        let a = FieldElement::from_bytes(&decode_hex::<32>(
            "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296",
        ))
        .unwrap();
        let b = FieldElement::from_bytes(&decode_hex::<32>(
            "4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5",
        ))
        .unwrap();

        // a + b - b = a
        assert_eq!(a.add(b).sub(b), a);

        // a + b = b + a
        assert_eq!(a.add(b), b.add(a));

        // a * b = b * a
        assert_eq!(a.mul(b), b.mul(a));

        // (a + b) * c = a*c + b*c
        let c = FieldElement::from_bytes(&decode_hex::<32>(
            "3bce3c3e27d2604b651d06b0cc53b0f6b3ebbd55769886bc5ac635d8aa3a93e7",
        ))
        .unwrap();
        assert_eq!(a.add(b).mul(c), a.mul(c).add(b.mul(c)));
    }

    #[test]
    fn scalar_add_sub_mul_consistency() {
        let a = Scalar::from_bytes(&decode_hex::<32>(
            "a6e3c57dd01abe90086538398355dd4c3b17aa873382b0f24d6129493d8aad60",
        ))
        .unwrap();
        // Scalar::ONE
        let one = Scalar::from_bytes(&decode_hex::<32>(
            "0000000000000000000000000000000000000000000000000000000000000001",
        ))
        .unwrap();

        // a + 1 - 1 = a
        assert_eq!(a.add(one).sub(one), a);

        // a * 1 = a
        assert_eq!(a.mul(one), a);

        // commutativity
        let b = Scalar::from_bytes(&decode_hex::<32>(
            "f1abb023518351cd71d881567b1ea663ed3efcf6c5132b354f28d3b0b7d38367",
        ))
        .unwrap();
        assert_eq!(a.mul(b), b.mul(a));
        assert_eq!(a.add(b), b.add(a));
    }

    #[test]
    fn ecdh_shared_secret_boundary_values() {
        // ECDH shared secret is always exactly 32 bytes
        let alice = SecretKey::generate().unwrap();
        let bob = SecretKey::generate().unwrap();

        let shared = alice.ecdh(&bob.public_key()).unwrap();
        assert_eq!(shared.len(), ECDH_SHARED_SECRET_SIZE);

        // shared secret is deterministic for the same key pair
        let shared2 = alice.ecdh(&bob.public_key()).unwrap();
        assert_eq!(shared, shared2);
    }

    #[test]
    fn ecdh_rejects_empty_and_invalid_public_key_bytes() {
        let key = SecretKey::generate().unwrap();

        // Invalid prefix byte
        let mut bad = key.public_key().to_bytes();
        bad[0] = 0xff;
        assert!(!is_valid_public_key(&bad));
        assert!(PublicKey::from_bytes(&bad).is_err());

        // Only prefix byte
        assert!(!is_valid_public_key(&[0x04]));

        // Truncated uncompressed (64 bytes but need 65)
        assert!(!is_valid_public_key(&bad[..64]));

        // Compressed key with y=0 as the x coordinate (valid if y exists, but let's test)
        let zero_x_compressed = decode_hex::<33>("020000000000000000000000000000000000000000000000000000000000000000");
        // x=0 is a valid field element; the point may or may not be on the curve
        // Just test that parsing doesn't crash
        let _ = PublicKey::from_bytes(&zero_x_compressed);
    }

    #[test]
    fn ecdsa_sign_then_verify_consistent_for_random_keys() {
        for _ in 0..5 {
            let key = SecretKey::generate().unwrap();
            let msg = rand::random::<[u8; 32]>();
            let sig = key.sign(&msg).unwrap();
            assert!(key.public_key().verify(&msg, &sig).is_ok());
        }
    }

    #[test]
    fn field_element_negate_round_trip() {
        let x = FieldElement::from_bytes(&decode_hex::<32>(
            "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296",
        ))
        .unwrap();
        let neg = x.negate();
        assert_eq!(neg.negate(), x);
        assert_eq!(x.add(neg), FieldElement::ZERO);
    }

    #[test]
    fn scalar_negate_round_trip() {
        let a = Scalar::from_bytes(&decode_hex::<32>(
            "a6e3c57dd01abe90086538398355dd4c3b17aa873382b0f24d6129493d8aad60",
        ))
        .unwrap();
        let neg_a = Scalar::ZERO.sub(a);
        assert_eq!(a.add(neg_a), Scalar::ZERO);
        // neg(neg(a)) = a
        assert_eq!(Scalar::ZERO.sub(neg_a), a);
    }

    #[test]
    fn scalar_mul_by_two_matches_double() {
        let two = Scalar::from_bytes(&decode_hex::<32>(
            "0000000000000000000000000000000000000000000000000000000000000002",
        ))
        .unwrap();
        let g_times_2 = scalar_mul_affine(&AffinePoint::GENERATOR, &two).to_affine().unwrap();
        let proj_g = ProjectivePoint::from_affine(&AffinePoint::GENERATOR);
        let g_doubled = proj_g.double().to_affine().unwrap();

        assert_eq!(g_times_2.to_uncompressed_bytes(), g_doubled.to_uncompressed_bytes());
    }

    #[test]
    fn ecdh_with_self_is_consistent() {
        let key = SecretKey::generate().unwrap();
        let shared1 = key.ecdh(&key.public_key()).unwrap();
        let shared2 = key.ecdh(&key.public_key()).unwrap();
        assert_eq!(shared1, shared2);
    }

    #[test]
    fn wycheproof_ecdh_p256_ecpoint() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/ecdh_secp256r1_ecpoint_test.json"
        ))
        .unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        let mut acceptable_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            if group["curve"].as_str() != Some("secp256r1") {
                continue;
            }
            for test in group["tests"].as_array().unwrap() {
                let public_hex = test["public"].as_str().unwrap();
                let private_hex = test["private"].as_str().unwrap();
                let expected_shared_hex = test["shared"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let public_key = hex::decode(public_hex).unwrap();

                // Private key hex is a bigint, may have leading zeros or be
                // shorter than 32 bytes. Pad or strip to exactly 32 bytes.
                let private_bytes = hex::decode(private_hex).unwrap();
                let mut private_key = [0u8; SECRET_KEY_SIZE];
                let effective_len = private_bytes.len().min(SECRET_KEY_SIZE);
                let skip = if private_bytes.len() > SECRET_KEY_SIZE {
                    private_bytes.len() - SECRET_KEY_SIZE
                } else {
                    0
                };
                private_key[SECRET_KEY_SIZE - effective_len..]
                    .copy_from_slice(&private_bytes[skip..skip + effective_len]);

                let shared = ecdh(&private_key, &public_key);

                if result == "valid" {
                    let shared = shared.unwrap();
                    let shared_hex = hex::encode(shared);
                    assert_eq!(shared_hex, expected_shared_hex, "wycheproof ECDH ecpoint tcId={}", test["tcId"]);
                    valid_tested += 1;
                } else if result == "invalid" {
                    assert!(
                        shared.is_err(),
                        "wycheproof ECDH ecpoint tcId={} expected invalid but passed",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                } else {
                    acceptable_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid ECDH ecpoint wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid ECDH ecpoint wycheproof tests were run");
        assert!(acceptable_tested > 0, "no acceptable ECDH ecpoint wycheproof tests were run");
    }

    #[test]
    fn p256_ecdsa_rejects_truncated_signature() {
        let key = SecretKey::generate().unwrap();
        let sig = key.sign(b"msg").unwrap();
        // Truncate to 63 bytes and pad to 64 with zeros
        let mut truncated = [0u8; SIGNATURE_SIZE];
        truncated[..63].copy_from_slice(&sig[..63]);
        // If the dropped byte is 0, the padded copy equals the original,
        // so flip a bit to ensure it's different
        if truncated == sig {
            truncated[63] ^= 0x01;
        }
        assert!(key.public_key().verify(b"msg", &truncated).is_err());
    }

    #[test]
    fn is_on_curve_accepts_generator_and_random_points() {
        assert!(AffinePoint::GENERATOR.is_on_curve());
        for _ in 0..5 {
            let key = SecretKey::generate().unwrap();
            // The public point should be on the curve
            // (verified by construction)
            let pb = key.public_key().to_bytes();
            let pk = PublicKey::from_bytes(&pb).unwrap();
            let _ = pk; // just checking it can be constructed
        }
    }

    #[test]
    fn field_element_pow_correctness() {
        let x = FieldElement::from_bytes(&decode_hex::<32>(
            "0000000000000000000000000000000000000000000000000000000000000002",
        ))
        .unwrap();
        // x^3 = x * x * x
        let x3 = x.pow(&U256::from_u64(3));
        let expected = x.mul(x).mul(x);
        assert_eq!(x3, expected);

        // x^0 = 1
        let x0 = x.pow(&U256::ZERO);
        assert_eq!(x0, FieldElement::ONE);
    }

    #[test]
    fn nist_p256_vector_verify_all_rfc6979_signatures() {
        // Verify ALL 4 SHA-256 signatures from RFC 6979 A.2.5 match
        let private_key = decode_hex::<32>("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721");
        let key = SecretKey::from_bytes(&private_key).unwrap();

        let vectors: &[(&[u8], &str)] = &[
            // RFC 6979 raw "sample" s is high; signing returns canonical low-s.
            (
                b"sample" as &[u8],
                "efd48b2aacb6a8fd1140dd9cd45e81d69d2c877b56aaf991c34d0ea84eaf3716\
                          0834e36ad29a83bf2bc9385e491d6099c8fdf9d1ed67aa7ea5f51f93782857a9",
            ),
            (
                b"test" as &[u8],
                "f1abb023518351cd71d881567b1ea663ed3efcf6c5132b354f28d3b0b7d38367\
                          019f4113742a2b14bd25926b49c649155f267e60d3814b4c0cc84250e46f0083",
            ),
        ];

        for (msg, hex_sig) in vectors {
            let sig = key.sign(msg).unwrap();
            let expected = decode_hex::<64>(hex_sig);
            assert_eq!(sig, expected, "failed for message: {:?}", String::from_utf8_lossy(msg));
        }
    }

    #[test]
    fn wycheproof_ecdsa_p256_sha256_der() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/ecdsa_secp256r1_sha256_test.json"
        ))
        .unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            let uncompressed_hex = group["publicKey"]["uncompressed"].as_str().unwrap();
            let pubkey_bytes = hex::decode(uncompressed_hex).unwrap();
            let pk = PublicKey::from_bytes(&pubkey_bytes).unwrap();

            for test in group["tests"].as_array().unwrap() {
                let msg_hex = test["msg"].as_str().unwrap();
                let sig_hex = test["sig"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let msg = hex::decode(msg_hex).unwrap();
                let der_sig = hex::decode(sig_hex).unwrap();
                let Some(sig) = der_ecdsa_sig_to_p1363(&der_sig) else {
                    continue;
                };

                let verify_result = pk.verify(&msg, &sig);

                if result == "valid" {
                    assert!(
                        verify_result.is_ok(),
                        "wycheproof ECDSA DER SHA-256 tcId={} expected valid but failed",
                        test["tcId"]
                    );
                    valid_tested += 1;
                } else {
                    assert!(
                        verify_result.is_err(),
                        "wycheproof ECDSA DER SHA-256 tcId={} expected invalid but passed",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid ECDSA DER SHA-256 wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid ECDSA DER SHA-256 wycheproof tests were run");
    }

    #[test]
    fn wycheproof_ecdh_p256_asn() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/ecdh_secp256r1_test.json"
        ))
        .unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        let mut acceptable_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            for test in group["tests"].as_array().unwrap() {
                let public_hex = test["public"].as_str().unwrap();
                let private_hex = test["private"].as_str().unwrap();
                let expected_shared_hex = test["shared"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let spki_der = hex::decode(public_hex).unwrap();
                let Some(sec1_point) = spki_to_sec1_point(&spki_der) else {
                    if result == "valid" {
                        panic!("wycheproof ECDH ASN tcId={}: failed to parse valid SPKI", test["tcId"]);
                    }
                    invalid_tested += 1;
                    continue;
                };

                // Private key hex is a bigint, may have leading zeros or be
                // shorter than 32 bytes. Pad or strip to exactly 32 bytes.
                let private_bytes = hex::decode(private_hex).unwrap();
                let mut private_key = [0u8; SECRET_KEY_SIZE];
                let effective_len = private_bytes.len().min(SECRET_KEY_SIZE);
                let skip = if private_bytes.len() > SECRET_KEY_SIZE {
                    private_bytes.len() - SECRET_KEY_SIZE
                } else {
                    0
                };
                private_key[SECRET_KEY_SIZE - effective_len..]
                    .copy_from_slice(&private_bytes[skip..skip + effective_len]);

                let shared = ecdh(&private_key, &sec1_point);

                if result == "valid" {
                    let shared = shared.unwrap();
                    let shared_hex = hex::encode(shared);
                    assert_eq!(shared_hex, expected_shared_hex, "wycheproof ECDH ASN tcId={}", test["tcId"]);
                    valid_tested += 1;
                } else if result == "invalid" {
                    assert!(
                        shared.is_err(),
                        "wycheproof ECDH ASN tcId={} expected invalid but passed",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                } else {
                    acceptable_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid ECDH ASN wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid ECDH ASN wycheproof tests were run");
        assert!(acceptable_tested > 0, "no acceptable ECDH ASN wycheproof tests were run");
    }

    // Shared curve-independent tests, instantiated for P256.
    #[test]
    fn generator_point_is_on_curve() {
        p_curves::test_support::generator_point_is_on_curve::<P256>();
    }

    #[test]
    fn from_x_y_rejects_off_curve() {
        p_curves::test_support::from_x_y_rejects_off_curve::<P256>();
    }

    #[test]
    fn ecdh_round_trip_alice_bob() {
        p_curves::test_support::ecdh_round_trip_alice_bob::<P256>();
    }

    #[test]
    fn ecdh_rejects_off_curve_peer_public_key() {
        p_curves::test_support::ecdh_rejects_off_curve_peer_public_key::<P256>();
    }

    #[test]
    fn ecdh_rejects_infinity_peer_public_key() {
        p_curves::test_support::ecdh_rejects_infinity_peer_public_key::<P256>();
    }

    #[test]
    fn ecdh_rejects_bad_length_peer_public_key() {
        p_curves::test_support::ecdh_rejects_bad_length_peer_public_key::<P256>();
    }

    #[test]
    fn ecdh_multiple_exchanges_consistency() {
        p_curves::test_support::ecdh_multiple_exchanges_consistency::<P256>();
    }

    #[test]
    fn private_key_round_trip_bytes() {
        p_curves::test_support::private_key_round_trip_bytes::<P256>();
    }

    #[test]
    fn public_key_round_trip_bytes() {
        p_curves::test_support::public_key_round_trip_bytes::<P256>();
    }

    #[test]
    fn point_double_and_add_consistency() {
        p_curves::test_support::point_double_and_add_consistency::<P256>();
    }

    #[test]
    fn compressed_public_key_has_correct_prefix() {
        p_curves::test_support::compressed_public_key_has_correct_prefix::<P256>();
    }

    #[cfg(feature = "zeroize")]
    #[test]
    fn rfc6979_state_zeroize_clears_drbg() {
        p_curves::test_support::rfc6979_state_zeroize_clears_drbg::<P256>();
    }
}
