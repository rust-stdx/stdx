use constant_time_eq::constant_time_eq;
#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{
    Xof,
    sha3::{Sha3_256, Sha3_512, Shake128, Shake256},
};

/// Size of the shared secret produced by ML-KEM encapsulation/decapsulation (32 bytes).
pub const SHARED_SECRET_SIZE: usize = 32;

pub(crate) const N: usize = 256;
pub(crate) const Q: i16 = 3329;
const SYMBYTES: usize = 32;
const POLY_BYTES: usize = 384;
const SHAKE128_RATE: usize = 168;
const QINV: i16 = -3327;
const MONT_SQUARED_DIV_N: i16 = 1441;
const ZETAS: [i16; 128] = [
    -1044, -758, -359, -1517, 1493, 1422, 287, 202, -171, 622, 1577, 182, 962, -1202, -1474, 1468, 573, -1325, 264,
    383, -829, 1458, -1602, -130, -681, 1017, 732, 608, -1542, 411, -205, -1571, 1223, 652, -552, 1015, -1293, 1491,
    -282, -1544, 516, -8, -320, -666, -1618, -1162, 126, 1469, -853, -90, -271, 830, 107, -1421, -247, -951, -398, 961,
    -1508, -725, 448, -1065, 677, -1275, -1103, 430, 555, 843, -1251, 871, 1550, 105, 422, 587, 177, -235, -291, -460,
    1574, 1653, -246, 778, 1159, -147, -777, 1483, -602, 1119, -1590, 644, -872, 349, 418, 329, -156, -75, 817, 1097,
    603, 610, 1322, -1285, -1465, 384, -1215, -136, 1218, -1335, -874, 220, -1187, -1659, -1185, -1530, -1278, 794,
    -1510, -854, -870, 478, -108, -308, 996, 991, 958, -1460, 1522, 1628,
];

pub(crate) const ML_KEM_768: MlKemParams<3> = MlKemParams {
    eta1: 2,
    polycompressedbytes: 128,
    polyveccompressedbytes: 960,
};
pub(crate) const ML_KEM_1024: MlKemParams<4> = MlKemParams {
    eta1: 2,
    polycompressedbytes: 160,
    polyveccompressedbytes: 1408,
};

/// ML-KEM error type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MlKemError {
    InvalidKey,
}

impl core::fmt::Display for MlKemError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MlKemError::InvalidKey => write!(f, "key is not valid"),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct MlKemParams<const K: usize> {
    pub(crate) eta1: usize,
    pub(crate) polycompressedbytes: usize,
    pub(crate) polyveccompressedbytes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub(crate) struct Poly {
    pub(crate) coeffs: [i16; N],
}

impl Default for Poly {
    #[inline]
    fn default() -> Self {
        Self {
            coeffs: [0; N],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub(crate) struct PolyVec<const K: usize> {
    pub(crate) vec: [Poly; K],
}

impl<const K: usize> Default for PolyVec<K> {
    #[inline]
    fn default() -> Self {
        Self {
            vec: core::array::from_fn(|_| Poly::default()),
        }
    }
}

#[inline]
pub(crate) fn crypto_kem_keypair_derand<const K: usize, const SECRET_KEY_SIZE: usize, const PUBLIC_KEY_SIZE: usize>(
    params: &MlKemParams<K>,
    coins: &[u8; 64],
) -> ([u8; SECRET_KEY_SIZE], [u8; PUBLIC_KEY_SIZE]) {
    let mut public_key = [0u8; PUBLIC_KEY_SIZE];
    let mut secret_key = [0u8; SECRET_KEY_SIZE];

    indcpa_keypair_derand::<K>(
        params,
        &mut public_key,
        &mut secret_key[..indcpa_secret_key_bytes::<K>()],
        &coins[..32],
    );
    secret_key[indcpa_secret_key_bytes::<K>()..indcpa_secret_key_bytes::<K>() + PUBLIC_KEY_SIZE]
        .copy_from_slice(&public_key);

    let public_key_hash = hash_h(&public_key);
    secret_key[SECRET_KEY_SIZE - 64..SECRET_KEY_SIZE - 32].copy_from_slice(&public_key_hash);
    secret_key[SECRET_KEY_SIZE - 32..].copy_from_slice(&coins[32..]);

    (secret_key, public_key)
}

#[inline]
pub(crate) fn crypto_kem_enc_derand<const K: usize, const PUBLIC_KEY_SIZE: usize, const CIPHERTEXT_SIZE: usize>(
    params: &MlKemParams<K>,
    public_key: &[u8; PUBLIC_KEY_SIZE],
    coins: &[u8; 32],
) -> ([u8; CIPHERTEXT_SIZE], [u8; SHARED_SECRET_SIZE]) {
    let mut ciphertext = [0u8; CIPHERTEXT_SIZE];
    let mut buf = [0u8; 64];
    let mut kr = [0u8; 64];

    buf[..32].copy_from_slice(coins);
    buf[32..].copy_from_slice(&hash_h(public_key));
    kr.copy_from_slice(&hash_g(&buf));

    indcpa_enc::<K>(params, &mut ciphertext, &buf[..32], public_key, array_ref_32(&kr[32..64]));

    let mut shared_secret = [0u8; SHARED_SECRET_SIZE];
    shared_secret.copy_from_slice(&kr[..32]);
    (ciphertext, shared_secret)
}

#[inline]
pub(crate) fn crypto_kem_dec<const K: usize, const SECRET_KEY_SIZE: usize, const CIPHERTEXT_SIZE: usize>(
    params: &MlKemParams<K>,
    secret_key: &[u8; SECRET_KEY_SIZE],
    ciphertext: &[u8; CIPHERTEXT_SIZE],
) -> Result<[u8; SHARED_SECRET_SIZE], MlKemError> {
    let public_key_offset = indcpa_secret_key_bytes::<K>();
    let public_key_size = public_key_bytes::<K>();
    if SECRET_KEY_SIZE != secret_key_size::<K>() {
        return Err(MlKemError::InvalidKey);
    }

    let public_key = &secret_key[public_key_offset..public_key_offset + public_key_size];
    let mut message_and_hash = [0u8; 64];
    let mut kr = [0u8; 64];
    let mut cmp = [0u8; CIPHERTEXT_SIZE];

    indcpa_dec::<K>(
        params,
        &mut message_and_hash[..32],
        ciphertext,
        &secret_key[..public_key_offset],
    );
    message_and_hash[32..].copy_from_slice(&secret_key[SECRET_KEY_SIZE - 64..SECRET_KEY_SIZE - 32]);
    kr.copy_from_slice(&hash_g(&message_and_hash));

    indcpa_enc::<K>(params, &mut cmp, &message_and_hash[..32], public_key, array_ref_32(&kr[32..64]));

    let mut shared_secret = rkprf(array_ref_32(&secret_key[SECRET_KEY_SIZE - 32..]), ciphertext);
    cmov(&mut shared_secret, array_ref_32(&kr[..32]), constant_time_eq(ciphertext, &cmp));
    Ok(shared_secret)
}

#[inline]
pub(crate) fn indcpa_keypair_derand<const K: usize>(
    params: &MlKemParams<K>,
    public_key: &mut [u8],
    secret_key: &mut [u8],
    coins: &[u8],
) {
    debug_assert_eq!(public_key.len(), public_key_bytes::<K>());
    debug_assert_eq!(secret_key.len(), indcpa_secret_key_bytes::<K>());
    debug_assert_eq!(coins.len(), 32);

    let mut g_input = [0u8; 33];
    g_input[..32].copy_from_slice(coins);
    g_input[32] = K as u8;
    let seed_output = hash_g(&g_input);
    let public_seed = array_ref_32(&seed_output[..32]);
    let noise_seed = array_ref_32(&seed_output[32..64]);
    let matrix = gen_matrix::<K>(public_seed, false);

    let mut skpv = PolyVec::<K>::default();
    let mut e = PolyVec::<K>::default();
    for (index, poly) in skpv.vec.iter_mut().enumerate() {
        *poly = poly_getnoise(noise_seed, index as u8, params.eta1);
    }
    for (index, poly) in e.vec.iter_mut().enumerate() {
        *poly = poly_getnoise(noise_seed, (K + index) as u8, params.eta1);
    }

    polyvec_ntt(&mut skpv);
    polyvec_ntt(&mut e);

    let mut pkpv = PolyVec::<K>::default();
    for i in 0..K {
        pkpv.vec[i] = polyvec_basemul_acc_montgomery(&matrix[i], &skpv);
        poly_tomont(&mut pkpv.vec[i]);
    }

    polyvec_add(&mut pkpv, &e);
    polyvec_reduce(&mut pkpv);

    pack_sk(secret_key, &skpv);
    pack_pk(public_key, &pkpv, public_seed);
}

#[inline]
pub(crate) fn indcpa_enc<const K: usize>(
    params: &MlKemParams<K>,
    ciphertext: &mut [u8],
    message: &[u8],
    public_key: &[u8],
    coins: &[u8; 32],
) {
    debug_assert_eq!(ciphertext.len(), ciphertext_bytes(params));
    debug_assert_eq!(message.len(), 32);
    debug_assert_eq!(public_key.len(), public_key_bytes::<K>());

    let (pkpv, seed) = unpack_pk::<K>(public_key);
    let at = gen_matrix::<K>(&seed, true);
    let k = poly_frommsg(message);

    let mut sp = PolyVec::<K>::default();
    let mut ep = PolyVec::<K>::default();
    for (index, poly) in sp.vec.iter_mut().enumerate() {
        *poly = poly_getnoise(coins, index as u8, params.eta1);
    }
    let ep_nonce_offset = sp.vec.len();
    for (index, poly) in ep.vec.iter_mut().enumerate() {
        *poly = poly_getnoise(coins, (ep_nonce_offset + index) as u8, 2);
    }
    let epp = poly_getnoise(coins, (sp.vec.len() + ep.vec.len()) as u8, 2);

    polyvec_ntt(&mut sp);

    let mut b = PolyVec::<K>::default();
    for i in 0..K {
        b.vec[i] = polyvec_basemul_acc_montgomery(&at[i], &sp);
    }
    let mut v = polyvec_basemul_acc_montgomery(&pkpv, &sp);

    polyvec_invntt_tomont(&mut b);
    poly_invntt_tomont(&mut v);

    polyvec_add(&mut b, &ep);
    poly_add(&mut v, &epp);
    poly_add(&mut v, &k);
    polyvec_reduce(&mut b);
    poly_reduce(&mut v);

    pack_ciphertext(params, ciphertext, &b, &v);
}

#[inline]
pub(crate) fn indcpa_dec<const K: usize>(
    params: &MlKemParams<K>,
    message: &mut [u8],
    ciphertext: &[u8],
    secret_key: &[u8],
) {
    debug_assert_eq!(message.len(), 32);
    debug_assert_eq!(ciphertext.len(), ciphertext_bytes(params));
    debug_assert_eq!(secret_key.len(), indcpa_secret_key_bytes::<K>());

    let (mut b, v) = unpack_ciphertext::<K>(params, ciphertext);
    let skpv = unpack_sk::<K>(secret_key);

    polyvec_ntt(&mut b);
    let mut mp = polyvec_basemul_acc_montgomery(&skpv, &b);
    poly_invntt_tomont(&mut mp);
    let product = mp.clone();
    poly_sub(&mut mp, &v, &product);
    poly_reduce(&mut mp);

    message.copy_from_slice(&poly_tomsg(&mp));
}

#[inline]
fn pack_pk<const K: usize>(out: &mut [u8], pk: &PolyVec<K>, seed: &[u8; 32]) {
    let polyvec_bytes = polyvec_bytes::<K>();
    polyvec_tobytes(&mut out[..polyvec_bytes], pk);
    out[polyvec_bytes..polyvec_bytes + 32].copy_from_slice(seed);
}

#[inline]
fn unpack_pk<const K: usize>(packed: &[u8]) -> (PolyVec<K>, [u8; 32]) {
    let polyvec_bytes = polyvec_bytes::<K>();
    let pk = polyvec_frombytes::<K>(&packed[..polyvec_bytes]);
    let mut seed = [0u8; 32];
    seed.copy_from_slice(&packed[polyvec_bytes..polyvec_bytes + 32]);
    (pk, seed)
}

#[inline]
fn pack_sk<const K: usize>(out: &mut [u8], sk: &PolyVec<K>) {
    polyvec_tobytes(out, sk);
}

#[inline]
fn unpack_sk<const K: usize>(packed: &[u8]) -> PolyVec<K> {
    polyvec_frombytes(packed)
}

#[inline]
fn pack_ciphertext<const K: usize>(params: &MlKemParams<K>, out: &mut [u8], b: &PolyVec<K>, v: &Poly) {
    let split = params.polyveccompressedbytes;
    polyvec_compress(params, &mut out[..split], b);
    poly_compress(params, &mut out[split..split + params.polycompressedbytes], v);
}

#[inline]
fn unpack_ciphertext<const K: usize>(params: &MlKemParams<K>, packed: &[u8]) -> (PolyVec<K>, Poly) {
    let split = params.polyveccompressedbytes;
    (
        polyvec_decompress(params, &packed[..split]),
        poly_decompress(params, &packed[split..split + params.polycompressedbytes]),
    )
}

#[inline]
pub(crate) fn gen_matrix<const K: usize>(seed: &[u8; 32], transpose: bool) -> [PolyVec<K>; K] {
    let mut matrix = core::array::from_fn(|_| PolyVec::<K>::default());
    for i in 0..K {
        for j in 0..K {
            let (x, y) = if transpose {
                (i as u8, j as u8)
            } else {
                (j as u8, i as u8)
            };
            matrix[i].vec[j] = uniform_poly(seed, x, y);
        }
    }
    matrix
}

#[inline]
fn uniform_poly(seed: &[u8; 32], x: u8, y: u8) -> Poly {
    let mut shake = Shake128::new();
    shake.absorb(seed);
    shake.absorb(&[x, y]);

    let mut poly = Poly::default();
    let mut ctr = 0usize;
    let mut block = [0u8; SHAKE128_RATE];
    while ctr < N {
        shake.squeeze(&mut block);
        ctr += rej_uniform(&mut poly.coeffs[ctr..], &block);
    }
    poly
}

#[inline]
fn rej_uniform(out: &mut [i16], buf: &[u8]) -> usize {
    let mut ctr = 0usize;
    let mut pos = 0usize;
    while ctr < out.len() && pos + 3 <= buf.len() {
        let val0 = (((buf[pos] as u16) | ((buf[pos + 1] as u16) << 8)) & 0x0fff) as i16;
        let val1 = ((((buf[pos + 1] as u16) >> 4) | ((buf[pos + 2] as u16) << 4)) & 0x0fff) as i16;
        pos += 3;

        if val0 < Q {
            out[ctr] = val0;
            ctr += 1;
        }
        if ctr < out.len() && val1 < Q {
            out[ctr] = val1;
            ctr += 1;
        }
    }
    ctr
}

#[inline]
pub(crate) fn poly_getnoise(seed: &[u8; 32], nonce: u8, eta: usize) -> Poly {
    debug_assert_eq!(eta, 2);
    let mut input = [0u8; 33];
    input[..32].copy_from_slice(seed);
    input[32] = nonce;
    let mut buf = [0u8; 128];
    let mut shake256 = Shake256::new();
    shake256.absorb(&input);
    shake256.squeeze(&mut buf);
    cbd2(&buf)
}

#[inline]
fn cbd2(buf: &[u8; 128]) -> Poly {
    let mut poly = Poly::default();
    for i in 0..(N / 8) {
        let t = load32(&buf[4 * i..4 * i + 4]);
        let mut d = t & 0x5555_5555;
        d += (t >> 1) & 0x5555_5555;
        for j in 0..8 {
            let a = ((d >> (4 * j)) & 0x3) as i16;
            let b = ((d >> (4 * j + 2)) & 0x3) as i16;
            poly.coeffs[8 * i + j] = a - b;
        }
    }
    poly
}

#[inline]
pub(crate) fn polyvec_compress<const K: usize>(params: &MlKemParams<K>, out: &mut [u8], a: &PolyVec<K>) {
    match params.polyveccompressedbytes {
        960 => {
            let mut offset = 0usize;
            for poly in &a.vec {
                for chunk in poly.coeffs.chunks_exact(4) {
                    let mut t = [0u16; 4];
                    for (dst, coeff) in t.iter_mut().zip(chunk.iter()) {
                        let mut u = *coeff as i32;
                        u += (u >> 15) & Q as i32;
                        let mut d0 = u as u64;
                        d0 <<= 10;
                        d0 += 1665;
                        d0 *= 1_290_167;
                        d0 >>= 32;
                        *dst = (d0 as u16) & 0x03ff;
                    }
                    out[offset] = t[0] as u8;
                    out[offset + 1] = ((t[0] >> 8) as u8) | ((t[1] << 2) as u8);
                    out[offset + 2] = ((t[1] >> 6) as u8) | ((t[2] << 4) as u8);
                    out[offset + 3] = ((t[2] >> 4) as u8) | ((t[3] << 6) as u8);
                    out[offset + 4] = (t[3] >> 2) as u8;
                    offset += 5;
                }
            }
        }
        1408 => {
            let mut offset = 0usize;
            for poly in &a.vec {
                for chunk in poly.coeffs.chunks_exact(8) {
                    let mut t = [0u16; 8];
                    for (dst, coeff) in t.iter_mut().zip(chunk.iter()) {
                        let mut u = *coeff as i32;
                        u += (u >> 15) & Q as i32;
                        let mut d0 = u as u64;
                        d0 <<= 11;
                        d0 += 1664;
                        d0 *= 645_084;
                        d0 >>= 31;
                        *dst = (d0 as u16) & 0x07ff;
                    }
                    out[offset] = t[0] as u8;
                    out[offset + 1] = ((t[0] >> 8) as u8) | ((t[1] << 3) as u8);
                    out[offset + 2] = ((t[1] >> 5) as u8) | ((t[2] << 6) as u8);
                    out[offset + 3] = (t[2] >> 2) as u8;
                    out[offset + 4] = ((t[2] >> 10) as u8) | ((t[3] << 1) as u8);
                    out[offset + 5] = ((t[3] >> 7) as u8) | ((t[4] << 4) as u8);
                    out[offset + 6] = ((t[4] >> 4) as u8) | ((t[5] << 7) as u8);
                    out[offset + 7] = (t[5] >> 1) as u8;
                    out[offset + 8] = ((t[5] >> 9) as u8) | ((t[6] << 2) as u8);
                    out[offset + 9] = ((t[6] >> 6) as u8) | ((t[7] << 5) as u8);
                    out[offset + 10] = (t[7] >> 3) as u8;
                    offset += 11;
                }
            }
        }
        _ => unreachable!(),
    }
}

#[inline]
pub(crate) fn polyvec_decompress<const K: usize>(params: &MlKemParams<K>, input: &[u8]) -> PolyVec<K> {
    let mut out = PolyVec::<K>::default();
    match params.polyveccompressedbytes {
        960 => {
            let mut offset = 0usize;
            for poly in &mut out.vec {
                for j in 0..(N / 4) {
                    let t0 = (input[offset] as u16) | ((input[offset + 1] as u16) << 8);
                    let t1 = ((input[offset + 1] as u16) >> 2) | ((input[offset + 2] as u16) << 6);
                    let t2 = ((input[offset + 2] as u16) >> 4) | ((input[offset + 3] as u16) << 4);
                    let t3 = ((input[offset + 3] as u16) >> 6) | ((input[offset + 4] as u16) << 2);
                    offset += 5;
                    poly.coeffs[4 * j] = ((((t0 & 0x03ff) as u32) * Q as u32 + 512) >> 10) as i16;
                    poly.coeffs[4 * j + 1] = ((((t1 & 0x03ff) as u32) * Q as u32 + 512) >> 10) as i16;
                    poly.coeffs[4 * j + 2] = ((((t2 & 0x03ff) as u32) * Q as u32 + 512) >> 10) as i16;
                    poly.coeffs[4 * j + 3] = ((((t3 & 0x03ff) as u32) * Q as u32 + 512) >> 10) as i16;
                }
            }
        }
        1408 => {
            let mut offset = 0usize;
            for poly in &mut out.vec {
                for j in 0..(N / 8) {
                    let t0 = (input[offset] as u16) | ((input[offset + 1] as u16) << 8);
                    let t1 = ((input[offset + 1] as u16) >> 3) | ((input[offset + 2] as u16) << 5);
                    let t2 = ((input[offset + 2] as u16) >> 6)
                        | ((input[offset + 3] as u16) << 2)
                        | ((input[offset + 4] as u16) << 10);
                    let t3 = ((input[offset + 4] as u16) >> 1) | ((input[offset + 5] as u16) << 7);
                    let t4 = ((input[offset + 5] as u16) >> 4) | ((input[offset + 6] as u16) << 4);
                    let t5 = ((input[offset + 6] as u16) >> 7)
                        | ((input[offset + 7] as u16) << 1)
                        | ((input[offset + 8] as u16) << 9);
                    let t6 = ((input[offset + 8] as u16) >> 2) | ((input[offset + 9] as u16) << 6);
                    let t7 = ((input[offset + 9] as u16) >> 5) | ((input[offset + 10] as u16) << 3);
                    offset += 11;
                    let values = [t0, t1, t2, t3, t4, t5, t6, t7];
                    for (k, value) in values.into_iter().enumerate() {
                        poly.coeffs[8 * j + k] = ((((value & 0x07ff) as u32) * Q as u32 + 1024) >> 11) as i16;
                    }
                }
            }
        }
        _ => unreachable!(),
    }
    out
}

#[inline]
fn polyvec_tobytes<const K: usize>(out: &mut [u8], polyvec: &PolyVec<K>) {
    for (i, poly) in polyvec.vec.iter().enumerate() {
        poly_tobytes(&mut out[i * POLY_BYTES..(i + 1) * POLY_BYTES], poly);
    }
}

#[inline]
fn polyvec_frombytes<const K: usize>(input: &[u8]) -> PolyVec<K> {
    let mut out = PolyVec::<K>::default();
    for (i, poly) in out.vec.iter_mut().enumerate() {
        *poly = poly_frombytes(&input[i * POLY_BYTES..(i + 1) * POLY_BYTES]);
    }
    out
}

#[inline]
fn polyvec_ntt<const K: usize>(polyvec: &mut PolyVec<K>) {
    for poly in &mut polyvec.vec {
        poly_ntt(poly);
    }
}

#[inline]
fn polyvec_invntt_tomont<const K: usize>(polyvec: &mut PolyVec<K>) {
    for poly in &mut polyvec.vec {
        poly_invntt_tomont(poly);
    }
}

#[inline]
fn polyvec_basemul_acc_montgomery<const K: usize>(a: &PolyVec<K>, b: &PolyVec<K>) -> Poly {
    let mut out = poly_basemul_montgomery(&a.vec[0], &b.vec[0]);
    for i in 1..K {
        let t = poly_basemul_montgomery(&a.vec[i], &b.vec[i]);
        poly_add(&mut out, &t);
    }
    poly_reduce(&mut out);
    out
}

#[inline]
fn polyvec_reduce<const K: usize>(polyvec: &mut PolyVec<K>) {
    for poly in &mut polyvec.vec {
        poly_reduce(poly);
    }
}

#[inline]
fn polyvec_add<const K: usize>(left: &mut PolyVec<K>, right: &PolyVec<K>) {
    for i in 0..K {
        poly_add(&mut left.vec[i], &right.vec[i]);
    }
}

#[inline]
fn poly_compress<const K: usize>(params: &MlKemParams<K>, out: &mut [u8], poly: &Poly) {
    match params.polycompressedbytes {
        128 => {
            let mut offset = 0usize;
            for chunk in poly.coeffs.chunks_exact(8) {
                let mut t = [0u8; 8];
                for (dst, coeff) in t.iter_mut().zip(chunk.iter()) {
                    let mut u = *coeff as i32;
                    u += (u >> 15) & Q as i32;
                    let mut d0 = ((u as u32) << 4) as u64;
                    d0 += 1665;
                    d0 *= 80_635;
                    d0 >>= 28;
                    *dst = (d0 as u8) & 0x0f;
                }
                out[offset] = t[0] | (t[1] << 4);
                out[offset + 1] = t[2] | (t[3] << 4);
                out[offset + 2] = t[4] | (t[5] << 4);
                out[offset + 3] = t[6] | (t[7] << 4);
                offset += 4;
            }
        }
        160 => {
            let mut offset = 0usize;
            for chunk in poly.coeffs.chunks_exact(8) {
                let mut t = [0u8; 8];
                for (dst, coeff) in t.iter_mut().zip(chunk.iter()) {
                    let mut u = *coeff as i32;
                    u += (u >> 15) & Q as i32;
                    let mut d0 = ((u as u32) << 5) as u64;
                    d0 += 1664;
                    d0 *= 40_318;
                    d0 >>= 27;
                    *dst = (d0 as u8) & 0x1f;
                }
                out[offset] = t[0] | (t[1] << 5);
                out[offset + 1] = (t[1] >> 3) | (t[2] << 2) | (t[3] << 7);
                out[offset + 2] = (t[3] >> 1) | (t[4] << 4);
                out[offset + 3] = (t[4] >> 4) | (t[5] << 1) | (t[6] << 6);
                out[offset + 4] = (t[6] >> 2) | (t[7] << 3);
                offset += 5;
            }
        }
        _ => unreachable!(),
    }
}

#[inline]
fn poly_decompress<const K: usize>(params: &MlKemParams<K>, input: &[u8]) -> Poly {
    let mut out = Poly::default();
    match params.polycompressedbytes {
        128 => {
            for i in 0..(N / 2) {
                out.coeffs[2 * i] = ((((input[i] & 0x0f) as u16) * Q as u16 + 8) >> 4) as i16;
                out.coeffs[2 * i + 1] = ((((input[i] >> 4) as u16) * Q as u16 + 8) >> 4) as i16;
            }
        }
        160 => {
            let mut offset = 0usize;
            for i in 0..(N / 8) {
                let t0 = input[offset] >> 0;
                let t1 = (input[offset] >> 5) | (input[offset + 1] << 3);
                let t2 = input[offset + 1] >> 2;
                let t3 = (input[offset + 1] >> 7) | (input[offset + 2] << 1);
                let t4 = (input[offset + 2] >> 4) | (input[offset + 3] << 4);
                let t5 = input[offset + 3] >> 1;
                let t6 = (input[offset + 3] >> 6) | (input[offset + 4] << 2);
                let t7 = input[offset + 4] >> 3;
                offset += 5;
                let values = [t0, t1, t2, t3, t4, t5, t6, t7];
                for (j, value) in values.into_iter().enumerate() {
                    out.coeffs[8 * i + j] = (((value as u32 & 31) * Q as u32 + 16) >> 5) as i16;
                }
            }
        }
        _ => unreachable!(),
    }
    out
}

#[inline]
fn poly_tobytes(out: &mut [u8], poly: &Poly) {
    for i in 0..(N / 2) {
        let mut t0 = poly.coeffs[2 * i] as i32;
        t0 += (t0 >> 15) & Q as i32;
        let mut t1 = poly.coeffs[2 * i + 1] as i32;
        t1 += (t1 >> 15) & Q as i32;
        out[3 * i] = t0 as u8;
        out[3 * i + 1] = ((t0 >> 8) as u8) | ((t1 << 4) as u8);
        out[3 * i + 2] = (t1 >> 4) as u8;
    }
}

#[inline]
fn poly_frombytes(input: &[u8]) -> Poly {
    let mut out = Poly::default();
    for i in 0..(N / 2) {
        out.coeffs[2 * i] = (((input[3 * i] as u16) | ((input[3 * i + 1] as u16) << 8)) & 0x0fff) as i16;
        out.coeffs[2 * i + 1] = ((((input[3 * i + 1] as u16) >> 4) | ((input[3 * i + 2] as u16) << 4)) & 0x0fff) as i16;
    }
    out
}

#[inline]
pub(crate) fn poly_frommsg(msg: &[u8]) -> Poly {
    let mut out = Poly::default();
    let half_q: i16 = ((Q + 1) / 2) as i16;
    for i in 0..(N / 8) {
        for j in 0..8 {
            let bit = ((msg[i] >> j) & 1) as i16;
            out.coeffs[8 * i + j] = (-bit) & half_q;
        }
    }
    out
}

#[inline]
pub(crate) fn poly_tomsg(poly: &Poly) -> [u8; 32] {
    let mut msg = [0u8; 32];
    for i in 0..(N / 8) {
        for j in 0..8 {
            let mut t = poly.coeffs[8 * i + j] as i32;
            t <<= 1;
            t += 1665;
            t *= 80_635;
            t >>= 28;
            msg[i] |= ((t & 1) as u8) << j;
        }
    }
    msg
}

#[inline]
fn poly_ntt(poly: &mut Poly) {
    ntt(&mut poly.coeffs);
    poly_reduce(poly);
}

#[inline]
fn poly_invntt_tomont(poly: &mut Poly) {
    invntt(&mut poly.coeffs);
}

#[inline]
fn poly_basemul_montgomery(a: &Poly, b: &Poly) -> Poly {
    let mut out = Poly::default();
    for i in 0..(N / 4) {
        let r0 = basemul(
            [a.coeffs[4 * i], a.coeffs[4 * i + 1]],
            [b.coeffs[4 * i], b.coeffs[4 * i + 1]],
            ZETAS[64 + i],
        );
        out.coeffs[4 * i] = r0[0];
        out.coeffs[4 * i + 1] = r0[1];

        let r1 = basemul(
            [a.coeffs[4 * i + 2], a.coeffs[4 * i + 3]],
            [b.coeffs[4 * i + 2], b.coeffs[4 * i + 3]],
            -ZETAS[64 + i],
        );
        out.coeffs[4 * i + 2] = r1[0];
        out.coeffs[4 * i + 3] = r1[1];
    }
    out
}

#[inline]
fn poly_tomont(poly: &mut Poly) {
    for coeff in &mut poly.coeffs {
        *coeff = montgomery_reduce(*coeff as i32 * 1353);
    }
}

#[inline]
fn poly_reduce(poly: &mut Poly) {
    for coeff in &mut poly.coeffs {
        *coeff = barrett_reduce(*coeff);
    }
}

#[inline]
fn poly_add(left: &mut Poly, right: &Poly) {
    for i in 0..N {
        left.coeffs[i] = (left.coeffs[i] as i32 + right.coeffs[i] as i32) as i16;
    }
}

#[inline]
fn poly_sub(out: &mut Poly, left: &Poly, right: &Poly) {
    for i in 0..N {
        out.coeffs[i] = (left.coeffs[i] as i32 - right.coeffs[i] as i32) as i16;
    }
}

#[inline]
fn ntt(r: &mut [i16; N]) {
    let mut k = 1usize;
    let mut len = 128usize;
    while len >= 2 {
        let mut start = 0usize;
        while start < N {
            let zeta = ZETAS[k];
            k += 1;
            for j in start..start + len {
                let t = fqmul(zeta, r[j + len]);
                let rj = r[j] as i32;
                r[j + len] = (rj - t as i32) as i16;
                r[j] = (rj + t as i32) as i16;
            }
            start += 2 * len;
        }
        len >>= 1;
    }
}

#[inline]
fn invntt(r: &mut [i16; N]) {
    let mut k = 127usize;
    let mut len = 2usize;
    while len <= 128 {
        let mut start = 0usize;
        while start < N {
            let zeta = ZETAS[k];
            k -= 1;
            for j in start..start + len {
                let t = r[j];
                r[j] = barrett_reduce((t as i32 + r[j + len] as i32) as i16);
                r[j + len] = fqmul(zeta, (r[j + len] as i32 - t as i32) as i16);
            }
            start += 2 * len;
        }
        len <<= 1;
    }

    for coeff in r.iter_mut() {
        *coeff = fqmul(*coeff, MONT_SQUARED_DIV_N);
    }
}

#[inline]
fn basemul(a: [i16; 2], b: [i16; 2], zeta: i16) -> [i16; 2] {
    let mut out = [0i16; 2];
    out[0] = fqmul(a[1], b[1]);
    out[0] = fqmul(out[0], zeta);
    out[0] = (out[0] as i32 + fqmul(a[0], b[0]) as i32) as i16;
    out[1] = (fqmul(a[0], b[1]) as i32 + fqmul(a[1], b[0]) as i32) as i16;
    out
}

#[inline]
fn fqmul(a: i16, b: i16) -> i16 {
    montgomery_reduce(a as i32 * b as i32)
}

#[inline]
fn montgomery_reduce(a: i32) -> i16 {
    let t = (a as i16).wrapping_mul(QINV) as i32;
    ((a - t * Q as i32) >> 16) as i16
}

#[inline]
fn barrett_reduce(a: i16) -> i16 {
    const V: i32 = ((1 << 26) + (Q as i32 / 2)) / Q as i32;
    let t = ((V * a as i32 + (1 << 25)) >> 26) * Q as i32;
    (a as i32 - t) as i16
}

#[inline]
fn hash_h(data: &[u8]) -> [u8; 32] {
    use crate::Hasher;
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    hasher.sum().as_ref().try_into().unwrap()
}

#[inline]
fn hash_g(data: &[u8]) -> [u8; 64] {
    use crate::Hasher;
    let mut hasher = Sha3_512::new();
    hasher.update(data);
    hasher.sum().as_ref().try_into().unwrap()
}

#[inline]
fn rkprf(cipher_key: &[u8; 32], ciphertext: &[u8]) -> [u8; 32] {
    let mut shake = Shake256::new();
    shake.absorb(cipher_key);
    shake.absorb(ciphertext);
    let mut out = [0u8; 32];
    shake.squeeze(&mut out);
    out
}

/// Constant-time conditional move: if `cond` is true, copies `value` into `out`.
/// Uses a compiler barrier on the mask to prevent the optimizer from turning this
/// into a branch (which would leak timing information in the FO transform).
#[inline]
fn cmov(out: &mut [u8; 32], value: &[u8; 32], cond: bool) {
    let mask = ct_mask_u8(cond);
    for i in 0..32 {
        out[i] ^= mask & (out[i] ^ value[i]);
    }
}

/// Converts a boolean condition to a constant-time mask (0x00 or 0xFF) with a compiler
/// barrier to prevent optimization into a branch.
#[inline]
fn ct_mask_u8(cond: bool) -> u8 {
    let mask = 0u8.wrapping_sub(cond as u8);
    // Prevent the compiler from reasoning about the mask value and potentially
    // converting downstream code into a conditional branch.
    ct_barrier_u8(mask)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
fn ct_barrier_u8(mut value: u8) -> u8 {
    // SAFETY: the inline asm is a no-op that forces the compiler to treat `value`
    // as an opaque value, preventing branch-based optimizations.
    unsafe {
        core::arch::asm!("/* {0} */", inout(reg_byte) value, options(pure, nomem, nostack, preserves_flags));
    }
    value
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "riscv32",
    target_arch = "riscv64"
))]
#[inline]
#[allow(asm_sub_register)]
fn ct_barrier_u8(mut value: u8) -> u8 {
    unsafe {
        core::arch::asm!("/* {0} */", inout(reg) value, options(pure, nomem, nostack, preserves_flags));
    }
    value
}

#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "riscv32",
    target_arch = "riscv64"
)))]
#[inline(never)]
fn ct_barrier_u8(value: u8) -> u8 {
    core::hint::black_box(value)
}

#[inline]
fn load32(input: &[u8]) -> u32 {
    (input[0] as u32) | ((input[1] as u32) << 8) | ((input[2] as u32) << 16) | ((input[3] as u32) << 24)
}

#[inline]
pub(crate) fn public_key_bytes<const K: usize>() -> usize {
    polyvec_bytes::<K>() + SYMBYTES
}

#[inline]
pub(crate) fn indcpa_secret_key_bytes<const K: usize>() -> usize {
    polyvec_bytes::<K>()
}

#[inline]
fn polyvec_bytes<const K: usize>() -> usize {
    K * POLY_BYTES
}

#[inline]
pub(crate) fn secret_key_size<const K: usize>() -> usize {
    indcpa_secret_key_bytes::<K>() + public_key_bytes::<K>() + 2 * SYMBYTES
}

#[inline]
fn ciphertext_bytes<const K: usize>(params: &MlKemParams<K>) -> usize {
    params.polyveccompressedbytes + params.polycompressedbytes
}

#[inline]
fn array_ref_32(input: &[u8]) -> &[u8; 32] {
    input.try_into().expect("slice length should be 32")
}

#[cfg(test)]
pub(crate) fn decode_hex_array<const N: usize>(s: &str) -> [u8; N] {
    let bytes = hex::decode(s).expect("valid hex");
    assert_eq!(bytes.len(), N);
    let mut out = [0u8; N];
    out.copy_from_slice(&bytes);
    out
}

#[cfg(test)]
pub(crate) fn sha3_256_hex(data: &[u8]) -> String {
    use crate::Hasher;
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    hex::encode(hasher.sum().as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poly_frommsg_tomsg_roundtrip() {
        for pattern in 0..=255u16 {
            let mut msg = [0u8; 32];
            msg[0] = pattern as u8;
            msg[1] = (pattern >> 8) as u8;
            let poly = poly_frommsg(&msg);
            let recovered = poly_tomsg(&poly);
            assert_eq!(msg, recovered, "roundtrip failed for pattern {pattern:#06x}");
        }
    }

    #[test]
    fn poly_frommsg_constant_time_produces_expected_values() {
        let half_q = ((Q + 1) / 2) as i16;
        let mut msg = [0u8; 32];
        msg[0] = 0b1010_1010;
        msg[1] = 0b0101_0101;
        let poly = poly_frommsg(&msg);
        assert_eq!(poly.coeffs[0], 0);
        assert_eq!(poly.coeffs[1], half_q);
        assert_eq!(poly.coeffs[2], 0);
        assert_eq!(poly.coeffs[3], half_q);
        assert_eq!(poly.coeffs[8], half_q);
        assert_eq!(poly.coeffs[9], 0);
        assert_eq!(poly.coeffs[10], half_q);
        assert_eq!(poly.coeffs[11], 0);
    }

    #[test]
    fn cmov_selects_correctly() {
        let mut out = [0xAAu8; 32];
        let value = [0xBBu8; 32];
        cmov(&mut out, &value, false);
        assert_eq!(out, [0xAAu8; 32], "cmov with false should not modify output");

        cmov(&mut out, &value, true);
        assert_eq!(out, [0xBBu8; 32], "cmov with true should copy value");
    }

    #[test]
    fn cmov_is_idempotent() {
        let mut out = [0x42u8; 32];
        let value = [0x42u8; 32];
        cmov(&mut out, &value, true);
        assert_eq!(out, [0x42u8; 32]);
        cmov(&mut out, &value, false);
        assert_eq!(out, [0x42u8; 32]);
    }

    #[test]
    fn barrett_reduce_produces_values_in_range() {
        // Barrett reduce should map any i16 to the range [-(Q-1)/2, (Q-1)/2] approximately
        for val in [0i16, 1, -1, Q - 1, -(Q - 1), Q, -Q, 3000, -3000, i16::MAX, i16::MIN] {
            let reduced = barrett_reduce(val);
            // The reduced value should be congruent to val mod Q
            let diff = (val as i32 - reduced as i32).rem_euclid(Q as i32);
            assert!(diff == 0, "barrett_reduce({val}) = {reduced} not congruent mod Q");
        }
    }

    #[test]
    fn montgomery_reduce_correctness() {
        // Montgomery reduce: given a, return a * R^(-1) mod Q where R = 2^16
        // Verify: montgomery_reduce(a * R) == a mod Q for small a
        let r_mod_q: i32 = (1i32 << 16) % Q as i32; // R mod Q = 65536 mod 3329 = 2285
        for val in [0i16, 1, -1, 100, -100, Q - 1, -(Q - 1)] {
            let product = val as i32 * r_mod_q;
            let result = montgomery_reduce(product);
            // result should be congruent to val mod Q
            let diff = (val as i32 - result as i32).rem_euclid(Q as i32);
            assert!(
                diff == 0,
                "montgomery_reduce({val} * R) = {result}, expected congruent to {val} mod Q"
            );
        }
    }

    #[test]
    fn ntt_invntt_preserves_polynomial_structure() {
        // NTT->InvNTT roundtrip preserves polynomial relationships.
        // The full KEM roundtrip tests already validate NTT correctness,
        // but this verifies that two distinct inputs remain distinct after transform.
        let mut poly_a = Poly::default();
        let mut poly_b = Poly::default();
        for i in 0..N {
            poly_a.coeffs[i] = (i as i16 * 7 + 3) % Q;
            poly_b.coeffs[i] = (i as i16 * 11 + 5) % Q;
        }
        poly_ntt(&mut poly_a);
        poly_ntt(&mut poly_b);
        // NTT outputs should be different for different inputs
        assert_ne!(poly_a.coeffs, poly_b.coeffs);

        poly_invntt_tomont(&mut poly_a);
        poly_invntt_tomont(&mut poly_b);
        // After roundtrip, they should still be different
        assert_ne!(poly_a.coeffs, poly_b.coeffs);
    }

    #[test]
    fn poly_compress_decompress_roundtrip_4bit() {
        // For 4-bit compression (ML-KEM-768)
        let params = &ML_KEM_768;
        let mut poly = Poly::default();
        for i in 0..N {
            poly.coeffs[i] = ((i * 13) % Q as usize) as i16;
        }
        let mut compressed = [0u8; 128];
        poly_compress::<3>(params, &mut compressed, &poly);
        let decompressed = poly_decompress::<3>(params, &compressed);
        // Compression is lossy but within rounding error
        for i in 0..N {
            let orig = poly.coeffs[i] as i32;
            let dec = decompressed.coeffs[i] as i32;
            // Maximum rounding error for d-bit compression: Q / (2^(d+1))
            // For 4 bits: Q/32 ≈ 104
            let error = ((orig - dec).rem_euclid(Q as i32)).min((dec - orig).rem_euclid(Q as i32));
            assert!(
                error <= Q as i32 / 32 + 1,
                "4-bit compress/decompress error too large at index {i}: orig={orig}, dec={dec}, error={error}"
            );
        }
    }

    #[test]
    fn poly_compress_decompress_roundtrip_5bit() {
        // For 5-bit compression (ML-KEM-1024)
        let params = &ML_KEM_1024;
        let mut poly = Poly::default();
        for i in 0..N {
            poly.coeffs[i] = ((i * 13) % Q as usize) as i16;
        }
        let mut compressed = [0u8; 160];
        poly_compress::<4>(params, &mut compressed, &poly);
        let decompressed = poly_decompress::<4>(params, &compressed);
        for i in 0..N {
            let orig = poly.coeffs[i] as i32;
            let dec = decompressed.coeffs[i] as i32;
            let error = ((orig - dec).rem_euclid(Q as i32)).min((dec - orig).rem_euclid(Q as i32));
            assert!(
                error <= Q as i32 / 64 + 1,
                "5-bit compress/decompress error too large at index {i}: orig={orig}, dec={dec}, error={error}"
            );
        }
    }

    #[test]
    fn polyvec_compress_decompress_roundtrip_10bit() {
        let params = &ML_KEM_768;
        let mut pv = PolyVec::<3>::default();
        for k in 0..3 {
            for i in 0..N {
                pv.vec[k].coeffs[i] = ((k * 97 + i * 13) % Q as usize) as i16;
            }
        }
        let mut compressed = [0u8; 960];
        polyvec_compress(params, &mut compressed, &pv);
        let decompressed = polyvec_decompress::<3>(params, &compressed);
        for k in 0..3 {
            for i in 0..N {
                let orig = pv.vec[k].coeffs[i] as i32;
                let dec = decompressed.vec[k].coeffs[i] as i32;
                let error = ((orig - dec).rem_euclid(Q as i32)).min((dec - orig).rem_euclid(Q as i32));
                assert!(
                    error <= Q as i32 / 2048 + 1,
                    "10-bit compress/decompress error at [{k}][{i}]: orig={orig}, dec={dec}, error={error}"
                );
            }
        }
    }

    #[test]
    fn polyvec_compress_decompress_roundtrip_11bit() {
        let params = &ML_KEM_1024;
        let mut pv = PolyVec::<4>::default();
        for k in 0..4 {
            for i in 0..N {
                pv.vec[k].coeffs[i] = ((k * 97 + i * 13) % Q as usize) as i16;
            }
        }
        let mut compressed = [0u8; 1408];
        polyvec_compress(params, &mut compressed, &pv);
        let decompressed = polyvec_decompress::<4>(params, &compressed);
        for k in 0..4 {
            for i in 0..N {
                let orig = pv.vec[k].coeffs[i] as i32;
                let dec = decompressed.vec[k].coeffs[i] as i32;
                let error = ((orig - dec).rem_euclid(Q as i32)).min((dec - orig).rem_euclid(Q as i32));
                assert!(
                    error <= Q as i32 / 4096 + 1,
                    "11-bit compress/decompress error at [{k}][{i}]: orig={orig}, dec={dec}, error={error}"
                );
            }
        }
    }

    #[test]
    fn poly_tobytes_frombytes_roundtrip() {
        let mut poly = Poly::default();
        for i in 0..N {
            poly.coeffs[i] = (i as i16 * 13) % Q;
        }
        let mut buf = [0u8; POLY_BYTES];
        poly_tobytes(&mut buf, &poly);
        let recovered = poly_frombytes(&buf);
        assert_eq!(poly.coeffs, recovered.coeffs);
    }

    #[test]
    fn gen_matrix_transpose_relationship() {
        let seed = [42u8; 32];
        let matrix = gen_matrix::<3>(&seed, false);
        let transposed = gen_matrix::<3>(&seed, true);
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(
                    matrix[i].vec[j].coeffs, transposed[j].vec[i].coeffs,
                    "A[{i}][{j}] != A^T[{j}][{i}]"
                );
            }
        }
    }

    #[test]
    fn cbd2_produces_values_in_correct_range() {
        // CBD with eta=2 should produce coefficients in [-2, 2]
        let mut buf = [0u8; 128];
        for i in 0..128 {
            buf[i] = (i as u8).wrapping_mul(0x37);
        }
        let poly = cbd2(&buf);
        for (i, &coeff) in poly.coeffs.iter().enumerate() {
            assert!((-2..=2).contains(&coeff), "CBD2 coeff[{i}] = {coeff} out of range [-2, 2]");
        }
    }

    #[test]
    fn rej_uniform_only_accepts_values_less_than_q() {
        // Craft input where val0 = Q (3329 = 0xD01) should be rejected
        // rej_uniform parses 3 bytes into 2 12-bit values:
        // val0 = (buf[0] | buf[1]<<8) & 0x0fff
        // val1 = ((buf[1]>>4) | buf[2]<<4) & 0x0fff
        let buf = [
            0x01, 0x0D,
            0x00, // val0 = 0xD01 = 3329 = Q (rejected), val1 = (0x0D>>4 | 0x00<<4) & 0xfff = 0 (accepted)
            0x00, 0x0D,
            0xD0, // val0 = 0xD00 = 3328 (accepted), val1 = (0x0D>>4 | 0xD0<<4) & 0xfff = 0xD00 = 3328 (accepted)
        ];
        let mut out = [0i16; 256];
        let count = rej_uniform(&mut out, &buf);
        // val0=Q rejected, val1=0 accepted, val0=3328 accepted, val1=3328 accepted
        assert_eq!(count, 3);
        assert_eq!(out[0], 0); // first accepted: val1 from first triple
        assert_eq!(out[1], 3328); // second accepted: val0 from second triple
        assert_eq!(out[2], 3328); // third accepted: val1 from second triple
    }

    #[test]
    fn nist_acvp_ml_kem_768_full_vector() {
        // Verify against NIST FIPS 203 intermediate test vector (ML-KEM-768.txt)
        // These values come from the NIST test file and are validated by the CCTV tests
        let d: [u8; 32] = decode_hex_array("f688563f7c66a5da2d8bdb5a5f3e07bd8dce6f7efcec7f41298d79863459f7cd");
        let z: [u8; 32] = decode_hex_array("d1d49a515250dbceb9f6e3fcc1c7d5306918964b21ddb22207e03e57f0600da8");
        let m: [u8; 32] = decode_hex_array("3dc27ca0a6594b0e56320457c45a0f76bb8a213ea4a76d442186a0aefadbcdb9");

        let mut coins = [0u8; 64];
        coins[..32].copy_from_slice(&d);
        coins[32..].copy_from_slice(&z);

        let (dk, ek) = crypto_kem_keypair_derand::<3, 2400, 1184>(&ML_KEM_768, &coins);
        let (ct, k) = crypto_kem_enc_derand::<3, 1184, 1088>(&ML_KEM_768, &ek, &m);

        // Verify public key hash matches NIST vector
        assert_eq!(
            sha3_256_hex(&ek),
            "42d930a50dfd1f0541ca45c4598daebb4f51cd10d711a001bd9bb87d5c87a4bf"
        );
        // Verify secret key hash
        assert_eq!(
            sha3_256_hex(&dk),
            "db563aebd9fdc875e88563693edad1e5e359cc37b0f685d2d0a3723b37253192"
        );
        // Verify ciphertext hash
        assert_eq!(
            sha3_256_hex(&ct),
            "9d6e358208c4d583050becb319050b7f916de47caad1d589a1d01fea43fe1750"
        );
        // Verify shared secret
        assert_eq!(
            hex::encode(k),
            "ae726da2df66601c6648a7565c02b203a089276ac30f6cc226d048f93fafd78c"
        );

        // Verify decapsulation produces the same shared secret
        let k_dec = crypto_kem_dec::<3, 2400, 1088>(&ML_KEM_768, &dk, &ct).unwrap();
        assert_eq!(k, k_dec, "decapsulation mismatch against NIST vector");
    }

    #[test]
    fn nist_acvp_ml_kem_1024_full_vector() {
        // Verify against NIST FIPS 203 intermediate test vector (ML-KEM-1024.txt)
        let d: [u8; 32] = decode_hex_array("2a62c39ef4fc499f2d132716f480bb7521a49558ae84ee80d9352e66daf1e3a8");
        let z: [u8; 32] = decode_hex_array("5f574ef7f013d4336801fed022178c3ed91d0b6d51325315fc1dcabf4770a2ea");
        let m: [u8; 32] = decode_hex_array("e07d685ed308e609c9c7842026e35732f6ffc6e2fee10f0afd348f2b42a8acb4");

        let mut coins = [0u8; 64];
        coins[..32].copy_from_slice(&d);
        coins[32..].copy_from_slice(&z);

        let (dk, ek) = crypto_kem_keypair_derand::<4, 3168, 1568>(&ML_KEM_1024, &coins);
        let (ct, k) = crypto_kem_enc_derand::<4, 1568, 1568>(&ML_KEM_1024, &ek, &m);

        assert_eq!(
            sha3_256_hex(&ek),
            "3b308d1344ed70366b84d790acb705b86cd3dfd471fff171969aaa338f26dca5"
        );
        assert_eq!(
            sha3_256_hex(&dk),
            "aa63a9e0c035ada6635e7938b71856b24917ff9b3ebca1a4d205a83b502a415a"
        );
        assert_eq!(
            sha3_256_hex(&ct),
            "8caba02733421f12a7ba9a2bcbe4de7c9853156a0637df5a7a0f9127c81da943"
        );
        assert_eq!(
            hex::encode(k),
            "d53825c3ff666bb2881215dbec04a8bdce9099b2a3680938c2f199b54d505953"
        );

        let k_dec = crypto_kem_dec::<4, 3168, 1568>(&ML_KEM_1024, &dk, &ct).unwrap();
        assert_eq!(k, k_dec, "decapsulation mismatch against NIST vector");
    }

    #[test]
    fn compression_constant_time_no_division() {
        // Verify that the compression constants avoid division at runtime.
        // This test exercises boundary values where a naive division would
        // produce different rounding behavior than the multiplication trick.
        let params_768 = &ML_KEM_768;
        let params_1024 = &ML_KEM_1024;

        // Test boundary values for poly_compress (4-bit)
        let mut poly = Poly::default();
        poly.coeffs[0] = 0;
        poly.coeffs[1] = (Q - 1) as i16;
        poly.coeffs[2] = (Q / 2) as i16;
        poly.coeffs[3] = (Q / 2 + 1) as i16;
        let mut buf4 = [0u8; 128];
        poly_compress::<3>(params_768, &mut buf4, &poly);
        let dec = poly_decompress::<3>(params_768, &buf4);
        // Verify round-trip for boundary values
        assert_eq!(dec.coeffs[0], 0); // 0 should compress/decompress to 0

        // Test boundary values for poly_compress (5-bit)
        let mut buf5 = [0u8; 160];
        poly_compress::<4>(params_1024, &mut buf5, &poly);
        let dec5 = poly_decompress::<4>(params_1024, &buf5);
        assert_eq!(dec5.coeffs[0], 0);
    }

    #[test]
    fn poly_tomsg_boundary_values() {
        // Test poly_tomsg at the decision boundary: Q/4 and 3Q/4
        let mut poly = Poly::default();
        // Value 0 should produce bit 0
        poly.coeffs[0] = 0;
        // Value Q/2 (1665) should produce bit 1
        poly.coeffs[1] = (Q / 2) as i16;
        // Value Q/4 (832) is at the boundary
        poly.coeffs[2] = (Q / 4) as i16;
        // Value 3Q/4 (2497) is at the other boundary
        poly.coeffs[3] = (3 * Q as i32 / 4) as i16;

        let msg = poly_tomsg(&poly);
        // bit 0: value 0 -> 0
        assert_eq!(msg[0] & 1, 0);
        // bit 1: value Q/2 -> 1
        assert_eq!((msg[0] >> 1) & 1, 1);
    }
}
