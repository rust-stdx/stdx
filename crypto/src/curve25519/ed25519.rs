use constant_time_eq::constant_time_eq;

use super::curve25519::{FieldElement, U256};
use crate::{EllipticCurveError, Hasher, sha2::Sha512};

pub const SECRET_KEY_SIZE: usize = 32;
pub const PUBLIC_KEY_SIZE: usize = 32;
pub const SIGNATURE_SIZE: usize = 64;

const MODULUS_L: U256 = U256::from_limbs([
    0x5812_631a_5cf5_d3ed,
    0x14de_f9de_a2f7_9cd6,
    0x0000_0000_0000_0000,
    0x1000_0000_0000_0000,
]);

// const P_PLUS_THREE_OVER_EIGHT: U256 = U256::from_limbs([
//     0xffff_ffff_ffff_fffe,
//     0xffff_ffff_ffff_ffff,
//     0xffff_ffff_ffff_ffff,
//     0x0fff_ffff_ffff_ffff,
// ]);

const EDWARDS_D: FieldElement = FieldElement(U256::from_limbs([
    0x75eb_4dca_1359_78a3,
    0x0070_0a4d_4141_d8ab,
    0x8cc7_4079_7779_e898,
    0x5203_6cee_2b6f_fe73,
]));

/// R = 2^256 mod L, used for fast scalar reduction.
const R: U256 = U256::from_limbs([
    0xd6ec_3174_8d98_951d,
    0xc6ef_5bf4_737d_cf70,
    0xffff_ffff_ffff_fffe,
    0x0fff_ffff_ffff_ffff,
]);

/// Barrett µ = floor(2^512 / L), used for fast wide reduction modulo L.
const BAR_MU: [u64; 5] = [
    0xed9c_e5a3_0a2c_131b,
    0x2106_215d_0863_29a7,
    0xffff_ffff_ffff_ffeb,
    0xffff_ffff_ffff_ffff,
    0x0000_0000_0000_000f,
];

const EDWARDS_2D: FieldElement = FieldElement(U256::from_limbs([
    0xebd6_9b94_26b2_f159,
    0x00e0_149a_8283_b156,
    0x198e_80f2_eef3_d130,
    0x2406_d9dc_56df_fce7,
]));

const SQRT_M1: FieldElement = FieldElement(U256::from_limbs([
    0xc4ee_1b27_4a0e_a0b0,
    0x2f43_1806_ad2f_e478,
    0x2b4d_0099_3dfb_d7a7,
    0x2b83_2480_4fc1_df0b,
]));

const BASEPOINT: EdwardsPoint = EdwardsPoint {
    x: FieldElement(U256::from_limbs([
        0xc9562d608f25d51a,
        0x692cc7609525a7b2,
        0xc0a4e231fdd6dc5c,
        0x216936d3cd6e53fe,
    ])),
    y: FieldElement(U256::from_limbs([
        0x6666666666666658,
        0x6666666666666666,
        0x6666666666666666,
        0x6666666666666666,
    ])),
    z: FieldElement(U256::from_limbs([
        0x0000000000000001,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
    ])),
    t: FieldElement(U256::from_limbs([
        0x6dde8ab3a5b7dda3,
        0x20f09f80775152f5,
        0x66ea4e8e64abe37d,
        0x67875f0fd78b7665,
    ])),
};

/// Ed25519 signing key (RFC 8032).
///
/// # Generating a key
///
/// ```ignore
/// use crypto::curve25519::ed25519::SecretKey;
///
/// let priv_key = SecretKey::generate();
/// let pub_key = priv_key.public_key();
/// ```
///
/// # Signing and verification
///
/// ```ignore
/// use crypto::curve25519::ed25519::SecretKey;
///
/// let priv_key = SecretKey::generate();
/// let pub_key = priv_key.public_key();
/// let sig = priv_key.sign(b"message");
/// assert!(pub_key.verify(b"message", &sig).is_ok());
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SecretKey {
    seed: [u8; SECRET_KEY_SIZE],
    scalar: Scalar,
    prefix: [u8; 32],
    public_point: EdwardsPoint,
    public_bytes: [u8; PUBLIC_KEY_SIZE],
}

impl SecretKey {
    #[cfg(feature = "random")]
    pub fn generate() -> SecretKey {
        let seed: [u8; SECRET_KEY_SIZE] = crate::random::random_bytes();
        SecretKey::from_bytes(&seed)
    }

    pub fn from_bytes(seed: &[u8; SECRET_KEY_SIZE]) -> SecretKey {
        let (scalar, prefix) = expand_secret(seed);
        let public_point = scalar_mul_base(&scalar);
        let public_bytes = public_point
            .to_bytes()
            .expect("basepoint multiplication must produce a valid point");
        SecretKey {
            seed: *seed,
            scalar,
            prefix,
            public_point,
            public_bytes,
        }
    }

    // pub fn from_seed_unchecked(seed: &[u8; PRIVATE_KEY_SIZE]) -> SecretKey {
    //     SecretKey::from_bytes(seed)
    // }

    pub fn sign(&self, message: &[u8]) -> [u8; SIGNATURE_SIZE] {
        let r = hash_to_scalar(&[&self.prefix, message]);
        let r_point = scalar_mul_base(&r)
            .to_bytes()
            .expect("basepoint multiplication must produce a valid point");

        let k = hash_to_scalar(&[&r_point, &self.public_bytes, message]);
        let s = r.add(k.mul(self.scalar));

        let mut signature = [0u8; SIGNATURE_SIZE];
        signature[..32].copy_from_slice(&r_point);
        signature[32..].copy_from_slice(&s.to_bytes());
        signature
    }

    #[inline]
    pub fn to_bytes(&self) -> [u8; SECRET_KEY_SIZE] {
        self.seed
    }

    #[inline]
    pub fn public_key(&self) -> PublicKey {
        PublicKey {
            point: self.public_point,
            bytes: self.public_bytes,
        }
    }
}

impl From<&[u8; SECRET_KEY_SIZE]> for SecretKey {
    fn from(bytes: &[u8; SECRET_KEY_SIZE]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&[u8]> for SecretKey {
    type Error = EllipticCurveError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::from_bytes(bytes.try_into().map_err(|_| EllipticCurveError::InvalidKey)?))
    }
}

/// Ed25519 public key for signature verification (RFC 8032).
///
/// # Verifying a signature
///
/// ```ignore
/// use crypto::curve25519::ed25519::{PublicKey, SecretKey};
///
/// let priv_key = SecretKey::generate();
/// let pub_key = priv_key.public_key();
/// let sig = priv_key.sign(b"message");
/// assert!(pub_key.verify(b"message", &sig).is_ok());
/// ```
///
/// # Deserializing from bytes
///
/// ```ignore
/// use crypto::curve25519::ed25519::PublicKey;
///
/// let bytes = [0u8; 32]; // replace with a real public key
/// let pub_key = PublicKey::from_bytes(&bytes);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicKey {
    point: EdwardsPoint,
    bytes: [u8; PUBLIC_KEY_SIZE],
}

impl PublicKey {
    pub fn from_bytes(key: &[u8; PUBLIC_KEY_SIZE]) -> Result<PublicKey, EllipticCurveError> {
        let point = EdwardsPoint::from_bytes(key.try_into().unwrap()).ok_or(EllipticCurveError::InvalidKey)?;
        Ok(PublicKey {
            point,
            bytes: *key,
        })
    }

    pub fn verify(&self, message: &[u8], signature: &[u8; SIGNATURE_SIZE]) -> Result<(), EllipticCurveError> {
        ed25519_verify(&self.point, message, signature)
    }

    #[inline]
    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_SIZE] {
        self.bytes
    }

    pub fn to_montgomery_u(&self) -> Option<FieldElement> {
        let inv_z = self.point.z.invert()?;
        let y = self.point.y.mul(inv_z);
        let one = FieldElement::ONE;
        let u = (one.add(y)).mul((one.sub(y)).invert()?);
        Some(u)
    }
}

impl TryFrom<&[u8]> for PublicKey {
    type Error = EllipticCurveError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(bytes.try_into().map_err(|_| EllipticCurveError::InvalidKey)?)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Scalar(U256);

impl Scalar {
    fn from_canonical_bytes(bytes: &[u8; 32]) -> Option<Self> {
        let value = U256::from_le_slice(bytes);
        if value.ct_ge(&MODULUS_L) {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Reduce an arbitrary-length byte sequence modulo L (the ed25519 subgroup order).
    ///
    /// Two fast paths:
    /// - <= 32 bytes: parse as U256 and repeatedly conditionally subtract L (at most 16 times,
    ///   since a 256-bit value is < 16·L). Constant-time via ct_select.
    /// - > 32 bytes: Horner evaluation in base 2^256. Split into 32-byte chunks processed
    ///   from most significant to least, using the precomputed R = 2^256 mod L for shifting
    ///   and the Barrett constant µ for fast modular multiplication of the accumulator by R.
    fn reduce_bytes_mod_l(bytes: &[u8]) -> Self {
        let len = bytes.len();
        if len <= 32 {
            let mut padded = [0u8; 32];
            padded[..len].copy_from_slice(bytes);
            let val = U256::from_le_slice(&padded);
            let mut result = val;
            let mut i = 16;
            while i > 0 {
                let (diff, borrow) = result.sub_raw(&MODULUS_L);
                result = U256::ct_select(&diff, &result, borrow == 0);
                i -= 1;
            }
            Self(result)
        } else {
            let chunk_count = (len + 31) / 32;
            let mut acc = U256::ZERO;
            let mut chunk_idx = chunk_count;
            while chunk_idx > 0 {
                chunk_idx -= 1;
                let chunk_start = chunk_idx * 32;
                let chunk_end = usize::min(chunk_start + 32, len);
                let chunk_len = chunk_end - chunk_start;

                if !acc.is_zero() {
                    acc = acc.mul_mod_barrett(&R, &MODULUS_L, &BAR_MU);
                }

                let mut padded = [0u8; 32];
                padded[..chunk_len].copy_from_slice(&bytes[chunk_start..chunk_end]);
                let mut chunk_reduced = U256::from_le_slice(&padded);
                let mut j = 16;
                while j > 0 {
                    let (diff, borrow) = chunk_reduced.sub_raw(&MODULUS_L);
                    chunk_reduced = U256::ct_select(&diff, &chunk_reduced, borrow == 0);
                    j -= 1;
                }

                acc = acc.add_mod(&chunk_reduced, &MODULUS_L);
            }
            Self(acc)
        }
    }

    #[inline]
    fn to_bytes(self) -> [u8; 32] {
        self.0.to_le_bytes_fixed::<32>()
    }

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.0.add_mod(&rhs.0, &MODULUS_L))
    }

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self(self.0.mul_mod_barrett(&rhs.0, &MODULUS_L, &BAR_MU))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EdwardsPoint {
    x: FieldElement,
    y: FieldElement,
    z: FieldElement,
    t: FieldElement,
}

impl EdwardsPoint {
    #[inline]
    fn identity() -> Self {
        Self {
            x: FieldElement::ZERO,
            y: FieldElement::ONE,
            z: FieldElement::ONE,
            t: FieldElement::ZERO,
        }
    }

    #[inline]
    fn from_affine(x: FieldElement, y: FieldElement) -> Self {
        Self {
            x,
            y,
            z: FieldElement::ONE,
            t: x.mul(y),
        }
    }

    fn from_bytes(bytes: &[u8; 32]) -> Option<Self> {
        let sign = (bytes[31] >> 7) == 1;
        let mut y_bytes = *bytes;
        y_bytes[31] &= 0x7f;
        let y = FieldElement::from_canonical_bytes(&y_bytes)?;
        let y2 = y.square();
        let u = y2.sub(FieldElement::ONE);
        let v = EDWARDS_D.mul(y2).add(FieldElement::ONE);
        let x2 = u.mul(v.invert()?);
        let mut x = sqrt(&x2)?;
        if x.is_zero() && sign {
            return None;
        }
        if x.is_odd() != sign {
            x = x.negate();
        }
        Some(Self::from_affine(x, y))
    }

    fn to_bytes(self) -> Option<[u8; 32]> {
        let inv_z = self.z.invert()?;
        let x = self.x.mul(inv_z);
        let y = self.y.mul(inv_z);
        let mut out = y.to_bytes();
        if x.is_odd() {
            out[31] |= 0x80;
        }
        Some(out)
    }

    #[inline]
    fn add(&self, rhs: &Self) -> Self {
        let a = self.y.sub(self.x).mul(rhs.y.sub(rhs.x));
        let b = self.y.add(self.x).mul(rhs.y.add(rhs.x));
        let c = self.t.mul(rhs.t).mul(EDWARDS_2D);
        let z1z2 = self.z.mul(rhs.z);
        let d = z1z2.add(z1z2);
        let e = b.sub(a);
        let f = d.sub(c);
        let g = d.add(c);
        let h = b.add(a);
        Self {
            x: e.mul(f),
            y: g.mul(h),
            t: e.mul(h),
            z: f.mul(g),
        }
    }

    #[inline]
    fn double(&self) -> Self {
        let a = self.x.square();
        let b = self.y.square();
        let z2 = self.z.square();
        let c = z2.add(z2);
        let d = a.negate();
        let e = self.x.add(self.y).square().sub(a).sub(b);
        let g = d.add(b);
        let f = g.sub(c);
        let h = d.sub(b);
        Self {
            x: e.mul(f),
            y: g.mul(h),
            t: e.mul(h),
            z: f.mul(g),
        }
    }

    #[inline]
    fn select(a: &Self, b: &Self, choice: bool) -> Self {
        Self {
            x: FieldElement::select(&a.x, &b.x, choice),
            y: FieldElement::select(&a.y, &b.y, choice),
            z: FieldElement::select(&a.z, &b.z, choice),
            t: FieldElement::select(&a.t, &b.t, choice),
        }
    }

    #[inline]
    fn mul_by_cofactor(&self) -> Self {
        self.double().double().double()
    }
}

#[inline]
fn sqrt(a: &FieldElement) -> Option<FieldElement> {
    let mut candidate = a.pow_sqrt_exponent();
    if !candidate.square().ct_eq(a) {
        candidate = candidate.mul(SQRT_M1);
    }
    if candidate.square().ct_eq(a) {
        Some(candidate)
    } else {
        None
    }
}

fn scalar_mul(point: &EdwardsPoint, scalar: &Scalar) -> EdwardsPoint {
    let mut table = [EdwardsPoint::identity(); 16];
    table[1] = *point;
    let mut i = 2;
    while i < 16 {
        table[i] = table[i - 1].add(point);
        i += 1;
    }

    scalar_mul_table(&table, scalar)
}

/// Optimized version of `scalar_mul` for `BASEPOINT` using a pre-computed table.
#[cfg(feature = "std")]
fn scalar_mul_base(scalar: &Scalar) -> EdwardsPoint {
    use std::sync::LazyLock;
    static TABLE: LazyLock<[EdwardsPoint; 16]> = LazyLock::new(|| {
        let mut t = [EdwardsPoint::identity(); 16];
        t[1] = BASEPOINT;
        let mut i = 2;
        while i < 16 {
            t[i] = t[i - 1].add(&BASEPOINT);
            i += 1;
        }
        t
    });

    scalar_mul_table(&TABLE, scalar)
}

#[cfg(not(feature = "std"))]
#[inline]
fn scalar_mul_base(scalar: &Scalar) -> EdwardsPoint {
    scalar_mul(&BASEPOINT, scalar)
}

#[inline]
fn scalar_mul_table(table: &[EdwardsPoint; 16], scalar: &Scalar) -> EdwardsPoint {
    let mut result = EdwardsPoint::identity();
    let mut win = 63;
    loop {
        let idx = scalar_window(scalar, win);
        let selected = ct_select_from_table(&table, idx);
        result = result.add(&selected);
        if win == 0 {
            break;
        }
        result = result.double().double().double().double();
        win -= 1;
    }
    result
}

#[inline]
fn ct_select_from_table(table: &[EdwardsPoint; 16], index: usize) -> EdwardsPoint {
    let mut result = table[0];
    let mut i = 1;
    while i < 16 {
        let diff = i ^ index;
        let choice = ((diff.wrapping_sub(1) >> (usize::BITS - 1)) & 1) != 0;
        result = EdwardsPoint::select(&table[i], &result, choice);
        i += 1;
    }
    result
}

#[inline]
fn scalar_window(scalar: &Scalar, window: usize) -> usize {
    let bit_pos = window * 4;
    let limb_idx = bit_pos / 64;
    let limb = scalar.0.limbs[limb_idx];
    ((limb >> (bit_pos % 64)) & 0xf) as usize
}

fn hash_to_scalar(parts: &[&[u8]]) -> Scalar {
    let mut hasher = Sha512::new();
    let mut i = 0usize;
    while i < parts.len() {
        hasher.update(parts[i]);
        i += 1;
    }
    let digest = hasher.sum();
    Scalar::reduce_bytes_mod_l(digest.as_ref())
}

fn expand_secret(private_key: &[u8; SECRET_KEY_SIZE]) -> (Scalar, [u8; 32]) {
    let mut digest = Sha512::hash(private_key);
    let digest: &mut [u8; 64] = digest.as_mut().try_into().unwrap();

    digest[0] &= 248;
    digest[31] &= 63;
    digest[31] |= 64;

    let scalar = Scalar::reduce_bytes_mod_l(&digest[..32]);
    let mut prefix = [0u8; 32];
    prefix.copy_from_slice(&digest[32..]);
    (scalar, prefix)
}

fn ed25519_verify(
    point: &EdwardsPoint,
    message: &[u8],
    signature: &[u8; SIGNATURE_SIZE],
) -> Result<(), EllipticCurveError> {
    let r_bytes: &[u8; 32] = &signature[..32].try_into().unwrap();
    let r = EdwardsPoint::from_bytes(r_bytes).ok_or(EllipticCurveError::InvalidSignature)?;

    let s =
        Scalar::from_canonical_bytes(&signature[32..].try_into().unwrap()).ok_or(EllipticCurveError::Unspecified)?;

    let k = hash_to_scalar(&[
        r_bytes,
        &point.to_bytes().ok_or(EllipticCurveError::Unspecified)?,
        message,
    ]);

    let lhs = scalar_mul_base(&s).mul_by_cofactor();
    let rhs = r.add(&scalar_mul(point, &k)).mul_by_cofactor();

    // SAFETY: this is okay to use non-contant time compare because
    match (lhs.to_bytes(), rhs.to_bytes()) {
        (Some(lhs), Some(rhs)) if constant_time_eq(&lhs, &rhs) => Ok(()),
        _ => Err(EllipticCurveError::InvalidSignature),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curve25519::x25519;

    const BASEPOINT_COMPRESSED: [u8; 32] = [
        0x58, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
        0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
    ];

    fn decode_hex<const N: usize>(hex_bytes: &str) -> [u8; N] {
        let bytes = hex::decode(hex_bytes).unwrap();
        assert_eq!(bytes.len(), N);
        let mut out = [0u8; N];
        out.copy_from_slice(&bytes);
        out
    }

    fn decode_hex_vec(hex_bytes: &str) -> Vec<u8> {
        hex::decode(hex_bytes).unwrap()
    }

    #[test]
    fn sign_verify_roundtrip() {
        let seed = decode_hex::<32>("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");
        let priv_key = SecretKey::from_bytes(&seed);
        let pub_key = priv_key.public_key();

        let messages: [&[u8]; 4] = [b"", b"hello", b"test message", &[0xffu8; 256]];
        for msg in &messages {
            let sig = priv_key.sign(msg);
            assert!(pub_key.verify(msg, &sig).is_ok());
            assert!(pub_key.verify(b"wrong", &sig).is_err());
        }
    }

    #[test]
    fn public_key_bytes_roundtrip() {
        let seed = decode_hex::<32>("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");
        let priv_key = SecretKey::from_bytes(&seed);
        let pub_key = priv_key.public_key();
        let pub_bytes = pub_key.to_bytes();
        let restored = PublicKey::from_bytes(&pub_bytes).unwrap();
        assert_eq!(pub_bytes, restored.to_bytes());
    }

    #[test]
    fn private_key_bytes_roundtrip() {
        let seed = decode_hex::<32>("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");
        let priv_key = SecretKey::from_bytes(&seed);
        assert_eq!(priv_key.to_bytes(), seed);
    }

    #[test]
    fn generate_produces_valid_keys() {
        let priv_key = SecretKey::generate();
        let pub_key = priv_key.public_key();
        let sig = priv_key.sign(b"hello");
        assert!(pub_key.verify(b"hello", &sig).is_ok());
    }

    #[test]
    fn rejects_invalid_public_key() {
        assert!(PublicKey::from_bytes(&[0xffu8; 32]).is_err());
        let p_enc = decode_hex::<32>("edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f");
        assert!(PublicKey::from_bytes(&p_enc).is_err());
    }

    fn check_vector(
        seed_hex: &'static str,
        public_key_hex: &'static str,
        message_hex: &'static str,
        signature_hex: &'static str,
    ) {
        let seed = decode_hex::<32>(seed_hex);
        let pk_expected = decode_hex::<32>(public_key_hex);
        let sig_expected = decode_hex::<64>(signature_hex);
        let msg = decode_hex_vec(message_hex);

        let priv_key = SecretKey::from_bytes(&seed);
        assert_eq!(priv_key.public_key().to_bytes(), pk_expected);
        assert_eq!(priv_key.sign(&msg), sig_expected);
        assert!(priv_key.public_key().verify(&msg, &sig_expected).is_ok());
    }

    #[test]
    fn rfc8032_vectors() {
        check_vector(
            "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60",
            "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
            "",
            "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b",
        );
        check_vector(
            "4ccd089b28ff96da9db6c346ec114e0f5b8a319f35aba624da8cf6ed4fb8a6fb",
            "3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c",
            "72",
            "92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00",
        );
        check_vector(
            "c5aa8df43f9f837bedb7442f31dcb7b166d38535076f094b85ce3a2e0b4458f7",
            "fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025",
            "af82",
            "6291d657deec24024827e69c3abe01a30ce548a284743a445e3680d7db5ac3ac18ff9b538d16f290ae67f760984dc6594a7c15e9716ed28dc027beceea1ec40a",
        );
    }

    #[test]
    fn go_golden_vectors() {
        let data = include_str!("../../testdata/ed25519/sign.input");

        for line in data.lines() {
            let mut parts = line.split(':');
            let private_and_public = parts.next().unwrap();
            let public_key = parts.next().unwrap();
            let message = parts.next().unwrap();
            let signature_with_message = parts.next().unwrap();
            assert!(parts.next().is_some());
            assert!(parts.next().is_none());

            check_vector(&private_and_public[..64], public_key, message, &signature_with_message[..128]);
        }
    }

    #[test]
    fn verify_rejects_tampering_and_non_canonical_s() {
        let seed = decode_hex::<32>("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");
        let pub_key = SecretKey::from_bytes(&seed).public_key();
        let signature = SecretKey::from_bytes(&seed).sign(b"message");

        assert!(pub_key.verify(b"message", &signature).is_ok());
        assert!(pub_key.verify(b"tampered", &signature).is_err());

        let mut bad_signature = signature;
        bad_signature[0] ^= 0x80;
        assert!(pub_key.verify(b"message", &bad_signature).is_err());

        let mut non_canonical_s = signature;
        non_canonical_s[32..].copy_from_slice(&[
            0xed, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58, 0xd6, 0x9c, 0xf7, 0xa2, 0xde, 0xf9, 0xde, 0x14, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10,
        ]);
        assert!(pub_key.verify(b"message", &non_canonical_s).is_err());
    }

    #[test]
    fn public_key_validation_rejects_invalid_encodings() {
        let valid = decode_hex::<32>("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a");
        assert!(PublicKey::from_bytes(&valid).is_ok());

        let mut invalid = [0xffu8; 32];
        assert!(PublicKey::from_bytes(&invalid).is_err());

        invalid = valid;
        invalid[31] |= 0x80;
        invalid[..31].fill(0);
        assert!(PublicKey::from_bytes(&invalid).is_err());
    }

    #[test]
    fn cctv_ed25519_vectors() {
        let data = include_str!("../../testdata/ed25519/cctv_vectors.txt");

        for line in data.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            assert_eq!(parts.len(), 5, "malformed line: {line}");
            let number = parts[0];
            let key_hex = parts[1];
            let sig_hex = parts[2];
            let msg_hex = parts[3];
            let flags_str = parts[4];

            let flags: Vec<&str> = if flags_str.is_empty() {
                vec![]
            } else {
                flags_str.split(',').collect()
            };

            let has_non_canonical_a = flags.contains(&"non_canonical_A");
            let has_non_canonical_r = flags.contains(&"non_canonical_R");
            let should_reject = has_non_canonical_a || has_non_canonical_r;

            let public_key = decode_hex::<32>(key_hex);
            let signature = decode_hex::<64>(sig_hex);
            let message = decode_hex_vec(msg_hex);

            let pub_key = PublicKey::from_bytes(&public_key);
            let result = pub_key.and_then(|pk| pk.verify(&message, &signature));

            if should_reject {
                assert!(
                    result.is_err(),
                    "vector #{number} should be rejected (flags: {flags_str}) but was accepted",
                );
            } else {
                assert!(
                    result.is_ok(),
                    "vector #{number} should be accepted (flags: {flags_str}) but was rejected",
                );
            }
        }
    }

    #[test]
    fn rfc8032_extended_vectors() {
        check_vector(
            "833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42",
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf",
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f",
            "dc2a4459e7369633a52b1bf277839a00201009a3efbf3ecb69bea2186c26b58909351fc9ac90b3ecfdfbc7c66431e0303dca179c138ac17ad9bef1177331a704",
        );
    }

    #[test]
    fn verify_rejects_all_zero_signature() {
        let public_key = decode_hex::<32>("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a");
        let signature = [0u8; 64];
        let pk = PublicKey::from_bytes(&public_key).unwrap();
        let result = pk.verify(b"test", &signature);
        assert!(result.is_err());
    }

    #[test]
    fn verify_rejects_s_equals_l() {
        let seed = decode_hex::<32>("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");
        let pub_key = SecretKey::from_bytes(&seed).public_key();
        let signature = SecretKey::from_bytes(&seed).sign(b"test");

        let mut bad_sig = signature;
        bad_sig[32..].copy_from_slice(&[
            0xed, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58, 0xd6, 0x9c, 0xf7, 0xa2, 0xde, 0xf9, 0xde, 0x14, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10,
        ]);
        assert!(pub_key.verify(b"test", &bad_sig).is_err());
    }

    #[test]
    fn verify_rejects_non_canonical_point_encodings() {
        let non_canonical_key = decode_hex::<32>("edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f");
        assert!(PublicKey::from_bytes(&non_canonical_key).is_err());

        let seed = decode_hex::<32>("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");
        let pub_key = SecretKey::from_bytes(&seed).public_key();
        let mut bad_sig = SecretKey::from_bytes(&seed).sign(b"test");
        bad_sig[..32].copy_from_slice(&[
            0xed, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f,
        ]);
        assert!(pub_key.verify(b"test", &bad_sig).is_err());
    }

    #[test]
    fn edwards_identity_point_roundtrip() {
        let id_bytes = [
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let point = EdwardsPoint::from_bytes(&id_bytes).unwrap();
        let roundtripped = point.to_bytes().unwrap();
        assert_eq!(roundtripped, id_bytes);
    }

    #[test]
    fn ed25519_to_montgomery_u_conversion() {
        let seed = decode_hex::<32>("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");
        let priv_key = SecretKey::from_bytes(&seed);
        let ed_pub = priv_key.public_key();

        let u = ed_pub
            .to_montgomery_u()
            .expect("valid ed point must convert to montgomery u");
        let u_bytes = u.to_bytes();

        let x_pub_from_ed = x25519::PublicKey::try_from(&ed_pub).unwrap();
        assert_eq!(u_bytes, x_pub_from_ed.to_bytes());

        assert_ne!(u_bytes, [0u8; 32], "montgomery u must be non-zero for non-identity point");
    }

    #[test]
    fn rfc8032_test_1024() {
        check_vector(
            "f5e5767cf153319517630f226876b86c8160cc583bc013744c6bf255f5cc0ee5",
            "278117fc144c72340f67d0f2316e8386ceffbf2b2428c9c51fef7c597f1d426e",
            "08b8b2b733424243760fe426a4b54908632110a66c2f6591eabd3345e3e4eb98fa6e264bf09efe12ee50f8f54e9f77b1e355f6c50544e23fb1433ddf73be84d879de7c0046dc4996d9e773f4bc9efe5738829adb26c81b37c93a1b270b20329d658675fc6ea534e0810a4432826bf58c941efb65d57a338bbd2e26640f89ffbc1a858efcb8550ee3a5e1998bd177e93a7363c344fe6b199ee5d02e82d522c4feba15452f80288a821a579116ec6dad2b3b310da903401aa62100ab5d1a36553e06203b33890cc9b832f79ef80560ccb9a39ce767967ed628c6ad573cb116dbefefd75499da96bd68a8a97b928a8bbc103b6621fcde2beca1231d206be6cd9ec7aff6f6c94fcd7204ed3455c68c83f4a41da4af2b74ef5c53f1d8ac70bdcb7ed185ce81bd84359d44254d95629e9855a94a7c1958d1f8ada5d0532ed8a5aa3fb2d17ba70eb6248e594e1a2297acbbb39d502f1a8c6eb6f1ce22b3de1a1f40cc24554119a831a9aad6079cad88425de6bde1a9187ebb6092cf67bf2b13fd65f27088d78b7e883c8759d2c4f5c65adb7553878ad575f9fad878e80a0c9ba63bcbcc2732e69485bbc9c90bfbd62481d9089beccf80cfe2df16a2cf65bd92dd597b0707e0917af48bbb75fed413d238f5555a7a569d80c3414a8d0859dc65a46128bab27af87a71314f318c782b23ebfe808b82b0ce26401d2e22f04d83d1255dc51addd3b75a2b1ae0784504df543af8969be3ea7082ff7fc9888c144da2af58429ec96031dbcad3dad9af0dcbaaaf268cb8fcffead94f3c7ca495e056a9b47acdb751fb73e666c6c655ade8297297d07ad1ba5e43f1bca32301651339e22904cc8c42f58c30c04aafdb038dda0847dd988dcda6f3bfd15c4b4c4525004aa06eeff8ca61783aacec57fb3d1f92b0fe2fd1a85f6724517b65e614ad6808d6f6ee34dff7310fdc82aebfd904b01e1dc54b2927094b2db68d6f903b68401adebf5a7e08d78ff4ef5d63653a65040cf9bfd4aca7984a74d37145986780fc0b16ac451649de6188a7dbdf191f64b5fc5e2ab47b57f7f7276cd419c17a3ca8e1b939ae49e488acba6b965610b5480109c8b17b80e1b7b750dfc7598d5d5011fd2dcc5600a32ef5b52a1ecc820e308aa342721aac0943bf6686b64b2579376504ccc493d97e6aed3fb0f9cd71a43dd497f01f17c0e2cb3797aa2a2f256656168e6c496afc5fb93246f6b1116398a346f1a641f3b041e989f7914f90cc2c7fff357876e506b50d334ba77c225bc307ba537152f3f1610e4eafe595f6d9d90d11faa933a15ef1369546868a7f3a45a96768d40fd9d03412c091c6315cf4fde7cb68606937380db2eaaa707b4c4185c32eddcdd306705e4dc1ffc872eeee475a64dfac86aba41c0618983f8741c5ef68d3a101e8a3b8cac60c905c15fc910840b94c00a0b9d0",
            "0aab4c900501b3e24d7cdf4663326a3a87df5e4843b2cbdb67cbf6e460fec350aa5371b1508f9f4528ecea23c436d94b5e8fcd4f681e30a6ac00a9704a188a03",
        );
    }

    #[test]
    fn wycheproof_ed25519_vectors() {
        #[derive(serde::Deserialize)]
        struct TestJson {
            #[serde(rename = "testGroups")]
            test_groups: Vec<TestGroup>,
        }

        #[derive(serde::Deserialize)]
        struct TestGroup {
            #[serde(rename = "publicKey")]
            public_key: PublicKeyJson,
            tests: Vec<TestCase>,
        }

        #[derive(serde::Deserialize)]
        struct PublicKeyJson {
            pk: String,
        }

        #[derive(serde::Deserialize)]
        struct TestCase {
            #[serde(rename = "tcId")]
            tc_id: u32,
            // #[allow(dead_code)]
            // comment: String,
            msg: String,
            sig: String,
            result: String,
        }

        let data = include_str!("../../testdata/wycheproof/testvectors_v1/ed25519_test.json");
        let parsed: TestJson = serde_json::from_str(data).unwrap();

        let mut valid_tested = 0usize;
        let mut invalid_tested = 0usize;
        let mut skipped = 0usize;

        for group in &parsed.test_groups {
            let public_key = decode_hex::<32>(&group.public_key.pk);
            let pk = PublicKey::from_bytes(&public_key).unwrap();

            for test in &group.tests {
                let msg = decode_hex_vec(&test.msg);
                let sig_hex = &test.sig;
                if sig_hex.len() != 128 {
                    skipped += 1;
                    continue;
                }
                let signature = decode_hex::<64>(sig_hex);
                let should_be_valid = test.result == "valid";

                let result = pk.verify(&msg, &signature);
                if should_be_valid {
                    assert!(
                        result.is_ok(),
                        "Wycheproof test #{}: expected valid but got {:?}",
                        test.tc_id,
                        result,
                    );
                    valid_tested += 1;
                } else {
                    assert!(result.is_err(), "Wycheproof test #{}: expected invalid but got ok", test.tc_id,);
                    invalid_tested += 1;
                }
            }
        }

        assert!(valid_tested > 0, "must test at least one valid Wycheproof vector");
        assert!(invalid_tested > 0, "must test at least one invalid Wycheproof vector");
        assert!(skipped > 0, "some truncated signatures should be skipped");

        eprintln!("Wycheproof ed25519: {valid_tested} valid, {invalid_tested} invalid, {skipped} skipped");
    }

    #[test]
    fn sign_verify_roundtrip_various_lengths() {
        let seed = decode_hex::<32>("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");
        let priv_key = SecretKey::from_bytes(&seed);
        let pub_key = priv_key.public_key();

        for len in [0, 1, 2, 16, 32, 64, 128, 255, 256, 1024] {
            let message: Vec<u8> = (0..len).map(|i| (i & 0xff) as u8).collect();
            let signature = priv_key.sign(&message);
            assert!(
                pub_key.verify(&message, &signature).is_ok(),
                "roundtrip failed for message length {len}"
            );
        }
    }

    #[test]
    fn fixed_window_matches_scalar_mul() {
        for _ in 0..100 {
            let s = Scalar(U256::from_limbs([
                rand::random::<u64>(),
                rand::random::<u64>(),
                rand::random::<u64>(),
                rand::random::<u64>(),
            ]));
            let s = Scalar(s.0.add_mod(&U256::ZERO, &MODULUS_L));
            let old_result = scalar_mul(&BASEPOINT, &s);
            let new_result = scalar_mul_base(&s);
            assert_eq!(old_result.to_bytes(), new_result.to_bytes(), "mismatch for scalar={:x}", s.0,);
        }
    }

    #[test]
    fn mul_mod_barrett_agrees_with_mul_mod() {
        for _ in 0..100 {
            let a = U256::from_limbs([
                rand::random::<u64>(),
                rand::random::<u64>(),
                rand::random::<u64>(),
                rand::random::<u64>(),
            ]);
            let b = U256::from_limbs([
                rand::random::<u64>(),
                rand::random::<u64>(),
                rand::random::<u64>(),
                rand::random::<u64>(),
            ]);
            let slow = a.mul_mod(&b, &MODULUS_L);
            let fast = a.mul_mod_barrett(&b, &MODULUS_L, &BAR_MU);
            assert_eq!(slow, fast, "mismatch for a={a:x}, b={b:x}");
        }
    }

    #[test]
    fn basepoint() {
        assert_eq!(BASEPOINT, EdwardsPoint::from_bytes(&BASEPOINT_COMPRESSED).unwrap())
    }
}
