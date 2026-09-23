//! Shared ML-DSA (FIPS 204) core, parameterized over the ML-DSA-44, ML-DSA-65
//! and ML-DSA-87 parameter sets.
//!
//! This module is an implementation detail of [`crate::mldsa`]; the public API
//! lives in the per-parameter-set modules (`mldsa44`, `mldsa65`, `mldsa87`).

use constant_time_eq::constant_time_eq;
#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{
    RandomError, Xof,
    sha3::{Shake128, Shake256},
};

/// Number of coefficients in a polynomial.
pub(crate) const N: usize = 256;
/// Size in bytes of a seed / private key.
pub(crate) const SEED_SIZE: usize = 32;
/// Maximum length in bytes of a context string.
pub(crate) const CONTEXT_MAX_LEN: usize = 255;

const Q: u32 = 8380417;
const D: u32 = 13;
const ONE: u32 = 4193792;
const MINUS_ONE: u32 = 4186625;
const RR: u32 = 2365951;
const QINV: u32 = 4236238847;
const N_INV: u32 = 16382;

const MAX_LAMBDA_OVER_4: usize = 64;
const MAX_W1_BYTES: usize = 8 * N * 6 / 8;
const MAX_POLYZ_BYTES: usize = 20 * N / 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MlDsaError {
    ContextTooLong,
    InvalidSignature,
    InvalidPublicKey,
    InvalidSignatureLength,
    Random(RandomError),
}

#[cfg(feature = "alloc")]
impl core::fmt::Display for MlDsaError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MlDsaError::ContextTooLong => write!(f, "context length exceeds 255 bytes"),
            MlDsaError::InvalidSignature => write!(f, "signature is not valid"),
            MlDsaError::InvalidPublicKey => write!(f, "public key is not valid"),
            MlDsaError::InvalidSignatureLength => write!(f, "signature length is not valid"),
            MlDsaError::Random(err) => write!(f, "{err}"),
        }
    }
}

impl From<RandomError> for MlDsaError {
    fn from(err: RandomError) -> Self {
        MlDsaError::Random(err)
    }
}

/// The parameter set constants that differ between ML-DSA-44, ML-DSA-65 and
/// ML-DSA-87.
///
/// The dimensions `k` and `l` are carried by const generics on the functions
/// that need them; everything else is stored here.
#[derive(Clone, Copy)]
pub(crate) struct MlDsaParams {
    /// Bound for the secret coefficients (2 or 4).
    pub(crate) eta: u32,
    /// Number of non-zero coefficients in the challenge polynomial (39/49/60).
    pub(crate) tau: usize,
    /// `tau * eta`.
    pub(crate) beta: u32,
    /// `1 << gamma1_bits`.
    pub(crate) gamma1: u32,
    /// `log2(gamma1)` (17 or 19).
    pub(crate) gamma1_bits: usize,
    /// `(q - 1) / gamma2_den`.
    pub(crate) gamma2: u32,
    /// Denominator of `gamma2` (32 or 88).
    pub(crate) gamma2_den: u32,
    /// Maximum number of hints (80/55/75).
    pub(crate) omega: usize,
    /// `lambda / 4` (32/48/64).
    pub(crate) lambda_over_4: usize,
    /// Encoded size of one `z` polynomial: `(gamma1_bits + 1) * N / 8`.
    pub(crate) polyz_bytes: usize,
    /// Encoded public key size in bytes.
    pub(crate) public_key_size: usize,
    /// Signature size in bytes.
    pub(crate) signature_size: usize,
}

pub(crate) const PARAMS_44: MlDsaParams = MlDsaParams {
    eta: 2,
    tau: 39,
    beta: 78,
    gamma1: 1 << 17,
    gamma1_bits: 17,
    gamma2: (Q - 1) / 88,
    gamma2_den: 88,
    omega: 80,
    lambda_over_4: 32,
    polyz_bytes: 18 * N / 8,
    public_key_size: 1312,
    signature_size: 2420,
};

pub(crate) const PARAMS_65: MlDsaParams = MlDsaParams {
    eta: 4,
    tau: 49,
    beta: 196,
    gamma1: 1 << 19,
    gamma1_bits: 19,
    gamma2: (Q - 1) / 32,
    gamma2_den: 32,
    omega: 55,
    lambda_over_4: 48,
    polyz_bytes: 20 * N / 8,
    public_key_size: 1952,
    signature_size: 3309,
};

pub(crate) const PARAMS_87: MlDsaParams = MlDsaParams {
    eta: 2,
    tau: 60,
    beta: 120,
    gamma1: 1 << 19,
    gamma1_bits: 19,
    gamma2: (Q - 1) / 32,
    gamma2_den: 32,
    omega: 75,
    lambda_over_4: 64,
    polyz_bytes: 20 * N / 8,
    public_key_size: 2592,
    signature_size: 4627,
};

type FieldElement = u32;

fn field_to_montgomery(a: u32) -> FieldElement {
    debug_assert!(a < Q);
    field_montgomery_mul(a, RR)
}

fn field_from_montgomery(a: FieldElement) -> u32 {
    field_montgomery_reduce(a as u64)
}

fn field_montgomery_reduce(x: u64) -> u32 {
    let t = (x as u32).wrapping_mul(QINV);
    let u = (x + (t as u64) * (Q as u64)) >> 32;
    field_reduce_once(u as u32)
}

fn field_montgomery_mul(a: FieldElement, b: FieldElement) -> FieldElement {
    field_montgomery_reduce(a as u64 * b as u64)
}

fn field_reduce_once(x: u32) -> FieldElement {
    let t = x.wrapping_sub(Q);
    let mask = ((t as i32) >> 31) as u32;
    t.wrapping_add(Q & mask)
}

fn field_add(a: FieldElement, b: FieldElement) -> FieldElement {
    field_reduce_once(a.wrapping_add(b))
}

fn field_sub(a: FieldElement, b: FieldElement) -> FieldElement {
    field_reduce_once(a.wrapping_sub(b).wrapping_add(Q))
}

fn field_sub_to_montgomery(a: u32, b: u32) -> FieldElement {
    let x = a.wrapping_sub(b).wrapping_add(Q);
    field_montgomery_mul(x, RR)
}

fn field_infinity_norm(r: FieldElement) -> u32 {
    let x = field_from_montgomery(r);
    let q_minus_x = Q - x;
    let half_q = Q / 2;
    let mask = ((half_q.wrapping_sub(x)) as i32 >> 31) as u32;
    (mask & q_minus_x) | (!mask & x)
}

fn field_centered_mod(r: FieldElement) -> i32 {
    let x = field_from_montgomery(r);
    let x = x as i32;
    let half_q = (Q / 2) as i32;
    let mask = ((half_q - x) >> 31) as i32;
    (mask & (x - Q as i32)) | (!mask & x)
}

fn power2round(r: FieldElement) -> (u16, FieldElement) {
    let rr = field_from_montgomery(r);
    let r1 = (rr + (1 << 12) - 1) >> 13;
    let r0 = field_sub_to_montgomery(rr, r1 << 13);
    (r1 as u16, r0)
}

fn highbits32(x: u32) -> u8 {
    let r1 = (x + 127) >> 7;
    let r1 = (r1 * 1025 + (1 << 21)) >> 22;
    (r1 & 0b1111) as u8
}

fn highbits88(x: u32) -> u8 {
    let r1 = (x + 127) >> 7;
    let r1 = (r1 * 11275 + (1 << 23)) >> 24;
    // r1 == 44 must map to 0; do it without a data-dependent branch.
    let d = r1 ^ 44;
    let not_eq = (d | d.wrapping_neg()) >> 31;
    (r1 * not_eq) as u8
}

fn highbits(x: u32, gamma2_den: u32) -> u8 {
    match gamma2_den {
        32 => highbits32(x),
        88 => highbits88(x),
        _ => unreachable!(),
    }
}

fn decompose32(r: FieldElement) -> (u8, i32) {
    let x = field_from_montgomery(r) as i32;
    let r1 = highbits32(x as u32);
    let r0 = x - (r1 as i32) * 2 * (Q as i32 - 1) / 32;
    let half_q = (Q / 2) as i32;
    let mask = ((half_q - r0) >> 31) as i32;
    let r0 = (mask & (r0 - Q as i32)) | (!mask & r0);
    (r1, r0)
}

fn decompose88(r: FieldElement) -> (u8, i32) {
    let x = field_from_montgomery(r) as i32;
    let r1 = highbits88(x as u32);
    let r0 = x - (r1 as i32) * 2 * (Q as i32 - 1) / 88;
    let half_q = (Q / 2) as i32;
    let mask = ((half_q - r0) >> 31) as i32;
    let r0 = (mask & (r0 - Q as i32)) | (!mask & r0);
    (r1, r0)
}

fn decompose(r: FieldElement, gamma2_den: u32) -> (u8, i32) {
    match gamma2_den {
        32 => decompose32(r),
        88 => decompose88(r),
        _ => unreachable!(),
    }
}

fn make_hint32(ct0: FieldElement, w: FieldElement, cs2: FieldElement) -> u8 {
    let r_plus_z = field_sub(w, cs2);
    let v1 = highbits32(field_from_montgomery(r_plus_z));
    let r = field_add(r_plus_z, ct0);
    let r1 = highbits32(field_from_montgomery(r));
    (v1 != r1) as u8
}

fn make_hint88(ct0: FieldElement, w: FieldElement, cs2: FieldElement) -> u8 {
    let r_plus_z = field_sub(w, cs2);
    let v1 = highbits88(field_from_montgomery(r_plus_z));
    let r = field_add(r_plus_z, ct0);
    let r1 = highbits88(field_from_montgomery(r));
    (v1 != r1) as u8
}

fn make_hint(ct0: FieldElement, w: FieldElement, cs2: FieldElement, gamma2_den: u32) -> u8 {
    match gamma2_den {
        32 => make_hint32(ct0, w, cs2),
        88 => make_hint88(ct0, w, cs2),
        _ => unreachable!(),
    }
}

fn use_hint32(r: FieldElement, hint: u8) -> u8 {
    let (r1, r0) = decompose32(r);
    if hint == 0 {
        return r1;
    }
    let r0_gt_0 = !(r0.wrapping_sub(1) >> 31) as u8;
    let r1_plus = r1.wrapping_add(1) & 0x0F;
    let r1_minus = r1.wrapping_sub(1) & 0x0F;
    (r0_gt_0 & r1_plus) | ((!r0_gt_0) & r1_minus)
}

fn use_hint88(r: FieldElement, hint: u8) -> u8 {
    const M: u8 = 44;
    let (mut r1, r0) = decompose88(r);
    if hint == 0 {
        return r1;
    }
    if r0 > 0 {
        if r1 == M - 1 {
            r1 = 0;
        } else {
            r1 += 1;
        }
    } else if r1 == 0 {
        r1 = M - 1;
    } else {
        r1 -= 1;
    }
    r1
}

fn use_hint(r: FieldElement, hint: u8, gamma2_den: u32) -> u8 {
    match gamma2_den {
        32 => use_hint32(r, hint),
        88 => use_hint88(r, hint),
        _ => unreachable!(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
struct Poly {
    coeffs: [FieldElement; N],
}

impl Default for Poly {
    fn default() -> Self {
        Self {
            coeffs: [0u32; N],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
struct NttPoly {
    coeffs: [FieldElement; N],
}

impl Default for NttPoly {
    fn default() -> Self {
        Self {
            coeffs: [0u32; N],
        }
    }
}

fn poly_add(a: &Poly, b: &Poly) -> Poly {
    let mut r = Poly::default();
    for i in 0..N {
        r.coeffs[i] = field_add(a.coeffs[i], b.coeffs[i]);
    }
    r
}

fn poly_sub(a: &Poly, b: &Poly) -> Poly {
    let mut r = Poly::default();
    for i in 0..N {
        r.coeffs[i] = field_sub(a.coeffs[i], b.coeffs[i]);
    }
    r
}

fn ntt_add(a: &NttPoly, b: &NttPoly) -> NttPoly {
    let mut r = NttPoly::default();
    for i in 0..N {
        r.coeffs[i] = field_add(a.coeffs[i], b.coeffs[i]);
    }
    r
}

fn ntt_sub(a: &NttPoly, b: &NttPoly) -> NttPoly {
    let mut r = NttPoly::default();
    for i in 0..N {
        r.coeffs[i] = field_sub(a.coeffs[i], b.coeffs[i]);
    }
    r
}

fn ntt_mul(a: &NttPoly, b: &NttPoly) -> NttPoly {
    let mut r = NttPoly::default();
    for i in 0..N {
        r.coeffs[i] = field_montgomery_mul(a.coeffs[i], b.coeffs[i]);
    }
    r
}

const ZETAS: [FieldElement; 256] = [
    4193792, 25847, 5771523, 7861508, 237124, 7602457, 7504169, 466468, 1826347, 2353451, 8021166, 6288512, 3119733,
    5495562, 3111497, 2680103, 2725464, 1024112, 7300517, 3585928, 7830929, 7260833, 2619752, 6271868, 6262231,
    4520680, 6980856, 5102745, 1757237, 8360995, 4010497, 280005, 2706023, 95776, 3077325, 3530437, 6718724, 4788269,
    5842901, 3915439, 4519302, 5336701, 3574422, 5512770, 3539968, 8079950, 2348700, 7841118, 6681150, 6736599,
    3505694, 4558682, 3507263, 6239768, 6779997, 3699596, 811944, 531354, 954230, 3881043, 3900724, 5823537, 2071892,
    5582638, 4450022, 6851714, 4702672, 5339162, 6927966, 3475950, 2176455, 6795196, 7122806, 1939314, 4296819,
    7380215, 5190273, 5223087, 4747489, 126922, 3412210, 7396998, 2147896, 2715295, 5412772, 4686924, 7969390, 5903370,
    7709315, 7151892, 8357436, 7072248, 7998430, 1349076, 1852771, 6949987, 5037034, 264944, 508951, 3097992, 44288,
    7280319, 904516, 3958618, 4656075, 8371839, 1653064, 5130689, 2389356, 8169440, 759969, 7063561, 189548, 4827145,
    3159746, 6529015, 5971092, 8202977, 1315589, 1341330, 1285669, 6795489, 7567685, 6940675, 5361315, 4499357,
    4751448, 3839961, 2091667, 3407706, 2316500, 3817976, 5037939, 2244091, 5933984, 4817955, 266997, 2434439, 7144689,
    3513181, 4860065, 4621053, 7183191, 5187039, 900702, 1859098, 909542, 819034, 495491, 6767243, 8337157, 7857917,
    7725090, 5257975, 2031748, 3207046, 4823422, 7855319, 7611795, 4784579, 342297, 286988, 5942594, 4108315, 3437287,
    5038140, 1735879, 203044, 2842341, 2691481, 5790267, 1265009, 4055324, 1247620, 2486353, 1595974, 4613401, 1250494,
    2635921, 4832145, 5386378, 1869119, 1903435, 7329447, 7047359, 1237275, 5062207, 6950192, 7929317, 1312455,
    3306115, 6417775, 7100756, 1917081, 5834105, 7005614, 1500165, 777191, 2235880, 3406031, 7838005, 5548557, 6709241,
    6533464, 5796124, 4656147, 594136, 4603424, 6366809, 2432395, 2454455, 8215696, 1957272, 3369112, 185531, 7173032,
    5196991, 162844, 1616392, 3014001, 810149, 1652634, 4686184, 6581310, 5341501, 3523897, 3866901, 269760, 2213111,
    7404533, 1717735, 472078, 7953734, 1723600, 6577327, 1910376, 6712985, 7276084, 8119771, 4546524, 5441381, 6144432,
    7959518, 6094090, 183443, 7403526, 1612842, 4834730, 7826001, 3919660, 8332111, 7018208, 3937738, 1400424, 7534263,
    1976782,
];

fn ntt(f: &Poly) -> NttPoly {
    let mut f = NttPoly {
        coeffs: f.coeffs,
    };
    let mut m: usize = 0;

    let mut len: usize = 128;
    while len >= 8 {
        let mut start: usize = 0;
        while start < N {
            m += 1;
            let zeta = ZETAS[m];
            let mid = start + len;
            for j in (start..mid).step_by(2) {
                let t = field_montgomery_mul(zeta, f.coeffs[j + len]);
                f.coeffs[j + len] = field_sub(f.coeffs[j], t);
                f.coeffs[j] = field_add(f.coeffs[j], t);
                let t = field_montgomery_mul(zeta, f.coeffs[j + len + 1]);
                f.coeffs[j + len + 1] = field_sub(f.coeffs[j + 1], t);
                f.coeffs[j + 1] = field_add(f.coeffs[j + 1], t);
            }
            start += 2 * len;
        }
        len /= 2;
    }

    let mut start: usize = 0;
    while start < N {
        m += 1;
        let zeta = ZETAS[m];
        let t = field_montgomery_mul(zeta, f.coeffs[start + 4]);
        f.coeffs[start + 4] = field_sub(f.coeffs[start], t);
        f.coeffs[start] = field_add(f.coeffs[start], t);
        let t = field_montgomery_mul(zeta, f.coeffs[start + 5]);
        f.coeffs[start + 5] = field_sub(f.coeffs[start + 1], t);
        f.coeffs[start + 1] = field_add(f.coeffs[start + 1], t);
        let t = field_montgomery_mul(zeta, f.coeffs[start + 6]);
        f.coeffs[start + 6] = field_sub(f.coeffs[start + 2], t);
        f.coeffs[start + 2] = field_add(f.coeffs[start + 2], t);
        let t = field_montgomery_mul(zeta, f.coeffs[start + 7]);
        f.coeffs[start + 7] = field_sub(f.coeffs[start + 3], t);
        f.coeffs[start + 3] = field_add(f.coeffs[start + 3], t);
        start += 8;
    }

    start = 0;
    while start < N {
        m += 1;
        let zeta = ZETAS[m];
        let t = field_montgomery_mul(zeta, f.coeffs[start + 2]);
        f.coeffs[start + 2] = field_sub(f.coeffs[start], t);
        f.coeffs[start] = field_add(f.coeffs[start], t);
        let t = field_montgomery_mul(zeta, f.coeffs[start + 3]);
        f.coeffs[start + 3] = field_sub(f.coeffs[start + 1], t);
        f.coeffs[start + 1] = field_add(f.coeffs[start + 1], t);
        start += 4;
    }

    start = 0;
    while start < N {
        m += 1;
        let zeta = ZETAS[m];
        let t = field_montgomery_mul(zeta, f.coeffs[start + 1]);
        f.coeffs[start + 1] = field_sub(f.coeffs[start], t);
        f.coeffs[start] = field_add(f.coeffs[start], t);
        start += 2;
    }

    f
}

fn invntt(f: &NttPoly) -> Poly {
    let mut f = NttPoly {
        coeffs: f.coeffs,
    };
    let mut m: usize = 255;

    let mut start: usize = 0;
    while start < N {
        let zeta = ZETAS[m];
        m -= 1;
        let t = f.coeffs[start];
        f.coeffs[start] = field_add(t, f.coeffs[start + 1]);
        f.coeffs[start + 1] = field_montgomery_mul(zeta, field_sub(f.coeffs[start + 1], t));
        start += 2;
    }

    start = 0;
    while start < N {
        let zeta = ZETAS[m];
        m -= 1;
        let t = f.coeffs[start];
        f.coeffs[start] = field_add(t, f.coeffs[start + 2]);
        f.coeffs[start + 2] = field_montgomery_mul(zeta, field_sub(f.coeffs[start + 2], t));
        let t = f.coeffs[start + 1];
        f.coeffs[start + 1] = field_add(t, f.coeffs[start + 3]);
        f.coeffs[start + 3] = field_montgomery_mul(zeta, field_sub(f.coeffs[start + 3], t));
        start += 4;
    }

    start = 0;
    while start < N {
        let zeta = ZETAS[m];
        m -= 1;
        let t = f.coeffs[start];
        f.coeffs[start] = field_add(t, f.coeffs[start + 4]);
        f.coeffs[start + 4] = field_montgomery_mul(zeta, field_sub(f.coeffs[start + 4], t));
        let t = f.coeffs[start + 1];
        f.coeffs[start + 1] = field_add(t, f.coeffs[start + 5]);
        f.coeffs[start + 5] = field_montgomery_mul(zeta, field_sub(f.coeffs[start + 5], t));
        let t = f.coeffs[start + 2];
        f.coeffs[start + 2] = field_add(t, f.coeffs[start + 6]);
        f.coeffs[start + 6] = field_montgomery_mul(zeta, field_sub(f.coeffs[start + 6], t));
        let t = f.coeffs[start + 3];
        f.coeffs[start + 3] = field_add(t, f.coeffs[start + 7]);
        f.coeffs[start + 7] = field_montgomery_mul(zeta, field_sub(f.coeffs[start + 7], t));
        start += 8;
    }

    let mut len: usize = 8;
    while len < N {
        let mut start: usize = 0;
        while start < N {
            let zeta = ZETAS[m];
            m -= 1;
            let mid = start + len;
            for j in (start..mid).step_by(2) {
                let t = f.coeffs[j];
                f.coeffs[j] = field_add(t, f.coeffs[j + len]);
                let diff = field_sub(f.coeffs[j + len], t);
                f.coeffs[j + len] = field_montgomery_mul(zeta, diff);
                let t = f.coeffs[j + 1];
                f.coeffs[j + 1] = field_add(t, f.coeffs[j + len + 1]);
                let diff = field_sub(f.coeffs[j + len + 1], t);
                f.coeffs[j + len + 1] = field_montgomery_mul(zeta, diff);
            }
            start += 2 * len;
        }
        len *= 2;
    }

    let mut r = Poly::default();
    for i in 0..N {
        r.coeffs[i] = field_montgomery_mul(f.coeffs[i], N_INV);
    }
    r
}

fn sample_ntt(rho: &[u8; 32], s: u8, r: u8) -> NttPoly {
    let mut shake = Shake128::new();
    shake.absorb(rho);
    shake.absorb(&[s, r]);

    let mut a = NttPoly::default();
    let mut j: usize = 0;
    let mut buf = [0u8; 168];
    let mut off: usize = 168;

    loop {
        if off >= 168 {
            shake.squeeze(&mut buf);
            off = 0;
        }
        let v = (buf[off] as u32) | ((buf[off + 1] as u32) << 8) | ((buf[off + 2] as u32) << 16);
        off += 3;
        let v = v & 0x7FFFFF;
        if v < Q {
            a.coeffs[j] = field_to_montgomery(v);
            j += 1;
            if j >= N {
                break;
            }
        }
    }
    a
}

fn coeff_from_half_byte(b: u8, eta: u32) -> Option<FieldElement> {
    match eta {
        2 => {
            if b > 14 {
                None
            } else {
                Some(field_sub_to_montgomery(2, (b % 5) as u32))
            }
        }
        4 => {
            if b > 8 {
                None
            } else {
                Some(field_sub_to_montgomery(4, b as u32))
            }
        }
        _ => unreachable!(),
    }
}

fn sample_bounded_poly(rho: &[u8], r: u8, eta: u32) -> Poly {
    let mut shake = Shake256::new();
    shake.absorb(rho);
    shake.absorb(&[r, 0]);

    let mut a = Poly::default();
    let mut j: usize = 0;
    let mut buf = [0u8; 136];
    let mut off: usize = 136;

    loop {
        if off >= 136 {
            shake.squeeze(&mut buf);
            off = 0;
        }
        let z0 = buf[off] & 0x0F;
        let z1 = buf[off] >> 4;
        off += 1;

        if let Some(c) = coeff_from_half_byte(z0, eta) {
            a.coeffs[j] = c;
            j += 1;
            if j >= N {
                break;
            }
        }
        if let Some(c) = coeff_from_half_byte(z1, eta) {
            a.coeffs[j] = c;
            j += 1;
            if j >= N {
                break;
            }
        }
    }
    a
}

fn sample_in_ball(rho: &[u8], tau: usize) -> Poly {
    let mut shake = Shake256::new();
    shake.absorb(rho);
    let mut s = [0u8; 8];
    shake.squeeze(&mut s);

    let mut c = Poly::default();
    let mut signs: u64 = u64::from_le_bytes(s);

    for i in (N - tau)..N {
        let mut jb = [0u8; 1];
        loop {
            shake.squeeze(&mut jb);
            if jb[0] as usize <= i {
                break;
            }
        }
        let j = jb[0] as usize;
        c.coeffs[i] = c.coeffs[j];
        if (signs & 1) == 0 {
            c.coeffs[j] = ONE;
        } else {
            c.coeffs[j] = MINUS_ONE;
        }
        signs >>= 1;
    }

    c
}

fn expand_mask(nonce: &[u8; 64], kappa: usize, params: &MlDsaParams) -> Poly {
    let mut shake = Shake256::new();
    shake.absorb(nonce);
    shake.absorb(&(kappa as u16).to_le_bytes());

    let mut buf = [0u8; MAX_POLYZ_BYTES];
    shake.squeeze(&mut buf[..params.polyz_bytes]);
    bitunpack(&buf[..params.polyz_bytes], params.gamma1_bits)
}

fn highbits_poly(w: &Poly, params: &MlDsaParams) -> [u8; N] {
    let mut r = [0u8; N];
    for i in 0..N {
        r[i] = highbits(field_from_montgomery(w.coeffs[i]), params.gamma2_den);
    }
    r
}

fn make_hint_poly(ct0: &Poly, w: &Poly, cs2: &Poly, params: &MlDsaParams) -> ([u8; N], usize) {
    let mut h = [0u8; N];
    let mut count = 0usize;
    for i in 0..N {
        h[i] = make_hint(ct0.coeffs[i], w.coeffs[i], cs2.coeffs[i], params.gamma2_den);
        count += h[i] as usize;
    }
    (h, count)
}

fn use_hint_poly(r: &Poly, h: &[u8; N], params: &MlDsaParams) -> [u8; N] {
    let mut w = [0u8; N];
    for i in 0..N {
        w[i] = use_hint(r.coeffs[i], h[i], params.gamma2_den);
    }
    w
}

/// Returns `true` if any coefficient of `w` has centered infinity norm `>= bound`.
///
/// All `N` coefficients are always inspected: the result is accumulated with a
/// branchless OR rather than returning at the first failing index, so the number
/// of iterations does not depend on the (potentially secret) coefficient values.
fn coefficients_exceed_bound(w: &Poly, bound: u32) -> bool {
    let mut exceeded = 0u32;
    for &c in w.coeffs.iter() {
        exceeded |= (field_infinity_norm(c) >= bound) as u32;
    }
    exceeded != 0
}

/// Returns `true` if any low-order part `r0` of `w` (per [`decompose`]) satisfies
/// `|r0| >= bound`.
///
/// All `N` coefficients are always inspected: the result is accumulated with a
/// branchless OR rather than returning at the first failing index, so the number
/// of iterations does not depend on the (potentially secret) coefficient values.
fn lowbits_exceed_bound(w: &Poly, bound: u32, gamma2_den: u32) -> bool {
    let mut exceeded = 0u32;
    for &c in w.coeffs.iter() {
        let (_, r0) = decompose(c, gamma2_den);
        let abs_r0 = (r0 ^ (r0 >> 31)).wrapping_sub(r0 >> 31) as u32;
        exceeded |= (abs_r0 >= bound) as u32;
    }
    exceeded != 0
}

fn pk_encode<const K: usize>(rho: &[u8; 32], t1: &[[u16; N]; K], out: &mut [u8]) {
    debug_assert_eq!(out.len(), 32 + K * N * 10 / 8);
    out[..32].copy_from_slice(rho);
    let mut pos = 32;

    for w in t1.iter() {
        for i in (0..N).step_by(4) {
            let c0 = w[i] as u32;
            let c1 = w[i + 1] as u32;
            let c2 = w[i + 2] as u32;
            let c3 = w[i + 3] as u32;
            out[pos] = (c0 & 0xFF) as u8;
            out[pos + 1] = ((c0 >> 8) | (c1 << 2)) as u8;
            out[pos + 2] = ((c1 >> 6) | (c2 << 4)) as u8;
            out[pos + 3] = ((c2 >> 4) | (c3 << 6)) as u8;
            out[pos + 4] = (c3 >> 2) as u8;
            pos += 5;
        }
    }
}

fn pk_decode<const K: usize>(params: &MlDsaParams, pk: &[u8]) -> Result<([u8; 32], [[u16; N]; K]), MlDsaError> {
    if pk.len() != params.public_key_size {
        return Err(MlDsaError::InvalidPublicKey);
    }
    let mut rho = [0u8; 32];
    rho.copy_from_slice(&pk[..32]);
    let mut t1 = [[0u16; N]; K];
    let mut pos = 32;

    for r in 0..K {
        for i in (0..N).step_by(4) {
            let b0 = pk[pos] as u16;
            let b1 = pk[pos + 1] as u16;
            let b2 = pk[pos + 2] as u16;
            let b3 = pk[pos + 3] as u16;
            let b4 = pk[pos + 4] as u16;
            t1[r][i] = b0 | ((b1 & 0b0000_0011) << 8);
            t1[r][i + 1] = (b1 >> 2) | ((b2 & 0b0000_1111) << 6);
            t1[r][i + 2] = (b2 >> 4) | ((b3 & 0b0011_1111) << 4);
            t1[r][i + 3] = (b3 >> 6) | ((b4 & 0b1111_1111) << 2);
            pos += 5;
        }
    }
    Ok((rho, t1))
}

fn bitpack_18(z: &Poly, out: &mut [u8]) {
    const B: u32 = 1 << 17;
    let mut q = 0usize;

    for i in (0..N).step_by(4) {
        let w0 = (B as i32 - field_centered_mod(z.coeffs[i])) as u32;
        out[q] = w0 as u8;
        out[q + 1] = (w0 >> 8) as u8;
        out[q + 2] = (w0 >> 16) as u8;
        let w1 = (B as i32 - field_centered_mod(z.coeffs[i + 1])) as u32;
        out[q + 2] |= (w1 << 2) as u8;
        out[q + 3] = (w1 >> 6) as u8;
        out[q + 4] = (w1 >> 14) as u8;
        let w2 = (B as i32 - field_centered_mod(z.coeffs[i + 2])) as u32;
        out[q + 4] |= (w2 << 4) as u8;
        out[q + 5] = (w2 >> 4) as u8;
        out[q + 6] = (w2 >> 12) as u8;
        let w3 = (B as i32 - field_centered_mod(z.coeffs[i + 3])) as u32;
        out[q + 6] |= (w3 << 6) as u8;
        out[q + 7] = (w3 >> 2) as u8;
        out[q + 8] = (w3 >> 10) as u8;
        q += 9;
    }
}

fn bitpack_20(z: &Poly, out: &mut [u8]) {
    let b = 1u32 << 19;
    let mut q = 0usize;

    for i in (0..N).step_by(2) {
        let w0 = (b as i32 - field_centered_mod(z.coeffs[i])) as u32;
        out[q] = w0 as u8;
        out[q + 1] = (w0 >> 8) as u8;
        out[q + 2] = (w0 >> 16) as u8;
        let w1 = (b as i32 - field_centered_mod(z.coeffs[i + 1])) as u32;
        out[q + 2] |= ((w1 & 0x0F) << 4) as u8;
        out[q + 3] = (w1 >> 4) as u8;
        out[q + 4] = (w1 >> 12) as u8;
        q += 5;
    }
}

fn bitpack(z: &Poly, gamma1_bits: usize, out: &mut [u8]) {
    match gamma1_bits {
        17 => bitpack_18(z, out),
        19 => bitpack_20(z, out),
        _ => unreachable!(),
    }
}

fn bitunpack_18(v: &[u8]) -> Poly {
    const B: u32 = 1 << 17;
    const MASK18: u32 = (1 << 18) - 1;
    let mut r = Poly::default();
    let mut p = v;

    for i in (0..N).step_by(4) {
        let w0 = (p[0] as u32) | ((p[1] as u32) << 8) | ((p[2] as u32) << 16);
        r.coeffs[i] = field_sub_to_montgomery(B, w0 & MASK18);
        let w1 = ((p[2] as u32) >> 2) | ((p[3] as u32) << 6) | ((p[4] as u32) << 14);
        r.coeffs[i + 1] = field_sub_to_montgomery(B, w1 & MASK18);
        let w2 = ((p[4] as u32) >> 4) | ((p[5] as u32) << 4) | ((p[6] as u32) << 12);
        r.coeffs[i + 2] = field_sub_to_montgomery(B, w2 & MASK18);
        let w3 = ((p[6] as u32) >> 6) | ((p[7] as u32) << 2) | ((p[8] as u32) << 10);
        r.coeffs[i + 3] = field_sub_to_montgomery(B, w3 & MASK18);
        p = &p[9..];
    }
    r
}

fn bitunpack_20(v: &[u8]) -> Poly {
    let b = 1u32 << 19;
    let mask20 = (1u32 << 20) - 1;
    let mut r = Poly::default();
    let mut p = v;

    for i in (0..N).step_by(2) {
        let w0 = (p[0] as u32) | ((p[1] as u32) << 8) | ((p[2] as u32) << 16);
        r.coeffs[i] = field_sub_to_montgomery(b, w0 & mask20);
        let w1 = ((p[2] as u32) >> 4) | ((p[3] as u32) << 4) | ((p[4] as u32) << 12);
        r.coeffs[i + 1] = field_sub_to_montgomery(b, w1 & mask20);
        p = &p[5..];
    }
    r
}

fn bitunpack(v: &[u8], gamma1_bits: usize) -> Poly {
    match gamma1_bits {
        17 => bitunpack_18(v),
        19 => bitunpack_20(v),
        _ => unreachable!(),
    }
}

fn hint_encode<const K: usize>(params: &MlDsaParams, h: &[[u8; N]; K], out: &mut [u8]) {
    let omega = params.omega;
    debug_assert_eq!(out.len(), omega + K);
    out.fill(0);
    let mut idx: usize = 0;

    for i in 0..K {
        for j in 0..N {
            if h[i][j] != 0 {
                out[idx] = j as u8;
                idx += 1;
            }
        }
        out[omega + i] = idx as u8;
    }
}

fn hint_decode<const K: usize>(params: &MlDsaParams, sig: &[u8]) -> Result<[[u8; N]; K], MlDsaError> {
    let omega = params.omega;
    debug_assert_eq!(sig.len(), omega + K);
    let mut h = [[0u8; N]; K];
    let mut idx: usize = 0;

    for i in 0..K {
        let limit = sig[omega + i] as usize;
        if limit < idx || limit > omega {
            return Err(MlDsaError::InvalidSignature);
        }
        // Track polynomial start so the ordering check doesn't fire across polynomial boundaries.
        let poly_start = idx;
        while idx < limit {
            let j = sig[idx] as usize;
            // FIPS 204 §6.2 Algorithm 24: indices within a polynomial must be strictly increasing.
            if idx > poly_start && sig[idx - 1] as usize >= j {
                return Err(MlDsaError::InvalidSignature);
            }
            if j >= N {
                return Err(MlDsaError::InvalidSignature);
            }
            h[i][j] = 1;
            idx += 1;
        }
    }
    for k in idx..omega {
        if sig[k] != 0 {
            return Err(MlDsaError::InvalidSignature);
        }
    }
    Ok(h)
}

fn sig_encode<const K: usize, const L: usize>(
    params: &MlDsaParams,
    ch: &[u8],
    z: &[Poly; L],
    h: &[[u8; N]; K],
    out: &mut [u8],
) {
    debug_assert_eq!(out.len(), params.signature_size);
    let lo4 = params.lambda_over_4;
    out[..lo4].copy_from_slice(ch);

    let mut pos = lo4;
    for poly in z.iter() {
        let pz = params.polyz_bytes;
        bitpack(poly, params.gamma1_bits, &mut out[pos..pos + pz]);
        pos += pz;
    }

    hint_encode::<K>(params, h, &mut out[pos..]);
}

fn sig_decode<const K: usize, const L: usize>(
    params: &MlDsaParams,
    sig: &[u8],
) -> Result<([u8; MAX_LAMBDA_OVER_4], [Poly; L], [[u8; N]; K]), MlDsaError> {
    if sig.len() != params.signature_size {
        return Err(MlDsaError::InvalidSignatureLength);
    }
    let lo4 = params.lambda_over_4;
    let mut ch = [0u8; MAX_LAMBDA_OVER_4];
    ch[..lo4].copy_from_slice(&sig[..lo4]);

    let mut z: [Poly; L] = core::array::from_fn(|_| Poly::default());
    let mut pos = lo4;
    for poly in z.iter_mut() {
        let pz = params.polyz_bytes;
        *poly = bitunpack(&sig[pos..pos + pz], params.gamma1_bits);
        pos += pz;
    }

    let h = hint_decode::<K>(params, &sig[pos..])?;

    Ok((ch, z, h))
}

fn w1_encode<const K: usize>(params: &MlDsaParams, w1: &[[u8; N]; K]) -> ([u8; MAX_W1_BYTES], usize) {
    let mut buf = [0u8; MAX_W1_BYTES];
    let mut pos = 0usize;

    match params.gamma2_den {
        32 => {
            // Coefficients are <= 15, four bits each.
            for w in w1.iter() {
                for i in (0..N).step_by(2) {
                    buf[pos] = w[i] | (w[i + 1] << 4);
                    pos += 1;
                }
            }
        }
        88 => {
            // Coefficients are <= 43, six bits each.
            for w in w1.iter() {
                for i in (0..N).step_by(4) {
                    let (b0, b1, b2, b3) = (w[i], w[i + 1], w[i + 2], w[i + 3]);
                    buf[pos] = b0 | (b1 << 6);
                    buf[pos + 1] = (b1 >> 2) | (b2 << 4);
                    buf[pos + 2] = (b2 >> 4) | (b3 << 2);
                    pos += 3;
                }
            }
        }
        _ => unreachable!(),
    }

    (buf, pos)
}

fn compute_matrix_a<const K: usize, const L: usize>(rho: &[u8; 32]) -> [[NttPoly; L]; K] {
    core::array::from_fn(|r| core::array::from_fn(|s| sample_ntt(rho, s as u8, r as u8)))
}

fn compute_matrix_a_into<const K: usize, const L: usize>(a: &mut [[NttPoly; L]; K], rho: &[u8; 32]) {
    for r in 0..K {
        for s in 0..L {
            a[r][s] = sample_ntt(rho, s as u8, r as u8);
        }
    }
}

pub(crate) fn compute_pubkey_hash(pk: &[u8]) -> [u8; 64] {
    let mut shake = Shake256::new();
    shake.absorb(pk);
    let mut tr = [0u8; 64];
    shake.squeeze(&mut tr);
    tr
}

fn compute_message_hash(tr: &[u8; 64], message: &[u8], ctx: &[u8]) -> Result<[u8; 64], MlDsaError> {
    if ctx.len() > CONTEXT_MAX_LEN {
        return Err(MlDsaError::ContextTooLong);
    }
    let mut shake = Shake256::new();
    shake.absorb(tr);
    shake.absorb(&[0u8]);
    shake.absorb(&[ctx.len() as u8]);
    shake.absorb(ctx);
    shake.absorb(message);
    let mut mu = [0u8; 64];
    shake.squeeze(&mut mu);
    Ok(mu)
}

fn compute_t1_hat<const K: usize>(t1: &[[u16; N]; K]) -> [NttPoly; K] {
    core::array::from_fn(|i| {
        let mut w = Poly::default();
        for j in 0..N {
            w.coeffs[j] = field_to_montgomery((t1[i][j] as u32) << D);
        }
        ntt(&w)
    })
}

/// Expanded key material shared by all ML-DSA parameter sets.
///
/// This is the only way to sign: key generation runs once when the key is
/// initialized and the resulting matrix `A` and secret vectors in the NTT
/// domain are cached, so signing does not repeat the expensive key generation.
///
/// The value is a plain fixed-size type that never allocates, so it can live in
/// a `static` on `no_std` and embedded targets.
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub(crate) struct MlDsaKeyMaterial<const K: usize, const L: usize, const PK_SIZE: usize> {
    seed: [u8; SEED_SIZE],
    key_bytes: [u8; 32],
    pk: [u8; PK_SIZE],
    tr: [u8; 64],
    a: [[NttPoly; L]; K],
    s1_hat: [NttPoly; L],
    s2_hat: [NttPoly; K],
    t0_hat: [NttPoly; K],
}

impl<const K: usize, const L: usize, const PK_SIZE: usize> MlDsaKeyMaterial<K, L, PK_SIZE> {
    /// Creates a zeroed, uninitialized key material value.
    ///
    /// SAFETY: all fields are plain arrays of integers and polynomials whose
    /// all-zero bit pattern is a valid value, so `zeroed` is sound here. The
    /// key must be initialized before signing.
    pub(crate) const fn new() -> Self {
        // SAFETY: see above.
        unsafe { core::mem::zeroed() }
    }

    /// Expands `seed` into a fresh signing key.
    pub(crate) fn from_seed(params: &MlDsaParams, seed: &[u8; SEED_SIZE]) -> Self {
        let mut this = Self::new();
        this.init(params, seed);
        this
    }

    /// Generates a fresh random signing key.
    #[cfg(feature = "random")]
    pub(crate) fn from_random(params: &MlDsaParams) -> Result<Self, MlDsaError> {
        let mut this = Self::new();
        this.generate(params)?;
        Ok(this)
    }

    /// Expands `seed` into a signing key, overwriting any previous state.
    pub(crate) fn init(&mut self, params: &MlDsaParams, seed: &[u8; SEED_SIZE]) {
        debug_assert_eq!(PK_SIZE, params.public_key_size);
        let mut shake = Shake256::new();
        shake.absorb(seed);
        shake.absorb(&[K as u8, L as u8]);
        let mut rho = [0u8; 32];
        let mut rhos = [0u8; 64];
        shake.squeeze(&mut rho);
        shake.squeeze(&mut rhos);
        shake.squeeze(&mut self.key_bytes);

        compute_matrix_a_into::<K, L>(&mut self.a, &rho);

        for r in 0..L {
            let s1 = sample_bounded_poly(&rhos, r as u8, params.eta);
            self.s1_hat[r] = ntt(&s1);
        }
        for r in 0..K {
            let s2 = sample_bounded_poly(&rhos, (L + r) as u8, params.eta);
            self.s2_hat[r] = ntt(&s2);
        }

        let mut t1 = [[0u16; N]; K];
        for i in 0..K {
            let mut t_hat = self.s2_hat[i].clone();
            for j in 0..L {
                t_hat = ntt_add(&t_hat, &ntt_mul(&self.a[i][j], &self.s1_hat[j]));
            }
            let t = invntt(&t_hat);
            let mut t0 = Poly::default();
            for j in 0..N {
                (t1[i][j], t0.coeffs[j]) = power2round(t.coeffs[j]);
            }
            self.t0_hat[i] = ntt(&t0);
        }

        pk_encode::<K>(&rho, &t1, &mut self.pk);
        self.tr = compute_pubkey_hash(&self.pk);
        self.seed = *seed;
    }

    #[cfg(feature = "random")]
    pub(crate) fn generate(&mut self, params: &MlDsaParams) -> Result<[u8; SEED_SIZE], MlDsaError> {
        let seed: [u8; SEED_SIZE] = crate::random::bytes()?;
        self.init(params, &seed);
        Ok(seed)
    }

    pub(crate) fn public_key(&self) -> &[u8; PK_SIZE] {
        &self.pk
    }

    pub(crate) fn seed(&self) -> &[u8; SEED_SIZE] {
        &self.seed
    }

    pub(crate) fn sign_derand_into(
        &self,
        params: &MlDsaParams,
        message: &[u8],
        ctx: &[u8],
        rnd: &[u8; 32],
        sig_out: &mut [u8],
    ) -> Result<(), MlDsaError> {
        let mu = compute_message_hash(&self.tr, message, ctx)?;
        self.sign_internal(params, &mu, rnd, sig_out);
        Ok(())
    }

    pub(crate) fn sign_external_mu_derand_into(
        &self,
        params: &MlDsaParams,
        mu: &[u8; 64],
        rnd: &[u8; 32],
        sig_out: &mut [u8],
    ) {
        self.sign_internal(params, mu, rnd, sig_out);
    }

    fn sign_internal(&self, params: &MlDsaParams, mu: &[u8; 64], rnd: &[u8; 32], sig_out: &mut [u8]) {
        debug_assert_eq!(sig_out.len(), params.signature_size);
        let a = &self.a;
        let s1_hat = &self.s1_hat;
        let s2_hat = &self.s2_hat;
        let t0_hat = &self.t0_hat;

        let gamma1beta = params.gamma1 - params.beta;
        let gamma2 = params.gamma2;
        let gamma2beta = gamma2 - params.beta;
        let lo4 = params.lambda_over_4;

        let mut h_shake = Shake256::new();
        h_shake.absorb(&self.key_bytes);
        h_shake.absorb(rnd);
        h_shake.absorb(mu);
        let mut nonce = [0u8; 64];
        h_shake.squeeze(&mut nonce);

        let mut kappa: usize = 0;

        loop {
            let mut y: [Poly; L] = core::array::from_fn(|_| Poly::default());
            for item in y.iter_mut() {
                *item = expand_mask(&nonce, kappa, params);
                kappa += 1;
            }

            let mut y_hat: [NttPoly; L] = core::array::from_fn(|_| NttPoly::default());
            for i in 0..L {
                y_hat[i] = ntt(&y[i]);
            }

            let mut w: [Poly; K] = core::array::from_fn(|_| Poly::default());
            for i in 0..K {
                let mut w_hat = NttPoly::default();
                for j in 0..L {
                    w_hat = ntt_add(&w_hat, &ntt_mul(&a[i][j], &y_hat[j]));
                }
                w[i] = invntt(&w_hat);
            }

            let mut w1 = [[0u8; N]; K];
            for i in 0..K {
                w1[i] = highbits_poly(&w[i], params);
            }

            let mut ch_shake = Shake256::new();
            ch_shake.absorb(mu);
            let (w1_bytes, w1_len) = w1_encode::<K>(params, &w1);
            ch_shake.absorb(&w1_bytes[..w1_len]);
            let mut ct = [0u8; MAX_LAMBDA_OVER_4];
            ch_shake.squeeze(&mut ct[..lo4]);

            let c = sample_in_ball(&ct[..lo4], params.tau);
            let c_hat = ntt(&c);

            let mut cs1: [Poly; L] = core::array::from_fn(|_| Poly::default());
            for i in 0..L {
                cs1[i] = invntt(&ntt_mul(&c_hat, &s1_hat[i]));
            }
            let mut cs2: [Poly; K] = core::array::from_fn(|_| Poly::default());
            for i in 0..K {
                cs2[i] = invntt(&ntt_mul(&c_hat, &s2_hat[i]));
            }

            // Each phase inspects every polynomial (no early `break`) so the
            // number of iterations does not reveal which index failed first.
            let mut z: [Poly; L] = core::array::from_fn(|_| Poly::default());
            let mut reject = false;
            for i in 0..L {
                z[i] = poly_add(&y[i], &cs1[i]);
                reject |= coefficients_exceed_bound(&z[i], gamma1beta);
            }
            if reject {
                continue;
            }

            for i in 0..K {
                let r0 = poly_sub(&w[i], &cs2[i]);
                reject |= lowbits_exceed_bound(&r0, gamma2beta, params.gamma2_den);
            }
            if reject {
                continue;
            }

            let mut ct0: [Poly; K] = core::array::from_fn(|_| Poly::default());
            for i in 0..K {
                ct0[i] = invntt(&ntt_mul(&c_hat, &t0_hat[i]));
                reject |= coefficients_exceed_bound(&ct0[i], gamma2);
            }
            if reject {
                continue;
            }

            let mut total_hints: usize = 0;
            let mut h = [[0u8; N]; K];
            for i in 0..K {
                let (hi, count) = make_hint_poly(&ct0[i], &w[i], &cs2[i], params);
                h[i] = hi;
                total_hints += count;
            }
            if total_hints > params.omega {
                continue;
            }

            sig_encode::<K, L>(params, &ct[..lo4], &z, &h, sig_out);
            return;
        }
    }
}

fn verify_internal<const K: usize, const L: usize, const PK_SIZE: usize, const SIG_SIZE: usize>(
    params: &MlDsaParams,
    pk: &[u8; PK_SIZE],
    mu: &[u8; 64],
    sig: &[u8; SIG_SIZE],
) -> Result<(), MlDsaError> {
    let (rho, t1) = pk_decode::<K>(params, pk)?;
    let (ch, z, h) = sig_decode::<K, L>(params, sig)?;

    let gamma1beta = params.gamma1 - params.beta;

    // FIPS 204 §6.2 Algorithm 3 step 5: check ||z||∞ < γ1 − β before the
    // expensive matrix-vector product.
    for item in z.iter() {
        if coefficients_exceed_bound(item, gamma1beta) {
            return Err(MlDsaError::InvalidSignature);
        }
    }

    let a = compute_matrix_a::<K, L>(&rho);
    let t1_hat = compute_t1_hat::<K>(&t1);

    let c = sample_in_ball(&ch[..params.lambda_over_4], params.tau);
    let c_hat = ntt(&c);

    let mut z_hat: [NttPoly; L] = core::array::from_fn(|_| NttPoly::default());
    for i in 0..L {
        z_hat[i] = ntt(&z[i]);
    }

    let mut w_approx: [Poly; K] = core::array::from_fn(|_| Poly::default());
    for i in 0..K {
        let mut w_hat = NttPoly::default();
        for j in 0..L {
            w_hat = ntt_add(&w_hat, &ntt_mul(&a[i][j], &z_hat[j]));
        }
        w_hat = ntt_sub(&w_hat, &ntt_mul(&c_hat, &t1_hat[i]));
        w_approx[i] = invntt(&w_hat);
    }

    let mut w1 = [[0u8; N]; K];
    for i in 0..K {
        w1[i] = use_hint_poly(&w_approx[i], &h[i], params);
    }

    let mut ch_shake = Shake256::new();
    ch_shake.absorb(mu);
    let (w1_bytes, w1_len) = w1_encode::<K>(params, &w1);
    ch_shake.absorb(&w1_bytes[..w1_len]);
    let mut computed_ch = [0u8; MAX_LAMBDA_OVER_4];
    ch_shake.squeeze(&mut computed_ch[..params.lambda_over_4]);

    if !constant_time_eq(&ch[..params.lambda_over_4], &computed_ch[..params.lambda_over_4]) {
        return Err(MlDsaError::InvalidSignature);
    }

    Ok(())
}

/// Verifies a signature over `message` with the optional context `ctx`.
pub(crate) fn verify_message<const K: usize, const L: usize, const PK_SIZE: usize, const SIG_SIZE: usize>(
    params: &MlDsaParams,
    pk: &[u8; PK_SIZE],
    message: &[u8],
    sig: &[u8; SIG_SIZE],
    ctx: &[u8],
) -> Result<(), MlDsaError> {
    let tr = compute_pubkey_hash(pk);
    let mu = compute_message_hash(&tr, message, ctx)?;
    verify_internal::<K, L, PK_SIZE, SIG_SIZE>(params, pk, &mu, sig)
}

/// Verifies a signature over a precomputed 64-byte message representative `mu`
/// (FIPS 204 "external mu" verification).
pub(crate) fn verify_external_mu<const K: usize, const L: usize, const PK_SIZE: usize, const SIG_SIZE: usize>(
    params: &MlDsaParams,
    pk: &[u8; PK_SIZE],
    mu: &[u8; 64],
    sig: &[u8; SIG_SIZE],
) -> Result<(), MlDsaError> {
    verify_internal::<K, L, PK_SIZE, SIG_SIZE>(params, pk, mu, sig)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ntt_round_trip() {
        let mut shake = Shake128::new();
        for _ in 0..100 {
            let mut poly = Poly::default();
            for j in 0..N {
                let mut b = [0u8; 4];
                shake.squeeze(&mut b);
                let x = u32::from_le_bytes(b) % Q;
                poly.coeffs[j] = field_to_montgomery(x);
            }
            let fwd = ntt(&poly);
            let back = invntt(&fwd);
            for j in 0..N {
                assert_eq!(poly.coeffs[j], back.coeffs[j], "NTT round-trip failed at coeff {}", j);
            }
        }
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn power2round_consistency() {
        for x in 0u32..Q {
            let mr = field_to_montgomery(x);
            let (r1, r0) = power2round(mr);
            let recovered = (r1 as u32) << D;

            let expected_r0 = if x >= recovered {
                x - recovered
            } else {
                x.wrapping_sub(recovered)
            };

            assert!(
                expected_r0 < (1 << D) || expected_r0 >= Q - (1 << D) + 1,
                "power2round: r0 out of range at x={}, r1={}, r0_expected={}",
                x,
                r1,
                expected_r0
            );

            let got_r0 = field_from_montgomery(r0);
            assert!(
                got_r0 == expected_r0 || got_r0 == expected_r0.wrapping_add(Q) || got_r0 == expected_r0.wrapping_sub(Q),
                "power2round: r0 mismatch at x={}, r1={}, expected_r0={}, got_r0={}",
                x,
                r1,
                expected_r0,
                got_r0
            );
        }
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn highbits32_exhaustive() {
        for x in 0u32..Q {
            let h = highbits32(x);
            assert!(h < 16, "highbits32: h={} out of range at x={}", h, x);
            let (r1, _) = decompose32(field_to_montgomery(x));
            assert_eq!(h, r1, "highbits32 vs decompose32 r1 mismatch at x={}", x);
        }
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn highbits88_exhaustive() {
        for x in 0u32..Q {
            let h = highbits88(x);
            assert!(h < 44, "highbits88: h={} out of range at x={}", h, x);
            let (r1, _) = decompose88(field_to_montgomery(x));
            assert_eq!(h, r1, "highbits88 vs decompose88 r1 mismatch at x={}", x);
        }
    }

    #[test]
    fn make_hint32_correctness() {
        let mut shake = Shake128::new();
        for _ in 0..5000 {
            let mut b = [0u8; 12];
            shake.squeeze(&mut b);
            let ct0_val = u32::from_le_bytes(b[0..4].try_into().unwrap()) % Q;
            let w_val = u32::from_le_bytes(b[4..8].try_into().unwrap()) % Q;
            let cs2_val = u32::from_le_bytes(b[8..12].try_into().unwrap()) % Q;
            let ct0 = field_to_montgomery(ct0_val);
            let w = field_to_montgomery(w_val);
            let cs2 = field_to_montgomery(cs2_val);
            let h = make_hint32(ct0, w, cs2);
            assert!(h == 0 || h == 1, "make_hint32: hint not 0 or 1");
        }
    }

    #[test]
    fn make_hint88_correctness() {
        let mut shake = Shake128::new();
        for _ in 0..5000 {
            let mut b = [0u8; 12];
            shake.squeeze(&mut b);
            let ct0_val = u32::from_le_bytes(b[0..4].try_into().unwrap()) % Q;
            let w_val = u32::from_le_bytes(b[4..8].try_into().unwrap()) % Q;
            let cs2_val = u32::from_le_bytes(b[8..12].try_into().unwrap()) % Q;
            let ct0 = field_to_montgomery(ct0_val);
            let w = field_to_montgomery(w_val);
            let cs2 = field_to_montgomery(cs2_val);
            let h = make_hint88(ct0, w, cs2);
            assert!(h == 0 || h == 1, "make_hint88: hint not 0 or 1");
        }
    }

    #[test]
    fn pk_encode_decode_round_trip_65() {
        let mut t1 = [[0u16; N]; 6];
        let mut shake = Shake128::new();
        for row in t1.iter_mut() {
            for c in row.iter_mut() {
                let mut b = [0u8; 2];
                shake.squeeze(&mut b);
                *c = u16::from_le_bytes(b) & 0x3FF;
            }
        }
        let rho = [7u8; 32];
        let mut pk = [0u8; PARAMS_65.public_key_size];
        pk_encode::<6>(&rho, &t1, &mut pk);
        let (rho2, t1_2) = pk_decode::<6>(&PARAMS_65, &pk).unwrap();
        assert_eq!(rho, rho2);
        assert_eq!(t1, t1_2);
    }

    #[test]
    fn bitpack_unpack_round_trip_18() {
        let mut shake = Shake128::new();
        let mut z = Poly::default();
        let b = 1i32 << 17;
        for c in z.coeffs.iter_mut() {
            let mut buf = [0u8; 4];
            shake.squeeze(&mut buf);
            let v = (u32::from_le_bytes(buf) % (2 * b as u32)) as i32 - (b - 1);
            *c = field_to_montgomery(v.rem_euclid(Q as i32) as u32);
        }
        let mut out = [0u8; 18 * N / 8];
        bitpack_18(&z, &mut out);
        let back = bitunpack_18(&out);
        assert_eq!(z, back);
    }

    #[test]
    fn bitpack_unpack_round_trip_20() {
        let mut shake = Shake128::new();
        let mut z = Poly::default();
        let b = 1i32 << 19;
        for c in z.coeffs.iter_mut() {
            let mut buf = [0u8; 4];
            shake.squeeze(&mut buf);
            let v = (u32::from_le_bytes(buf) % (2 * b as u32)) as i32 - (b - 1);
            *c = field_to_montgomery(v.rem_euclid(Q as i32) as u32);
        }
        let mut out = [0u8; 20 * N / 8];
        bitpack_20(&z, &mut out);
        let back = bitunpack_20(&out);
        assert_eq!(z, back);
    }
}
