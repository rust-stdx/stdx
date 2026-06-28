use big_number::{Uint, mac};

use crate::{EllipticCurveError, Hasher, hmac::Hmac, sha2::Sha384};

/// Size of a P-384 private key in bytes (48 bytes).
pub const PRIVATE_KEY_SIZE: usize = 48;
/// Size of a compressed P-384 public key in bytes (49 bytes, includes 0x02/0x03 prefix).
pub const PUBLIC_KEY_COMPRESSED_SIZE: usize = 49;
/// Size of an uncompressed P-384 public key in bytes (97 bytes, includes 0x04 prefix).
pub const PUBLIC_KEY_UNCOMPRESSED_SIZE: usize = 97;
/// Size of a P-384 ECDSA signature in bytes (96 bytes, r || s).
pub const SIGNATURE_SIZE: usize = 96;
/// Size of the raw ECDH shared secret in bytes (48 bytes). **Must not** be used directly
/// as an encryption key; apply a KDF first.
pub const ECDH_SHARED_SECRET_SIZE: usize = 48;

/// P-384 (secp384r1) ECDSA private key.
///
/// Supports signing and ECDH key agreement.
///
/// # Signing
///
/// ```ignore
/// use crypto::p384::PrivateKey;
///
/// let key = PrivateKey::generate().unwrap();
/// let signature = key.sign(b"message").unwrap();
/// ```
///
/// # ECDH key exchange
///
/// ```ignore
/// use crypto::p384::PrivateKey;
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

/// P-384 (secp384r1) ECDSA public key.
///
/// Supports signature verification and ECDH key agreement.
///
/// # Verification
///
/// ```ignore
/// use crypto::p384::PrivateKey;
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

type U384 = Uint<384, 6>;

const MODULUS_P: U384 = U384::from_limbs([
    0x00000000ffffffff,
    0xffffffff00000000,
    0xfffffffffffffffe,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
]);
const MODULUS_N: U384 = U384::from_limbs([
    0xecec196accc52973,
    0x581a0db248b0a77a,
    0xc7634d81f4372ddf,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
]);
const P_MINUS_TWO: U384 = U384::from_limbs([
    0x00000000fffffffd,
    0xffffffff00000000,
    0xfffffffffffffffe,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
]);
const P_PLUS_ONE_OVER_FOUR: U384 = U384::from_limbs([
    0x0000000040000000,
    0xbfffffffc0000000,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0x3fffffffffffffff,
]);
const N_MINUS_TWO: U384 = U384::from_limbs([
    0xecec196accc52971,
    0x581a0db248b0a77a,
    0xc7634d81f4372ddf,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
]);

const CURVE_B: FieldElement = FieldElement(U384::from_limbs([
    0x2a85c8edd3ec2aef,
    0xc656398d8a2ed19d,
    0x0314088f5013875a,
    0x181d9c6efe814112,
    0x988e056be3f82d19,
    0xb3312fa7e23ee7e4,
]));
const GENERATOR_X: FieldElement = FieldElement(U384::from_limbs([
    0x3a545e3872760ab7,
    0x5502f25dbf55296c,
    0x59f741e082542a38,
    0x6e1d3b628ba79b98,
    0x8eb1c71ef320ad74,
    0xaa87ca22be8b0537,
]));
const GENERATOR_Y: FieldElement = FieldElement(U384::from_limbs([
    0x7a431d7c90ea0e5f,
    0x0a60b1ce1d7e819d,
    0xe9da3113b5f0b8c0,
    0xf8f41dbd289a147c,
    0x5d9e98bf9292dc29,
    0x3617de4a96262c6f,
]));

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
fn p384_fast_mul_mod(a: &U384, b: &U384) -> U384 {
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
    let mut result = U384::from_limbs([r0 as u64, r1 as u64, r2 as u64, r3 as u64, r4 as u64, r5 as u64]);
    for _ in 0..8 {
        let (sub, borrow) = result.sub_raw(&MODULUS_P);
        result = U384::ct_select(&sub, &result, borrow == 0);
    }
    result
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FieldElement(U384);

impl FieldElement {
    const ZERO: Self = Self(U384::ZERO);
    const ONE: Self = Self(U384::ONE);

    #[inline]
    fn from_bytes(bytes: &[u8; 48]) -> Option<Self> {
        let value = U384::from_be_slice(bytes);
        if value.ct_ge(&MODULUS_P) {
            None
        } else {
            Some(Self(value))
        }
    }

    #[inline]
    fn to_bytes(self) -> [u8; 48] {
        self.0.to_be_bytes_fixed::<48>()
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
        Self(p384_fast_mul_mod(&self.0, &rhs.0))
    }

    #[inline]
    fn triple(self) -> Self {
        self.double().add(self)
    }

    #[inline]
    fn negate(self) -> Self {
        let (diff, _) = MODULUS_P.sub_raw(&self.0);
        Self(U384::ct_select(&U384::ZERO, &diff, self.is_zero()))
    }

    #[inline]
    fn pow(self, exponent: &U384) -> Self {
        let mut result = Self::ONE;
        let mut i = 384usize;
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
        if U384::ct_eq(&self.0, &candidate.square().0) {
            Some(candidate)
        } else {
            None
        }
    }

    #[inline]
    fn select(a: &Self, b: &Self, choice: bool) -> Self {
        Self(U384::ct_select(&a.0, &b.0, choice))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Scalar(U384);

impl Scalar {
    const ZERO: Self = Self(U384::ZERO);
    const ONE: Self = Self(U384::ONE);

    #[inline]
    fn from_bytes(bytes: &[u8; 48]) -> Option<Self> {
        let value = U384::from_be_slice(bytes);
        if value.is_zero() || value.ct_ge(&MODULUS_N) {
            None
        } else {
            Some(Self(value))
        }
    }

    #[inline]
    fn from_hash(hash: &[u8; 48]) -> Self {
        let value = U384::from_be_slice(hash);
        let (sub_value, _) = value.sub_raw(&MODULUS_N);
        let reduced = U384::ct_select(&sub_value, &value, value.ct_ge(&MODULUS_N));
        Self(reduced)
    }

    #[inline]
    fn to_bytes(self) -> [u8; 48] {
        self.0.to_be_bytes_fixed::<48>()
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
    fn scalar_pow(self, exponent: &U384) -> U384 {
        let mut result = Scalar::ONE;
        let mut i = 384usize;
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
        Self(U384::ct_select(&a.0, &b.0, choice))
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
        out[1..49].copy_from_slice(&self.x.to_bytes());
        out[49..97].copy_from_slice(&self.y.to_bytes());
        out
    }

    #[inline]
    fn to_compressed_bytes(&self) -> [u8; PUBLIC_KEY_COMPRESSED_SIZE] {
        let mut out = [0u8; PUBLIC_KEY_COMPRESSED_SIZE];
        out[0] = if self.y.is_odd() { 0x03 } else { 0x02 };
        out[1..49].copy_from_slice(&self.x.to_bytes());
        out
    }

    fn from_sec1_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes.len() {
            PUBLIC_KEY_UNCOMPRESSED_SIZE if bytes[0] == 0x04 => {
                let x = FieldElement::from_bytes(bytes[1..49].try_into().unwrap())?;
                let y = FieldElement::from_bytes(bytes[49..97].try_into().unwrap())?;
                Self::new(x, y)
            }
            PUBLIC_KEY_COMPRESSED_SIZE if bytes[0] == 0x02 || bytes[0] == 0x03 => {
                let x = FieldElement::from_bytes(bytes[1..49].try_into().unwrap())?;
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
    let mut bit = 384usize;
    while bit > 0 {
        bit -= 1;
        acc = acc.double();
        let candidate = acc.add_mixed(base);
        acc = ProjectivePoint::select(&candidate, &acc, scalar.bit(bit));
    }
    acc
}

#[inline]
fn hash_message(message: &[u8]) -> [u8; 48] {
    let digest = Sha384::hash(message);
    digest.as_ref().try_into().unwrap()
}

#[inline]
fn hmac_sha384(key: &[u8], data: &[u8]) -> [u8; 48] {
    let mac = Hmac::<Sha384>::mac(key, data);
    mac.as_ref().try_into().unwrap()
}

fn bits2octets(hash: &[u8; 48]) -> [u8; 48] {
    Scalar::from_hash(hash).to_bytes()
}

fn rfc6979_init_state(private_key: &Scalar, message_hash: &[u8; 48]) -> ([u8; 48], [u8; 48]) {
    let x = private_key.to_bytes();
    let h1 = bits2octets(message_hash);

    let mut v = [0x01u8; 48];
    let mut k = [0u8; 48];

    let mut buf = [0u8; 145];
    buf[..48].copy_from_slice(&v);
    buf[48] = 0x00;
    buf[49..97].copy_from_slice(&x);
    buf[97..145].copy_from_slice(&h1);
    k = hmac_sha384(&k, &buf);
    v = hmac_sha384(&k, &v);

    buf[..48].copy_from_slice(&v);
    buf[48] = 0x01;
    k = hmac_sha384(&k, &buf);
    v = hmac_sha384(&k, &v);

    (k, v)
}

fn rfc6979_retry(k: &mut [u8; 48], v: &mut [u8; 48]) {
    let mut retry_buf = [0u8; 49];
    retry_buf[..48].copy_from_slice(v);
    retry_buf[48] = 0x00;
    *k = hmac_sha384(k, &retry_buf);
    *v = hmac_sha384(k, v);
}

fn rfc6979_retry_clone(k: &[u8; 48], v: &[u8; 48]) -> ([u8; 48], [u8; 48]) {
    let mut retry_buf = [0u8; 49];
    retry_buf[..48].copy_from_slice(v);
    retry_buf[48] = 0x00;
    let k_new = hmac_sha384(k, &retry_buf);
    let v_new = hmac_sha384(&k_new, v);
    (k_new, v_new)
}

fn ct_select_bytes<const N: usize>(a: &[u8; N], b: &[u8; N], choice: bool) -> [u8; N] {
    let mask = (choice as u8).wrapping_neg();
    let mut out = [0u8; N];
    for i in 0..N {
        out[i] = (a[i] & mask) | (b[i] & !mask);
    }
    out
}

fn rfc6979_generate_k(private_key: &Scalar, message_hash: &[u8; 48]) -> Scalar {
    let (mut k, mut v) = rfc6979_init_state(private_key, message_hash);

    let mut candidate = [0u8; 48];
    let mut found = false;

    for _ in 0..3 {
        v = hmac_sha384(&k, &v);
        let val = U384::from_be_slice(&v);
        let is_valid = !val.is_zero() && !val.ct_ge(&MODULUS_N);

        let take = is_valid && !found;
        candidate = ct_select_bytes(&v, &candidate, take);
        found = found || is_valid;

        let (k_retry, v_retry) = rfc6979_retry_clone(&k, &v);
        k = ct_select_bytes(&k, &k_retry, !is_valid);
        v = ct_select_bytes(&v, &v_retry, !is_valid);
    }

    if found {
        return Scalar::from_bytes(&candidate).unwrap_or(Scalar::ZERO);
    }

    v = hmac_sha384(&k, &v);
    if let Some(sc) = Scalar::from_bytes(&v) {
        return sc;
    }

    loop {
        v = hmac_sha384(&k, &v);
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
        out[..48].copy_from_slice(&r.to_bytes());
        out[48..].copy_from_slice(&s.to_bytes());
        return Ok(out);
    }

    Err(EllipticCurveError::Unspecified)
}

fn ecdsa_verify_inner(
    public_point: &AffinePoint,
    message: &[u8],
    signature: &[u8; SIGNATURE_SIZE],
) -> Result<(), EllipticCurveError> {
    let r = Scalar::from_bytes(signature[..48].try_into().unwrap()).ok_or(EllipticCurveError::Unspecified)?;
    let s = Scalar::from_bytes(signature[48..].try_into().unwrap()).ok_or(EllipticCurveError::Unspecified)?;
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
        let key = PrivateKey::from_bytes(&private_key).unwrap();
        let uncompressed = key.public_key();
        let compressed = derive_public_key_compressed(&private_key).unwrap();
        let signature = key.sign(b"sample").unwrap();

        assert!(uncompressed.verify(b"sample", &signature).is_ok());
        let point = AffinePoint::from_sec1_bytes(&compressed).unwrap();
        assert!(ecdsa_verify_inner(&point, b"sample", &signature).is_ok());
    }

    #[test]
    fn invalid_inputs_are_rejected() {
        let invalid_private_key = [0u8; PRIVATE_KEY_SIZE];
        assert!(PrivateKey::from_bytes(&invalid_private_key).is_err());
        assert!(derive_public_key_uncompressed(&invalid_private_key).is_err());
        assert!(derive_public_key_compressed(&invalid_private_key).is_err());

        let private_key = decode_hex::<48>(
            "6b9d3dad2e1b8c1c05b19875b6659f4de23c3b667bf297ba9aa47740787137d8\
             96d5724e4c70a825f872c9ea60d2edf5",
        );
        let key = PrivateKey::from_bytes(&private_key).unwrap();
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
            let mut wrong_msg = msg.to_vec();
            wrong_msg.push(0x42);
            assert!(pub_key.verify(&wrong_msg, &sig).is_err());
        }
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
        let private_key1 = decode_hex::<48>(
            "6b9d3dad2e1b8c1c05b19875b6659f4de23c3b667bf297ba9aa47740787137d8\
             96d5724e4c70a825f872c9ea60d2edf5",
        );
        let private_key2 = decode_hex::<48>(
            "0000000000000000000000000000000000000000000000000000000000000000\
             00000000000000000000000000000001",
        );
        let key1 = PrivateKey::from_bytes(&private_key1).unwrap();
        let key2 = PrivateKey::from_bytes(&private_key2).unwrap();

        let sig = key1.sign(b"message").unwrap();
        assert!(key2.public_key().verify(b"message", &sig).is_err());
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
            let key = PrivateKey::from_bytes(&private_key).unwrap();
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
    fn generator_point_is_on_curve() {
        assert!(AffinePoint::GENERATOR.is_on_curve());
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
        assert_eq!(result.x, GENERATOR_X);
        let neg_gy = GENERATOR_Y.negate();
        assert_eq!(result.y, neg_gy);
    }

    #[test]
    fn ecdh_round_trip_alice_bob() {
        let alice = PrivateKey::generate().unwrap();
        let bob = PrivateKey::generate().unwrap();

        let alice_shared = alice.ecdh(&bob.public_key()).unwrap();
        let bob_shared = bob.ecdh(&alice.public_key()).unwrap();

        assert_eq!(alice_shared, bob_shared);
        assert_eq!(alice_shared.len(), ECDH_SHARED_SECRET_SIZE);
    }

    #[test]
    fn ecdh_rejects_off_curve_peer_public_key() {
        let alice = PrivateKey::generate().unwrap();
        let mut bad_pub = alice.public_key().to_bytes().to_vec();
        bad_pub[96] ^= 0x01;
        assert!(!is_valid_public_key(&bad_pub));
        assert!(ecdh(&alice.to_bytes(), &bad_pub).is_err());
    }

    #[test]
    fn ecdh_rejects_infinity_peer_public_key() {
        let alice = PrivateKey::generate().unwrap();
        let infinity = [0x00u8];
        assert!(ecdh(&alice.to_bytes(), &infinity).is_err());
    }

    #[test]
    fn ecdh_rejects_bad_length_peer_public_key() {
        let alice = PrivateKey::generate().unwrap();
        assert!(ecdh(&alice.to_bytes(), &[]).is_err());
        assert!(ecdh(&alice.to_bytes(), &[0x04, 0x00]).is_err());
        let mut long = [0x04u8; 200];
        long[0] = 0x04;
        assert!(ecdh(&alice.to_bytes(), &long).is_err());
    }

    #[test]
    fn ecdh_rejects_invalid_private_key_zero() {
        let zero_key = [0u8; 48];
        assert!(PrivateKey::from_bytes(&zero_key).is_err());
        let bob = PrivateKey::generate().unwrap();
        assert!(ecdh(&zero_key, &bob.public_key().to_bytes()).is_err());
    }

    #[test]
    fn ecdh_multiple_exchanges_consistency() {
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

        assert_ne!(alice_bob, alice_charlie);
        assert_ne!(alice_bob, bob_charlie);
        assert_ne!(alice_charlie, bob_charlie);
    }

    #[test]
    fn ecdsa_rejects_non_canonical_r_and_s() {
        let key = PrivateKey::generate().unwrap();
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
        let key = PrivateKey::generate().unwrap();
        let pub_key = key.public_key();
        let sig = key.sign(b"message").unwrap();

        assert!(pub_key.verify(b"tampered", &sig).is_err());

        let mut bad_sig = sig;
        bad_sig[10] ^= 0x80;
        assert!(pub_key.verify(b"message", &bad_sig).is_err());
    }

    #[test]
    fn public_key_rejects_off_curve_point() {
        let key = PrivateKey::generate().unwrap();
        let mut off_curve = key.public_key().to_bytes();
        off_curve[96] ^= 0x01;
        assert!(!is_valid_public_key(&off_curve));
        assert!(PublicKey::from_bytes(&off_curve).is_err());
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
    fn point_double_and_add_consistency() {
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
    fn compressed_public_key_has_correct_prefix() {
        for _ in 0..5 {
            let key = PrivateKey::generate().unwrap();
            let compressed = derive_public_key_compressed(&key.to_bytes()).unwrap();
            let prefix = compressed[0];
            assert!(prefix == 0x02 || prefix == 0x03, "invalid compressed prefix: {prefix:#x}");
        }
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
    fn is_on_curve_accepts_generator_and_random_points() {
        assert!(AffinePoint::GENERATOR.is_on_curve());
        for _ in 0..5 {
            let key = PrivateKey::generate().unwrap();
            let pb = key.public_key().to_bytes();
            let pk = PublicKey::from_bytes(&pb).unwrap();
            let _ = pk;
        }
    }

    #[test]
    fn ecdh_with_self_is_consistent() {
        let key = PrivateKey::generate().unwrap();
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
            "../testdata/wycheproof/testvectors_v1/ecdsa_secp384r1_sha384_p1363_test.json"
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
            "../testdata/wycheproof/testvectors_v1/ecdsa_secp384r1_sha384_test.json"
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
            "../testdata/wycheproof/testvectors_v1/ecdh_secp384r1_ecpoint_test.json"
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
        let data: serde_json::Value =
            serde_json::from_str(include_str!("../testdata/wycheproof/testvectors_v1/ecdh_secp384r1_test.json"))
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
}
