use big_number::Uint;

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
// TODO: zeroize
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SecretKey {
    scalar: Scalar,
    public_point: AffinePoint,
}

impl SecretKey {
    #[cfg(feature = "random")]
    pub fn generate() -> Result<SecretKey, EllipticCurveError> {
        let key: [u8; SECRET_KEY_SIZE] = crate::random::random_bytes();
        Self::from_bytes(&key)
    }

    pub fn from_bytes(key: &[u8; SECRET_KEY_SIZE]) -> Result<SecretKey, EllipticCurveError> {
        let scalar = Scalar::from_bytes(key).ok_or(EllipticCurveError::InvalidKey)?;
        let public_point = scalar_mul_generator(&scalar)
            .to_affine()
            .ok_or(EllipticCurveError::Unspecified)?;
        Ok(SecretKey {
            scalar,
            public_point,
        })
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey {
            point: self.public_point,
        }
    }

    pub fn sign(&self, message: &[u8]) -> Result<[u8; SIGNATURE_SIZE], EllipticCurveError> {
        ecdsa_sign_inner(&self.scalar, message)
    }

    pub fn ecdh(&self, peer_public: &PublicKey) -> Result<[u8; ECDH_SHARED_SECRET_SIZE], EllipticCurveError> {
        ecdh_inner(&self.scalar, &peer_public.point)
    }

    pub fn to_bytes(&self) -> [u8; SECRET_KEY_SIZE] {
        self.scalar.to_bytes()
    }
}

/// P-224 (secp224r1) ECDSA public key.
///
/// Supports signature verification and ECDH key agreement. Both compressed
/// (29-byte) and uncompressed (57-byte) SEC1 encodings are accepted on input;
/// use [`to_bytes`](Self::to_bytes) to export uncompressed, and
/// [`to_compressed_bytes`](Self::to_compressed_bytes) to export compressed.
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
pub struct PublicKey {
    point: AffinePoint,
}

impl PublicKey {
    #[inline]
    pub fn from_bytes(key: &[u8]) -> Result<PublicKey, EllipticCurveError> {
        let point = AffinePoint::from_sec1_bytes(key).ok_or(EllipticCurveError::InvalidKey)?;
        Ok(PublicKey {
            point,
        })
    }

    /// Build a public key from raw affine x and y coordinates (both
    /// big-endian, 28 bytes each). Returns `InvalidKey` if the coordinates
    /// are not a valid point on the P-224 curve.
    ///
    /// This is useful when importing keys from formats like JWK where `x`
    /// and `y` are available directly.
    #[inline]
    pub fn from_x_y(x_bytes: &[u8; 28], y_bytes: &[u8; 28]) -> Result<PublicKey, EllipticCurveError> {
        let x = FieldElement::from_bytes(x_bytes).ok_or(EllipticCurveError::InvalidKey)?;
        let y = FieldElement::from_bytes(y_bytes).ok_or(EllipticCurveError::InvalidKey)?;
        let point = AffinePoint::new(x, y).ok_or(EllipticCurveError::InvalidKey)?;
        Ok(PublicKey {
            point,
        })
    }

    pub fn verify(&self, message: &[u8], signature: &[u8; SIGNATURE_SIZE]) -> Result<(), EllipticCurveError> {
        ecdsa_verify_inner(&self.point, message, signature)
    }

    #[inline]
    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_UNCOMPRESSED_SIZE] {
        self.point.to_uncompressed_bytes()
    }

    /// Returns the 29-byte compressed SEC1 encoding (`0x02`/`0x03 || x`).
    ///
    /// # Errors
    ///
    /// A valid `PublicKey` always has a compressed encoding, so this never
    /// returns `Err`; the `Result` mirrors the other fallible key operations.
    #[inline]
    pub fn to_compressed_bytes(&self) -> [u8; PUBLIC_KEY_COMPRESSED_SIZE] {
        self.point.to_compressed_bytes()
    }

    /// Returns the `X` and `Y` points as big-endian arrays.
    #[inline]
    pub fn x_y(&self) -> ([u8; 28], [u8; 28]) {
        (self.point.x.to_bytes(), self.point.y.to_bytes())
    }
}

type U224 = Uint<224, 4>;

// P-224 values are 224 bits (28 bytes), so they do not fill an integral number
// of 64-bit limbs. `Uint`'s slice helpers assume a whole number of limbs, so
// use these local converters to pack/unpack the 28-byte big-endian encoding.
#[inline]
fn u224_from_be(bytes: &[u8; 28]) -> U224 {
    U224::from_limbs([
        u64::from_be_bytes(bytes[20..28].try_into().unwrap()),
        u64::from_be_bytes(bytes[12..20].try_into().unwrap()),
        u64::from_be_bytes(bytes[4..12].try_into().unwrap()),
        u32::from_be_bytes(bytes[0..4].try_into().unwrap()) as u64,
    ])
}

#[inline]
fn u224_to_be(value: U224) -> [u8; 28] {
    let mut out = [0u8; 28];
    out[20..28].copy_from_slice(&value.limbs[0].to_be_bytes());
    out[12..20].copy_from_slice(&value.limbs[1].to_be_bytes());
    out[4..12].copy_from_slice(&value.limbs[2].to_be_bytes());
    out[0..4].copy_from_slice(&(value.limbs[3] as u32).to_be_bytes());
    out
}

const MODULUS_P: U224 = U224::from_limbs([
    0x0000_0000_0000_0001,
    0xffff_ffff_0000_0000,
    0xffff_ffff_ffff_ffff,
    0x0000_0000_ffff_ffff,
]);

const MODULUS_N: U224 = U224::from_limbs([
    0x13dd_2945_5c5c_2a3d,
    0xffff_16a2_e0b8_f03e,
    0xffff_ffff_ffff_ffff,
    0x0000_0000_ffff_ffff,
]);

const P_MINUS_TWO: U224 = U224::from_limbs([
    0xffff_ffff_ffff_ffff,
    0xffff_fffe_ffff_ffff,
    0xffff_ffff_ffff_ffff,
    0x0000_0000_ffff_ffff,
]);

const N_MINUS_TWO: U224 = U224::from_limbs([
    0x13dd_2945_5c5c_2a3b,
    0xffff_16a2_e0b8_f03e,
    0xffff_ffff_ffff_ffff,
    0x0000_0000_ffff_ffff,
]);

// (p - 1) / 2, used for the Legendre symbol during square-root computation.
const P_MINUS_ONE_OVER_TWO: U224 = U224::from_limbs([
    0x0000_0000_0000_0000,
    0xffff_ffff_8000_0000,
    0xffff_ffff_ffff_ffff,
    0x0000_0000_7fff_ffff,
]);

// P-224 has p = 2^224 - 2^96 + 1, so p - 1 = 2^96 * d with d = 2^128 - 1.
// These constants are used by the Tonelli-Shanks square-root algorithm.
const TONELLI_S: usize = 96;
const TONELLI_D: U224 = U224::from_limbs([
    0xffff_ffff_ffff_ffff,
    0xffff_ffff_ffff_ffff,
    0x0000_0000_0000_0000,
    0x0000_0000_0000_0000,
]);
const TONELLI_D_PLUS_ONE_OVER_TWO: U224 = U224::from_limbs([
    0x0000_0000_0000_0000,
    0x8000_0000_0000_0000,
    0x0000_0000_0000_0000,
    0x0000_0000_0000_0000,
]);
// Smallest quadratic non-residue modulo p (verified: 11^((p-1)/2) = -1).
const TONELLI_NON_RESIDUE: FieldElement = FieldElement(U224::from_u64(11));

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

const CURVE_B: FieldElement = FieldElement(U224::from_limbs([
    0x270b_3943_2355_ffb4,
    0x5044_b0b7_d7bf_d8ba,
    0x0c04_b3ab_f541_3256,
    0x0000_0000_b405_0a85,
]));

const GENERATOR_X: FieldElement = FieldElement(U224::from_limbs([
    0x3432_80d6_115c_1d21,
    0x4a03_c1d3_56c2_1122,
    0x6bb4_bf7f_3213_90b9,
    0x0000_0000_b70e_0cbd,
]));

const GENERATOR_Y: FieldElement = FieldElement(U224::from_limbs([
    0x44d5_8199_8500_7e34,
    0xcd43_75a0_5a07_4764,
    0xb5f7_23fb_4c22_dfe6,
    0x0000_0000_bd37_6388,
]));

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FieldElement(U224);

impl FieldElement {
    const ZERO: Self = Self(U224::ZERO);
    const ONE: Self = Self(U224::ONE);

    #[inline]
    fn from_bytes(bytes: &[u8; 28]) -> Option<Self> {
        let value = u224_from_be(bytes);
        if value.ct_ge(&MODULUS_P) {
            None
        } else {
            Some(Self(value))
        }
    }

    #[inline]
    fn to_bytes(self) -> [u8; 28] {
        u224_to_be(self.0)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[inline]
    fn is_odd(&self) -> bool {
        self.0.is_odd()
    }

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.0.add_mod(&rhs.0, &MODULUS_P))
    }

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0.sub_mod(&rhs.0, &MODULUS_P))
    }

    #[inline]
    fn double(self) -> Self {
        Self(self.0.double_mod(&MODULUS_P))
    }

    #[inline]
    fn square(self) -> Self {
        self.mul(self)
    }

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self(self.0.mul_mod_barrett(&rhs.0, &MODULUS_P, &P_MU))
    }

    #[inline]
    fn triple(self) -> Self {
        self.double().add(self)
    }

    #[inline]
    fn negate(self) -> Self {
        let (diff, _) = MODULUS_P.sub_raw(&self.0);
        Self(U224::ct_select(&U224::ZERO, &diff, self.is_zero()))
    }

    #[inline]
    fn pow(self, exponent: &U224) -> Self {
        let mut result = Self::ONE;
        let mut i = 224usize;
        while i > 0 {
            i -= 1;
            result = result.square();
            let product = result.mul(self);
            result = Self::select(&product, &result, exponent.bit(i));
        }
        result
    }

    #[inline]
    fn invert(self) -> Option<Self> {
        Some(self.pow(&P_MINUS_TWO))
    }

    /// Square root modulo p. Returns `None` when `self` is a quadratic
    /// non-residue.
    ///
    /// P-224 has p ≡ 1 (mod 4), so the simple `a^((p+1)/4)` formula used by
    /// P-256/P-384 is unavailable; this uses Tonelli-Shanks instead. It is only
    /// invoked on public data (SEC1 decompression), so its data-dependent
    /// control flow does not leak secrets.
    #[inline]
    fn sqrt(self) -> Option<Self> {
        if self.is_zero() {
            return Some(Self::ZERO);
        }
        if self.pow(&P_MINUS_ONE_OVER_TWO) != Self::ONE {
            return None;
        }

        let mut m = TONELLI_S;
        let mut c = TONELLI_NON_RESIDUE.pow(&TONELLI_D);
        let mut t = self.pow(&TONELLI_D);
        let mut r = self.pow(&TONELLI_D_PLUS_ONE_OVER_TWO);

        while t != Self::ONE {
            // Find the least i (0 < i < m) such that t^(2^i) == 1.
            let mut i = 0usize;
            let mut probe = t;
            while probe != Self::ONE {
                probe = probe.square();
                i += 1;
                if i >= m {
                    return None;
                }
            }

            let mut b = c;
            for _ in 0..(m - i - 1) {
                b = b.square();
            }

            m = i;
            c = b.square();
            t = t.mul(c);
            r = r.mul(b);
        }

        if r.square() == self { Some(r) } else { None }
    }

    #[inline]
    fn select(a: &Self, b: &Self, choice: bool) -> Self {
        Self(U224::ct_select(&a.0, &b.0, choice))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Scalar(U224);

impl Scalar {
    const ZERO: Self = Self(U224::ZERO);
    const ONE: Self = Self(U224::ONE);

    #[inline]
    fn from_bytes(bytes: &[u8; 28]) -> Option<Self> {
        let value = u224_from_be(bytes);
        if value.is_zero() || value.ct_ge(&MODULUS_N) {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Reduce a 224-bit big-endian value modulo n. Unlike [`Self::from_bytes`],
    /// this accepts zero and values greater than or equal to n.
    #[inline]
    fn from_reduced_bytes(bytes: &[u8; 28]) -> Self {
        let value = u224_from_be(bytes);
        let (sub_value, _) = value.sub_raw(&MODULUS_N);
        let reduced = U224::ct_select(&sub_value, &value, value.ct_ge(&MODULUS_N));
        Self(reduced)
    }

    /// Reduce a SHA-256 digest to a scalar. FIPS 186-4 truncates the digest to
    /// the leftmost 224 bits (the first 28 bytes) before reduction.
    #[inline]
    fn from_hash(hash: &[u8; 32]) -> Self {
        Self::from_reduced_bytes(&hash[..28].try_into().unwrap())
    }

    #[inline]
    fn to_bytes(self) -> [u8; 28] {
        u224_to_be(self.0)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[inline]
    fn bit(&self, index: usize) -> bool {
        self.0.bit(index)
    }

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.0.add_mod(&rhs.0, &MODULUS_N))
    }

    #[cfg(test)]
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0.sub_mod(&rhs.0, &MODULUS_N))
    }

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self(self.0.mul_mod_barrett(&rhs.0, &MODULUS_N, &N_MU))
    }

    #[inline]
    fn invert(self) -> Option<Self> {
        Some(Self(self.scalar_pow(&N_MINUS_TWO)))
    }

    #[inline]
    fn scalar_pow(self, exponent: &U224) -> U224 {
        let mut result = Scalar::ONE;
        let mut i = 224usize;
        while i > 0 {
            i -= 1;
            result = result.mul(result);
            let product = result.mul(self);
            result = Scalar::select(&product, &result, exponent.bit(i));
        }
        result.0
    }

    #[inline]
    fn select(a: &Self, b: &Self, choice: bool) -> Self {
        Self(U224::ct_select(&a.0, &b.0, choice))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AffinePoint {
    x: FieldElement,
    y: FieldElement,
    infinity: bool,
}

impl AffinePoint {
    const GENERATOR: Self = Self {
        x: GENERATOR_X,
        y: GENERATOR_Y,
        infinity: false,
    };

    #[inline]
    fn new(x: FieldElement, y: FieldElement) -> Option<Self> {
        let point = Self {
            x,
            y,
            infinity: false,
        };
        if point.is_on_curve() { Some(point) } else { None }
    }

    #[inline]
    fn is_on_curve(&self) -> bool {
        if self.infinity {
            return false;
        }
        let x2 = self.x.square();
        let x3 = x2.mul(self.x);
        let rhs = x3.sub(self.x.triple()).add(CURVE_B);
        self.y.square() == rhs
    }

    #[inline]
    fn to_uncompressed_bytes(&self) -> [u8; PUBLIC_KEY_UNCOMPRESSED_SIZE] {
        let mut out = [0u8; PUBLIC_KEY_UNCOMPRESSED_SIZE];
        out[0] = 0x04;
        out[1..29].copy_from_slice(&self.x.to_bytes());
        out[29..57].copy_from_slice(&self.y.to_bytes());
        out
    }

    /// SEC1 compressed encoding: `0x02`/`0x03 || x`, selecting the prefix from
    /// the parity of `y`.
    #[inline]
    fn to_compressed_bytes(&self) -> [u8; PUBLIC_KEY_COMPRESSED_SIZE] {
        let mut out = [0u8; PUBLIC_KEY_COMPRESSED_SIZE];
        out[0] = if self.y.is_odd() { 0x03 } else { 0x02 };
        out[1..29].copy_from_slice(&self.x.to_bytes());
        out
    }

    fn from_sec1_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes.len() {
            PUBLIC_KEY_UNCOMPRESSED_SIZE if bytes[0] == 0x04 => {
                let x = FieldElement::from_bytes(bytes[1..29].try_into().unwrap())?;
                let y = FieldElement::from_bytes(bytes[29..57].try_into().unwrap())?;
                Self::new(x, y)
            }
            PUBLIC_KEY_COMPRESSED_SIZE if bytes[0] == 0x02 || bytes[0] == 0x03 => {
                let x = FieldElement::from_bytes(bytes[1..29].try_into().unwrap())?;
                let rhs = x.square().mul(x).sub(x.triple()).add(CURVE_B);
                let y = rhs.sqrt()?;
                let y_is_odd = y.is_odd();
                let select_neg = y_is_odd != (bytes[0] == 0x03);
                let y = FieldElement::select(&y.negate(), &y, select_neg);
                Self::new(x, y)
            }
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ProjectivePoint {
    x: FieldElement,
    y: FieldElement,
    z: FieldElement,
}

impl ProjectivePoint {
    const IDENTITY: Self = Self {
        x: FieldElement::ZERO,
        y: FieldElement::ONE,
        z: FieldElement::ZERO,
    };

    #[cfg(test)]
    #[inline]
    fn from_affine(point: &AffinePoint) -> Self {
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
    fn is_identity(&self) -> bool {
        self.z.is_zero()
    }

    #[inline]
    fn select(a: &Self, b: &Self, choice: bool) -> Self {
        Self {
            x: FieldElement::select(&a.x, &b.x, choice),
            y: FieldElement::select(&a.y, &b.y, choice),
            z: FieldElement::select(&a.z, &b.z, choice),
        }
    }

    #[inline]
    fn to_affine(&self) -> Option<AffinePoint> {
        if self.is_identity() {
            return None;
        }
        let z_inv = self.z.invert()?;
        AffinePoint::new(self.x.mul(z_inv), self.y.mul(z_inv))
    }

    fn add(&self, rhs: &Self) -> Self {
        let xx = self.x.mul(rhs.x);
        let yy = self.y.mul(rhs.y);
        let zz = self.z.mul(rhs.z);
        let xy_pairs = self.x.add(self.y).mul(rhs.x.add(rhs.y)).sub(xx.add(yy));
        let yz_pairs = self.y.add(self.z).mul(rhs.y.add(rhs.z)).sub(yy.add(zz));
        let xz_pairs = self.x.add(self.z).mul(rhs.x.add(rhs.z)).sub(xx.add(zz));

        let bzz_part = xz_pairs.sub(CURVE_B.mul(zz));
        let bzz3_part = bzz_part.triple();
        let yy_m_bzz3 = yy.sub(bzz3_part);
        let yy_p_bzz3 = yy.add(bzz3_part);

        let zz3 = zz.triple();
        let bxz_part = CURVE_B.mul(xz_pairs).sub(zz3.add(xx));
        let bxz3_part = bxz_part.triple();
        let xx3_m_zz3 = xx.triple().sub(zz3);

        Self {
            x: yy_p_bzz3.mul(xy_pairs).sub(yz_pairs.mul(bxz3_part)),
            y: yy_p_bzz3.mul(yy_m_bzz3).add(xx3_m_zz3.mul(bxz3_part)),
            z: yy_m_bzz3.mul(yz_pairs).add(xy_pairs.mul(xx3_m_zz3)),
        }
    }

    fn add_mixed(&self, rhs: &AffinePoint) -> Self {
        if rhs.infinity {
            return *self;
        }

        let xx = self.x.mul(rhs.x);
        let yy = self.y.mul(rhs.y);
        let xy_pairs = self.x.add(self.y).mul(rhs.x.add(rhs.y)).sub(xx.add(yy));
        let yz_pairs = rhs.y.mul(self.z).add(self.y);
        let xz_pairs = rhs.x.mul(self.z).add(self.x);

        let bz_part = xz_pairs.sub(CURVE_B.mul(self.z));
        let bz3_part = bz_part.triple();
        let yy_m_bzz3 = yy.sub(bz3_part);
        let yy_p_bzz3 = yy.add(bz3_part);

        let z3 = self.z.triple();
        let bxz_part = CURVE_B.mul(xz_pairs).sub(z3.add(xx));
        let bxz3_part = bxz_part.triple();
        let xx3_m_zz3 = xx.triple().sub(z3);

        Self {
            x: yy_p_bzz3.mul(xy_pairs).sub(yz_pairs.mul(bxz3_part)),
            y: yy_p_bzz3.mul(yy_m_bzz3).add(xx3_m_zz3.mul(bxz3_part)),
            z: yy_m_bzz3.mul(yz_pairs).add(xy_pairs.mul(xx3_m_zz3)),
        }
    }

    fn double(&self) -> Self {
        let xx = self.x.square();
        let yy = self.y.square();
        let zz = self.z.square();
        let xy2 = self.x.mul(self.y).double();
        let xz2 = self.x.mul(self.z).double();

        let bzz_part = CURVE_B.mul(zz).sub(xz2);
        let bzz3_part = bzz_part.triple();
        let yy_m_bzz3 = yy.sub(bzz3_part);
        let yy_p_bzz3 = yy.add(bzz3_part);
        let y_frag = yy_p_bzz3.mul(yy_m_bzz3);
        let x_frag = yy_m_bzz3.mul(xy2);

        let zz3 = zz.triple();
        let bxz2_part = CURVE_B.mul(xz2).sub(zz3.add(xx));
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

fn scalar_mul_generator(scalar: &Scalar) -> ProjectivePoint {
    scalar_mul_affine(&AffinePoint::GENERATOR, scalar)
}

fn scalar_mul_affine(base: &AffinePoint, scalar: &Scalar) -> ProjectivePoint {
    let mut acc = ProjectivePoint::IDENTITY;
    let mut bit = 224usize;
    while bit > 0 {
        bit -= 1;
        acc = acc.double();
        let candidate = acc.add_mixed(base);
        acc = ProjectivePoint::select(&candidate, &acc, scalar.bit(bit));
    }
    acc
}

#[inline]
fn hash_message(message: &[u8]) -> [u8; 32] {
    let digest = Sha256::hash(message);
    return digest.as_ref().try_into().unwrap();
}

#[inline]
fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
    let mac = Hmac::<Sha256>::mac(key, data);
    return mac.as_ref().try_into().unwrap();
}

// FIPS 186-4 truncates a SHA-256 digest to the leftmost 224 bits, then reduces
// modulo n. The result is the 28-byte `bits2octets` value used by RFC 6979.
fn bits2octets(hash: &[u8; 32]) -> [u8; 28] {
    Scalar::from_hash(hash).to_bytes()
}

// RFC 6979 DRBG state uses the full 32-byte SHA-256 output for V and K, while
// int2octets(x) and bits2octets(h1) are only 28 bytes (the size of the order n).
fn rfc6979_init_state(private_key: &Scalar, message_hash: &[u8; 32]) -> ([u8; 32], [u8; 32]) {
    let x = private_key.to_bytes();
    let h1 = bits2octets(message_hash);

    let mut v = [0x01u8; 32];
    let mut k = [0u8; 32];

    let mut buf = [0u8; 89];
    buf[..32].copy_from_slice(&v);
    buf[32] = 0x00;
    buf[33..61].copy_from_slice(&x);
    buf[61..89].copy_from_slice(&h1);
    k = hmac_sha256(&k, &buf);
    v = hmac_sha256(&k, &v);

    buf[..32].copy_from_slice(&v);
    buf[32] = 0x01;
    k = hmac_sha256(&k, &buf);
    v = hmac_sha256(&k, &v);

    (k, v)
}

fn rfc6979_retry(k: &mut [u8; 32], v: &mut [u8; 32]) {
    let mut retry_buf = [0u8; 33];
    retry_buf[..32].copy_from_slice(v);
    retry_buf[32] = 0x00;
    *k = hmac_sha256(k, &retry_buf);
    *v = hmac_sha256(k, v);
}

fn rfc6979_retry_clone(k: &[u8; 32], v: &[u8; 32]) -> ([u8; 32], [u8; 32]) {
    let mut retry_buf = [0u8; 33];
    retry_buf[..32].copy_from_slice(v);
    retry_buf[32] = 0x00;
    let k_new = hmac_sha256(k, &retry_buf);
    let v_new = hmac_sha256(&k_new, v);
    (k_new, v_new)
}

// Branch-free byte-level select: returns a[i] if choice else b[i].
fn ct_select_bytes<const N: usize>(a: &[u8; N], b: &[u8; N], choice: bool) -> [u8; N] {
    let mask = (choice as u8).wrapping_neg();
    let mut out = [0u8; N];
    for i in 0..N {
        out[i] = (a[i] & mask) | (b[i] & !mask);
    }
    out
}

fn rfc6979_generate_k(private_key: &Scalar, message_hash: &[u8; 32]) -> Scalar {
    let (mut k, mut v) = rfc6979_init_state(private_key, message_hash);

    // Fixed 3 iterations with constant-time state selection, mirroring the
    // P-256/P-384 implementations. The first valid candidate is returned.
    let mut candidate = [0u8; 28];
    let mut found = false;

    for _ in 0..3 {
        v = hmac_sha256(&k, &v);
        let candidate_bytes: [u8; 28] = v[..28].try_into().unwrap();
        let val = u224_from_be(&candidate_bytes);
        let is_valid = !val.is_zero() && !val.ct_ge(&MODULUS_N);

        let take = is_valid && !found;
        candidate = ct_select_bytes(&candidate_bytes, &candidate, take);
        found = found || is_valid;

        let (k_retry, v_retry) = rfc6979_retry_clone(&k, &v);
        k = ct_select_bytes(&k, &k_retry, !is_valid);
        v = ct_select_bytes(&v, &v_retry, !is_valid);
    }

    if found {
        return Scalar::from_bytes(&candidate).unwrap_or(Scalar::ZERO);
    }

    // Fallback (probability < 2^-96): emit additional HMAC outputs.
    v = hmac_sha256(&k, &v);
    if let Some(sc) = Scalar::from_bytes(&v[..28].try_into().unwrap()) {
        return sc;
    }

    loop {
        v = hmac_sha256(&k, &v);
        if let Some(sc) = Scalar::from_bytes(&v[..28].try_into().unwrap()) {
            return sc;
        }
        rfc6979_retry(&mut k, &mut v);
    }
}

fn parse_private_key(private_key: &[u8; SECRET_KEY_SIZE]) -> Result<Scalar, EllipticCurveError> {
    Scalar::from_bytes(private_key).ok_or(EllipticCurveError::InvalidKey)
}

fn parse_public_key(public_key: &[u8]) -> Result<AffinePoint, EllipticCurveError> {
    AffinePoint::from_sec1_bytes(public_key).ok_or(EllipticCurveError::InvalidKey)
}

#[cfg(test)]
fn derive_public_key_uncompressed(
    private_key: &[u8; SECRET_KEY_SIZE],
) -> Result<[u8; PUBLIC_KEY_UNCOMPRESSED_SIZE], EllipticCurveError> {
    let scalar = parse_private_key(private_key)?;
    let point = scalar_mul_generator(&scalar)
        .to_affine()
        .ok_or(EllipticCurveError::Unspecified)?;
    Ok(point.to_uncompressed_bytes())
}

#[cfg(test)]
fn derive_public_key_compressed(
    private_key: &[u8; SECRET_KEY_SIZE],
) -> Result<[u8; PUBLIC_KEY_COMPRESSED_SIZE], EllipticCurveError> {
    let scalar = parse_private_key(private_key)?;
    let point = scalar_mul_generator(&scalar)
        .to_affine()
        .ok_or(EllipticCurveError::Unspecified)?;
    Ok(point.to_compressed_bytes())
}

fn ecdh_inner(scalar: &Scalar, peer_point: &AffinePoint) -> Result<[u8; ECDH_SHARED_SECRET_SIZE], EllipticCurveError> {
    let shared_point = scalar_mul_affine(peer_point, scalar)
        .to_affine()
        .ok_or(EllipticCurveError::Unspecified)?;
    Ok(shared_point.x.to_bytes())
}

pub fn ecdh(
    secret_key: &[u8; SECRET_KEY_SIZE],
    peer_public_key: &[u8],
) -> Result<[u8; ECDH_SHARED_SECRET_SIZE], EllipticCurveError> {
    let scalar = parse_private_key(secret_key)?;
    let peer_point = parse_public_key(peer_public_key)?;
    ecdh_inner(&scalar, &peer_point)
}

fn ecdsa_sign_inner(scalar: &Scalar, message: &[u8]) -> Result<[u8; SIGNATURE_SIZE], EllipticCurveError> {
    let message_hash = hash_message(message);
    let z = Scalar::from_hash(&message_hash);

    // Fixed 2-iteration loop for the astronomically unlikely r=0 / s=0 retry.
    for _ in 0..2 {
        let k = rfc6979_generate_k(scalar, &message_hash);

        let r_point = scalar_mul_generator(&k)
            .to_affine()
            .ok_or(EllipticCurveError::Unspecified)?;
        let r = Scalar::from_reduced_bytes(&r_point.x.to_bytes());
        if r.is_zero() {
            continue;
        }

        let kinv = k.invert().ok_or(EllipticCurveError::Unspecified)?;
        let s = kinv.mul(z.add(r.mul(*scalar)));
        if s.is_zero() {
            continue;
        }

        let mut out = [0u8; SIGNATURE_SIZE];
        out[..28].copy_from_slice(&r.to_bytes());
        out[28..].copy_from_slice(&s.to_bytes());
        return Ok(out);
    }

    Err(EllipticCurveError::Unspecified)
}

fn ecdsa_verify_inner(
    public_point: &AffinePoint,
    message: &[u8],
    signature: &[u8; SIGNATURE_SIZE],
) -> Result<(), EllipticCurveError> {
    let r = Scalar::from_bytes(signature[..28].try_into().unwrap()).ok_or(EllipticCurveError::Unspecified)?;
    let s = Scalar::from_bytes(signature[28..].try_into().unwrap()).ok_or(EllipticCurveError::Unspecified)?;
    let z = Scalar::from_hash(&hash_message(message));

    let w = s.invert().ok_or(EllipticCurveError::Unspecified)?;
    let u1 = z.mul(w);
    let u2 = r.mul(w);

    let point = scalar_mul_generator(&u1).add(&scalar_mul_affine(public_point, &u2));
    let affine = point.to_affine().ok_or(EllipticCurveError::Unspecified)?;
    let x_mod_n = Scalar::from_reduced_bytes(&affine.x.to_bytes());

    if x_mod_n == r {
        Ok(())
    } else {
        Err(EllipticCurveError::Unspecified)
    }
}

pub fn is_valid_public_key(public_key: &[u8]) -> bool {
    AffinePoint::from_sec1_bytes(public_key).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let sample_signature = key.sign(b"sample").unwrap();
        let expected_sample = decode_hex::<56>(
            "61aa3da010e8e8406c656bc477a7a7189895e7e840cdfe8ff42307ba\
             bc814050dab5d23770879494f9e0a680dc1af7161991bde692b10101",
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
    fn rfc6979_nonce_generation_matches_known_value() {
        let private_key = Scalar::from_bytes(&decode_hex::<28>(RFC6979_PRIVATE_KEY)).unwrap();
        let hash = hash_message(b"sample");
        assert_eq!(
            rfc6979_generate_k(&private_key, &hash).to_bytes(),
            decode_hex::<28>("ad3029e0278f80643de33917ce6908c70a8ff50a411f06e41dedfcdc")
        );
    }

    #[test]
    fn rfc6979_test_message_nonce_matches_known_value() {
        let private_key = Scalar::from_bytes(&decode_hex::<28>(RFC6979_PRIVATE_KEY)).unwrap();
        let hash = hash_message(b"test");
        assert_eq!(
            rfc6979_generate_k(&private_key, &hash).to_bytes(),
            decode_hex::<28>("ff86f57924da248d6e44e8154eb69f0ae2aebaee9931d0b5a969f904")
        );
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
    fn compressed_public_key_has_correct_prefix() {
        for _ in 0..5 {
            let key = SecretKey::generate().unwrap();
            let compressed = key.public_key().to_compressed_bytes();
            let prefix = compressed[0];
            assert!(prefix == 0x02 || prefix == 0x03, "invalid compressed prefix: {prefix:#x}");

            // Prefix must encode the parity of y.
            let (_, y) = key.public_key().x_y();
            let expected_prefix = if y[27] & 1 == 1 { 0x03 } else { 0x02 };
            assert_eq!(prefix, expected_prefix);

            // Public export matches the internal derivation helper.
            assert_eq!(compressed, derive_public_key_compressed(&key.to_bytes()).unwrap());
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
        // Squares always have a root that squares back to the input.
        for _ in 0..200 {
            let bytes: [u8; 28] = rand::random();
            let Some(x) = FieldElement::from_bytes(&bytes) else {
                continue;
            };
            let square = x.square();
            let root = square.sqrt().expect("square should have a square root");
            assert_eq!(root.square(), square);
        }

        // 11 is the smallest quadratic non-residue modulo p.
        let non_residue = FieldElement(U224::from_u64(11));
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
    fn generator_point_is_on_curve() {
        assert!(AffinePoint::GENERATOR.is_on_curve());
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
        assert_eq!(result.x, GENERATOR_X);
        assert_eq!(result.y, GENERATOR_Y.negate());
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
    fn ecdh_round_trip_alice_bob() {
        let alice = SecretKey::generate().unwrap();
        let bob = SecretKey::generate().unwrap();

        let alice_shared = alice.ecdh(&bob.public_key()).unwrap();
        let bob_shared = bob.ecdh(&alice.public_key()).unwrap();

        assert_eq!(alice_shared, bob_shared);
        assert_eq!(alice_shared.len(), ECDH_SHARED_SECRET_SIZE);
    }

    #[test]
    fn ecdh_rejects_off_curve_peer_public_key() {
        let alice = SecretKey::generate().unwrap();
        let mut bad_pub = alice.public_key().to_bytes().to_vec();
        bad_pub[56] ^= 0x01;
        assert!(!is_valid_public_key(&bad_pub));
        assert!(ecdh(&alice.to_bytes(), &bad_pub).is_err());
    }

    #[test]
    fn ecdh_rejects_infinity_peer_public_key() {
        let alice = SecretKey::generate().unwrap();
        assert!(ecdh(&alice.to_bytes(), &[0x00u8]).is_err());
    }

    #[test]
    fn ecdh_rejects_bad_length_peer_public_key() {
        let alice = SecretKey::generate().unwrap();
        assert!(ecdh(&alice.to_bytes(), &[]).is_err());
        assert!(ecdh(&alice.to_bytes(), &[0x04, 0x00]).is_err());
        let long = [0x04u8; 200];
        assert!(ecdh(&alice.to_bytes(), &long).is_err());
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
    fn ecdh_multiple_exchanges_consistency() {
        let alice = SecretKey::generate().unwrap();
        let bob = SecretKey::generate().unwrap();
        let charlie = SecretKey::generate().unwrap();

        let alice_bob = alice.ecdh(&bob.public_key()).unwrap();
        let bob_alice = bob.ecdh(&alice.public_key()).unwrap();
        assert_eq!(alice_bob, bob_alice);

        let alice_charlie = alice.ecdh(&charlie.public_key()).unwrap();
        let charlie_alice = charlie.ecdh(&alice.public_key()).unwrap();
        assert_eq!(alice_charlie, charlie_alice);

        assert_ne!(alice_bob, alice_charlie);
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
    fn from_x_y_rejects_off_curve() {
        assert!(PublicKey::from_x_y(&[0u8; 28], &[0u8; 28]).is_err());
    }

    #[test]
    fn private_key_round_trip_bytes() {
        let key = SecretKey::generate().unwrap();
        let bytes = key.to_bytes();
        let key2 = SecretKey::from_bytes(&bytes).unwrap();
        assert_eq!(key.to_bytes(), key2.to_bytes());
        assert_eq!(key.public_key().to_bytes(), key2.public_key().to_bytes());
    }

    #[test]
    fn public_key_round_trip_bytes() {
        let key = SecretKey::generate().unwrap();
        let pub_key = key.public_key();
        let pub_key2 = PublicKey::from_bytes(&pub_key.to_bytes()).unwrap();
        assert_eq!(pub_key.to_bytes(), pub_key2.to_bytes());
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
    fn point_double_and_add_consistency() {
        let g = AffinePoint::GENERATOR;
        let proj_g = ProjectivePoint::from_affine(&g);
        assert_eq!(
            proj_g.double().to_affine().unwrap().to_uncompressed_bytes(),
            proj_g.add(&proj_g).to_affine().unwrap().to_uncompressed_bytes(),
        );
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
            "../testdata/wycheproof/testvectors_v1/ecdsa_secp224r1_sha256_p1363_test.json"
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
            "../testdata/wycheproof/testvectors_v1/ecdsa_secp224r1_sha256_test.json"
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
            include_str!("../testdata/wycheproof/testvectors_v1/ecdh_secp224r1_ecpoint_test.json"),
            false,
        );
    }

    #[test]
    fn wycheproof_ecdh_p224_asn() {
        wycheproof_ecdh_case(
            include_str!("../testdata/wycheproof/testvectors_v1/ecdh_secp224r1_test.json"),
            true,
        );
    }
}
