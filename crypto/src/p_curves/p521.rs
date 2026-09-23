//! P-521 (secp521r1) ECDSA and ECDH.
//!
//! See [`SecretKey`] and [`PublicKey`] for the signing, verification and key
//! agreement APIs.

use big_number::{Uint, mac};

use super::p_curves::{self, Curve, UintOps, field_pow};
use crate::{EllipticCurveError, Hasher, RandomError, hmac::Hmac, sha2::Sha512};

/// Size of a P-521 secret key in bytes (66 bytes).
pub const SECRET_KEY_SIZE: usize = 66;
/// Size of a compressed P-521 public key in bytes (67 bytes, includes 0x02/0x03 prefix).
pub const PUBLIC_KEY_COMPRESSED_SIZE: usize = 67;
/// Size of an uncompressed P-521 public key in bytes (133 bytes, includes 0x04 prefix).
pub const PUBLIC_KEY_UNCOMPRESSED_SIZE: usize = 133;
/// Size of a P-521 ECDSA signature in bytes (132 bytes, r || s).
pub const SIGNATURE_SIZE: usize = 132;
/// Size of the raw ECDH shared secret in bytes (66 bytes). **Must not** be used directly
/// as an encryption key; apply a KDF first.
pub const ECDH_SHARED_SECRET_SIZE: usize = 66;

/// P-521 (secp521r1) ECDSA secret key.
///
/// Supports signing and ECDH key agreement.
///
/// # Signing
///
/// ```ignore
/// use crypto::p521::SecretKey;
///
/// let key = SecretKey::generate().unwrap();
/// let signature = key.sign(b"message").unwrap();
/// ```
///
/// # ECDH key exchange
///
/// ```ignore
/// use crypto::p521::SecretKey;
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
pub struct SecretKey(p_curves::SecretKey<P521>);

impl SecretKey {
    /// Generates a fresh random secret key.
    #[cfg(feature = "random")]
    pub fn generate() -> Result<SecretKey, EllipticCurveError> {
        Ok(SecretKey(p_curves::SecretKey::generate()?))
    }

    /// Builds a secret key from its 66-byte big-endian scalar.
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

    /// Signs `message` with ECDSA using SHA-512 and deterministic (RFC 6979)
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

    /// Returns the secret scalar as 66 big-endian bytes.
    pub fn to_bytes(&self) -> [u8; SECRET_KEY_SIZE] {
        self.0.to_bytes()
    }
}

/// P-521 (secp521r1) ECDSA public key.
///
/// Supports signature verification and ECDH key agreement.
///
/// # Verification
///
/// ```ignore
/// use crypto::p521::SecretKey;
///
/// let key = SecretKey::generate().unwrap();
/// let signature = key.sign(b"message").unwrap();
/// assert!(key.public_key().verify(b"message", &signature).is_ok());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PublicKey(p_curves::PublicKey<P521>);

impl PublicKey {
    /// Parses a compressed (67-byte) or uncompressed (133-byte) SEC1 encoding.
    ///
    /// Returns [`EllipticCurveError::InvalidKey`] when the encoding is not a
    /// valid point on the P-521 curve.
    #[inline]
    pub fn from_bytes(key: &[u8]) -> Result<PublicKey, EllipticCurveError> {
        Ok(PublicKey(p_curves::PublicKey::from_bytes(key)?))
    }

    /// Builds a public key from raw affine x and y coordinates (both
    /// big-endian, 66 bytes each). Returns `InvalidKey` if the coordinates
    /// are not a valid point on the P-521 curve.
    ///
    /// This is useful when importing keys from formats like JWK where `x`
    /// and `y` are available directly.
    #[inline]
    pub fn from_x_y(x_bytes: &[u8; 66], y_bytes: &[u8; 66]) -> Result<PublicKey, EllipticCurveError> {
        Ok(PublicKey(p_curves::PublicKey::from_x_y(x_bytes, y_bytes)?))
    }

    /// Verifies an ECDSA signature over `message` using SHA-512.
    ///
    /// Both canonical low-s and non-canonical high-s signatures are accepted,
    /// matching typical ECDSA interoperability. Use [`Self::verify_strict`] to
    /// additionally reject high-s signatures.
    pub fn verify(&self, message: &[u8], signature: &[u8; SIGNATURE_SIZE]) -> Result<(), EllipticCurveError> {
        self.0.verify(message, signature)
    }

    /// Verifies an ECDSA signature over `message` using SHA-512, additionally
    /// rejecting non-canonical high-s signatures (`s > n / 2`).
    ///
    /// Use this when signature malleability must be excluded (Bitcoin/EIP-2
    /// style). Note that some third-party signers emit high-s signatures, which
    /// this method will reject; [`Self::verify`] accepts both forms.
    pub fn verify_strict(&self, message: &[u8], signature: &[u8; SIGNATURE_SIZE]) -> Result<(), EllipticCurveError> {
        self.0.verify_strict(message, signature)
    }

    /// Returns the 133-byte uncompressed SEC1 encoding (`0x04 || x || y`).
    #[inline]
    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_UNCOMPRESSED_SIZE] {
        self.0.to_bytes()
    }

    /// Returns the 67-byte compressed SEC1 encoding (`0x02`/`0x03 || x`).
    #[inline]
    pub fn to_compressed_bytes(&self) -> [u8; PUBLIC_KEY_COMPRESSED_SIZE] {
        self.0.to_compressed_bytes()
    }

    /// Returns the `X` and `Y` points as big-endian arrays.
    #[inline]
    pub fn x_y(&self) -> ([u8; 66], [u8; 66]) {
        self.0.x_y()
    }
}

/// Marker type carrying the P-521 curve parameters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct P521;

type U521 = Uint<521, 9>;
type CurveUint = U521;

const MODULUS_P: CurveUint = CurveUint::from_limbs([
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0x00000000000001ff,
]);

const MODULUS_N: CurveUint = CurveUint::from_limbs([
    0xbb6fb71e91386409,
    0x3bb5c9b8899c47ae,
    0x7fcc0148f709a5d0,
    0x51868783bf2f966b,
    0xfffffffffffffffa,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0x00000000000001ff,
]);

// floor(n / 2). A signature scalar `s` is "high" when `s > N_HALF`, in which
// case it is normalized to `n - s` so that signatures are non-malleable.
const N_HALF: CurveUint = CurveUint::from_limbs([
    0x5db7db8f489c3204,
    0x1ddae4dc44ce23d7,
    0xbfe600a47b84d2e8,
    0x28c343c1df97cb35,
    0xfffffffffffffffd,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0x00000000000000ff,
]);

const P_MINUS_TWO: CurveUint = CurveUint::from_limbs([
    0xfffffffffffffffd,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0x00000000000001ff,
]);

// (p + 1) / 4 = 2^519. P-521 has p = 2^521 - 1, hence a square root of `a` is
// `a^((p + 1) / 4)`.
const P_PLUS_ONE_OVER_FOUR: CurveUint = CurveUint::from_limbs([
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000080,
]);

const N_MINUS_TWO: CurveUint = CurveUint::from_limbs([
    0xbb6fb71e91386407,
    0x3bb5c9b8899c47ae,
    0x7fcc0148f709a5d0,
    0x51868783bf2f966b,
    0xfffffffffffffffa,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0x00000000000001ff,
]);

const CURVE_B: CurveUint = CurveUint::from_limbs([
    0xef451fd46b503f00,
    0x3573df883d2c34f1,
    0x1652c0bd3bb1bf07,
    0x56193951ec7e937b,
    0xb8b489918ef109e1,
    0xa2da725b99b315f3,
    0x929a21a0b68540ee,
    0x953eb9618e1c9a1f,
    0x0000000000000051,
]);

const GENERATOR_X: CurveUint = CurveUint::from_limbs([
    0xf97e7e31c2e5bd66,
    0x3348b3c1856a429b,
    0xfe1dc127a2ffa8de,
    0xa14b5e77efe75928,
    0xf828af606b4d3dba,
    0x9c648139053fb521,
    0x9e3ecb662395b442,
    0x858e06b70404e9cd,
    0x00000000000000c6,
]);

const GENERATOR_Y: CurveUint = CurveUint::from_limbs([
    0x88be94769fd16650,
    0x353c7086a272c240,
    0xc550b9013fad0761,
    0x97ee72995ef42640,
    0x17afbd17273e662c,
    0x98f54449579b4468,
    0x5c8a5fb42c7d1bd9,
    0x39296a789a3bc004,
    0x0000000000000118,
]);

// Reduce a big-endian value modulo n, accepting zero and values >= n.
#[inline]
fn reduce_mod(value: CurveUint) -> CurveUint {
    let (sub_value, _) = value.sub_raw(&MODULUS_N);
    CurveUint::ct_select(&sub_value, &value, value.ct_ge(&MODULUS_N))
}

// P-521 fast modular multiplication.
//
// Since p = 2^521 - 1, we have 2^521 ≡ 1 (mod p). The 1042-bit product is
// split at bit 521 into its low part `L` and high part `H`, so that
// `P ≡ L + H (mod p)`. Because `L, H < 2^521`, `L + H < 2^522 = 2p + 2`, so at
// most two conditional subtractions bring the result below `p`.
fn p521_fast_mul_mod(a: &CurveUint, b: &CurveUint) -> CurveUint {
    let al = a.limbs;
    let bl = b.limbs;

    let mut prod = [0u64; 18];
    for i in 0..9 {
        let mut carry = 0u64;
        for j in 0..9 {
            let (v, cc) = mac(prod[i + j], al[i], bl[j], carry);
            prod[i + j] = v;
            carry = cc;
        }
        prod[i + 9] = carry;
    }

    // L = product mod 2^521 (limb 8 holds only its low 9 bits).
    let mut lo = [0u64; 9];
    lo[..8].copy_from_slice(&prod[..8]);
    lo[8] = prod[8] & 0x1ff;

    // H = product >> 521. Bit 521 is limb 8, bit 9, so each high limb is a
    // 9-bit shift of the product.
    let mut hi = [0u64; 9];
    for j in 0..9 {
        hi[j] = (prod[8 + j] >> 9) | (prod[9 + j] << 55);
    }

    let (mut result, _) = CurveUint::from_limbs(lo).add_raw(&CurveUint::from_limbs(hi));
    for _ in 0..2 {
        let (reduced, borrow) = result.sub_raw(&MODULUS_P);
        result = CurveUint::ct_select(&reduced, &result, borrow == 0);
    }
    result
}

impl Curve for P521 {
    type U = CurveUint;
    type FieldBytes = [u8; SECRET_KEY_SIZE];
    type DigestBytes = [u8; 64];
    type CompressedBytes = [u8; PUBLIC_KEY_COMPRESSED_SIZE];
    type UncompressedBytes = [u8; PUBLIC_KEY_UNCOMPRESSED_SIZE];
    type SignatureBytes = [u8; SIGNATURE_SIZE];

    const FIELD_BITS: usize = 521;
    const FIELD_BYTES: usize = SECRET_KEY_SIZE;
    const DIGEST_BYTES: usize = 64;
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
        p521_fast_mul_mod(a, b)
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
        Sha512::hash(data).as_ref().try_into().unwrap()
    }

    #[inline]
    fn hmac(key: &[u8], data: &[u8]) -> Self::DigestBytes {
        Hmac::<Sha512>::mac(key, data).as_ref().try_into().unwrap()
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
        // The group order is 521 bits while a field element occupies 66 bytes
        // (528 bits). Clear the unused high bits and retry on the vanishingly
        // rare value that still lands outside `[1, n - 1]`.
        loop {
            let mut bytes: Self::FieldBytes = crate::random::bytes()?;
            bytes[0] &= 0x01;
            if p_curves::Scalar::<P521>::from_bytes(&bytes).is_some() {
                return Ok(bytes);
            }
        }
    }
}

/// Returns `true` when `public_key` is a valid SEC1 encoding of a point on the
/// P-521 curve.
pub fn is_valid_public_key(public_key: &[u8]) -> bool {
    p_curves::is_valid_public_key::<P521>(public_key)
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    use super::*;
    use crate::p_curves::p_curves;

    type FieldElement = p_curves::FieldElement<P521>;
    type Scalar = p_curves::Scalar<P521>;
    type AffinePoint = p_curves::AffinePoint<P521>;
    type ProjectivePoint = p_curves::ProjectivePoint<P521>;
    type Rfc6979 = p_curves::Rfc6979<P521>;

    // Concrete wrappers around the generic core helpers.
    fn hash_message(message: &[u8]) -> [u8; 64] {
        p_curves::hash_message::<P521>(message)
    }

    fn hmac_digest(key: &[u8], data: &[u8]) -> [u8; 64] {
        p_curves::hmac_digest::<P521>(key, data)
    }

    fn bits2octets(hash: &[u8; 64]) -> [u8; SECRET_KEY_SIZE] {
        p_curves::bits2octets::<P521>(hash)
    }

    fn derive_public_key_uncompressed(
        private_key: &[u8; SECRET_KEY_SIZE],
    ) -> Result<[u8; PUBLIC_KEY_UNCOMPRESSED_SIZE], crate::EllipticCurveError> {
        p_curves::derive_public_key_uncompressed::<P521>(private_key)
    }

    fn derive_public_key_compressed(
        private_key: &[u8; SECRET_KEY_SIZE],
    ) -> Result<[u8; PUBLIC_KEY_COMPRESSED_SIZE], crate::EllipticCurveError> {
        p_curves::derive_public_key_compressed::<P521>(private_key)
    }

    fn ecdsa_sign_inner_impl(
        scalar: &Scalar,
        message: &[u8],
        force_first_retry: bool,
    ) -> Result<[u8; SIGNATURE_SIZE], crate::EllipticCurveError> {
        p_curves::ecdsa_sign_inner_impl::<P521>(scalar, message, force_first_retry)
    }

    fn ecdsa_verify_inner(
        public_point: &AffinePoint,
        message: &[u8],
        signature: &[u8; SIGNATURE_SIZE],
    ) -> Result<(), crate::EllipticCurveError> {
        p_curves::ecdsa_verify_inner::<P521>(public_point, message, signature)
    }

    fn parse_public_key(public_key: &[u8]) -> Result<AffinePoint, crate::EllipticCurveError> {
        p_curves::parse_public_key::<P521>(public_key)
    }

    fn scalar_mul_generator(scalar: &Scalar) -> ProjectivePoint {
        p_curves::scalar_mul_generator::<P521>(scalar)
    }

    fn scalar_mul_affine(base: &AffinePoint, scalar: &Scalar) -> ProjectivePoint {
        p_curves::scalar_mul_affine::<P521>(base, scalar)
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

    fn der_ecdsa_sig_to_p1363(der: &[u8]) -> Option<[u8; SIGNATURE_SIZE]> {
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
        if rtag != 0x02 || rval.is_empty() || rval.len() > 67 {
            return None;
        }
        let (stag, sval) = der_read_tlv(inner, &mut inner_offset)?;
        if stag != 0x02 || sval.is_empty() || sval.len() > 67 {
            return None;
        }
        if inner_offset != inner.len() {
            return None;
        }
        let r_valid = if rval.len() == 66 && rval[0] >= 0x80 {
            false
        } else if rval.len() == 67 && rval[0] != 0 {
            false
        } else if rval.len() == 67 && rval[0] == 0 && rval[1] < 0x80 {
            false
        } else {
            rval.len() <= 67
        };
        let s_valid = if sval.len() == 66 && sval[0] >= 0x80 {
            false
        } else if sval.len() == 67 && sval[0] != 0 {
            false
        } else if sval.len() == 67 && sval[0] == 0 && sval[1] < 0x80 {
            false
        } else {
            sval.len() <= 67
        };
        if !r_valid || !s_valid {
            return None;
        }

        let r_trimmed = if rval.len() == 67 && rval[0] == 0 {
            &rval[1..]
        } else {
            rval
        };
        let s_trimmed = if sval.len() == 67 && sval[0] == 0 {
            &sval[1..]
        } else {
            sval
        };
        if r_trimmed.len() > 66 || s_trimmed.len() > 66 {
            return None;
        }
        let mut sig = [0u8; SIGNATURE_SIZE];
        sig[66 - r_trimmed.len()..66].copy_from_slice(r_trimmed);
        sig[132 - s_trimmed.len()..132].copy_from_slice(s_trimmed);
        Some(sig)
    }

    fn spki_to_sec1_point(spki: &[u8]) -> Option<Vec<u8>> {
        let ec_public_key_oid: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01];
        let secp521r1_oid: &[u8] = &[0x2b, 0x81, 0x04, 0x00, 0x23];
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
        if oid2_tag != 0x06 || oid2 != secp521r1_oid {
            return None;
        }
        let (_bs_tag, bs_val) = der_read_tlv(outer, &mut inner)?;
        if _bs_tag != 0x03 || bs_val.is_empty() {
            return None;
        }
        Some(bs_val[1..].to_vec())
    }

    // RFC 6979 A.2.7: P-521 with SHA-512 test key pair.
    const RFC6979_PRIVATE_KEY: &str = "00fad06daa62ba3b25d2fb40133da757205de67f5bb0018fee8c86e1b68c7e75c\
                                       aa896eb32f1f47c70855836a6d16fcc1466f6d8fbec67db89ec0c08b0e996b83\
                                       538";
    const RFC6979_PUBLIC_X: &str = "01894550d0785932e00eaa23b694f213f8c3121f86dc97a04e5a7167db4e5bcd3\
                                    71123d46e45db6b5d5370a7f20fb633155d38ffa16d2bd761dcac474b9a2f502\
                                    3a4";
    const RFC6979_PUBLIC_Y: &str = "00493101c962cd4d2fddf782285e64584139c2f91b47f87ff82354d6630f746a2\
                                    8a0db25741b5b34a828008b22acc23f924faafbd4d33f81ea66956dfeaa2bfdf\
                                    cf5";

    #[test]
    fn derive_public_key_generator_matches_sec1_base_point() {
        let mut private_key = [0u8; SECRET_KEY_SIZE];
        private_key[65] = 1;
        let derived = derive_public_key_uncompressed(&private_key).unwrap();
        let expected = decode_hex::<133>(
            "0400c6858e06b70404e9cd9e3ecb662395b4429c648139053fb521f828af606b4d3dbaa14b5e77efe75928fe1dc127a2ffa8de3348b3c1856a429bf97e7e31c2e5bd66011839296a789a3bc0045c8a5fb42c7d1bd998f54449579b446817afbd17273e662c97ee72995ef42640c550b9013fad0761353c7086a272c24088be94769fd16650",
        );
        assert_eq!(derived, expected);
    }

    #[test]
    fn derive_public_key_matches_rfc6979_vector() {
        let private_key = decode_hex::<66>(RFC6979_PRIVATE_KEY);
        let expected = decode_hex::<133>(
            "0401894550d0785932e00eaa23b694f213f8c3121f86dc97a04e5a7167db4e5bcd371123d46e45db6b5d5370a7f20fb633155d38ffa16d2bd761dcac474b9a2f5023a400493101c962cd4d2fddf782285e64584139c2f91b47f87ff82354d6630f746a28a0db25741b5b34a828008b22acc23f924faafbd4d33f81ea66956dfeaa2bfdfcf5",
        );
        assert_eq!(derive_public_key_uncompressed(&private_key).unwrap(), expected);
        assert_eq!(
            PublicKey::from_x_y(&decode_hex::<66>(RFC6979_PUBLIC_X), &decode_hex::<66>(RFC6979_PUBLIC_Y),)
                .unwrap()
                .to_bytes(),
            expected,
        );

        // Compressed form derives from the same point; prefix encodes y parity.
        let compressed = derive_public_key_compressed(&private_key).unwrap();
        assert_eq!(&compressed[1..], &decode_hex::<66>(RFC6979_PUBLIC_X));
    }

    #[test]
    fn ecdsa_sign_matches_rfc6979_vectors() {
        let private_key = decode_hex::<66>(RFC6979_PRIVATE_KEY);
        let key = SecretKey::from_bytes(&private_key).unwrap();

        // RFC 6979 A.2.7, SHA-512 "sample". The vector's s is already the
        // canonical low-s value, so the emitted signature matches it verbatim.
        let sample_signature = key.sign(b"sample").unwrap();
        let expected_sample = decode_hex::<132>(
            "00c328fafcbd79dd77850370c46325d987cb525569fb63c5d3bc53950e6d4c5f174e25a1ee9017b5d450606add152b534931d7d4e8455cc91f9b15bf05ec36e377fa\
             00617cce7cf5064806c467f678d3b4080d6f1cc50af26ca209417308281b68af282623eaa63e5b5c0723d8b8c37ff0777b1a20f8ccb1dccc43997f1ee0e44da4a67a",
        );
        assert_eq!(sample_signature, expected_sample);

        // The "test" vector's `s` (`01fb…dce3`) is in high-s form, so the
        // canonical low-s signature emitted here is `r || (n - s)`.
        let test_signature = key.sign(b"test").unwrap();
        let expected_test = decode_hex::<132>(
            "013e99020abf5cee7525d16b69b229652ab6bdf2affcaef38773b4b7d08725f10cdb93482fdcc54edcee91eca4166b2a7c6265ef0ce2bd7051b7cef945babd47ee6d00042ffec398b558634c67b6ad86e931cfe39915831747f97d879529f067081875e0871c978df9bb9587cc2b53e229002daeeea20ec39fefe7398bf4a1c063538726",
        );
        assert_eq!(test_signature, expected_test);
    }

    #[test]
    fn rfc6979_nonce_generation_matches_known_value() {
        // P-521's order (521 bits) is wider than SHA-512's output (512 bits),
        // so RFC 6979 must concatenate two HMAC blocks before `bits2int`.
        let private_key = Scalar::from_bytes(&decode_hex::<66>(RFC6979_PRIVATE_KEY)).unwrap();
        let hash = hash_message(b"sample");
        assert_eq!(
            Rfc6979::new(&private_key, &hash).generate().to_bytes(),
            decode_hex::<66>(
                "01dae2ea071f8110dc26882d4d5eae0621a3256fc8847fb9022e2b7d28e6f10198b1574fdd03a9053c08a1854a168aa5a57470ec97dd5ce090124ef52a2f7ecbffd3"
            )
        );
    }

    #[test]
    fn rfc6979_test_message_nonce_matches_known_value() {
        let private_key = Scalar::from_bytes(&decode_hex::<66>(RFC6979_PRIVATE_KEY)).unwrap();
        let hash = hash_message(b"test");
        assert_eq!(
            Rfc6979::new(&private_key, &hash).generate().to_bytes(),
            decode_hex::<66>(
                "016200813020ec986863bedfc1b121f605c1215645018aea1a7b215a564de9eb1b38a67aa1128b80ce391c4fb71187654aaa3431027bfc7f395766ca988c964dc56d"
            )
        );
    }

    #[test]
    fn rfc6979_retry_advances_drbg() {
        let private_key = Scalar::from_bytes(&decode_hex::<66>(RFC6979_PRIVATE_KEY)).unwrap();
        let hash = hash_message(b"sample");

        let mut drbg = Rfc6979::new(&private_key, &hash);
        let k1 = drbg.generate();
        assert_eq!(
            k1.to_bytes(),
            decode_hex::<66>(
                "01dae2ea071f8110dc26882d4d5eae0621a3256fc8847fb9022e2b7d28e6f10198b1574fdd03a9053c08a1854a168aa5a57470ec97dd5ce090124ef52a2f7ecbffd3"
            )
        );

        // RFC 6979 §3.2 h.3: an r=0/s=0 retry must continue the DRBG, not
        // reproduce the same nonce.
        drbg.retry();
        let k2 = drbg.generate();
        assert_ne!(k1.to_bytes(), k2.to_bytes());
    }

    #[test]
    fn ecdsa_forced_retry_uses_fresh_nonce() {
        let private_key = decode_hex::<66>(RFC6979_PRIVATE_KEY);
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
    fn ecdsa_sign_emits_canonical_low_s() {
        let private_key = decode_hex::<66>(RFC6979_PRIVATE_KEY);
        let key = SecretKey::from_bytes(&private_key).unwrap();
        for msg in [&b"sample"[..], b"test", b"", b"another message"] {
            let sig = key.sign(msg).unwrap();
            let s = Scalar::from_bytes(sig[66..].try_into().unwrap()).unwrap();
            assert!(!s.is_high(), "sign produced a high-s signature for {:?}", msg);
        }
    }

    #[test]
    fn ecdsa_verify_strict_rejects_high_s() {
        let private_key = decode_hex::<66>(RFC6979_PRIVATE_KEY);
        let key = SecretKey::from_bytes(&private_key).unwrap();
        let public_key = key.public_key();
        let signature = key.sign(b"sample").unwrap();

        // Canonical low-s signature passes both permissive and strict verification.
        assert!(public_key.verify(b"sample", &signature).is_ok());
        assert!(public_key.verify_strict(b"sample", &signature).is_ok());

        // The malleable counterpart (r, n - s) is rejected only by strict verify.
        let r = Scalar::from_bytes(signature[..66].try_into().unwrap()).unwrap();
        let s = Scalar::from_bytes(signature[66..].try_into().unwrap()).unwrap();
        let high_s = Scalar::ZERO.sub(s);
        assert!(high_s.is_high());

        let mut malleable = [0u8; SIGNATURE_SIZE];
        malleable[..66].copy_from_slice(&r.to_bytes());
        malleable[66..].copy_from_slice(&high_s.to_bytes());

        assert!(public_key.verify(b"sample", &malleable).is_ok());
        assert!(public_key.verify_strict(b"sample", &malleable).is_err());
    }

    #[test]
    fn scalar_from_bytes_rejects_boundary_values() {
        let zero = [0u8; 66];
        assert!(Scalar::from_bytes(&zero).is_none());

        let n_bytes = decode_hex::<66>(
            "01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e91386409",
        );
        assert!(Scalar::from_bytes(&n_bytes).is_none());

        let n_minus_1 = decode_hex::<66>(
            "01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e91386408",
        );
        assert!(Scalar::from_bytes(&n_minus_1).is_some());

        let one = decode_hex::<66>(
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001",
        );
        assert!(Scalar::from_bytes(&one).is_some());
    }

    #[test]
    fn field_element_from_bytes_rejects_boundary_values() {
        let p_bytes = decode_hex::<66>(
            "01ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        );
        assert!(FieldElement::from_bytes(&p_bytes).is_none());

        let p_minus_1 = decode_hex::<66>(
            "01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe",
        );
        assert!(FieldElement::from_bytes(&p_minus_1).is_some());

        let zero = [0u8; 66];
        assert!(FieldElement::from_bytes(&zero).is_some());
    }

    #[test]
    fn point_decompression_round_trip() {
        let keys: &[&str] = &[
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001",
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002",
            RFC6979_PRIVATE_KEY,
        ];

        for key_hex in keys {
            let private_key = decode_hex::<66>(key_hex);
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
        let k = Scalar::from_bytes(&decode_hex::<66>(
            "01894550d0785932e00eaa23b694f213f8c3121f86dc97a04e5a7167db4e5bcd371123d46e45db6b5d5370a7f20fb633155d38ffa16d2bd761dcac474b9a2f5023a4",
        ))
        .unwrap();
        let k_inv = k.invert().unwrap();
        let product = k.mul(k_inv);
        assert_eq!(product, Scalar::ONE);
    }

    #[test]
    fn field_element_inversion_correctness() {
        let x = FieldElement::from_bytes(&decode_hex::<66>(
            "00c6858e06b70404e9cd9e3ecb662395b4429c648139053fb521f828af606b4d3dbaa14b5e77efe75928fe1dc127a2ffa8de3348b3c1856a429bf97e7e31c2e5bd66",
        ))
        .unwrap();
        let x_inv = x.invert().unwrap();
        let product = x.mul(x_inv);
        assert_eq!(product, FieldElement::ONE);
    }

    #[test]
    fn p521_fast_mul_mod_matches_generic() {
        for _ in 0..1000 {
            let a_bytes: [u8; 66] = rand::random();
            let b_bytes: [u8; 66] = rand::random();
            let a_opt = FieldElement::from_bytes(&a_bytes);
            let b_opt = FieldElement::from_bytes(&b_bytes);
            if a_opt.is_none() || b_opt.is_none() {
                continue;
            }
            let a = a_opt.unwrap();
            let b = b_opt.unwrap();
            let expected = U521::from_limbs({
                let mut p = [0u64; 18];
                for i in 0..9 {
                    let mut c = 0u64;
                    for j in 0..9 {
                        let (v, cc) = mac(p[i + j], a.0.limbs[i], b.0.limbs[j], c);
                        p[i + j] = v;
                        c = cc;
                    }
                    p[i + 9] = c;
                }
                let mut rem = [0u64; 9];
                for bi in (0..1152).rev() {
                    let li = bi / 64;
                    let pi = bi % 64;
                    let bit = ((p[li] >> pi) & 1) as u64;
                    let mut shifted = [0u64; 9];
                    let mut carry = bit;
                    for j in 0..9 {
                        let next = rem[j] >> 63;
                        shifted[j] = (rem[j] << 1) | carry;
                        carry = next;
                    }
                    let (red, br) = U521::from_limbs(shifted).sub_raw(&MODULUS_P);
                    if carry == 1 || br == 0 {
                        rem = red.limbs;
                    } else {
                        rem = shifted;
                    }
                }
                rem
            });
            let fast = p521_fast_mul_mod(&a.0, &b.0);
            assert_eq!(expected, fast, "mismatch in p521_fast_mul_mod");
        }
    }

    #[test]
    fn scalar_mul_generator_n_gives_identity() {
        let n_minus_1 = Scalar::from_bytes(&decode_hex::<66>(
            "01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e91386408",
        ))
        .unwrap();
        let result = scalar_mul_generator(&n_minus_1).to_affine().unwrap();
        assert_eq!(result.x, FieldElement::from_uint(GENERATOR_X));
        let neg_gy = FieldElement::from_uint(GENERATOR_Y).negate();
        assert_eq!(result.y, neg_gy);
    }

    #[test]
    fn scalar_mul_by_two_matches_double() {
        let two = Scalar::from_bytes(&decode_hex::<66>("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002"))
            .unwrap();
        let g_times_2 = scalar_mul_affine(&AffinePoint::GENERATOR, &two).to_affine().unwrap();
        let proj_g = ProjectivePoint::from_affine(&AffinePoint::GENERATOR);
        let g_doubled = proj_g.double().to_affine().unwrap();

        assert_eq!(g_times_2.to_uncompressed_bytes(), g_doubled.to_uncompressed_bytes());
    }

    #[test]
    fn ecdh_deterministic_vector_against_generator() {
        // ECDH between the RFC 6979 private key and the generator: the shared
        // secret is the x-coordinate of the derived public key.
        let private_key = decode_hex::<66>(RFC6979_PRIVATE_KEY);
        let generator = decode_hex::<133>(
            "0400c6858e06b70404e9cd9e3ecb662395b4429c648139053fb521f828af606b4d3dbaa14b5e77efe75928fe1dc127a2ffa8de3348b3c1856a429bf97e7e31c2e5bd66011839296a789a3bc0045c8a5fb42c7d1bd998f54449579b446817afbd17273e662c97ee72995ef42640c550b9013fad0761353c7086a272c24088be94769fd16650",
        );

        let key = SecretKey::from_bytes(&private_key).unwrap();
        let shared = ecdh(&private_key, &generator).unwrap();
        assert_eq!(shared, key.public_key().x_y().0);
        assert_eq!(shared, decode_hex::<66>(RFC6979_PUBLIC_X));
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
    fn ecdh_rejects_invalid_private_key_zero() {
        let zero_key = [0u8; 66];
        assert!(SecretKey::from_bytes(&zero_key).is_err());
        let bob = SecretKey::generate().unwrap();
        assert!(ecdh(&zero_key, &bob.public_key().to_bytes()).is_err());
    }

    #[test]
    fn ecdsa_rejects_non_canonical_r_and_s() {
        let key = SecretKey::generate().unwrap();
        let mut bad_r = key.sign(b"msg").unwrap();
        bad_r[..66].copy_from_slice(&decode_hex::<66>(
            "01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e9138640a",
        ));
        assert!(key.public_key().verify(b"msg", &bad_r).is_err());

        let mut bad_s = key.sign(b"msg").unwrap();
        bad_s[66..].copy_from_slice(&decode_hex::<66>(
            "01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e9138640a",
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
        off_curve[132] ^= 0x01;
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
            &decode_hex::<66>(
                "00c6858e06b70404e9cd9e3ecb662395b4429c648139053fb521f828af606b4d3dbaa14b5e77efe75928fe1dc127a2ffa8de3348b3c1856a429bf97e7e31c2e5bd66",
            ),
            &decode_hex::<66>(
                "011839296a789a3bc0045c8a5fb42c7d1bd998f54449579b446817afbd17273e662c97ee72995ef42640c550b9013fad0761353c7086a272c24088be94769fd16650",
            ),
        )
        .unwrap();
        let from_sec1 = PublicKey::from_bytes(&key.to_bytes()).unwrap();
        assert_eq!(key, from_sec1);
    }

    #[test]
    fn field_element_add_sub_mul_consistency() {
        let a = FieldElement::from_bytes(&decode_hex::<66>(
            "00c6858e06b70404e9cd9e3ecb662395b4429c648139053fb521f828af606b4d3dbaa14b5e77efe75928fe1dc127a2ffa8de3348b3c1856a429bf97e7e31c2e5bd66",
        ))
        .unwrap();
        let b = FieldElement::from_bytes(&decode_hex::<66>(
            "011839296a789a3bc0045c8a5fb42c7d1bd998f54449579b446817afbd17273e662c97ee72995ef42640c550b9013fad0761353c7086a272c24088be94769fd16650",
        ))
        .unwrap();

        assert_eq!(a.add(b).sub(b), a);
        assert_eq!(a.add(b), b.add(a));
        assert_eq!(a.mul(b), b.mul(a));

        let c = FieldElement::from_bytes(&decode_hex::<66>(
            "0051953eb9618e1c9a1f929a21a0b68540eea2da725b99b315f3b8b489918ef109e156193951ec7e937b1652c0bd3bb1bf073573df883d2c34f1ef451fd46b503f00",
        ))
        .unwrap();
        assert_eq!(a.add(b).mul(c), a.mul(c).add(b.mul(c)));
    }

    #[test]
    fn scalar_add_sub_mul_consistency() {
        let a = Scalar::from_bytes(&decode_hex::<66>(
            "01894550d0785932e00eaa23b694f213f8c3121f86dc97a04e5a7167db4e5bcd371123d46e45db6b5d5370a7f20fb633155d38ffa16d2bd761dcac474b9a2f5023a4",
        ))
        .unwrap();
        let one = Scalar::from_bytes(&decode_hex::<66>("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001"))
            .unwrap();

        assert_eq!(a.add(one).sub(one), a);
        assert_eq!(a.mul(one), a);

        let b = Scalar::from_bytes(&decode_hex::<66>(
            "00493101c962cd4d2fddf782285e64584139c2f91b47f87ff82354d6630f746a28a0db25741b5b34a828008b22acc23f924faafbd4d33f81ea66956dfeaa2bfdfcf5",
        ))
        .unwrap();
        assert_eq!(a.mul(b), b.mul(a));
        assert_eq!(a.add(b), b.add(a));
    }

    #[test]
    fn field_element_negate_round_trip() {
        let x = FieldElement::from_bytes(&decode_hex::<66>(
            "00c6858e06b70404e9cd9e3ecb662395b4429c648139053fb521f828af606b4d3dbaa14b5e77efe75928fe1dc127a2ffa8de3348b3c1856a429bf97e7e31c2e5bd66",
        ))
        .unwrap();
        let neg = x.negate();
        assert_eq!(neg.negate(), x);
        assert_eq!(x.add(neg), FieldElement::ZERO);
    }

    #[test]
    fn field_element_pow_correctness() {
        let x = FieldElement::from_bytes(&decode_hex::<66>("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002"))
            .unwrap();
        let x3 = x.pow(&U521::from_u64(3));
        let expected = x.mul(x).mul(x);
        assert_eq!(x3, expected);

        let x0 = x.pow(&U521::ZERO);
        assert_eq!(x0, FieldElement::ONE);
    }

    #[test]
    fn sqrt_matches_squares_and_rejects_non_residues() {
        for _ in 0..100 {
            let bytes: [u8; 66] = rand::random();
            let Some(x) = FieldElement::from_bytes(&bytes) else {
                continue;
            };
            let square = x.square();
            let root = square.sqrt().expect("square should have a square root");
            assert_eq!(root.square(), square);
        }
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

    // --- Wycheproof test vectors ---

    #[test]
    fn wycheproof_ecdsa_p521_sha512_p1363() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/ecdsa_secp521r1_sha512_p1363_test.json"
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
                    assert!(
                        verify_result.is_ok(),
                        "wycheproof ECDSA P521 P1363 tcId={} expected valid but failed",
                        test["tcId"]
                    );
                    valid_tested += 1;
                } else {
                    assert!(
                        verify_result.is_err(),
                        "wycheproof ECDSA P521 P1363 tcId={} expected invalid but passed",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid ECDSA P521 P1363 wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid ECDSA P521 P1363 wycheproof tests were run");
    }

    #[test]
    fn wycheproof_ecdsa_p521_sha512_der() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/ecdsa_secp521r1_sha512_test.json"
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
                        "wycheproof ECDSA P521 DER SHA-512 tcId={} expected valid but failed",
                        test["tcId"]
                    );
                    valid_tested += 1;
                } else {
                    assert!(
                        verify_result.is_err(),
                        "wycheproof ECDSA P521 DER SHA-512 tcId={} expected invalid but passed",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid ECDSA P521 DER SHA-512 wycheproof tests were run");
        assert!(
            invalid_tested > 0,
            "no invalid ECDSA P521 DER SHA-512 wycheproof tests were run"
        );
    }

    #[test]
    fn wycheproof_ecdh_p521_ecpoint() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/ecdh_secp521r1_ecpoint_test.json"
        ))
        .unwrap();
        let mut valid_tested = 0u64;
        let mut invalid_tested = 0u64;
        let mut acceptable_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            if group["curve"].as_str() != Some("secp521r1") {
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
                        "wycheproof ECDH P521 ecpoint tcId={}",
                        test["tcId"]
                    );
                    valid_tested += 1;
                } else if result == "invalid" {
                    assert!(
                        shared.is_err(),
                        "wycheproof ECDH P521 ecpoint tcId={} expected invalid but passed",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                } else {
                    acceptable_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid ECDH P521 ecpoint wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid ECDH P521 ecpoint wycheproof tests were run");
        assert!(
            acceptable_tested > 0,
            "no acceptable ECDH P521 ecpoint wycheproof tests were run"
        );
    }

    #[test]
    fn wycheproof_ecdh_p521_asn() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../../testdata/wycheproof/testvectors_v1/ecdh_secp521r1_test.json"
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
                        panic!("wycheproof ECDH P521 ASN tcId={}: failed to parse valid SPKI", test["tcId"]);
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
                        "wycheproof ECDH P521 ASN tcId={}",
                        test["tcId"]
                    );
                    valid_tested += 1;
                } else if result == "invalid" {
                    assert!(
                        shared.is_err(),
                        "wycheproof ECDH P521 ASN tcId={} expected invalid but passed",
                        test["tcId"]
                    );
                    invalid_tested += 1;
                } else {
                    acceptable_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid ECDH P521 ASN wycheproof tests were run");
        assert!(invalid_tested > 0, "no invalid ECDH P521 ASN wycheproof tests were run");
        assert!(acceptable_tested > 0, "no acceptable ECDH P521 ASN wycheproof tests were run");
    }

    // The `bits2octets` and message-hash paths must reduce a 512-bit digest
    // that is shorter than the 521-bit order.
    #[test]
    fn bits2octets_reduces_digest() {
        let hash = hash_message(b"sample");
        assert_eq!(bits2octets(&hash), <P521 as Curve>::bits2int_octets(&hash));
    }

    // Shared curve-independent tests, instantiated for P521.
    #[test]
    fn generator_point_is_on_curve() {
        p_curves::test_support::generator_point_is_on_curve::<P521>();
    }

    #[test]
    fn from_x_y_rejects_off_curve() {
        p_curves::test_support::from_x_y_rejects_off_curve::<P521>();
    }

    #[test]
    fn ecdh_round_trip_alice_bob() {
        p_curves::test_support::ecdh_round_trip_alice_bob::<P521>();
    }

    #[test]
    fn ecdh_rejects_off_curve_peer_public_key() {
        p_curves::test_support::ecdh_rejects_off_curve_peer_public_key::<P521>();
    }

    #[test]
    fn ecdh_rejects_infinity_peer_public_key() {
        p_curves::test_support::ecdh_rejects_infinity_peer_public_key::<P521>();
    }

    #[test]
    fn ecdh_rejects_bad_length_peer_public_key() {
        p_curves::test_support::ecdh_rejects_bad_length_peer_public_key::<P521>();
    }

    #[test]
    fn ecdh_multiple_exchanges_consistency() {
        p_curves::test_support::ecdh_multiple_exchanges_consistency::<P521>();
    }

    #[test]
    fn private_key_round_trip_bytes() {
        p_curves::test_support::private_key_round_trip_bytes::<P521>();
    }

    #[test]
    fn public_key_round_trip_bytes() {
        p_curves::test_support::public_key_round_trip_bytes::<P521>();
    }

    #[test]
    fn point_double_and_add_consistency() {
        p_curves::test_support::point_double_and_add_consistency::<P521>();
    }

    #[test]
    fn compressed_public_key_has_correct_prefix() {
        p_curves::test_support::compressed_public_key_has_correct_prefix::<P521>();
    }

    #[cfg(feature = "zeroize")]
    #[test]
    fn rfc6979_state_zeroize_clears_drbg() {
        p_curves::test_support::rfc6979_state_zeroize_clears_drbg::<P521>();
    }
}
