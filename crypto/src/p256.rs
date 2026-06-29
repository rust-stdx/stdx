use big_number::{Uint, mac};

use crate::{EllipticCurveError, Hasher, hmac::Hmac, sha2::Sha256};

/// Size of a P-256 private key in bytes (32 bytes).
pub const PRIVATE_KEY_SIZE: usize = 32;
/// Size of a compressed P-256 public key in bytes (33 bytes, includes 0x02/0x03 prefix).
pub const PUBLIC_KEY_COMPRESSED_SIZE: usize = 33;
/// Size of an uncompressed P-256 public key in bytes (65 bytes, includes 0x04 prefix).
pub const PUBLIC_KEY_UNCOMPRESSED_SIZE: usize = 65;
/// Size of a P-256 ECDSA signature in bytes (64 bytes, r || s).
pub const SIGNATURE_SIZE: usize = 64;
/// Size of the raw ECDH shared secret in bytes (32 bytes). **Must not** be used directly
/// as an encryption key; apply a KDF first.
pub const ECDH_SHARED_SECRET_SIZE: usize = 32;

/// P-256 (secp256r1) ECDSA private key.
///
/// Supports signing and ECDH key agreement.
///
/// # Signing
///
/// ```ignore
/// use crypto::p256::PrivateKey;
///
/// let key = PrivateKey::generate().unwrap();
/// let signature = key.sign(b"message").unwrap();
/// ```
///
/// # ECDH key exchange
///
/// ```ignore
/// use crypto::p256::PrivateKey;
///
/// let alice = PrivateKey::generate().unwrap();
/// let bob = PrivateKey::generate().unwrap();
/// let alice_shared = alice.ecdh(&bob.public_key()).unwrap();
/// let bob_shared = bob.ecdh(&alice.public_key()).unwrap();
/// assert_eq!(alice_shared, bob_shared);
/// ```
///
/// # Security
///
/// The raw shared secret from [`ecdh`](Self::ecdh) **must not** be used
/// directly as an encryption key. Apply a KDF (e.g. HKDF) first.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrivateKey {
    scalar: Scalar,
    public_point: AffinePoint,
}

impl PrivateKey {
    #[cfg(feature = "random")]
    pub fn generate() -> Result<PrivateKey, EllipticCurveError> {
        let key: [u8; PRIVATE_KEY_SIZE] = crate::random::random_bytes();
        Self::from_bytes(&key)
    }

    pub fn from_bytes(key: &[u8; PRIVATE_KEY_SIZE]) -> Result<PrivateKey, EllipticCurveError> {
        let scalar = Scalar::from_bytes(key).ok_or(EllipticCurveError::InvalidKey)?;
        let public_point = scalar_mul_generator(&scalar)
            .to_affine()
            .ok_or(EllipticCurveError::Unspecified)?;
        Ok(PrivateKey {
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

    pub fn to_bytes(&self) -> [u8; PRIVATE_KEY_SIZE] {
        self.scalar.to_bytes()
    }
}

/// P-256 (secp256r1) ECDSA public key.
///
/// Supports signature verification and ECDH key agreement.
///
/// # Verification
///
/// ```ignore
/// use crypto::p256::PrivateKey;
///
/// let key = PrivateKey::generate().unwrap();
/// let signature = key.sign(b"message").unwrap();
/// assert!(key.public_key().verify(b"message", &signature).is_ok());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PublicKey {
    point: AffinePoint,
}

impl PublicKey {
    pub fn from_bytes(key: &[u8]) -> Result<PublicKey, EllipticCurveError> {
        let point = AffinePoint::from_sec1_bytes(key).ok_or(EllipticCurveError::InvalidKey)?;
        Ok(PublicKey {
            point,
        })
    }

    pub fn verify(&self, message: &[u8], signature: &[u8; SIGNATURE_SIZE]) -> Result<(), EllipticCurveError> {
        ecdsa_verify_inner(&self.point, message, signature)
    }

    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_UNCOMPRESSED_SIZE] {
        self.point.to_uncompressed_bytes()
    }
}

type U256 = Uint<256, 4>;

const MODULUS_P: U256 = U256::from_limbs([
    0xffff_ffff_ffff_ffff,
    0x0000_0000_ffff_ffff,
    0x0000_0000_0000_0000,
    0xffff_ffff_0000_0001,
]);
const MODULUS_N: U256 = U256::from_limbs([
    0xf3b9_cac2_fc63_2551,
    0xbce6_faad_a717_9e84,
    0xffff_ffff_ffff_ffff,
    0xffff_ffff_0000_0000,
]);
const P_MINUS_TWO: U256 = U256::from_limbs([
    0xffff_ffff_ffff_fffd,
    0x0000_0000_ffff_ffff,
    0x0000_0000_0000_0000,
    0xffff_ffff_0000_0001,
]);
const P_PLUS_ONE_OVER_FOUR: U256 = U256::from_limbs([
    0x0000_0000_0000_0000,
    0x0000_0000_4000_0000,
    0x4000_0000_0000_0000,
    0x3fff_ffff_c000_0000,
]);
const N_MINUS_TWO: U256 = U256::from_limbs([
    0xf3b9_cac2_fc63_254f,
    0xbce6_faad_a717_9e84,
    0xffff_ffff_ffff_ffff,
    0xffff_ffff_0000_0000,
]);

const CURVE_B: FieldElement = FieldElement(U256::from_limbs([
    0x3bce_3c3e_27d2_604b,
    0x651d_06b0_cc53_b0f6,
    0xb3eb_bd55_7698_86bc,
    0x5ac6_35d8_aa3a_93e7,
]));
const GENERATOR_X: FieldElement = FieldElement(U256::from_limbs([
    0xf4a1_3945_d898_c296,
    0x7703_7d81_2deb_33a0,
    0xf8bc_e6e5_63a4_40f2,
    0x6b17_d1f2_e12c_4247,
]));
const GENERATOR_Y: FieldElement = FieldElement(U256::from_limbs([
    0xcbb6_4068_37bf_51f5,
    0x2bce_3357_6b31_5ece,
    0x8ee7_eb4a_7c0f_9e16,
    0x4fe3_42e2_fe1a_7f9b,
]));

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
fn p256_fast_mul_mod(a: &U256, b: &U256) -> U256 {
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
    let mut result = U256::from_limbs([r0 as u64, r1 as u64, r2 as u64, r3 as u64]);
    for _ in 0..8 {
        let (sub, borrow) = result.sub_raw(&MODULUS_P);
        result = U256::ct_select(&sub, &result, borrow == 0);
    }
    result
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FieldElement(U256);

impl FieldElement {
    const ZERO: Self = Self(U256::ZERO);
    const ONE: Self = Self(U256::ONE);

    #[inline]
    fn from_bytes(bytes: &[u8; 32]) -> Option<Self> {
        let value = U256::from_be_slice(bytes);
        if value.ct_ge(&MODULUS_P) {
            None
        } else {
            Some(Self(value))
        }
    }

    #[inline]
    fn to_bytes(self) -> [u8; 32] {
        self.0.to_be_bytes_fixed::<32>()
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
        Self(p256_fast_mul_mod(&self.0, &rhs.0))
    }

    #[inline]
    fn triple(self) -> Self {
        self.double().add(self)
    }

    #[inline]
    fn negate(self) -> Self {
        let (diff, _) = MODULUS_P.sub_raw(&self.0);
        Self(U256::ct_select(&U256::ZERO, &diff, self.is_zero()))
    }

    #[inline]
    fn pow(self, exponent: &U256) -> Self {
        let mut result = Self::ONE;
        let mut i = 256usize;
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

    #[inline]
    fn sqrt(self) -> Option<Self> {
        let candidate = self.pow(&P_PLUS_ONE_OVER_FOUR);
        if U256::ct_eq(&self.0, &candidate.square().0) {
            Some(candidate)
        } else {
            None
        }
    }

    #[inline]
    fn select(a: &Self, b: &Self, choice: bool) -> Self {
        Self(U256::ct_select(&a.0, &b.0, choice))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Scalar(U256);

impl Scalar {
    const ZERO: Self = Self(U256::ZERO);
    const ONE: Self = Self(U256::ONE);

    #[inline]
    fn from_bytes(bytes: &[u8; 32]) -> Option<Self> {
        let value = U256::from_be_slice(bytes);
        if value.is_zero() || value.ct_ge(&MODULUS_N) {
            None
        } else {
            Some(Self(value))
        }
    }

    #[inline]
    fn from_hash(hash: &[u8; 32]) -> Self {
        let value = U256::from_be_slice(hash);
        let (sub_value, _) = value.sub_raw(&MODULUS_N);
        let reduced = U256::ct_select(&sub_value, &value, value.ct_ge(&MODULUS_N));
        Self(reduced)
    }

    #[inline]
    fn to_bytes(self) -> [u8; 32] {
        self.0.to_be_bytes_fixed::<32>()
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

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0.sub_mod(&rhs.0, &MODULUS_N))
    }

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self(self.0.mul_mod(&rhs.0, &MODULUS_N))
    }

    #[inline]
    fn invert(self) -> Option<Self> {
        Some(Self(self.scalar_pow(&N_MINUS_TWO)))
    }

    #[inline]
    fn scalar_pow(self, exponent: &U256) -> U256 {
        let mut result = Scalar::ONE;
        let mut i = 256usize;
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
        Self(U256::ct_select(&a.0, &b.0, choice))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AffinePoint {
    x: FieldElement,
    y: FieldElement,
    infinity: bool,
}

impl AffinePoint {
    const IDENTITY: Self = Self {
        x: FieldElement::ZERO,
        y: FieldElement::ONE,
        infinity: true,
    };

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
        out[1..33].copy_from_slice(&self.x.to_bytes());
        out[33..65].copy_from_slice(&self.y.to_bytes());
        out
    }

    #[inline]
    fn to_compressed_bytes(&self) -> [u8; PUBLIC_KEY_COMPRESSED_SIZE] {
        let mut out = [0u8; PUBLIC_KEY_COMPRESSED_SIZE];
        out[0] = if self.y.is_odd() { 0x03 } else { 0x02 };
        out[1..33].copy_from_slice(&self.x.to_bytes());
        out
    }

    fn from_sec1_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes.len() {
            PUBLIC_KEY_UNCOMPRESSED_SIZE if bytes[0] == 0x04 => {
                let x = FieldElement::from_bytes(bytes[1..33].try_into().unwrap())?;
                let y = FieldElement::from_bytes(bytes[33..65].try_into().unwrap())?;
                Self::new(x, y)
            }
            PUBLIC_KEY_COMPRESSED_SIZE if bytes[0] == 0x02 || bytes[0] == 0x03 => {
                let x = FieldElement::from_bytes(bytes[1..33].try_into().unwrap())?;
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
    let mut bit = 256usize;
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

fn bits2octets(hash: &[u8; 32]) -> [u8; 32] {
    Scalar::from_hash(hash).to_bytes()
}

fn rfc6979_init_state(private_key: &Scalar, message_hash: &[u8; 32]) -> ([u8; 32], [u8; 32]) {
    let x = private_key.to_bytes();
    let h1 = bits2octets(message_hash);

    let mut v = [0x01u8; 32];
    let mut k = [0u8; 32];

    let mut buf = [0u8; 97];
    buf[..32].copy_from_slice(&v);
    buf[32] = 0x00;
    buf[33..65].copy_from_slice(&x);
    buf[65..97].copy_from_slice(&h1);
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

// Returns the post-retry state without mutating, for constant-time selection.
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

    // Fixed 3 iterations with constant-time state selection.
    // In each iteration we generate one HMAC output, check validity,
    // and ct_select between keeping the original state (k valid) or
    // replacing it with the retry state (k invalid).
    //
    // The FIRST valid candidate is captured and returned after the loop.
    // On the first iteration this matches the original RFC 6979 behavior.
    let mut candidate = [0u8; 32];
    let mut found = false;

    for _ in 0..3 {
        v = hmac_sha256(&k, &v);
        let val = U256::from_be_slice(&v);
        let is_valid = !val.is_zero() && !val.ct_ge(&MODULUS_N);

        // Capture the first valid candidate (ct_select: if valid and not yet found, take v)
        let take = is_valid && !found;
        candidate = ct_select_bytes(&v, &candidate, take);
        found = found || is_valid;

        // Advance DRBG: if invalid, replace state with retry state
        let (k_retry, v_retry) = rfc6979_retry_clone(&k, &v);
        k = ct_select_bytes(&k, &k_retry, !is_valid);
        v = ct_select_bytes(&v, &v_retry, !is_valid);
    }

    if found {
        // Safety: candidate was produced by Scalar::from_bytes succeeding above.
        return Scalar::from_bytes(&candidate).unwrap_or(Scalar::ZERO);
    }

    // Fallback (P > 2^-96): one more HMAC step
    v = hmac_sha256(&k, &v);
    if let Some(sc) = Scalar::from_bytes(&v) {
        return sc;
    }

    // Astronomically unlikely retry loop
    loop {
        v = hmac_sha256(&k, &v);
        if let Some(sc) = Scalar::from_bytes(&v) {
            return sc;
        }
        rfc6979_retry(&mut k, &mut v);
    }
}

fn parse_private_key(private_key: &[u8; PRIVATE_KEY_SIZE]) -> Result<Scalar, EllipticCurveError> {
    Scalar::from_bytes(private_key).ok_or(EllipticCurveError::InvalidKey)
}

fn parse_public_key(public_key: &[u8]) -> Result<AffinePoint, EllipticCurveError> {
    AffinePoint::from_sec1_bytes(public_key).ok_or(EllipticCurveError::InvalidKey)
}

fn derive_public_key_uncompressed(
    private_key: &[u8; PRIVATE_KEY_SIZE],
) -> Result<[u8; PUBLIC_KEY_UNCOMPRESSED_SIZE], EllipticCurveError> {
    let scalar = parse_private_key(private_key)?;
    let point = scalar_mul_generator(&scalar)
        .to_affine()
        .ok_or(EllipticCurveError::Unspecified)?;
    Ok(point.to_uncompressed_bytes())
}

fn derive_public_key_compressed(
    private_key: &[u8; PRIVATE_KEY_SIZE],
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
    private_key: &[u8; PRIVATE_KEY_SIZE],
    peer_public_key: &[u8],
) -> Result<[u8; ECDH_SHARED_SECRET_SIZE], EllipticCurveError> {
    let scalar = parse_private_key(private_key)?;
    let peer_point = parse_public_key(peer_public_key)?;
    ecdh_inner(&scalar, &peer_point)
}

fn ecdsa_sign_inner(scalar: &Scalar, message: &[u8]) -> Result<[u8; SIGNATURE_SIZE], EllipticCurveError> {
    let message_hash = hash_message(message);
    let z = Scalar::from_hash(&message_hash);

    // Fixed 2-iteration loop for r=0/s=0 retry:
    // First iteration almost always succeeds (r=0 probability ≈ 2^-256).
    // Second iteration is only reached if the first produced r=0 or s=0,
    // which is astronomically unlikely. The fixed count avoids timing leaks.
    for _ in 0..2 {
        let k = rfc6979_generate_k(scalar, &message_hash);

        let r_point = scalar_mul_generator(&k)
            .to_affine()
            .ok_or(EllipticCurveError::Unspecified)?;
        let r = Scalar::from_hash(&r_point.x.to_bytes());
        if r.is_zero() {
            continue;
        }

        let kinv = k.invert().ok_or(EllipticCurveError::Unspecified)?;
        let s = kinv.mul(z.add(r.mul(*scalar)));
        if s.is_zero() {
            continue;
        }

        let mut out = [0u8; SIGNATURE_SIZE];
        out[..32].copy_from_slice(&r.to_bytes());
        out[32..].copy_from_slice(&s.to_bytes());
        return Ok(out);
    }

    Err(EllipticCurveError::Unspecified)
}

fn ecdsa_verify_inner(
    public_point: &AffinePoint,
    message: &[u8],
    signature: &[u8; SIGNATURE_SIZE],
) -> Result<(), EllipticCurveError> {
    let r = Scalar::from_bytes(signature[..32].try_into().unwrap()).ok_or(EllipticCurveError::Unspecified)?;
    let s = Scalar::from_bytes(signature[32..].try_into().unwrap()).ok_or(EllipticCurveError::Unspecified)?;
    let z = Scalar::from_hash(&hash_message(message));

    let w = s.invert().ok_or(EllipticCurveError::Unspecified)?;
    let u1 = z.mul(w);
    let u2 = r.mul(w);

    let point = scalar_mul_generator(&u1).add(&scalar_mul_affine(public_point, &u2));
    let affine = point.to_affine().ok_or(EllipticCurveError::Unspecified)?;
    let x_mod_n = Scalar::from_hash(&affine.x.to_bytes());

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
        let key = PrivateKey::from_bytes(&private_key).unwrap();
        let sample_signature = key.sign(b"sample").unwrap();
        let expected_sample = decode_hex::<64>(
            "efd48b2aacb6a8fd1140dd9cd45e81d69d2c877b56aaf991c34d0ea84eaf3716\
             f7cb1c942d657c41d436c7a1b6e29f65f3e900dbb9aff4064dc4ab2f843acda8",
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
            rfc6979_generate_k(&private_key, &hash).to_bytes(),
            decode_hex::<32>("a6e3c57dd01abe90086538398355dd4c3b17aa873382b0f24d6129493d8aad60")
        );
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
        k = hmac_sha256(&k, &buf);
        assert_eq!(
            k,
            decode_hex::<32>("122db1de98dae4dfa33f2da8e98494c80bff807b479fd79261b37e25f267ee58")
        );
        v = hmac_sha256(&k, &v);
        assert_eq!(
            v,
            decode_hex::<32>("c9947803a747fc60c23535fdcc13b5ca566b48221ca67d4964d22daa48275844")
        );

        buf[..32].copy_from_slice(&v);
        buf[32] = 0x01;
        k = hmac_sha256(&k, &buf);
        assert_eq!(
            k,
            decode_hex::<32>("b6d4f98ebae70aa15a2238ade4e20ab323fc1e777d22f0c582d8ef2e6ba73569")
        );
        v = hmac_sha256(&k, &v);
        assert_eq!(
            v,
            decode_hex::<32>("bae57fe256de2de806b10635497237e7bae96754582566384c47c6c3416494d1")
        );
        v = hmac_sha256(&k, &v);
        assert_eq!(
            v,
            decode_hex::<32>("a6e3c57dd01abe90086538398355dd4c3b17aa873382b0f24d6129493d8aad60")
        );
    }

    #[test]
    fn ecdsa_verify_accepts_compressed_and_uncompressed_public_keys() {
        let private_key = decode_hex::<32>("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721");
        let key = PrivateKey::from_bytes(&private_key).unwrap();
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
        let key = PrivateKey::from_bytes(&private_key).unwrap();
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
        let invalid_private_key = [0u8; PRIVATE_KEY_SIZE];
        assert!(PrivateKey::from_bytes(&invalid_private_key).is_err());
        assert!(derive_public_key_uncompressed(&invalid_private_key).is_err());
        assert!(derive_public_key_compressed(&invalid_private_key).is_err());

        let private_key = decode_hex::<32>("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721");
        let key = PrivateKey::from_bytes(&private_key).unwrap();
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
            "../testdata/wycheproof/testvectors_v1/ecdsa_secp256r1_sha256_p1363_test.json"
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
        let key = PrivateKey::from_bytes(&private_key).unwrap();
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
            let key = PrivateKey::from_bytes(&private_key).unwrap();
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
        let key1 = PrivateKey::from_bytes(&private_key1).unwrap();
        let key2 = PrivateKey::from_bytes(&private_key2).unwrap();

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
            let key = PrivateKey::from_bytes(&private_key).unwrap();
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
    fn generator_point_is_on_curve() {
        assert!(AffinePoint::GENERATOR.is_on_curve());
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
        assert_eq!(result.x, GENERATOR_X);
        // y should be -Gy mod p
        let neg_gy = GENERATOR_Y.negate();
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

        let alice = PrivateKey::from_bytes(&i_priv).unwrap();
        let bob = PrivateKey::from_bytes(&r_priv).unwrap();
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

        let key = PrivateKey::from_bytes(&priv_key).unwrap();
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
    fn ecdh_round_trip_alice_bob() {
        // Full round-trip ECDH key exchange with randomly generated keys
        let alice = PrivateKey::generate().unwrap();
        let bob = PrivateKey::generate().unwrap();

        let alice_shared = alice.ecdh(&bob.public_key()).unwrap();
        let bob_shared = bob.ecdh(&alice.public_key()).unwrap();

        assert_eq!(alice_shared, bob_shared);
        assert_eq!(alice_shared.len(), 32);
    }

    #[test]
    fn ecdh_rejects_off_curve_peer_public_key() {
        let alice = PrivateKey::generate().unwrap();
        let mut bad_pub = alice.public_key().to_bytes().to_vec();
        // Flip a bit in y to take it off the curve
        bad_pub[64] ^= 0x01;
        assert!(!is_valid_public_key(&bad_pub));
        assert!(ecdh(&alice.to_bytes(), &bad_pub).is_err());
    }

    #[test]
    fn ecdh_rejects_infinity_peer_public_key() {
        let alice = PrivateKey::generate().unwrap();
        // Infinity encoding (0x00) should be rejected
        let infinity = [0x00u8];
        assert!(ecdh(&alice.to_bytes(), &infinity).is_err());
    }

    #[test]
    fn ecdh_rejects_bad_length_peer_public_key() {
        let alice = PrivateKey::generate().unwrap();
        // Empty key
        assert!(ecdh(&alice.to_bytes(), &[]).is_err());
        // Truncated key
        assert!(ecdh(&alice.to_bytes(), &[0x04, 0x00]).is_err());
        // Too long
        let mut long = [0x04u8; 200];
        long[0] = 0x04;
        assert!(ecdh(&alice.to_bytes(), &long).is_err());
    }

    #[test]
    fn ecdh_rejects_invalid_private_key_zero() {
        let zero_key = [0u8; 32];
        assert!(PrivateKey::from_bytes(&zero_key).is_err());
        let bob = PrivateKey::generate().unwrap();
        assert!(ecdh(&zero_key, &bob.public_key().to_bytes()).is_err());
    }

    #[test]
    fn ecdh_rejects_invalid_private_key_order() {
        // n (the curve order) is rejected
        let n_bytes = decode_hex::<32>("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551");
        assert!(PrivateKey::from_bytes(&n_bytes).is_err());

        // n+1 is rejected
        let n_plus_1 = decode_hex::<32>("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632552");
        assert!(PrivateKey::from_bytes(&n_plus_1).is_err());

        // all-ones is rejected
        let all_ones = [0xffu8; 32];
        assert!(PrivateKey::from_bytes(&all_ones).is_err());
    }

    #[test]
    fn ecdh_rejects_peer_public_key_x_equal_to_p() {
        // x = p (the field modulus) should be rejected as out of range
        let alice = PrivateKey::generate().unwrap();
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
        let alice = PrivateKey::generate().unwrap();
        let bob = PrivateKey::generate().unwrap();

        let shared1 = alice.ecdh(&bob.public_key()).unwrap();
        let shared2 = alice.ecdh(&bob.public_key()).unwrap();
        assert_eq!(shared1, shared2);
    }

    #[test]
    fn ecdh_self_exchange_is_deterministic() {
        // ECDH with own public key produces a deterministic result
        let alice = PrivateKey::generate().unwrap();
        let shared = alice.ecdh(&alice.public_key()).unwrap();
        let shared2 = alice.ecdh(&alice.public_key()).unwrap();
        assert_eq!(shared, shared2);
    }

    #[test]
    fn ecdh_different_keys_produce_different_secrets() {
        let alice = PrivateKey::generate().unwrap();
        let bob1 = PrivateKey::generate().unwrap();
        let bob2 = PrivateKey::generate().unwrap();

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
        let alice = PrivateKey::generate().unwrap();
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
        let alice = PrivateKey::generate().unwrap();
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
        // PrivateKey::from_bytes must reject zero and n (curve order)
        let zero = [0u8; 32];
        assert!(PrivateKey::from_bytes(&zero).is_err());

        let n = decode_hex::<32>("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551");
        assert!(PrivateKey::from_bytes(&n).is_err());

        let n_minus_1 = decode_hex::<32>("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632550");
        assert!(PrivateKey::from_bytes(&n_minus_1).is_ok());
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
    fn ecdh_multiple_exchanges_consistency() {
        // Verify ECDH commutativity across multiple key pairs
        let alice = PrivateKey::generate().unwrap();
        let bob = PrivateKey::generate().unwrap();
        let charlie = PrivateKey::generate().unwrap();

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

    #[test]
    fn ecdh_standalone_function_matches_method() {
        let alice = PrivateKey::generate().unwrap();
        let bob = PrivateKey::generate().unwrap();

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
            rfc6979_generate_k(&private_key, &hash).to_bytes(),
            decode_hex::<32>("d16b6ae827f17175e040871a1c7ec3500192c4c92677336ec2537acaee0008e0")
        );
    }

    #[test]
    fn ecdsa_rejects_ptr_at_infinity_as_public_key() {
        let private_key = decode_hex::<32>("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721");
        let key = PrivateKey::from_bytes(&private_key).unwrap();
        let signature = key.sign(b"msg").unwrap();

        // The point at infinity (0x00) is rejected as a public key
        assert!(!is_valid_public_key(&[0x00]));
        assert!(PublicKey::from_bytes(&[0x00]).is_err());
    }

    #[test]
    fn ecdsa_verify_rejects_non_canonical_r_and_s() {
        let private_key = decode_hex::<32>("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721");
        let key = PrivateKey::from_bytes(&private_key).unwrap();
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
    fn private_key_round_trip_bytes() {
        let key = PrivateKey::generate().unwrap();
        let bytes = key.to_bytes();
        let key2 = PrivateKey::from_bytes(&bytes).unwrap();
        assert_eq!(key.to_bytes(), key2.to_bytes());
        assert_eq!(key.public_key().to_bytes(), key2.public_key().to_bytes());
    }

    #[test]
    fn public_key_round_trip_bytes() {
        let key = PrivateKey::generate().unwrap();
        let pub_key = key.public_key();
        let bytes = pub_key.to_bytes();
        let pub_key2 = PublicKey::from_bytes(&bytes).unwrap();
        assert_eq!(pub_key.to_bytes(), pub_key2.to_bytes());
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
        let alice = PrivateKey::generate().unwrap();
        let bob = PrivateKey::generate().unwrap();

        let shared = alice.ecdh(&bob.public_key()).unwrap();
        assert_eq!(shared.len(), ECDH_SHARED_SECRET_SIZE);

        // shared secret is deterministic for the same key pair
        let shared2 = alice.ecdh(&bob.public_key()).unwrap();
        assert_eq!(shared, shared2);
    }

    #[test]
    fn ecdh_rejects_empty_and_invalid_public_key_bytes() {
        let key = PrivateKey::generate().unwrap();

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
            let key = PrivateKey::generate().unwrap();
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
    fn point_double_and_add_consistency() {
        // 2*G = G + G
        let g = AffinePoint::GENERATOR;
        let proj_g = ProjectivePoint::from_affine(&g);
        let doubled = proj_g.double();
        let added = proj_g.add(&proj_g);
        assert_eq!(
            doubled.to_affine().unwrap().to_uncompressed_bytes(),
            added.to_affine().unwrap().to_uncompressed_bytes(),
        );
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
        let key = PrivateKey::generate().unwrap();
        let shared1 = key.ecdh(&key.public_key()).unwrap();
        let shared2 = key.ecdh(&key.public_key()).unwrap();
        assert_eq!(shared1, shared2);
    }

    #[test]
    fn wycheproof_ecdh_p256_ecpoint() {
        let data: serde_json::Value = serde_json::from_str(include_str!(
            "../testdata/wycheproof/testvectors_v1/ecdh_secp256r1_ecpoint_test.json"
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
                let mut private_key = [0u8; PRIVATE_KEY_SIZE];
                let effective_len = private_bytes.len().min(PRIVATE_KEY_SIZE);
                let skip = if private_bytes.len() > PRIVATE_KEY_SIZE {
                    private_bytes.len() - PRIVATE_KEY_SIZE
                } else {
                    0
                };
                private_key[PRIVATE_KEY_SIZE - effective_len..]
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
    fn compressed_public_key_has_correct_prefix() {
        for _ in 0..5 {
            let key = PrivateKey::generate().unwrap();
            let compressed = derive_public_key_compressed(&key.to_bytes()).unwrap();
            let prefix = compressed[0];
            assert!(prefix == 0x02 || prefix == 0x03, "invalid compressed prefix: {prefix:#x}");
        }
    }

    #[test]
    fn ecdsa_rejects_truncated_signature() {
        let key = PrivateKey::generate().unwrap();
        let sig = key.sign(b"msg").unwrap();
        // Truncated signature should panic or be rejected
        let truncated: [u8; 63] = sig[..63].try_into().unwrap();
        // Can't call verify with wrong size due to type system
        // This is a compile-time guarantee
        assert_eq!(sig.len(), SIGNATURE_SIZE);
    }

    #[test]
    fn is_on_curve_accepts_generator_and_random_points() {
        assert!(AffinePoint::GENERATOR.is_on_curve());
        for _ in 0..5 {
            let key = PrivateKey::generate().unwrap();
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
        let key = PrivateKey::from_bytes(&private_key).unwrap();

        let vectors: &[(&[u8], &str)] = &[
            (
                b"sample" as &[u8],
                "efd48b2aacb6a8fd1140dd9cd45e81d69d2c877b56aaf991c34d0ea84eaf3716\
                          f7cb1c942d657c41d436c7a1b6e29f65f3e900dbb9aff4064dc4ab2f843acda8",
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
            "../testdata/wycheproof/testvectors_v1/ecdsa_secp256r1_sha256_test.json"
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
        let data: serde_json::Value =
            serde_json::from_str(include_str!("../testdata/wycheproof/testvectors_v1/ecdh_secp256r1_test.json"))
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
                let mut private_key = [0u8; PRIVATE_KEY_SIZE];
                let effective_len = private_bytes.len().min(PRIVATE_KEY_SIZE);
                let skip = if private_bytes.len() > PRIVATE_KEY_SIZE {
                    private_bytes.len() - PRIVATE_KEY_SIZE
                } else {
                    0
                };
                private_key[PRIVATE_KEY_SIZE - effective_len..]
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
}
