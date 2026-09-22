//! P-384 (secp384r1) ECDSA and ECDH.
//!
//! See [`SecretKey`] and [`PublicKey`] for the signing, verification and key
//! agreement APIs.

use big_number::{Uint, mac};

use super::p_curves::{self, Curve, UintOps, field_pow};
use crate::{EllipticCurveError, Hasher, hmac::Hmac, sha2::Sha384};

/// Size of a P-384 secret key in bytes (48 bytes).
pub const SECRET_KEY_SIZE: usize = 48;
/// Size of a compressed P-384 public key in bytes (49 bytes, includes 0x02/0x03 prefix).
pub const PUBLIC_KEY_COMPRESSED_SIZE: usize = 49;
/// Size of an uncompressed P-384 public key in bytes (97 bytes, includes 0x04 prefix).
pub const PUBLIC_KEY_UNCOMPRESSED_SIZE: usize = 97;
/// Size of a P-384 ECDSA signature in bytes (96 bytes, r || s).
pub const SIGNATURE_SIZE: usize = 96;
/// Size of the raw ECDH shared secret in bytes (48 bytes). **Must not** be used directly
/// as an encryption key; apply a KDF first.
pub const ECDH_SHARED_SECRET_SIZE: usize = 48;

/// P-384 (secp384r1) ECDSA secret key.
///
/// Supports signing and ECDH key agreement.
///
/// # Signing
///
/// ```ignore
/// use crypto::p384::SecretKey;
///
/// let key = SecretKey::generate().unwrap();
/// let signature = key.sign(b"message").unwrap();
/// ```
///
/// # ECDH key exchange
///
/// ```ignore
/// use crypto::p384::SecretKey;
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
pub struct SecretKey(p_curves::SecretKey<P384>);

impl SecretKey {
    /// Generates a fresh random secret key.
    #[cfg(feature = "random")]
    pub fn generate() -> Result<SecretKey, EllipticCurveError> {
        Ok(SecretKey(p_curves::SecretKey::generate()?))
    }

    /// Builds a secret key from its 48-byte big-endian scalar.
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

    /// Signs `message` with ECDSA using SHA-384 and deterministic (RFC 6979)
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

    /// Returns the secret scalar as 48 big-endian bytes.
    pub fn to_bytes(&self) -> [u8; SECRET_KEY_SIZE] {
        self.0.to_bytes()
    }
}

/// P-384 (secp384r1) ECDSA public key.
///
/// Supports signature verification and ECDH key agreement.
///
/// # Verification
///
/// ```ignore
/// use crypto::p384::SecretKey;
///
/// let key = SecretKey::generate().unwrap();
/// let signature = key.sign(b"message").unwrap();
/// assert!(key.public_key().verify(b"message", &signature).is_ok());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PublicKey(p_curves::PublicKey<P384>);

impl PublicKey {
    /// Parses a compressed (49-byte) or uncompressed (97-byte) SEC1 encoding.
    ///
    /// Returns [`EllipticCurveError::InvalidKey`] when the encoding is not a
    /// valid point on the P-384 curve.
    #[inline]
    pub fn from_bytes(key: &[u8]) -> Result<PublicKey, EllipticCurveError> {
        Ok(PublicKey(p_curves::PublicKey::from_bytes(key)?))
    }

    /// Builds a public key from raw affine x and y coordinates (both
    /// big-endian, 48 bytes each). Returns `InvalidKey` if the coordinates
    /// are not a valid point on the P-384 curve.
    ///
    /// This is useful when importing keys from formats like JWK where `x`
    /// and `y` are available directly.
    #[inline]
    pub fn from_x_y(x_bytes: &[u8; 48], y_bytes: &[u8; 48]) -> Result<PublicKey, EllipticCurveError> {
        Ok(PublicKey(p_curves::PublicKey::from_x_y(x_bytes, y_bytes)?))
    }

    /// Verifies an ECDSA signature over `message` using SHA-384.
    ///
    /// Both canonical low-s and non-canonical high-s signatures are accepted,
    /// matching typical ECDSA interoperability. Use [`Self::verify_strict`] to
    /// additionally reject high-s signatures.
    pub fn verify(&self, message: &[u8], signature: &[u8; SIGNATURE_SIZE]) -> Result<(), EllipticCurveError> {
        self.0.verify(message, signature)
    }

    /// Verifies an ECDSA signature over `message` using SHA-384, additionally
    /// rejecting non-canonical high-s signatures (`s > n / 2`).
    ///
    /// Use this when signature malleability must be excluded (Bitcoin/EIP-2
    /// style). Note that some third-party signers emit high-s signatures, which
    /// this method will reject; [`Self::verify`] accepts both forms.
    pub fn verify_strict(&self, message: &[u8], signature: &[u8; SIGNATURE_SIZE]) -> Result<(), EllipticCurveError> {
        self.0.verify_strict(message, signature)
    }

    /// Returns the 97-byte uncompressed SEC1 encoding (`0x04 || x || y`).
    #[inline]
    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_UNCOMPRESSED_SIZE] {
        self.0.to_bytes()
    }

    /// Returns the 49-byte compressed SEC1 encoding (`0x02`/`0x03 || x`).
    #[inline]
    pub fn to_compressed_bytes(&self) -> [u8; PUBLIC_KEY_COMPRESSED_SIZE] {
        self.0.to_compressed_bytes()
    }

    /// Returns the `X` and `Y` points as big-endian arrays.
    #[inline]
    pub fn x_y(&self) -> ([u8; 48], [u8; 48]) {
        self.0.x_y()
    }
}

/// Marker type carrying the P-384 curve parameters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct P384;

type U384 = Uint<384, 6>;
type CurveUint = U384;

const MODULUS_P: CurveUint = CurveUint::from_limbs([
    0x00000000ffffffff,
    0xffffffff00000000,
    0xfffffffffffffffe,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
]);

const MODULUS_N: CurveUint = CurveUint::from_limbs([
    0xecec196accc52973,
    0x581a0db248b0a77a,
    0xc7634d81f4372ddf,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
]);

// floor(n / 2). A signature scalar `s` is "high" when `s > N_HALF`, in which
// case it is normalized to `n - s` so that signatures are non-malleable.
const N_HALF: CurveUint = CurveUint::from_limbs([
    0x76760cb5666294b9,
    0xac0d06d9245853bd,
    0xe3b1a6c0fa1b96ef,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0x7fffffffffffffff,
]);

const P_MINUS_TWO: CurveUint = CurveUint::from_limbs([
    0x00000000fffffffd,
    0xffffffff00000000,
    0xfffffffffffffffe,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
]);

const P_PLUS_ONE_OVER_FOUR: CurveUint = CurveUint::from_limbs([
    0x0000000040000000,
    0xbfffffffc0000000,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0x3fffffffffffffff,
]);

const N_MINUS_TWO: CurveUint = CurveUint::from_limbs([
    0xecec196accc52971,
    0x581a0db248b0a77a,
    0xc7634d81f4372ddf,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
]);

const CURVE_B: CurveUint = CurveUint::from_limbs([
    0x2a85c8edd3ec2aef,
    0xc656398d8a2ed19d,
    0x0314088f5013875a,
    0x181d9c6efe814112,
    0x988e056be3f82d19,
    0xb3312fa7e23ee7e4,
]);

const GENERATOR_X: CurveUint = CurveUint::from_limbs([
    0x3a545e3872760ab7,
    0x5502f25dbf55296c,
    0x59f741e082542a38,
    0x6e1d3b628ba79b98,
    0x8eb1c71ef320ad74,
    0xaa87ca22be8b0537,
]);

const GENERATOR_Y: CurveUint = CurveUint::from_limbs([
    0x7a431d7c90ea0e5f,
    0x0a60b1ce1d7e819d,
    0xe9da3113b5f0b8c0,
    0xf8f41dbd289a147c,
    0x5d9e98bf9292dc29,
    0x3617de4a96262c6f,
]);

// P-384 fast reduction constants: S_i = 2^(64i) mod p for i=6..11
// Derived from p = 2^384 - 2^128 - 2^96 + 2^32 - 1
// Each S_i is a 6-limb U384 value.
const S6: [u64; 6] = [
    0xffffffff00000001,
    0x00000000ffffffff,
    0x0000000000000001,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
];

const S7: [u64; 6] = [
    0x0000000000000000,
    0xffffffff00000001,
    0x00000000ffffffff,
    0x0000000000000001,
    0x0000000000000000,
    0x0000000000000000,
];

const S8: [u64; 6] = [
    0x0000000000000000,
    0x0000000000000000,
    0xffffffff00000001,
    0x00000000ffffffff,
    0x0000000000000001,
    0x0000000000000000,
];

const S9: [u64; 6] = [
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0xffffffff00000001,
    0x00000000ffffffff,
    0x0000000000000001,
];

const S10: [u64; 6] = [
    0xffffffff00000001,
    0x00000000ffffffff,
    0x0000000000000001,
    0x0000000000000000,
    0xffffffff00000001,
    0x00000000ffffffff,
];

const S11: [u64; 6] = [
    0x00000001ffffffff,
    0xfffffffe00000000,
    0x00000001ffffffff,
    0x0000000000000001,
    0x0000000000000000,
    0xffffffff00000001,
];

#[inline]
fn ct_select_u128(a: u128, b: u128, choice: bool) -> u128 {
    let mask = (choice as u128).wrapping_neg();
    (a & mask) | (b & !mask)
}

// P-384 fast modular multiplication using u128 accumulators.
// All loops run fixed iteration counts with ct_select for constant-time.
fn p384_fast_mul_mod(a: &CurveUint, b: &CurveUint) -> CurveUint {
    let al = a.limbs;
    let bl = b.limbs;

    let mut prod = [0u64; 12];
    for i in 0..6 {
        let mut carry = 0u64;
        for j in 0..6 {
            let (v, cc) = mac(prod[i + j], al[i], bl[j], carry);
            prod[i + j] = v;
            carry = cc;
        }
        prod[i + 6] = carry;
    }

    const MASK: u128 = 0xffffffffffffffff;
    let c0 = [
        S6[0] as u128,
        S6[1] as u128,
        S6[2] as u128,
        S6[3] as u128,
        S6[4] as u128,
        S6[5] as u128,
    ];
    let c1 = [
        S7[0] as u128,
        S7[1] as u128,
        S7[2] as u128,
        S7[3] as u128,
        S7[4] as u128,
        S7[5] as u128,
    ];
    let c2 = [
        S8[0] as u128,
        S8[1] as u128,
        S8[2] as u128,
        S8[3] as u128,
        S8[4] as u128,
        S8[5] as u128,
    ];
    let c3 = [
        S9[0] as u128,
        S9[1] as u128,
        S9[2] as u128,
        S9[3] as u128,
        S9[4] as u128,
        S9[5] as u128,
    ];
    let c4 = [
        S10[0] as u128,
        S10[1] as u128,
        S10[2] as u128,
        S10[3] as u128,
        S10[4] as u128,
        S10[5] as u128,
    ];
    let c5 = [
        S11[0] as u128,
        S11[1] as u128,
        S11[2] as u128,
        S11[3] as u128,
        S11[4] as u128,
        S11[5] as u128,
    ];
    let coeffs: [&[u128; 6]; 6] = [&c0, &c1, &c2, &c3, &c4, &c5];

    let mut r0 = prod[0] as u128;
    let mut r1 = prod[1] as u128;
    let mut r2 = prod[2] as u128;
    let mut r3 = prod[3] as u128;
    let mut r4 = prod[4] as u128;
    let mut r5 = prod[5] as u128;

    for i in 0..6 {
        let w = prod[6 + i] as u128;
        let c = coeffs[i];

        r0 = r0.wrapping_add(w.wrapping_mul(c[0]));
        r1 = r1.wrapping_add(w.wrapping_mul(c[1]));
        r2 = r2.wrapping_add(w.wrapping_mul(c[2]));
        r3 = r3.wrapping_add(w.wrapping_mul(c[3]));
        r4 = r4.wrapping_add(w.wrapping_mul(c[4]));
        r5 = r5.wrapping_add(w.wrapping_mul(c[5]));

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
            let carry = r3 >> 64;
            r4 = r4.wrapping_add(carry);
            r3 &= MASK;
            let carry = r4 >> 64;
            r5 = r5.wrapping_add(carry);
            r4 &= MASK;

            let residual = r5 >> 64;
            let need_reduce = residual != 0;

            let rr5 = r5 & MASK;
            let rr0 = r0.wrapping_add(residual.wrapping_mul(c0[0]));
            let rr1 = r1.wrapping_add(residual.wrapping_mul(c0[1]));
            let rr2 = r2.wrapping_add(residual.wrapping_mul(c0[2]));
            let rr3 = r3.wrapping_add(residual.wrapping_mul(c0[3]));
            let rr4 = r4.wrapping_add(residual.wrapping_mul(c0[4]));
            let rr5r = rr5.wrapping_add(residual.wrapping_mul(c0[5]));

            r0 = ct_select_u128(rr0, r0, need_reduce);
            r1 = ct_select_u128(rr1, r1, need_reduce);
            r2 = ct_select_u128(rr2, r2, need_reduce);
            r3 = ct_select_u128(rr3, r3, need_reduce);
            r4 = ct_select_u128(rr4, r4, need_reduce);
            r5 = ct_select_u128(rr5r, r5, need_reduce);
        }
    }

    // Fixed 8 conditional subtractions (result may be up to ~16×p).
    let mut result = CurveUint::from_limbs([r0 as u64, r1 as u64, r2 as u64, r3 as u64, r4 as u64, r5 as u64]);
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

impl Curve for P384 {
    type U = CurveUint;
    type FieldBytes = [u8; SECRET_KEY_SIZE];
    type DigestBytes = [u8; 48];
    type CompressedBytes = [u8; PUBLIC_KEY_COMPRESSED_SIZE];
    type UncompressedBytes = [u8; PUBLIC_KEY_UNCOMPRESSED_SIZE];
    type SignatureBytes = [u8; SIGNATURE_SIZE];

    const FIELD_BITS: usize = 384;
    const FIELD_BYTES: usize = SECRET_KEY_SIZE;
    const DIGEST_BYTES: usize = 48;
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
        p384_fast_mul_mod(a, b)
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
        Sha384::hash(data).as_ref().try_into().unwrap()
    }

    #[inline]
    fn hmac(key: &[u8], data: &[u8]) -> Self::DigestBytes {
        Hmac::<Sha384>::mac(key, data).as_ref().try_into().unwrap()
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
    fn random_secret_key() -> Self::FieldBytes {
        crate::random::random_bytes()
    }
}

/// Returns `true` when `public_key` is a valid SEC1 encoding of a point on the
/// P-384 curve.
pub fn is_valid_public_key(public_key: &[u8]) -> bool {
    p_curves::is_valid_public_key::<P384>(public_key)
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    use super::*;
    use crate::p_curves::p_curves;

    type FieldElement = p_curves::FieldElement<P384>;
    type Scalar = p_curves::Scalar<P384>;
    type AffinePoint = p_curves::AffinePoint<P384>;
    type ProjectivePoint = p_curves::ProjectivePoint<P384>;
    type Rfc6979 = p_curves::Rfc6979<P384>;

    // Concrete wrappers around the generic core helpers.
    fn hash_message(message: &[u8]) -> [u8; 48] {
        p_curves::hash_message::<P384>(message)
    }

    fn hmac_digest(key: &[u8], data: &[u8]) -> [u8; 48] {
        p_curves::hmac_digest::<P384>(key, data)
    }

    fn bits2octets(hash: &[u8; 48]) -> [u8; SECRET_KEY_SIZE] {
        p_curves::bits2octets::<P384>(hash)
    }

    fn derive_public_key_uncompressed(
        private_key: &[u8; SECRET_KEY_SIZE],
    ) -> Result<[u8; PUBLIC_KEY_UNCOMPRESSED_SIZE], crate::EllipticCurveError> {
        p_curves::derive_public_key_uncompressed::<P384>(private_key)
    }

    fn derive_public_key_compressed(
        private_key: &[u8; SECRET_KEY_SIZE],
    ) -> Result<[u8; PUBLIC_KEY_COMPRESSED_SIZE], crate::EllipticCurveError> {
        p_curves::derive_public_key_compressed::<P384>(private_key)
    }

    fn ecdsa_sign_inner_impl(
        scalar: &Scalar,
        message: &[u8],
        force_first_retry: bool,
    ) -> Result<[u8; SIGNATURE_SIZE], crate::EllipticCurveError> {
        p_curves::ecdsa_sign_inner_impl::<P384>(scalar, message, force_first_retry)
    }

    fn ecdsa_verify_inner(
        public_point: &AffinePoint,
        message: &[u8],
        signature: &[u8; SIGNATURE_SIZE],
    ) -> Result<(), crate::EllipticCurveError> {
        p_curves::ecdsa_verify_inner::<P384>(public_point, message, signature)
    }

    fn parse_public_key(public_key: &[u8]) -> Result<AffinePoint, crate::EllipticCurveError> {
        p_curves::parse_public_key::<P384>(public_key)
    }

    fn scalar_mul_generator(scalar: &Scalar) -> ProjectivePoint {
        p_curves::scalar_mul_generator::<P384>(scalar)
    }

    fn scalar_mul_affine(base: &AffinePoint, scalar: &Scalar) -> ProjectivePoint {
        p_curves::scalar_mul_affine::<P384>(base, scalar)
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
    fn private_key_zeroize_clears_scalar() {
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

    fn der_ecdsa_sig_to_p1363(der: &[u8]) -> Option<[u8; 96]> {
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
        if rtag != 0x02 || rval.is_empty() || rval.len() > 49 {
            return None;
        }
        let (stag, sval) = der_read_tlv(inner, &mut inner_offset)?;
        if stag != 0x02 || sval.is_empty() || sval.len() > 49 {
            return None;
        }
        if inner_offset != inner.len() {
            return None;
        }
        let r_valid = if rval.len() == 48 && rval[0] >= 0x80 {
            false
        } else if rval.len() == 49 && rval[0] != 0 {
            false
        } else if rval.len() == 49 && rval[0] == 0 && rval[1] < 0x80 {
            false
        } else if rval.len() > 49 {
            false
        } else {
            true
        };
        let s_valid = if sval.len() == 48 && sval[0] >= 0x80 {
            false
        } else if sval.len() == 49 && sval[0] != 0 {
            false
        } else if sval.len() == 49 && sval[0] == 0 && sval[1] < 0x80 {
            false
        } else if sval.len() > 49 {
            false
        } else {
            true
        };
        if !r_valid || !s_valid {
            return None;
        }

        let r_trimmed = if rval.len() == 49 && rval[0] == 0 {
            &rval[1..]
        } else {
            rval
        };
        let s_trimmed = if sval.len() == 49 && sval[0] == 0 {
            &sval[1..]
        } else {
            sval
        };
        if r_trimmed.len() > 48 || s_trimmed.len() > 48 {
            return None;
        }
        let mut sig = [0u8; 96];
        sig[48 - r_trimmed.len()..48].copy_from_slice(r_trimmed);
        sig[96 - s_trimmed.len()..96].copy_from_slice(s_trimmed);
        Some(sig)
    }

    fn spki_to_sec1_point(spki: &[u8]) -> Option<Vec<u8>> {
        let ec_public_key_oid: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01];
        let secp384r1_oid: &[u8] = &[0x2b, 0x81, 0x04, 0x00, 0x22];
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
        if oid2_tag != 0x06 || oid2 != secp384r1_oid {
            return None;
        }
        let (_bs_tag, bs_val) = der_read_tlv(outer, &mut inner)?;
        if _bs_tag != 0x03 || bs_val.is_empty() {
            return None;
        }
        Some(bs_val[1..].to_vec())
    }

    #[test]
    fn derive_public_key_generator_matches_sec1_base_point() {
        let mut private_key = [0u8; 48];
        private_key[47] = 1;
        let derived = derive_public_key_uncompressed(&private_key).unwrap();
        let expected = decode_hex::<97>(
            "04aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a38\
             5502f25dbf55296c3a545e3872760ab73617de4a96262c6f5d9e98bf9292dc29\
             f8f41dbd289a147ce9da3113b5f0b8c00a60b1ce1d7e819d7a431d7c90ea0e5f",
        );
        assert_eq!(derived, expected);
    }

    #[test]
    fn ecdsa_verify_accepts_compressed_and_uncompressed_public_keys() {
        let private_key = decode_hex::<48>(
            "6b9d3dad2e1b8c1c05b19875b6659f4de23c3b667bf297ba9aa47740787137d8\
             96d5724e4c70a825f872c9ea60d2edf5",
        );
        let key = SecretKey::from_bytes(&private_key).unwrap();
        let uncompressed = key.public_key();
        let compressed = derive_public_key_compressed(&private_key).unwrap();
        let signature = key.sign(b"sample").unwrap();

        assert!(uncompressed.verify(b"sample", &signature).is_ok());
        let point = AffinePoint::from_sec1_bytes(&compressed).unwrap();
        assert!(ecdsa_verify_inner(&point, b"sample", &signature).is_ok());
    }

    #[test]
    fn invalid_inputs_are_rejected() {
        let invalid_private_key = [0u8; SECRET_KEY_SIZE];
        assert!(SecretKey::from_bytes(&invalid_private_key).is_err());
        assert!(derive_public_key_uncompressed(&invalid_private_key).is_err());
        assert!(derive_public_key_compressed(&invalid_private_key).is_err());

        let private_key = decode_hex::<48>(
            "6b9d3dad2e1b8c1c05b19875b6659f4de23c3b667bf297ba9aa47740787137d8\
             96d5724e4c70a825f872c9ea60d2edf5",
        );
        let key = SecretKey::from_bytes(&private_key).unwrap();
        let signature = key.sign(b"msg").unwrap();
        let mut zero_r = signature;
        zero_r[..48].fill(0);
        assert!(key.public_key().verify(b"msg", &zero_r).is_err());
    }

    #[test]
    fn public_key_validation_accepts_known_good_points() {
        assert!(is_valid_public_key(&decode_hex::<97>(
            "04aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a38\
             5502f25dbf55296c3a545e3872760ab73617de4a96262c6f5d9e98bf9292dc29\
             f8f41dbd289a147ce9da3113b5f0b8c00a60b1ce1d7e819d7a431d7c90ea0e5f",
        )));
    }

    #[test]
    fn ecdsa_sign_verify_round_trip_multiple_messages() {
        let private_key = decode_hex::<48>(
            "6b9d3dad2e1b8c1c05b19875b6659f4de23c3b667bf297ba9aa47740787137d8\
             96d5724e4c70a825f872c9ea60d2edf5",
        );
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
            let mut wrong_msg = msg.to_vec();
            wrong_msg.push(0x42);
            assert!(pub_key.verify(&wrong_msg, &sig).is_err());
        }
    }

    #[test]
    fn rfc6979_retry_advances_drbg() {
        let private_key = Scalar::from_bytes(&decode_hex::<48>(
            "6b9d3dad2e1b8c1c05b19875b6659f4de23c3b667bf297ba9aa47740787137d8\
             96d5724e4c70a825f872c9ea60d2edf5",
        ))
        .unwrap();
        let hash = hash_message(b"sample");

        let mut drbg = Rfc6979::new(&private_key, &hash);
        let k1 = drbg.generate();
        assert_eq!(
            k1.to_bytes(),
            decode_hex::<48>(
                "94ed910d1a099dad3254e9242ae85abde4ba15168eaf0ca87a555fd56d10fbca\
                 2907e3e83ba95368623b8c4686915cf9"
            )
        );

        // RFC 6979 §3.2 h.3: an r=0/s=0 retry must continue the DRBG, not
        // reproduce the same nonce.
        drbg.retry();
        let k2 = drbg.generate();
        assert_ne!(k1.to_bytes(), k2.to_bytes());
        assert_eq!(
            k2.to_bytes(),
            decode_hex::<48>(
                "9d63ce4c96d070a67f7bee49e870b64838c0ac65bb7440cf46017dca69d35d23\
                 6219aae5e00a9f01f13e7774be1339fc"
            )
        );
    }

    #[test]
    fn ecdsa_forced_retry_uses_fresh_nonce() {
        let private_key = decode_hex::<48>(
            "6b9d3dad2e1b8c1c05b19875b6659f4de23c3b667bf297ba9aa47740787137d8\
             96d5724e4c70a825f872c9ea60d2edf5",
        );
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
    fn ecdsa_sign_verify_different_keys() {
        let keys: &[&str] = &[
            "0000000000000000000000000000000000000000000000000000000000000000\
             00000000000000000000000000000001",
            "0000000000000000000000000000000000000000000000000000000000000000\
             00000000000000000000000000000002",
            "a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f90011\
             2233445566778899aabbccddeeff0011",
        ];

        for key_hex in keys {
            let private_key = decode_hex::<48>(key_hex);
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
        let private_key1 = decode_hex::<48>(
            "6b9d3dad2e1b8c1c05b19875b6659f4de23c3b667bf297ba9aa47740787137d8\
             96d5724e4c70a825f872c9ea60d2edf5",
        );
        let private_key2 = decode_hex::<48>(
            "0000000000000000000000000000000000000000000000000000000000000000\
             00000000000000000000000000000001",
        );
        let key1 = SecretKey::from_bytes(&private_key1).unwrap();
        let key2 = SecretKey::from_bytes(&private_key2).unwrap();

        let sig = key1.sign(b"message").unwrap();
        assert!(key2.public_key().verify(b"message", &sig).is_err());
    }

    #[test]
    fn ecdsa_sign_emits_canonical_low_s() {
        let private_key = decode_hex::<48>(
            "6b9d3dad2e1b8c1c05b19875b6659f4de23c3b667bf297ba9aa47740787137d8\
             96d5724e4c70a825f872c9ea60d2edf5",
        );
        let key = SecretKey::from_bytes(&private_key).unwrap();
        for msg in [&b"sample"[..], b"test", b"", b"another message"] {
            let sig = key.sign(msg).unwrap();
            let s = Scalar::from_bytes(sig[48..].try_into().unwrap()).unwrap();
            assert!(!s.is_high(), "sign produced a high-s signature for {:?}", msg);
        }
    }

    #[test]
    fn ecdsa_verify_strict_rejects_high_s() {
        let private_key = decode_hex::<48>(
            "6b9d3dad2e1b8c1c05b19875b6659f4de23c3b667bf297ba9aa47740787137d8\
             96d5724e4c70a825f872c9ea60d2edf5",
        );
        let key = SecretKey::from_bytes(&private_key).unwrap();
        let public_key = key.public_key();
        let signature = key.sign(b"sample").unwrap();

        // Canonical low-s signature passes both permissive and strict verification.
        assert!(public_key.verify(b"sample", &signature).is_ok());
        assert!(public_key.verify_strict(b"sample", &signature).is_ok());

        // The malleable counterpart (r, n - s) is rejected only by strict verify.
        let r = Scalar::from_bytes(signature[..48].try_into().unwrap()).unwrap();
        let s = Scalar::from_bytes(signature[48..].try_into().unwrap()).unwrap();
        let high_s = Scalar::ZERO.sub(s);
        assert!(high_s.is_high());

        let mut malleable = [0u8; SIGNATURE_SIZE];
        malleable[..48].copy_from_slice(&r.to_bytes());
        malleable[48..].copy_from_slice(&high_s.to_bytes());

        assert!(public_key.verify(b"sample", &malleable).is_ok());
        assert!(public_key.verify_strict(b"sample", &malleable).is_err());
    }

    #[test]
    fn scalar_from_bytes_rejects_boundary_values() {
        let zero = [0u8; 48];
        assert!(Scalar::from_bytes(&zero).is_none());

        let n_bytes = decode_hex::<48>(
            "ffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf\
             581a0db248b0a77aecec196accc52973",
        );
        assert!(Scalar::from_bytes(&n_bytes).is_none());

        let n_minus_1 = decode_hex::<48>(
            "ffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf\
             581a0db248b0a77aecec196accc52972",
        );
        assert!(Scalar::from_bytes(&n_minus_1).is_some());

        let one = decode_hex::<48>(
            "0000000000000000000000000000000000000000000000000000000000000000\
             00000000000000000000000000000001",
        );
        assert!(Scalar::from_bytes(&one).is_some());
    }

    #[test]
    fn field_element_from_bytes_rejects_boundary_values() {
        let p_bytes = decode_hex::<48>(
            "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffff0000000000000000ffffffff",
        );
        assert!(FieldElement::from_bytes(&p_bytes).is_none());

        let p_minus_1 = decode_hex::<48>(
            "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffff0000000000000000fffffffe",
        );
        assert!(FieldElement::from_bytes(&p_minus_1).is_some());

        let zero = [0u8; 48];
        assert!(FieldElement::from_bytes(&zero).is_some());
    }

    #[test]
    fn point_decompression_round_trip() {
        let keys: &[&str] = &[
            "0000000000000000000000000000000000000000000000000000000000000000\
             00000000000000000000000000000001",
            "0000000000000000000000000000000000000000000000000000000000000000\
             00000000000000000000000000000002",
            "6b9d3dad2e1b8c1c05b19875b6659f4de23c3b667bf297ba9aa47740787137d8\
             96d5724e4c70a825f872c9ea60d2edf5",
            "a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f90011\
             2233445566778899aabbccddeeff0011",
        ];

        for key_hex in keys {
            let private_key = decode_hex::<48>(key_hex);
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
    fn scalar_inversion_correctness() {
        let k = Scalar::from_bytes(&decode_hex::<48>(
            "c22b201cc45cd130ef80acfc70e84fa17b91b0ffbfe4c9c44eda37e1ad1d7f8f\
             ae4c4c8b52559930e08ba1c822c105b0",
        ))
        .unwrap();
        let k_inv = k.invert().unwrap();
        let product = k.mul(k_inv);
        assert_eq!(product, Scalar::ONE);
    }

    #[test]
    fn field_element_inversion_correctness() {
        let x = FieldElement::from_bytes(&decode_hex::<48>(
            "aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a38\
             5502f25dbf55296c3a545e3872760ab7",
        ))
        .unwrap();
        let x_inv = x.invert().unwrap();
        let product = x.mul(x_inv);
        assert_eq!(product, FieldElement::ONE);
    }

    #[test]
    fn p384_fast_mul_mod_matches_generic() {
        for _ in 0..1000 {
            let a_bytes: [u8; 48] = rand::random();
            let b_bytes: [u8; 48] = rand::random();
            let a_opt = FieldElement::from_bytes(&a_bytes);
            let b_opt = FieldElement::from_bytes(&b_bytes);
            if a_opt.is_none() || b_opt.is_none() {
                continue;
            }
            let a = a_opt.unwrap();
            let b = b_opt.unwrap();
            let expected = U384::from_limbs({
                let mut p = [0u64; 12];
                for i in 0..6 {
                    let mut c = 0u64;
                    for j in 0..6 {
                        let (v, cc) = mac(p[i + j], a.0.limbs[i], b.0.limbs[j], c);
                        p[i + j] = v;
                        c = cc;
                    }
                    p[i + 6] = c;
                }
                let mut rem = [0u64; 6];
                for bi in (0..768).rev() {
                    let li = bi / 64;
                    let pi = bi % 64;
                    let bit = ((p[li] >> pi) & 1) as u64;
                    let mut shifted = [0u64; 6];
                    let mut carry = bit;
                    for j in 0..6 {
                        let next = rem[j] >> 63;
                        shifted[j] = (rem[j] << 1) | carry;
                        carry = next;
                    }
                    let (red, br) = U384::from_limbs(shifted).sub_raw(&MODULUS_P);
                    if carry == 1 || br == 0 {
                        rem = red.limbs;
                    } else {
                        rem = shifted;
                    }
                }
                rem
            });
            let fast = p384_fast_mul_mod(&a.0, &b.0);
            assert_eq!(expected, fast, "mismatch in p384_fast_mul_mod");
        }
    }

    #[test]
    fn scalar_mul_generator_n_gives_identity() {
        let n_minus_1 = Scalar::from_bytes(&decode_hex::<48>(
            "ffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf\
             581a0db248b0a77aecec196accc52972",
        ))
        .unwrap();
        let result = scalar_mul_generator(&n_minus_1).to_affine().unwrap();
        assert_eq!(result.x, FieldElement::from_uint(GENERATOR_X));
        let neg_gy = FieldElement::from_uint(GENERATOR_Y).negate();
        assert_eq!(result.y, neg_gy);
    }

    #[test]
    fn ecdh_rejects_invalid_private_key_zero() {
        let zero_key = [0u8; 48];
        assert!(SecretKey::from_bytes(&zero_key).is_err());
        let bob = SecretKey::generate().unwrap();
        assert!(ecdh(&zero_key, &bob.public_key().to_bytes()).is_err());
    }

    #[test]
    fn ecdsa_rejects_non_canonical_r_and_s() {
        let key = SecretKey::generate().unwrap();
        let valid_sig = key.sign(b"msg").unwrap();

        let mut bad_r = valid_sig;
        bad_r[..48].copy_from_slice(&decode_hex::<48>(
            "ffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52974",
        ));
        assert!(key.public_key().verify(b"msg", &bad_r).is_err());

        let mut bad_s = valid_sig;
        bad_s[48..].copy_from_slice(&decode_hex::<48>(
            "ffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52974",
        ));
        assert!(key.public_key().verify(b"msg", &bad_s).is_err());
    }

    #[test]
    fn verify_rejects_tampered_message_and_signature() {
        let key = SecretKey::generate().unwrap();
        let pub_key = key.public_key();
        let sig = key.sign(b"message").unwrap();

        assert!(pub_key.verify(b"tampered", &sig).is_err());

        let mut bad_sig = sig;
        bad_sig[10] ^= 0x80;
        assert!(pub_key.verify(b"message", &bad_sig).is_err());
    }

    #[test]
    fn public_key_rejects_off_curve_point() {
        let key = SecretKey::generate().unwrap();
        let mut off_curve = key.public_key().to_bytes();
        off_curve[96] ^= 0x01;
        assert!(!is_valid_public_key(&off_curve));
        assert!(PublicKey::from_bytes(&off_curve).is_err());
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
    fn from_x_y_matches_generator() {
        let key = PublicKey::from_x_y(
            &decode_hex::<48>(
                "aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a38\
                 5502f25dbf55296c3a545e3872760ab7",
            ),
            &decode_hex::<48>(
                "3617de4a96262c6f5d9e98bf9292dc29f8f41dbd289a147ce9da3113b5f0b8c0\
                 0a60b1ce1d7e819d7a431d7c90ea0e5f",
            ),
        )
        .unwrap();
        let from_sec1 = PublicKey::from_bytes(&key.to_bytes()).unwrap();
        assert_eq!(key, from_sec1);
    }

    #[test]
    fn field_element_add_sub_mul_consistency() {
        let a = FieldElement::from_bytes(&decode_hex::<48>(
            "aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a38\
             5502f25dbf55296c3a545e3872760ab7",
        ))
        .unwrap();
        let b = FieldElement::from_bytes(&decode_hex::<48>(
            "3617de4a96262c6f5d9e98bf9292dc29f8f41dbd289a147ce9da3113b5f0b8c0\
             0a60b1ce1d7e819d7a431d7c90ea0e5f",
        ))
        .unwrap();

        assert_eq!(a.add(b).sub(b), a);
        assert_eq!(a.add(b), b.add(a));
        assert_eq!(a.mul(b), b.mul(a));

        let c = FieldElement::from_bytes(&decode_hex::<48>(
            "2a85c8edd3ec2aefc656398d8a2ed19d0314088f5013875a181d9c6efe814112\
             988e056be3f82d19b3312fa7e23ee7e4",
        ))
        .unwrap();
        assert_eq!(a.add(b).mul(c), a.mul(c).add(b.mul(c)));
    }

    #[test]
    fn scalar_add_sub_mul_consistency() {
        let a = Scalar::from_bytes(&decode_hex::<48>(
            "c22b201cc45cd130ef80acfc70e84fa17b91b0ffbfe4c9c44eda37e1ad1d7f8f\
             ae4c4c8b52559930e08ba1c822c105b0",
        ))
        .unwrap();
        let one = Scalar::from_bytes(&decode_hex::<48>(
            "0000000000000000000000000000000000000000000000000000000000000000\
             00000000000000000000000000000001",
        ))
        .unwrap();

        assert_eq!(a.add(one).sub(one), a);
        assert_eq!(a.mul(one), a);

        let b = Scalar::from_bytes(&decode_hex::<48>(
            "7cf1be7a45d8d72e6e974229bfad108f3d2d4aa6208248adb9343258e4f30f80\
             8252a11a87ddc7e0d8ba3b5e28878944",
        ))
        .unwrap();
        assert_eq!(a.mul(b), b.mul(a));
        assert_eq!(a.add(b), b.add(a));
    }

    #[test]
    fn field_element_negate_round_trip() {
        let x = FieldElement::from_bytes(&decode_hex::<48>(
            "aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a38\
             5502f25dbf55296c3a545e3872760ab7",
        ))
        .unwrap();
        let neg = x.negate();
        assert_eq!(neg.negate(), x);
        assert_eq!(x.add(neg), FieldElement::ZERO);
    }

    #[test]
    fn scalar_mul_by_two_matches_double() {
        let two = Scalar::from_bytes(&decode_hex::<48>(
            "0000000000000000000000000000000000000000000000000000000000000000\
             00000000000000000000000000000002",
        ))
        .unwrap();
        let g_times_2 = scalar_mul_affine(&AffinePoint::GENERATOR, &two).to_affine().unwrap();
        let proj_g = ProjectivePoint::from_affine(&AffinePoint::GENERATOR);
        let g_doubled = proj_g.double().to_affine().unwrap();

        assert_eq!(g_times_2.to_uncompressed_bytes(), g_doubled.to_uncompressed_bytes());
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
    fn is_on_curve_accepts_generator_and_random_points() {
        assert!(AffinePoint::GENERATOR.is_on_curve());
        for _ in 0..5 {
            let key = SecretKey::generate().unwrap();
            let pb = key.public_key().to_bytes();
            let pk = PublicKey::from_bytes(&pb).unwrap();
            let _ = pk;
        }
    }

    #[test]
    fn ecdh_with_self_is_consistent() {
        let key = SecretKey::generate().unwrap();
        let shared1 = key.ecdh(&key.public_key()).unwrap();
        let shared2 = key.ecdh(&key.public_key()).unwrap();
        assert_eq!(shared1, shared2);
    }

    #[test]
    fn field_element_pow_correctness() {
        let x = FieldElement::from_bytes(&decode_hex::<48>(
            "0000000000000000000000000000000000000000000000000000000000000000\
             00000000000000000000000000000002",
        ))
        .unwrap();
        let x3 = x.pow(&U384::from_u64(3));
        let expected = x.mul(x).mul(x);
        assert_eq!(x3, expected);

        let x0 = x.pow(&U384::ZERO);
        assert_eq!(x0, FieldElement::ONE);
    }

    // --- Wycheproof test vectors ---

    #[test]
    fn wycheproof_ecdsa_p384_sha384_p1363() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/ecdsa_secp384r1_sha384_p1363_test.json"
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
                        "wycheproof ECDSA P384 P1363 tcId={} expected valid but failed",
                        test["tcId"]
                    );
                    valid_tested += 1;
                } else {
                    assert!(
                        verify_result.is_err(),
                        "wycheproof ECDSA P384 P1363 tcId={} expected invalid but passed",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid ECDSA P384 P1363 wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid ECDSA P384 P1363 wycheproof tests were run");
    }

    #[test]
    fn wycheproof_ecdsa_p384_sha384_der() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/ecdsa_secp384r1_sha384_test.json"
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
                        "wycheproof ECDSA P384 DER SHA-384 tcId={} expected valid but failed",
                        test["tcId"]
                    );
                    valid_tested += 1;
                } else {
                    assert!(
                        verify_result.is_err(),
                        "wycheproof ECDSA P384 DER SHA-384 tcId={} expected invalid but passed",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid ECDSA P384 DER SHA-384 wycheproof tests were run");
        assert!(
            invalid_tested > 0,
            "no invalid ECDSA P384 DER SHA-384 wycheproof tests were run"
        );
    }

    #[test]
    fn wycheproof_ecdh_p384_ecpoint() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/ecdh_secp384r1_ecpoint_test.json"
        ))
        .unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        let mut acceptable_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            if group["curve"].as_str() != Some("secp384r1") {
                continue;
            }
            for test in group["tests"].as_array().unwrap() {
                let public_hex = test["public"].as_str().unwrap();
                let private_hex = test["private"].as_str().unwrap();
                let expected_shared_hex = test["shared"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let public_key = hex::decode(public_hex).unwrap();

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
                    assert_eq!(
                        shared_hex, expected_shared_hex,
                        "wycheproof ECDH P384 ecpoint tcId={}",
                        test["tcId"]
                    );
                    valid_tested += 1;
                } else if result == "invalid" {
                    assert!(
                        shared.is_err(),
                        "wycheproof ECDH P384 ecpoint tcId={} expected invalid but passed",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                } else {
                    acceptable_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid ECDH P384 ecpoint wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid ECDH P384 ecpoint wycheproof tests were run");
        assert!(
            acceptable_tested > 0,
            "no acceptable ECDH P384 ecpoint wycheproof tests were run"
        );
    }

    #[test]
    fn wycheproof_ecdh_p384_asn() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/ecdh_secp384r1_test.json"
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
                        panic!("wycheproof ECDH P384 ASN tcId={}: failed to parse valid SPKI", test["tcId"]);
                    }
                    invalid_tested += 1;
                    continue;
                };

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
                    assert_eq!(
                        shared_hex, expected_shared_hex,
                        "wycheproof ECDH P384 ASN tcId={}",
                        test["tcId"]
                    );
                    valid_tested += 1;
                } else if result == "invalid" {
                    assert!(
                        shared.is_err(),
                        "wycheproof ECDH P384 ASN tcId={} expected invalid but passed",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                } else {
                    acceptable_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid ECDH P384 ASN wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid ECDH P384 ASN wycheproof tests were run");
        assert!(acceptable_tested > 0, "no acceptable ECDH P384 ASN wycheproof tests were run");
    }

    // Shared curve-independent tests, instantiated for P384.
    #[test]
    fn generator_point_is_on_curve() {
        p_curves::test_support::generator_point_is_on_curve::<P384>();
    }

    #[test]
    fn from_x_y_rejects_off_curve() {
        p_curves::test_support::from_x_y_rejects_off_curve::<P384>();
    }

    #[test]
    fn ecdh_round_trip_alice_bob() {
        p_curves::test_support::ecdh_round_trip_alice_bob::<P384>();
    }

    #[test]
    fn ecdh_rejects_off_curve_peer_public_key() {
        p_curves::test_support::ecdh_rejects_off_curve_peer_public_key::<P384>();
    }

    #[test]
    fn ecdh_rejects_infinity_peer_public_key() {
        p_curves::test_support::ecdh_rejects_infinity_peer_public_key::<P384>();
    }

    #[test]
    fn ecdh_rejects_bad_length_peer_public_key() {
        p_curves::test_support::ecdh_rejects_bad_length_peer_public_key::<P384>();
    }

    #[test]
    fn ecdh_multiple_exchanges_consistency() {
        p_curves::test_support::ecdh_multiple_exchanges_consistency::<P384>();
    }

    #[test]
    fn private_key_round_trip_bytes() {
        p_curves::test_support::private_key_round_trip_bytes::<P384>();
    }

    #[test]
    fn public_key_round_trip_bytes() {
        p_curves::test_support::public_key_round_trip_bytes::<P384>();
    }

    #[test]
    fn point_double_and_add_consistency() {
        p_curves::test_support::point_double_and_add_consistency::<P384>();
    }

    #[test]
    fn compressed_public_key_has_correct_prefix() {
        p_curves::test_support::compressed_public_key_has_correct_prefix::<P384>();
    }

    #[cfg(feature = "zeroize")]
    #[test]
    fn rfc6979_state_zeroize_clears_drbg() {
        p_curves::test_support::rfc6979_state_zeroize_clears_drbg::<P384>();
    }
}
