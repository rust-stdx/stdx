use constant_time_eq::constant_time_eq;

use super::{
    curve25519::{FieldElement, U256},
    ed25519,
};
use crate::{EllipticCurveError, Hasher, sha2::Sha512};

pub const KEY_SIZE: usize = 32;
pub const SHARED_SECRET_SIZE: usize = 32;

const A24: FieldElement = FieldElement(U256::from_u64(121665));

const BASEPOINT_U: [u8; 32] = {
    let mut u = [0u8; 32];
    u[0] = 9;
    u
};

/// X25519 Diffie-Hellman key exchange private key (RFC 7748).
///
/// # Generating a key
///
/// ```ignore
/// use crypto::curve25519::x25519::SecretKey;
///
/// let alice = SecretKey::generate().unwrap();
/// let bob = SecretKey::generate().unwrap();
/// ```
///
/// # Diffie-Hellman key exchange
///
/// ```ignore
/// use crypto::curve25519::x25519::SecretKey;
///
/// let alice = SecretKey::generate().unwrap();
/// let bob = SecretKey::generate().unwrap();
/// let alice_shared = alice.ecdh(&bob.public_key()).unwrap();
/// let bob_shared = bob.ecdh(&alice.public_key()).unwrap();
/// assert_eq!(alice_shared, bob_shared);
/// ```
///
/// [`ecdh`](Self::ecdh) returns [`EllipticCurveError::InvalidSharedSecret`]
/// when the computed shared secret is all zeros. This happens whenever the
/// peer public key is a low-order point (`u = 0`, `u = 1`, `u = p - 1`, or
/// one of the two order-8 points), because the clamped scalar is a multiple
/// of the curve's cofactor and therefore annihilates such a point.
///
/// **Security impact:** an all-zero shared secret is not secret at all. Every
/// peer that sends a low-order public key — including a malicious one — knows
/// the value, so any key derived from it (even through a KDF) is known to the
/// attacker. This yields a complete confidentiality and authentication break
/// for protocols that do not independently detect the degenerate output.
/// Callers should therefore treat the error as a fatal protocol failure and
/// abort the handshake.
///
/// If the caller's protocol already binds both public keys into the derived
/// key (so that a non-contributory secret cannot be exploited), the error can
/// be deliberately ignored by substituting the all-zero secret:
///
/// ```ignore
/// let shared = alice.ecdh(&bob.public_key()).unwrap_or_default();
/// ```
///
/// # Conversion from Ed25519
///
/// An Ed25519 [`SecretKey`](crate::curve25519::ed25519::SecretKey) can be
/// converted to an X25519 key via [`From<&ed25519::SecretKey>`](From).
///
/// # Security
///
/// The shared secret produced by [`ecdh`](Self::ecdh) **must not** be used
/// directly as an encryption key. Apply a KDF (e.g. HKDF) first.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct SecretKey {
    bytes: [u8; KEY_SIZE],
    public_key: FieldElement,
}

impl SecretKey {
    /// Generates a fresh random secret key.
    ///
    /// Returns [`EllipticCurveError::Random`] when the operating system's
    /// random number generator is unavailable or fails.
    #[cfg(feature = "random")]
    pub fn generate() -> Result<SecretKey, EllipticCurveError> {
        let bytes: [u8; KEY_SIZE] = crate::random::bytes()?;
        Ok(SecretKey::from_bytes(&bytes))
    }

    pub fn from_bytes(bytes: &[u8; KEY_SIZE]) -> SecretKey {
        let public_key = x25519_inner(bytes, FieldElement::from_relaxed_bytes(&BASEPOINT_U));
        SecretKey {
            bytes: *bytes,
            public_key,
        }
    }

    #[inline]
    pub fn to_bytes(&self) -> [u8; KEY_SIZE] {
        self.bytes
    }

    #[inline]
    pub fn public_key(&self) -> PublicKey {
        PublicKey {
            u: self.public_key,
        }
    }

    /// Perform a Diffie-Hellman key exchange to derive a shared secret.
    /// The shared secret **IS NOT SAFE** to use directly as an encryption key and an additional
    /// key derivation operation must be applied to it.
    ///
    /// # Errors
    ///
    /// Returns [`EllipticCurveError::InvalidSharedSecret`] if the computed
    /// shared secret is all zeros, which occurs when `peer` is a low-order
    /// public key. Such a secret is publicly known and must not be used to
    /// derive keys unless the protocol independently binds the peer public
    /// key into the derivation. See the [type-level documentation](SecretKey)
    /// for details.
    pub fn ecdh(&self, peer: &PublicKey) -> Result<[u8; SHARED_SECRET_SIZE], EllipticCurveError> {
        let result = x25519_inner(&self.bytes, peer.u);
        if result.is_zero() {
            return Err(EllipticCurveError::InvalidSharedSecret);
        }
        Ok(result.to_bytes())
    }
}

impl From<&ed25519::SecretKey> for SecretKey {
    fn from(ed25519_key: &ed25519::SecretKey) -> Self {
        let digest = Sha512::hash(&ed25519_key.to_bytes());
        let mut expanded = [0u8; 64];
        expanded.copy_from_slice(digest.as_ref());
        expanded[0] &= 248;
        expanded[31] &= 127;
        expanded[31] |= 64;
        let mut bytes = [0u8; KEY_SIZE];
        bytes.copy_from_slice(&expanded[..32]);

        let public_key = x25519_inner(&bytes, FieldElement::from_relaxed_bytes(&BASEPOINT_U));

        SecretKey {
            bytes,
            public_key,
        }
    }
}

impl From<&[u8; KEY_SIZE]> for SecretKey {
    fn from(bytes: &[u8; KEY_SIZE]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&[u8]> for SecretKey {
    type Error = EllipticCurveError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::from_bytes(bytes.try_into().map_err(|_| EllipticCurveError::InvalidKey)?))
    }
}

/// X25519 Diffie-Hellman key exchange public key (RFC 7748).
///
/// # Deserializing from bytes
///
/// ```ignore
/// use crypto::curve25519::x25519::PublicKey;
///
/// let bytes = [0u8; 32]; // replace with a peer's public key
/// let pub_key = PublicKey::from_bytes(&bytes);
/// ```
///
/// # Conversion from Ed25519
///
/// An Ed25519 [`PublicKey`](crate::curve25519::ed25519::PublicKey) can be
/// converted to an X25519 public key via [`TryFrom<&ed25519::PublicKey>`](TryFrom).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicKey {
    u: FieldElement,
}

/// Canonical u-coordinates of the low-order points on Curve25519.
///
/// These are the points whose order divides the curve cofactor (8). Because
/// X25519 clamps the scalar to a multiple of 8, multiplying any of them by a
/// valid scalar yields the point at infinity, encoded as `u = 0`. Non-canonical
/// encodings (`u >= p`) are reduced by [`FieldElement::from_relaxed_bytes`]
/// before this comparison, so they are covered too.
const LOW_ORDER_U: [[u8; KEY_SIZE]; 5] = [
    // u = 0
    [0u8; KEY_SIZE],
    // u = 1
    {
        let mut u = [0u8; KEY_SIZE];
        u[0] = 1;
        u
    },
    // u = p - 1
    [
        0xec, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f,
    ],
    // order-8 point
    [
        0xe0, 0xeb, 0x7a, 0x7c, 0x3b, 0x41, 0xb8, 0xae, 0x16, 0x56, 0xe3, 0xfa, 0xf1, 0x9f, 0xc4, 0x6a, 0xda, 0x09,
        0x8d, 0xeb, 0x9c, 0x32, 0xb1, 0xfd, 0x86, 0x62, 0x05, 0x16, 0x5f, 0x49, 0xb8, 0x00,
    ],
    // order-8 point
    [
        0x5f, 0x9c, 0x95, 0xbc, 0xa3, 0x50, 0x8c, 0x24, 0xb1, 0xd0, 0xb1, 0x55, 0x9c, 0x83, 0xef, 0x5b, 0x04, 0x44,
        0x5c, 0xc4, 0x58, 0x1c, 0x8e, 0x86, 0xd8, 0x22, 0x4e, 0xdd, 0xd0, 0x9f, 0x11, 0x57,
    ],
];

impl PublicKey {
    pub fn from_bytes(bytes: &[u8; KEY_SIZE]) -> Self {
        let u = FieldElement::from_relaxed_bytes(bytes);
        PublicKey {
            u,
        }
    }

    #[inline]
    pub fn to_bytes(&self) -> [u8; KEY_SIZE] {
        self.u.to_bytes()
    }

    /// Returns `true` if this public key is a low-order point on Curve25519.
    ///
    /// Low-order public keys are never produced by honest key generation.
    /// Passing one to [`SecretKey::ecdh`] produces an all-zero, publicly known
    /// shared secret, so it must be rejected or otherwise neutralized. The
    /// comparison is constant-time with respect to the key bytes.
    pub fn is_low_order(&self) -> bool {
        let bytes = self.u.to_bytes();
        let mut found = 0u8;
        for candidate in &LOW_ORDER_U {
            found |= constant_time_eq(&bytes, candidate) as u8;
        }
        found != 0
    }
}

impl From<&[u8; KEY_SIZE]> for PublicKey {
    fn from(bytes: &[u8; KEY_SIZE]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&[u8]> for PublicKey {
    type Error = EllipticCurveError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::from_bytes(bytes.try_into().map_err(|_| EllipticCurveError::InvalidKey)?))
    }
}

impl TryFrom<&ed25519::PublicKey> for PublicKey {
    type Error = EllipticCurveError;

    fn try_from(ed25519_public_key: &ed25519::PublicKey) -> Result<Self, Self::Error> {
        let mut y_bytes = ed25519_public_key.to_bytes();
        y_bytes[31] &= 0x7f;
        let y = FieldElement::from_canonical_bytes(&y_bytes).ok_or(EllipticCurveError::InvalidKey)?;
        let one = FieldElement::ONE;
        let u = (one.add(y)).mul((one.sub(y)).invert().ok_or(EllipticCurveError::InvalidKey)?);
        Ok(PublicKey {
            u,
        })
    }
}

#[inline]
fn clamp_scalar(mut scalar: [u8; 32]) -> [u8; 32] {
    scalar[0] &= 248;
    scalar[31] &= 127;
    scalar[31] |= 64;
    scalar
}

#[inline]
fn cswap(swap: bool, a: &mut FieldElement, b: &mut FieldElement) {
    let tmp = FieldElement::select(b, a, swap);
    *b = FieldElement::select(a, b, swap);
    *a = tmp;
}

fn x25519_inner(scalar: &[u8; 32], u: FieldElement) -> FieldElement {
    let clamped = clamp_scalar(*scalar);
    let x_1 = u;
    let mut x_2 = FieldElement::ONE;
    let mut z_2 = FieldElement::ZERO;
    let mut x_3 = u;
    let mut z_3 = FieldElement::ONE;
    let mut swap = false;

    let mut t: isize = 254;
    while t >= 0 {
        let k_t = ((clamped[(t as usize) / 8] >> ((t as usize) % 8)) & 1) != 0;

        swap ^= k_t;
        cswap(swap, &mut x_2, &mut x_3);
        cswap(swap, &mut z_2, &mut z_3);
        swap = k_t;

        let a = x_2.add(z_2);
        let aa = a.square();
        let b = x_2.sub(z_2);
        let bb = b.square();
        let e = aa.sub(bb);
        let c = x_3.add(z_3);
        let d = x_3.sub(z_3);
        let da = d.mul(a);
        let cb = c.mul(b);
        x_3 = da.add(cb).square();
        z_3 = x_1.mul(da.sub(cb).square());
        x_2 = aa.mul(bb);
        z_2 = e.mul(aa.add(A24.mul(e)));

        t -= 1;
    }

    cswap(swap, &mut x_2, &mut x_3);
    cswap(swap, &mut z_2, &mut z_3);

    if z_2.is_zero() {
        return FieldElement::ZERO;
    }

    x_2.mul(z_2.invert().expect("z_2 must be non-zero"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curve25519::curve25519::{P, U256};

    #[cfg(feature = "zeroize")]
    #[test]
    fn secret_key_zeroize_clears_bytes() {
        use zeroize::Zeroize;

        let mut key = SecretKey::from_bytes(&[1u8; KEY_SIZE]);
        assert_ne!(key.to_bytes(), [0u8; KEY_SIZE]);
        key.zeroize();
        assert_eq!(key.to_bytes(), [0u8; KEY_SIZE]);
    }

    fn x25519(private_key: &[u8; KEY_SIZE], public_key: &[u8; KEY_SIZE]) -> [u8; SHARED_SECRET_SIZE] {
        let priv_key = SecretKey::from_bytes(private_key);
        let pub_key = PublicKey::from_bytes(public_key);
        priv_key
            .ecdh(&pub_key)
            .expect("test vector must have a non-zero shared secret")
    }

    fn x25519_checked(
        private_key: &[u8; KEY_SIZE],
        public_key: &[u8; KEY_SIZE],
    ) -> Result<[u8; SHARED_SECRET_SIZE], EllipticCurveError> {
        SecretKey::from_bytes(private_key).ecdh(&PublicKey::from_bytes(public_key))
    }

    fn decode_hex<const N: usize>(hex_str: &str) -> [u8; N] {
        let bytes = hex::decode(hex_str).unwrap();
        assert_eq!(bytes.len(), N, "hex string must decode to exactly {N} bytes");
        let mut out = [0u8; N];
        out.copy_from_slice(&bytes);
        out
    }

    struct DhTestVector {
        alice_private: &'static str,
        alice_public: &'static str,
        bob_private: &'static str,
        bob_public: &'static str,
        shared_secret: &'static str,
    }

    #[test]
    fn key_exchange() {
        let alice_priv = decode_hex::<32>("77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a");
        let bob_priv = decode_hex::<32>("5dab087e624a8a4b79e17f8b83800ee66f3bb1292618b6fd1c2f8b27ff88e0eb");
        let expected_alice_pub = decode_hex::<32>("8520f0098930a754748b7ddcb43ef75a0dbf3a0d26381af4eba4a98eaa9b4e6a");
        let expected_bob_pub = decode_hex::<32>("de9edb7d7b7dc1b4d35b61c2ece435373f8343c85b78674dadfc7e146f882b4f");
        let expected_shared = decode_hex::<32>("4a5d9d5ba4ce2de1728e3bf480350f25e07e21c947d19e3376f09b3c1e161742");

        let alice = SecretKey::from_bytes(&alice_priv);
        let bob = SecretKey::from_bytes(&bob_priv);

        assert_eq!(alice.public_key().to_bytes(), expected_alice_pub);
        assert_eq!(bob.public_key().to_bytes(), expected_bob_pub);

        let alice_shared = alice.ecdh(&bob.public_key()).unwrap();
        let bob_shared = bob.ecdh(&alice.public_key()).unwrap();

        assert_eq!(alice_shared, expected_shared);
        assert_eq!(bob_shared, expected_shared);
    }

    #[test]
    fn generate_produces_valid_keys() {
        let alice = SecretKey::generate().unwrap();
        let bob = SecretKey::generate().unwrap();

        let alice_shared = alice.ecdh(&bob.public_key()).unwrap();
        let bob_shared = bob.ecdh(&alice.public_key()).unwrap();
        assert_eq!(alice_shared, bob_shared);
        assert_eq!(alice_shared.len(), 32);
    }

    #[test]
    fn public_key_bytes_roundtrip() {
        let key = SecretKey::generate().unwrap();
        let pub_key = key.public_key();
        let bytes = pub_key.to_bytes();
        let restored = PublicKey::from_bytes(&bytes);
        assert_eq!(bytes, restored.to_bytes());
    }

    #[test]
    fn private_key_bytes_roundtrip() {
        let orig = SecretKey::generate().unwrap();
        let bytes = orig.to_bytes();
        let restored = SecretKey::from_bytes(&bytes);
        assert_eq!(bytes, restored.to_bytes());
        assert_eq!(orig.public_key().to_bytes(), restored.public_key().to_bytes());
    }

    #[test]
    fn from_ed25519_public_key() {
        let ed_seed = decode_hex::<32>("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");
        let ed_priv = ed25519::SecretKey::from_bytes(&ed_seed);
        let ed_pub = ed_priv.public_key();
        let x_pub = PublicKey::try_from(&ed_pub).unwrap();

        assert_ne!(x_pub.to_bytes(), [0u8; 32]);
    }

    #[test]
    fn from_ed25519_ecdh_roundtrip() {
        let ed_alice = ed25519::SecretKey::generate().unwrap();
        let ed_bob = ed25519::SecretKey::generate().unwrap();

        let x_alice = SecretKey::from(&ed_alice);
        let x_bob = SecretKey::from(&ed_bob);

        let x_alice_pub = PublicKey::try_from(&ed_alice.public_key()).unwrap();
        let x_bob_pub = PublicKey::try_from(&ed_bob.public_key()).unwrap();

        let alice_shared = x_alice.ecdh(&x_bob_pub).unwrap();
        let bob_shared = x_bob.ecdh(&x_alice_pub).unwrap();
        assert_eq!(alice_shared, bob_shared);
    }

    #[test]
    fn rfc7748_section_5_2_vector_1() {
        let scalar = decode_hex::<32>("a546e36bf0527c9d3b16154b82465edd62144c0ac1fc5a18506a2244ba449ac4");
        let u = decode_hex::<32>("e6db6867583030db3594c1a424b15f7c726624ec26b3353b10a903a6d0ab1c4c");
        let expected = decode_hex::<32>("c3da55379de9c6908e94ea4df28d084f32eccf03491c71f754b4075577a28552");

        let output = x25519(&scalar, &u);
        assert_eq!(output, expected);
    }

    #[test]
    fn rfc7748_section_5_2_vector_2() {
        let scalar = decode_hex::<32>("4b66e9d4d1b4673c5ad22691957d6af5c11b6421e0ea01d42ca4169e7918ba0d");
        let u = decode_hex::<32>("e5210f12786811d3f4b7959d0538ae2c31dbe7106fc03c3efc4cd549c715a493");
        let expected = decode_hex::<32>("95cbde9476e8907d7aade45cb4b873f88b595a68799fa152e6f8f7647aac7957");

        let output = x25519(&scalar, &u);
        assert_eq!(output, expected);
    }

    #[test]
    fn rfc7748_section_5_2_iterative_1() {
        let k = decode_hex::<32>("0900000000000000000000000000000000000000000000000000000000000000");
        let u = k;
        let expected = decode_hex::<32>("422c8e7a6227d7bca1350b3e2bb7279f7897b87bb6854b783c60e80311ae3079");

        let output = x25519(&k, &u);
        assert_eq!(output, expected);
    }

    #[test]
    fn rfc7748_section_5_2_iterative_1000() {
        let mut k = decode_hex::<32>("0900000000000000000000000000000000000000000000000000000000000000");
        let mut u = k;
        for _ in 0..1000 {
            let out = x25519(&k, &u);
            u = k;
            k = out;
        }
        let expected = decode_hex::<32>("684cf59ba83309552800ef566f2f4d3c1c3887c49360e3875f2eb94d99532c51");
        assert_eq!(k, expected);
    }

    #[test]
    #[ignore = "takes about 1 minute; run with -- --ignored"]
    fn rfc7748_section_5_2_iterative_1000000() {
        let mut k = decode_hex::<32>("0900000000000000000000000000000000000000000000000000000000000000");
        let mut u = k;
        for _ in 0..1000000 {
            let out = x25519(&k, &u);
            u = k;
            k = out;
        }
        let expected = decode_hex::<32>("7c3911e0ab2586fd864497297e575e6f3bc601c0883c30df5f4dd2d24f665424");
        assert_eq!(k, expected);
    }

    #[test]
    fn rfc7748_section_5_2_dh_exchange() {
        let vectors: [DhTestVector; 1] = [DhTestVector {
            alice_private: "77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a",
            alice_public: "8520f0098930a754748b7ddcb43ef75a0dbf3a0d26381af4eba4a98eaa9b4e6a",
            bob_private: "5dab087e624a8a4b79e17f8b83800ee66f3bb1292618b6fd1c2f8b27ff88e0eb",
            bob_public: "de9edb7d7b7dc1b4d35b61c2ece435373f8343c85b78674dadfc7e146f882b4f",
            shared_secret: "4a5d9d5ba4ce2de1728e3bf480350f25e07e21c947d19e3376f09b3c1e161742",
        }];

        for v in &vectors {
            let alice_private = decode_hex::<32>(v.alice_private);
            let alice_public = decode_hex::<32>(v.alice_public);
            let bob_private = decode_hex::<32>(v.bob_private);
            let bob_public = decode_hex::<32>(v.bob_public);
            let expected = decode_hex::<32>(v.shared_secret);

            let alice_computed_public = SecretKey::from_bytes(&alice_private).public_key().to_bytes();
            assert_eq!(alice_computed_public, alice_public, "Alice public key mismatch");

            let bob_computed_public = SecretKey::from_bytes(&bob_private).public_key().to_bytes();
            assert_eq!(bob_computed_public, bob_public, "Bob public key mismatch");

            let alice_shared = x25519(&alice_private, &bob_public);
            assert_eq!(alice_shared, expected, "Alice shared secret mismatch");

            let bob_shared = x25519(&bob_private, &alice_public);
            assert_eq!(bob_shared, expected, "Bob shared secret mismatch");
        }
    }

    #[test]
    fn basepoint_multiplication_identity_pattern() {
        let key = decode_hex::<32>("77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a");
        let public = SecretKey::from_bytes(&key).public_key().to_bytes();
        let basepoint = decode_hex::<32>("0900000000000000000000000000000000000000000000000000000000000000");
        let direct = x25519(&key, &basepoint);
        assert_eq!(public, direct);
    }

    #[test]
    fn low_order_point_zero() {
        let scalar = decode_hex::<32>("77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a");
        let all_zero = [0u8; 32];
        assert_eq!(
            x25519_checked(&scalar, &all_zero),
            Err(EllipticCurveError::InvalidSharedSecret),
            "X25519 with u=0 must be rejected"
        );
    }

    #[test]
    fn low_order_point_u_one() {
        let scalar = decode_hex::<32>("77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a");
        let mut u_one = [0u8; 32];
        u_one[0] = 1;
        assert_eq!(
            x25519_checked(&scalar, &u_one),
            Err(EllipticCurveError::InvalidSharedSecret),
            "X25519 with u=1 must be rejected"
        );
    }

    #[test]
    fn low_order_points_are_rejected() {
        let scalar = decode_hex::<32>("77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a");
        for u in LOW_ORDER_U {
            assert!(
                PublicKey::from_bytes(&u).is_low_order(),
                "u={} must be low order",
                hex::encode(u)
            );
            assert_eq!(
                x25519_checked(&scalar, &u),
                Err(EllipticCurveError::InvalidSharedSecret),
                "low-order u={} must be rejected",
                hex::encode(u)
            );
        }
    }

    #[test]
    fn non_canonical_low_order_points_are_rejected() {
        let scalar = decode_hex::<32>("77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a");
        // `u = p` reduces to 0 and `u = p + 1` reduces to 1; both are low order.
        let non_canonical = [
            decode_hex::<32>("edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f"),
            decode_hex::<32>("eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f"),
        ];
        for u in non_canonical {
            assert!(
                PublicKey::from_bytes(&u).is_low_order(),
                "u={} must be low order",
                hex::encode(u)
            );
            assert_eq!(
                x25519_checked(&scalar, &u),
                Err(EllipticCurveError::InvalidSharedSecret),
                "non-canonical low-order u={} must be rejected",
                hex::encode(u)
            );
        }
    }

    #[test]
    fn is_low_order_false_for_honest_keys() {
        let basepoint = decode_hex::<32>("0900000000000000000000000000000000000000000000000000000000000000");
        assert!(!PublicKey::from_bytes(&basepoint).is_low_order());
        for _ in 0..16 {
            assert!(!SecretKey::generate().unwrap().public_key().is_low_order());
        }
    }

    #[test]
    fn all_zero_private_key() {
        let key = [0u8; 32];
        let u = decode_hex::<32>("e6db6867583030db3594c1a424b15f7c726624ec26b3353b10a903a6d0ab1c4c");
        let output = x25519(&key, &u);
        let expected = x25519(
            &decode_hex::<32>("0000000000000000000000000000000000000000000000000000000000000040"),
            &u,
        );
        assert_eq!(output, expected);
    }

    // #[test]
    // fn public_key_exceeds_prime_is_reduced() {
    //     let scalar = decode_hex::<32>("77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a");
    //     let p_bytes = P.to_le_bytes_fixed::<32>();
    //     let result = x25519(&scalar, &p_bytes);
    //     assert!(result.is_ok(), "Non-canonical values must be accepted per RFC 7748");
    // }

    #[test]
    fn wycheproof_valid_vectors() {
        let shared = x25519(
            &decode_hex::<32>("c8a9d5a91091ad851c668b0736c1c9a02936c0d3ad62670858088047ba057475"),
            &decode_hex::<32>("504a36999f489cd2fdbc08baff3d88fa00569ba986cba22548ffde80f9806829"),
        );
        assert_eq!(
            shared,
            decode_hex::<32>("436a2c040cf45fea9b29a0cb81b1f41458f863d0d61b453d0a982720d6d61320")
        );

        let shared = x25519(
            &decode_hex::<32>("a8386f7f16c50731d64f82e6a170b142a4e34f31fd7768fcb8902925e7d1e25a"),
            &decode_hex::<32>("0400000000000000000000000000000000000000000000000000000000000000"),
        );
        assert_eq!(
            shared,
            decode_hex::<32>("34b7e4fa53264420d9f943d15513902342b386b172a0b0b7c8b8f2dd3d669f59")
        );

        let shared = x25519(
            &decode_hex::<32>("a046e36bf0527c9d3b16154b82465edd62144c0ac1fc5a18506a2244ba449a44"),
            &decode_hex::<32>("e6db6867583030db3594c1a424b15f7c726624ec26b3353b10a903a6d0ab1c4c"),
        );
        assert_eq!(
            shared,
            decode_hex::<32>("c3da55379de9c6908e94ea4df28d084f32eccf03491c71f754b4075577a28552")
        );
    }

    #[test]
    fn wycheproof_low_order_and_zero_shared() {
        let private = decode_hex::<32>("786a33a4f7af297a20e7642925932bf509e7070fa1bc36986af1eb13f4f50b55");

        for u in [
            "0000000000000000000000000000000000000000000000000000000000000000",
            "0100000000000000000000000000000000000000000000000000000000000000",
            "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
        ] {
            assert_eq!(
                x25519_checked(&private, &decode_hex::<32>(u)),
                Err(EllipticCurveError::InvalidSharedSecret),
                "low-order u={u} must be rejected"
            );
        }
    }

    #[test]
    fn wycheproof_non_canonical_public_keys() {
        let shared = x25519(
            &decode_hex::<32>("0016b62af5cabde8c40938ebf2108e05d27fa0533ed85d70015ad4ad39762d54"),
            &decode_hex::<32>("efffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f"),
        );
        assert_eq!(
            shared,
            decode_hex::<32>("b4d10e832714972f96bd3382e4d082a21a8333a16315b3ffb536061d2482360d")
        );
    }

    #[test]
    fn golang_crypto_vectors() {
        struct GoVector {
            scalar: [u8; 32],
            base: [u8; 32],
            expected: [u8; 32],
        }

        let vectors = [
            GoVector {
                scalar: decode_hex::<32>("668fb9f76ad971c81ac900071a1560bce2ca00cac7e67af99348913761434014"),
                base: decode_hex::<32>("db5f32b7f841e7a1a00968effded12735fc47a3eb13b579aacadeae80939a7dd"),
                expected: decode_hex::<32>("090d85e599ea8e2beeb61304d37be10ec5c905f9927d32f42a9a0afb3e0b4074"),
            },
            GoVector {
                scalar: decode_hex::<32>("636695e34f75b9a279c8706fad1289f2c0b1e22e16f8b8861729c10a582958af"),
                base: decode_hex::<32>("090d0701f8fde28f70043b83f2346225419b18a7f27e9e3d2bfd04e10f3d213e"),
                expected: decode_hex::<32>("bf26ec7ec413061733d44070ea67cab02a85dc1be8cfe1ff73d541cc08325506"),
            },
        ];

        for (i, v) in vectors.iter().enumerate() {
            let result = x25519(&v.scalar, &v.base);
            assert_eq!(result, v.expected, "Go vector {} failed", i);
        }
    }

    #[test]
    fn additional_boringssl_vectors() {
        struct Vector {
            scalar: [u8; 32],
            base: [u8; 32],
            expected: [u8; 32],
        }

        let vectors = [
            Vector {
                scalar: decode_hex::<32>("203161c3159a876a2beaec29d2427fb0c7c30d382cd013d27cc3d393db0daf6f"),
                base: decode_hex::<32>("6ab95d1abe68c09b005c3db9042cc91ac849f7e94a2a4a9b893678970b7b95bf"),
                expected: decode_hex::<32>("11edaedc95ff78f563a1c8f15591c071dea092b4d7ecaac8e0387b5a160c4e5d"),
            },
            Vector {
                scalar: decode_hex::<32>("13d65491fe75f203a008b4415abc60d532e695dbd2f1e803accb34b2b72c3d70"),
                base: decode_hex::<32>("2e784e04ca0073336256a839255ed2f7d4796a64cdc37f1eb0e5c4c8d1d1e0f5"),
                expected: decode_hex::<32>("563e8c9adaa7d73101b0f2ead3cae1ea5d8fcd5cd36080bb8e6ec03d61450917"),
            },
        ];

        for (i, v) in vectors.iter().enumerate() {
            let result = x25519(&v.scalar, &v.base);
            assert_eq!(result, v.expected, "BoringSSL vector {} failed", i);
        }
    }

    #[test]
    fn wycheproof_twist_vectors() {
        let shared = x25519(
            &decode_hex::<32>("d85d8c061a50804ac488ad774ac716c3f5ba714b2712e048491379a500211958"),
            &decode_hex::<32>("63aa40c6e38346c5caf23a6df0a5e6c80889a08647e551b3563449befcfc9733"),
        );
        assert_eq!(
            shared,
            decode_hex::<32>("279df67a7c4611db4708a0e8282b195e5ac0ed6f4b2f292c6fbd0acac30d1332")
        );

        let shared = x25519(
            &decode_hex::<32>("d03edde9f3e7b799045f9ac3793d4a9277dadeadc41bec0290f81f744f73775f"),
            &decode_hex::<32>("0200000000000000000000000000000000000000000000000000000000000000"),
        );
        assert_eq!(
            shared,
            decode_hex::<32>("b87a1722cc6c1e2feecb54e97abd5a22acc27616f78f6e315fd2b73d9f221e57")
        );
    }

    #[test]
    fn more_boringssl_vectors() {
        struct Vector {
            scalar: [u8; 32],
            base: [u8; 32],
            expected: [u8; 32],
        }

        let vectors = [Vector {
            scalar: decode_hex::<32>("203161c3159a876a2beaec29d2427fb0c7c30d382cd013d27cc3d393db0daf6f"),
            base: decode_hex::<32>("6ab95d1abe68c09b005c3db9042cc91ac849f7e94a2a4a9b893678970b7b95bf"),
            expected: decode_hex::<32>("11edaedc95ff78f563a1c8f15591c071dea092b4d7ecaac8e0387b5a160c4e5d"),
        }];

        for (i, v) in vectors.iter().enumerate() {
            let result = x25519(&v.scalar, &v.base);
            assert_eq!(result, v.expected, "BoringSSL vector {} failed", i);
        }
    }

    #[test]
    fn clamped_zero_scalar_produces_known_public_key() {
        let scalar = [0u8; 32];
        let expected = decode_hex::<32>("2fe57da347cd62431528daac5fbb290730fff684afc4cfc2ed90995f58cb3b74");
        let pub_key = SecretKey::from_bytes(&scalar).public_key().to_bytes();
        assert_eq!(pub_key, expected, "clamped zero scalar must produce deterministic public key");
    }

    #[test]
    fn max_scalar_x25519_against_basepoint() {
        let scalar = [0xffu8; 32];
        let pub_key = SecretKey::from_bytes(&scalar).public_key().to_bytes();
        let direct = x25519(
            &scalar,
            &decode_hex::<32>("0900000000000000000000000000000000000000000000000000000000000000"),
        );
        assert_eq!(pub_key, direct);
    }

    #[test]
    fn x25519_self_ecdh_consistency() {
        let alice = SecretKey::generate().unwrap();
        let alice_pub = alice.public_key();
        let alice_shared = alice.ecdh(&alice_pub).unwrap();
        assert_eq!(alice_shared.len(), 32);
    }

    #[test]
    fn non_canonical_above_p_reduces_correctly() {
        let scalar = decode_hex::<32>("c8a9d5a91091ad851c668b0736c1c9a02936c0d3ad62670858088047ba057475");
        let (sum, _) = P.add_raw(&U256::from_u64(2));
        let mut p_plus_2_bytes = sum.to_le_bytes_fixed::<32>();
        p_plus_2_bytes[31] &= 0x7f;
        let result = x25519(&scalar, &p_plus_2_bytes);

        let u_2 = decode_hex::<32>("0200000000000000000000000000000000000000000000000000000000000000");
        let expected = x25519(&scalar, &u_2);
        assert_eq!(result, expected, "X25519(scalar, p+2) must equal X25519(scalar, 2)");
    }

    // --- Wycheproof test vectors ---

    #[test]
    fn wycheproof_x25519() {
        let data: serde_json::Value =
            serde_json::from_str(include_str!("../../testdata/wycheproof/testvectors_v1/x25519_test.json")).unwrap();
        let mut valid_tested = 0u64;
        let mut acceptable_tested = 0u64;
        for group in data["testGroups"].as_array().unwrap() {
            if group["curve"].as_str() != Some("curve25519") {
                continue;
            }
            for test in group["tests"].as_array().unwrap() {
                let public_hex = test["public"].as_str().unwrap();
                let private_hex = test["private"].as_str().unwrap();
                let expected_shared_hex = test["shared"].as_str().unwrap();
                let result = test["result"].as_str().unwrap();

                let public_key = decode_hex::<32>(public_hex);
                let private_key = decode_hex::<32>(private_hex);

                let shared = x25519_checked(&private_key, &public_key);

                if result == "valid" {
                    let shared = shared.expect("valid wycheproof x25519 vector must not error");
                    assert_eq!(
                        hex::encode(shared),
                        expected_shared_hex,
                        "wycheproof x25519 tcId={} shared secret mismatch",
                        test["tcId"]
                    );
                    valid_tested += 1;
                } else {
                    // Non-valid vectors (typically low-order public keys) may be
                    // rejected or, if accepted, must match the expected secret.
                    if let Ok(shared) = shared {
                        assert_eq!(
                            hex::encode(shared),
                            expected_shared_hex,
                            "wycheproof x25519 tcId={} acceptable shared secret mismatch",
                            test["tcId"]
                        );
                    }
                    acceptable_tested += 1;
                }
            }
        }
        assert!(valid_tested > 0, "no valid wycheproof x25519 tests were run");
        assert!(acceptable_tested > 0, "no acceptable wycheproof x25519 tests were run");
    }
}
