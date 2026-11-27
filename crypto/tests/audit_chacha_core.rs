//! Tests for the ChaCha/XChaCha core, HChaCha20, Poly1305 and
//! ChaCha20-Poly1305 / XChaCha20-Poly1305.
//!
//! Everything here is checked against implementations written independently of
//! the code under test:
//!
//! * a straightforward ChaCha block function (plain `u32` quarter rounds plus
//!   `to_le_bytes`, no unsafe, no SIMD) driven block-by-block from an explicit
//!   counter, and an independent HChaCha20;
//! * a Poly1305 reference that does exact 260-bit schoolbook multiplication in
//!   `u64` limbs and reduces modulo `2^130 - 5` by repeatedly folding the high
//!   part (`x = (x mod 2^130) + 5 * (x >> 130)`), i.e. full reduction with no
//!   26-bit limbs and no shared code with the 3×64-bit implementation;
//! * a full RFC 8439 AEAD construction built on top of those two references.
//!
//! The references are themselves validated against published RFC 8439 test
//! vectors before they are used as an oracle.
//!
//! Coverage:
//! - RFC 8439 §2.4.2 / A.2 / §2.5.2 / A.3 / §2.8.2 / A.5 vectors.
//! - differential sweeps for every length 0..=600 (generic path < 128 bytes,
//!   SIMD dispatch at and above the 128-byte threshold).
//! - every 2-way split at 1/15/16/17/63/64/65/127/128/129 and many 3-way
//!   splits, plus deterministic pseudo-random splits.
//! - counter exhaustion for both layouts, including leftover-only consumption
//!   at the very end of the counter space.
//! - guard canaries and non-zero slice offsets.
//! - Poly1305 lengths 0..=512, high-bit keys, clamping-invariance, chunked
//!   `update`, partial-block padding.
//! - AEAD lengths 15/16/17/63/64/65 and AAD length boundaries, tampering,
//!   in-place and owned-buffer APIs, failure buffer semantics.

use crypto::{
    Aead, AeadError, StreamCipher, StreamCipherError,
    chacha::{ChaCha8Djb, ChaCha20Djb, ChaCha20Ietf, ChaCha20Poly1305, XChaCha20Djb, XChaCha20Ietf, XChaCha20Poly1305},
    poly1305::Poly1305,
};
use random::{Rng, RngExt, SeedableRng, rngs::ChaChaRng};

// ---------------------------------------------------------------------------
// Independent ChaCha reference
// ---------------------------------------------------------------------------

const SIGMA: [u32; 4] = [0x61707865, 0x3320646e, 0x79622d32, 0x6b206574];

fn ref_quarter_round(s: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    s[a] = s[a].wrapping_add(s[b]);
    s[d] ^= s[a];
    s[d] = s[d].rotate_left(16);
    s[c] = s[c].wrapping_add(s[d]);
    s[b] ^= s[c];
    s[b] = s[b].rotate_left(12);
    s[a] = s[a].wrapping_add(s[b]);
    s[d] ^= s[a];
    s[d] = s[d].rotate_left(8);
    s[c] = s[c].wrapping_add(s[d]);
    s[b] ^= s[c];
    s[b] = s[b].rotate_left(7);
}

/// One ChaCha block from a complete 16-word state.
fn ref_block(rounds: usize, initial: &[u32; 16]) -> [u8; 64] {
    let mut s = *initial;
    for _ in 0..rounds / 2 {
        ref_quarter_round(&mut s, 0, 4, 8, 12);
        ref_quarter_round(&mut s, 1, 5, 9, 13);
        ref_quarter_round(&mut s, 2, 6, 10, 14);
        ref_quarter_round(&mut s, 3, 7, 11, 15);
        ref_quarter_round(&mut s, 0, 5, 10, 15);
        ref_quarter_round(&mut s, 1, 6, 11, 12);
        ref_quarter_round(&mut s, 2, 7, 8, 13);
        ref_quarter_round(&mut s, 3, 4, 9, 14);
    }
    let mut out = [0u8; 64];
    for i in 0..16 {
        out[i * 4..i * 4 + 4].copy_from_slice(&s[i].wrapping_add(initial[i]).to_le_bytes());
    }
    out
}

fn ref_key_words(key: &[u8; 32]) -> [u32; 8] {
    let mut w = [0u32; 8];
    for i in 0..8 {
        w[i] = u32::from_le_bytes(key[i * 4..i * 4 + 4].try_into().unwrap());
    }
    w
}

/// Full state (counter words left at zero) for the DJB layout.
fn ref_state_djb(key: &[u8; 32], nonce: &[u8; 8]) -> [u32; 16] {
    let kw = ref_key_words(key);
    let mut s = [0u32; 16];
    s[..4].copy_from_slice(&SIGMA);
    s[4..12].copy_from_slice(&kw);
    s[14] = u32::from_le_bytes(nonce[0..4].try_into().unwrap());
    s[15] = u32::from_le_bytes(nonce[4..8].try_into().unwrap());
    s
}

/// Full state (counter word left at zero) for the IETF layout.
fn ref_state_ietf(key: &[u8; 32], nonce: &[u8; 12]) -> [u32; 16] {
    let kw = ref_key_words(key);
    let mut s = [0u32; 16];
    s[..4].copy_from_slice(&SIGMA);
    s[4..12].copy_from_slice(&kw);
    for i in 0..3 {
        s[13 + i] = u32::from_le_bytes(nonce[i * 4..i * 4 + 4].try_into().unwrap());
    }
    s
}

/// Serialize the keystream starting at `counter` into `out`, block by block.
fn ref_keystream(rounds: usize, base: &[u32; 16], is_ietf: bool, counter: u64, out: &mut [u8]) {
    for (i, chunk) in out.chunks_mut(64).enumerate() {
        let c = counter + i as u64;
        let mut s = *base;
        s[12] = c as u32;
        if !is_ietf {
            s[13] = (c >> 32) as u32;
        }
        let ks = ref_block(rounds, &s);
        chunk.copy_from_slice(&ks[..chunk.len()]);
    }
}

/// Independent HChaCha20 (draft-irtf-cfrg-xchacha-03 §2.2).
fn ref_hchacha20(key: &[u8; 32], input: &[u8; 16]) -> [u8; 32] {
    let kw = ref_key_words(key);
    let mut s = [0u32; 16];
    s[..4].copy_from_slice(&SIGMA);
    s[4..12].copy_from_slice(&kw);
    for i in 0..4 {
        s[12 + i] = u32::from_le_bytes(input[i * 4..i * 4 + 4].try_into().unwrap());
    }
    for _ in 0..10 {
        ref_quarter_round(&mut s, 0, 4, 8, 12);
        ref_quarter_round(&mut s, 1, 5, 9, 13);
        ref_quarter_round(&mut s, 2, 6, 10, 14);
        ref_quarter_round(&mut s, 3, 7, 11, 15);
        ref_quarter_round(&mut s, 0, 5, 10, 15);
        ref_quarter_round(&mut s, 1, 6, 11, 12);
        ref_quarter_round(&mut s, 2, 7, 8, 13);
        ref_quarter_round(&mut s, 3, 4, 9, 14);
    }
    let mut out = [0u8; 32];
    for i in 0..4 {
        out[i * 4..i * 4 + 4].copy_from_slice(&s[i].to_le_bytes());
    }
    for i in 0..4 {
        out[16 + i * 4..16 + i * 4 + 4].copy_from_slice(&s[12 + i].to_le_bytes());
    }
    out
}

// ---------------------------------------------------------------------------
// Independent Poly1305 reference (exact multi-limb arithmetic)
// ---------------------------------------------------------------------------

type Limbs = [u64; 8];

const REF_P: Limbs = [0xffff_ffff_ffff_fffb, 0xffff_ffff_ffff_ffff, 0x3, 0, 0, 0, 0, 0];

fn limbs_from_u128(v: u128) -> Limbs {
    [v as u64, (v >> 64) as u64, 0, 0, 0, 0, 0, 0]
}

fn limbs_add(a: &Limbs, b: &Limbs) -> Limbs {
    let mut out = [0u64; 8];
    let mut carry: u128 = 0;
    for i in 0..8 {
        let s = a[i] as u128 + b[i] as u128 + carry;
        out[i] = s as u64;
        carry = s >> 64;
    }
    assert_eq!(carry, 0, "reference addition overflow");
    out
}

/// Schoolbook multiply; each cell is normalized immediately so no `u128`
/// accumulator can overflow. Inputs are known to be tiny (<= 3 limbs × 2 limbs).
fn limbs_mul(a: &Limbs, b: &Limbs) -> Limbs {
    let mut out = [0u64; 8];
    for i in 0..8 {
        if a[i] == 0 {
            continue;
        }
        let mut carry: u128 = 0;
        for j in 0..(8 - i) {
            if b[j] == 0 && carry == 0 {
                continue;
            }
            let cur = out[i + j] as u128 + (a[i] as u128) * (b[j] as u128) + carry;
            out[i + j] = cur as u64;
            carry = cur >> 64;
        }
        assert_eq!(carry, 0, "reference multiplication overflow");
    }
    out
}

fn limbs_mul_small(a: &Limbs, k: u64) -> Limbs {
    let mut out = [0u64; 8];
    let mut carry: u128 = 0;
    for i in 0..8 {
        let cur = (a[i] as u128) * (k as u128) + carry;
        out[i] = cur as u64;
        carry = cur >> 64;
    }
    assert_eq!(carry, 0, "reference small multiply overflow");
    out
}

/// `x >> 130` (130 = 2 * 64 + 2).
fn limbs_shr130(x: &Limbs) -> Limbs {
    let mut out = [0u64; 8];
    for i in 0..6 {
        let low = if i + 2 < 8 { x[i + 2] >> 2 } else { 0 };
        let high = if i + 3 < 8 { x[i + 3] << 62 } else { 0 };
        out[i] = low | high;
    }
    out
}

/// `x mod 2^130`.
fn limbs_low130(x: &Limbs) -> Limbs {
    [x[0], x[1], x[2] & 3, 0, 0, 0, 0, 0]
}

fn limbs_ge_p(x: &Limbs) -> bool {
    for i in (0..8).rev() {
        if x[i] > REF_P[i] {
            return true;
        }
        if x[i] < REF_P[i] {
            return false;
        }
    }
    true
}

fn limbs_sub_p(x: &Limbs) -> Limbs {
    let mut out = [0u64; 8];
    let mut borrow: u128 = 0;
    for i in 0..8 {
        let sub = REF_P[i] as u128 + borrow;
        let a = x[i] as u128;
        if a >= sub {
            out[i] = (a - sub) as u64;
            borrow = 0;
        } else {
            out[i] = ((1u128 << 64) + a - sub) as u64;
            borrow = 1;
        }
    }
    out
}

/// Exact reduction modulo `2^130 - 5`, using `2^130 == 5`.
fn limbs_reduce(mut x: Limbs) -> Limbs {
    loop {
        let high = limbs_shr130(&x);
        if high == [0u64; 8] {
            break;
        }
        let low = limbs_low130(&x);
        x = limbs_add(&low, &limbs_mul_small(&high, 5));
    }
    while limbs_ge_p(&x) {
        x = limbs_sub_p(&x);
    }
    x
}

fn poly1305_reference(key: &[u8; 32], msg: &[u8]) -> [u8; 16] {
    // RFC 8439 §2.5.1 clamping.
    let r = u128::from_le_bytes(key[0..16].try_into().unwrap()) & 0x0fff_fffc_0fff_fffc_0fff_fffc_0fff_ffff;
    let s = u128::from_le_bytes(key[16..32].try_into().unwrap());
    let r = limbs_from_u128(r);

    let mut h: Limbs = [0u64; 8];
    for chunk in msg.chunks(16) {
        // 16 message bytes followed by the 0x01 padding byte at position len.
        let mut block = [0u8; 17];
        block[..chunk.len()].copy_from_slice(chunk);
        block[chunk.len()] = 1;
        let mut m = limbs_from_u128(u128::from_le_bytes(block[0..16].try_into().unwrap()));
        m[2] = block[16] as u64;
        h = limbs_reduce(limbs_mul(&limbs_add(&h, &m), &r));
    }

    let h_low128 = h[0] as u128 | ((h[1] as u128) << 64);
    (h_low128.wrapping_add(s)).to_le_bytes()
}

// ---------------------------------------------------------------------------
// Independent AEAD reference (RFC 8439 §2.8)
// ---------------------------------------------------------------------------

fn le64(v: usize) -> [u8; 8] {
    (v as u64).to_le_bytes()
}

fn ref_aead_with_key(key: &[u8; 32], nonce: &[u8; 12], plaintext: &[u8], aad: &[u8]) -> (Vec<u8>, [u8; 16]) {
    let base = ref_state_ietf(key, nonce);
    // One-time Poly1305 key: first 32 bytes of block 0.
    let mut block0 = [0u8; 64];
    ref_keystream(20, &base, true, 0, &mut block0);
    let otk: [u8; 32] = block0[..32].try_into().unwrap();

    let mut ks = vec![0u8; plaintext.len()];
    ref_keystream(20, &base, true, 1, &mut ks);
    let ciphertext: Vec<u8> = plaintext.iter().zip(&ks).map(|(p, k)| p ^ k).collect();

    let mut mac_input = Vec::new();
    mac_input.extend_from_slice(aad);
    mac_input.resize(mac_input.len() + (16 - aad.len() % 16) % 16, 0);
    mac_input.extend_from_slice(&ciphertext);
    mac_input.resize(mac_input.len() + (16 - ciphertext.len() % 16) % 16, 0);
    mac_input.extend_from_slice(&le64(aad.len()));
    mac_input.extend_from_slice(&le64(ciphertext.len()));

    let tag = poly1305_reference(&otk, &mac_input);
    (ciphertext, tag)
}

fn ref_xaead(key: &[u8; 32], nonce: &[u8; 24], plaintext: &[u8], aad: &[u8]) -> (Vec<u8>, [u8; 16]) {
    let subkey = ref_hchacha20(key, nonce[..16].try_into().unwrap());
    let mut n = [0u8; 12];
    n[4..].copy_from_slice(&nonce[16..24]);
    ref_aead_with_key(&subkey, &n, plaintext, aad)
}

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

fn pattern(len: usize) -> Vec<u8> {
    (0..len).map(|i| (i as u16 * 37 + 11) as u8).collect()
}

fn key32() -> [u8; 32] {
    let mut k = [0u8; 32];
    for i in 0..32 {
        k[i] = (i as u16 * 7 + 3) as u8;
    }
    k
}

fn key32_high() -> [u8; 32] {
    [0xffu8; 32]
}

fn key32_random() -> [u8; 32] {
    let mut s = 0x1234_5678_9abc_def0u64;
    let mut k = [0u8; 32];
    for b in &mut k {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        *b = s as u8;
    }
    k
}

fn nonce8() -> [u8; 8] {
    [0x1f, 0xe2, 0x3d, 0x04, 0x95, 0xa6, 0x77, 0x88]
}

fn nonce12() -> [u8; 12] {
    [0x00, 0x01, 0x02, 0x03, 0x84, 0x85, 0x86, 0x87, 0x08, 0xf9, 0xfa, 0xfb]
}

fn nonce24() -> [u8; 24] {
    let mut n = [0u8; 24];
    for i in 0..24 {
        n[i] = (i as u16 * 11 + 1) as u8;
    }
    n
}

/// Encrypt `buf` with a `StreamCipher` in chunks given by `splits`, wrapping
/// around to cover the whole buffer.
fn xor_chunked<S: StreamCipher>(cipher: &mut S, buf: &mut [u8], splits: &[usize]) {
    let mut offset = 0;
    let mut i = 0;
    while offset < buf.len() {
        let take = if splits.is_empty() {
            buf.len()
        } else {
            splits[i % splits.len()]
        };
        let end = (offset + take).min(buf.len());
        cipher.xor_keystream(&mut buf[offset..end]).unwrap();
        offset = end;
        i += 1;
    }
}

// ---------------------------------------------------------------------------
// Reference self-checks (so the oracle itself is anchored to RFC vectors)
// ---------------------------------------------------------------------------

#[test]
fn references_match_published_vectors() {
    // RFC 8439 §2.5.2
    let key: [u8; 32] = hex::decode("85d6be7857556d337f4452fe42d506a80103808afb0db2fd4abff6af4149f51b")
        .unwrap()
        .try_into()
        .unwrap();
    let msg = b"Cryptographic Forum Research Group";
    assert_eq!(
        hex::encode(poly1305_reference(&key, msg)),
        "a8061dc1305136c6c22b8baf0c0127a9",
        "poly1305 reference diverges from RFC 8439 §2.5.2"
    );

    // RFC 8439 A.3 #1 (zero key)
    assert_eq!(
        hex::encode(poly1305_reference(&[0u8; 32], &[0u8; 64])),
        "00000000000000000000000000000000"
    );

    // BoringSSL poly1305_tests vectors (also in RFC 8439 A.3)
    let cases: [(&str, &str, &str); 5] = [
        (
            "0200000000000000000000000000000000000000000000000000000000000000",
            "ffffffffffffffffffffffffffffffff",
            "03000000000000000000000000000000",
        ),
        (
            "02000000000000000000000000000000ffffffffffffffffffffffffffffffff",
            "02000000000000000000000000000000",
            "03000000000000000000000000000000",
        ),
        (
            "0100000000000000000000000000000000000000000000000000000000000000",
            "fffffffffffffffffffffffffffffffff0ffffffffffffffffffffffffffffff11000000000000000000000000000000",
            "05000000000000000000000000000000",
        ),
        (
            "0200000000000000000000000000000000000000000000000000000000000000",
            "fdffffffffffffffffffffffffffffff",
            "faffffffffffffffffffffffffffffff",
        ),
        (
            "0100000000000000040000000000000000000000000000000000000000000000",
            "e33594d7505e43b900000000000000003394d7505e4379cd010000000000000000000000000000000000000000000000",
            "13000000000000000000000000000000",
        ),
    ];
    for (k, m, want) in cases {
        let k: [u8; 32] = hex::decode(k).unwrap().try_into().unwrap();
        let m = hex::decode(m).unwrap();
        assert_eq!(hex::encode(poly1305_reference(&k, &m)), want, "poly1305 reference diverges");
    }

    // RFC 8439 §2.4.2 / A.2 vector #1 for the ChaCha reference.
    let key = [0u8; 32];
    let n = [0u8; 12];
    let base = ref_state_ietf(&key, &n);
    let mut ks = [0u8; 64];
    ref_keystream(20, &base, true, 0, &mut ks);
    assert_eq!(
        hex::encode(ks),
        "76b8e0ada0f13d90405d6ae55386bd28bdd219b8a08ded1aa836efcc8b770dc7\
         da41597c5157488d7724e03fb8d84a37\
         6a43b8f41518a11cc387b669b2ee6586",
        "chacha reference diverges from RFC 8439 A.2 #1"
    );

    // draft-irtf-cfrg-xchacha-03 §2.2.1
    let key: [u8; 32] = hex::decode("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f")
        .unwrap()
        .try_into()
        .unwrap();
    let input: [u8; 16] = hex::decode("000000090000004a0000000031415927")
        .unwrap()
        .try_into()
        .unwrap();
    assert_eq!(
        hex::encode(ref_hchacha20(&key, &input)),
        "82413b4227b27bfed30e42508a877d73a0f9e4d58a74a853c12ec41326d3ecdc",
        "HChaCha20 reference diverges from the XChaCha draft"
    );

    // RFC 8439 §2.8.2 must match the independently built AEAD.
    let key: [u8; 32] = hex::decode("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f")
        .unwrap()
        .try_into()
        .unwrap();
    let nonce: [u8; 12] = hex::decode("070000004041424344454647").unwrap().try_into().unwrap();
    let aad = hex::decode("50515253c0c1c2c3c4c5c6c7").unwrap();
    let pt = hex::decode(concat!(
        "4c616469657320616e642047656e746c656d656e206f662074686520636c6173",
        "73206f66202739393a204966204920636f756c64206f6666657220796f75206f",
        "6e6c79206f6e652074697020666f7220746865206675747572652c2073756e73",
        "637265656e20776f756c642062652069742e"
    ))
    .unwrap();
    let (ct, tag) = ref_aead_with_key(&key, &nonce, &pt, &aad);
    assert_eq!(
        hex::encode(&ct),
        concat!(
            "d31a8d34648e60db7b86afbc53ef7ec2a4aded51296e08fea9e2b5a736ee62d6",
            "3dbea45e8ca9671282fafb69da92728b1a71de0a9e060b2905d6a5b67ecd3b36",
            "92ddbd7f2d778b8c9803aee328091b58fab324e4fad675945585808b4831d7bc",
            "3ff4def08e4b7a9de576d26586cec64b6116"
        ),
        "AEAD reference ciphertext diverges from RFC 8439 §2.8.2"
    );
    assert_eq!(
        hex::encode(tag),
        "1ae10b594f09e26a7e902ecbd0600691",
        "AEAD reference tag diverges from RFC 8439 §2.8.2"
    );
}

// ---------------------------------------------------------------------------
// ChaCha: RFC vectors
// ---------------------------------------------------------------------------

#[test]
fn chacha20_ietf_rfc8439_vectors() {
    struct V {
        key: &'static str,
        nonce: &'static str,
        counter: u32,
        pt: &'static str,
        ct: &'static str,
    }
    let vectors = [
        V {
            key: "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
            nonce: "000000000000004a00000000",
            counter: 1,
            pt: concat!(
                "4c616469657320616e642047656e746c656d656e206f662074686520636c6173",
                "73206f66202739393a204966204920636f756c64206f6666657220796f75206f",
                "6e6c79206f6e652074697020666f7220746865206675747572652c2073756e73",
                "637265656e20776f756c642062652069742e"
            ),
            ct: concat!(
                "6e2e359a2568f98041ba0728dd0d6981e97e7aec1d4360c20a27afccfd9fae0b",
                "f91b65c5524733ab8f593dabcd62b3571639d624e65152ab8f530c359f0861d8",
                "07ca0dbf500d6a6156a38e088a22b65e52bc514d16ccf806818ce91ab7793736",
                "5af90bbf74a35be6b40b8eedf2785e42874d"
            ),
        },
        V {
            key: "0000000000000000000000000000000000000000000000000000000000000000",
            nonce: "000000000000000000000000",
            counter: 0,
            pt: "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            ct: "76b8e0ada0f13d90405d6ae55386bd28bdd219b8a08ded1aa836efcc8b770dc7\
                 da41597c5157488d7724e03fb8d84a376a43b8f41518a11cc387b669b2ee6586",
        },
        V {
            key: "0000000000000000000000000000000000000000000000000000000000000001",
            nonce: "000000000000000000000002",
            counter: 1,
            pt: "",
            ct: "",
        },
    ];

    for (i, v) in vectors.iter().enumerate() {
        let key: [u8; 32] = hex::decode(v.key).unwrap().try_into().unwrap();
        let nonce: [u8; 12] = hex::decode(v.nonce).unwrap().try_into().unwrap();
        let mut buf = hex::decode(v.pt).unwrap();
        let mut c = ChaCha20Ietf::new(&key, &nonce);
        c.set_counter(v.counter);
        c.xor_keystream(&mut buf).unwrap();
        assert_eq!(hex::encode(&buf), hex::encode(hex::decode(v.ct).unwrap()), "IETF vector {i}");

        // Decrypt back.
        let mut c = ChaCha20Ietf::new(&key, &nonce);
        c.set_counter(v.counter);
        c.xor_keystream(&mut buf).unwrap();
        assert_eq!(hex::encode(&buf), hex::encode(hex::decode(v.pt).unwrap()), "IETF roundtrip {i}");
    }
}

#[test]
fn chacha20_djb_rfc8439_appendix_a2() {
    struct V {
        key: &'static str,
        nonce: &'static str,
        counter: u64,
        pt: &'static str,
        ct: &'static str,
    }
    let vectors = [
        V {
            key: "0000000000000000000000000000000000000000000000000000000000000000",
            nonce: "0000000000000000",
            counter: 0,
            pt: "0000000000000000000000000000000000000000000000000000000000000000\
                 0000000000000000000000000000000000000000000000000000000000000000",
            ct: "76b8e0ada0f13d90405d6ae55386bd28bdd219b8a08ded1aa836efcc8b770dc7\
                 da41597c5157488d7724e03fb8d84a376a43b8f41518a11cc387b669b2ee6586",
        },
        V {
            key: "0000000000000000000000000000000000000000000000000000000000000001",
            nonce: "0000000000000002",
            counter: 1,
            pt: "",
            ct: "",
        },
        V {
            key: "1c9240a5eb55d38af333888604f6b5f0473917c1402b80099dca5cbc207075c0",
            nonce: "0000000000000002",
            counter: 42,
            pt: concat!(
                "2754776173206272696c6c69672c20616e642074686520736c6974687920746f",
                "7665730a446964206779726520616e642067696d626c6520696e207468652077",
                "6162653a0a416c6c206d696d737920776572652074686520626f726f676f7665",
                "732c0a416e6420746865206d6f6d65207261746873206f757467726162652e"
            ),
            ct: concat!(
                "62e6347f95ed87a45ffae7426f27a1df5fb69110044c0d73118effa95b01e5cf",
                "166d3df2d721caf9b21e5fb14c616871fd84c54f9d65b283196c7fe4f60553eb",
                "f39c6402c42234e32a356b3e764312a61a5532055716ead6962568f87d3f3f77",
                "04c6a8d1bcd1bf4d50d6154b6da731b187b58dfd728afa36757a797ac188d1"
            ),
        },
    ];

    for (i, v) in vectors.iter().enumerate() {
        let key: [u8; 32] = hex::decode(v.key).unwrap().try_into().unwrap();
        let nonce: [u8; 8] = hex::decode(v.nonce).unwrap().try_into().unwrap();
        let pt = hex::decode(v.pt).unwrap();
        let mut buf = pt.clone();
        let mut c = ChaCha20Djb::new(&key, &nonce);
        c.set_counter(v.counter);
        c.xor_keystream(&mut buf).unwrap();
        assert_eq!(hex::encode(&buf), hex::encode(hex::decode(v.ct).unwrap()), "DJB vector {i}");
        let mut c = ChaCha20Djb::new(&key, &nonce);
        c.set_counter(v.counter);
        c.xor_keystream(&mut buf).unwrap();
        assert_eq!(buf, pt, "DJB roundtrip {i}");
    }
}

#[test]
fn hchacha20_vectors() {
    use crypto::chacha::hchacha20;

    let key: [u8; 32] = hex::decode("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f")
        .unwrap()
        .try_into()
        .unwrap();
    let input: [u8; 16] = hex::decode("000000090000004a0000000031415927")
        .unwrap()
        .try_into()
        .unwrap();
    assert_eq!(
        hex::encode(hchacha20(&key, &input)),
        "82413b4227b27bfed30e42508a877d73a0f9e4d58a74a853c12ec41326d3ecdc"
    );

    // Differential over deterministic inputs.
    let mut rng = ChaChaRng::seed_from_u64(0xdead_beef_cafe_babe);
    for _ in 0..512 {
        let mut k = [0u8; 32];
        let mut in16 = [0u8; 16];
        for b in &mut k {
            *b = rng.next_u64() as u8;
        }
        for b in &mut in16 {
            *b = rng.next_u64() as u8;
        }
        assert_eq!(hchacha20(&k, &in16), ref_hchacha20(&k, &in16), "hchacha20 mismatch");
    }
}

#[test]
fn xchacha20_matches_reference() {
    // draft-irtf-cfrg-xchacha-03 §A.3.2.1 (counter = 0).
    let key: [u8; 32] = hex::decode("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f")
        .unwrap()
        .try_into()
        .unwrap();
    let nonce: [u8; 24] = hex::decode("404142434445464748494a4b4c4d4e4f5051525354555658")
        .unwrap()
        .try_into()
        .unwrap();
    let expected = hex::decode(concat!(
        "4559abba4e48c16102e8bb2c05e6947f50a786de162f9b0b7e592a9b53d0d4e9",
        "8d8d6410d540a1a6375b26d80dace4fab52384c731acbf16a5923c0c48d3575d",
        "4d0d2c673b666faa731061277701093a6bf7a158a8864292a41c48e3a9b4c0da",
        "ece0f8d98d0d7e05b37a307bbb66333164ec9e1b24ea0d6c3ffddcec4f68e744",
        "3056193a03c810e11344ca06d8ed8a2bfb1e8d48cfa6bc0eb4e2464b74814240",
        "7c9f431aee769960e15ba8b96890466ef2457599852385c661f752ce20f9da0c",
        "09ab6b19df74e76a95967446f8d0fd415e7bee2a12a114c20eb5292ae7a349ae",
        "577820d5520a1f3fb62a17ce6a7e68fa7c79111d8860920bc048ef43fe84486c",
        "cb87c25f0ae045f0cce1e7989a9aa220a28bdd4827e751a24a6d5c62d790a663",
        "93b93111c1a55dd7421a10184974c7c5",
    ))
    .unwrap();
    let plaintext = hex::decode(concat!(
        "5468652064686f6c65202870726f6e6f756e6365642022646f6c652229206973",
        "20616c736f206b6e6f776e2061732074686520417369617469632077696c6420",
        "646f672c2072656420646f672c20616e642077686973746c696e6720646f672e",
        "2049742069732061626f7574207468652073697a65206f662061204765726d61",
        "6e20736865706865726420627574206c6f6f6b73206d6f7265206c696b652061",
        "206c6f6e672d6c656767656420666f782e205468697320686967686c7920656c",
        "757369766520616e6420736b696c6c6564206a756d70657220697320636c6173",
        "736966696564207769746820776f6c7665732c20636f796f7465732c206a6163",
        "6b616c732c20616e6420666f78657320696e20746865207461786f6e6f6d6963",
        "2066616d696c792043616e696461652e",
    ))
    .unwrap();
    assert_eq!(plaintext.len(), expected.len());
    let mut buf = plaintext.clone();
    XChaCha20Ietf::new(&key, &nonce).xor_keystream(&mut buf).unwrap();
    assert_eq!(buf, expected);
    // Decrypt back.
    let mut back = buf.clone();
    XChaCha20Ietf::new(&key, &nonce).xor_keystream(&mut back).unwrap();
    assert_eq!(back, plaintext);

    // Both XChaCha layouts against the reference for many lengths.
    for len in 0..=300usize {
        let pt = pattern(len);
        for &variant in &[0u8, 1] {
            let mut got = pt.clone();
            let mut ks = vec![0u8; len];
            if variant == 0 {
                XChaCha20Ietf::new(&key, &nonce).xor_keystream(&mut got).unwrap();
                let sub = ref_hchacha20(&key, nonce[..16].try_into().unwrap());
                let mut n = [0u8; 12];
                n[4..].copy_from_slice(&nonce[16..24]);
                ref_keystream(20, &ref_state_ietf(&sub, &n), true, 0, &mut ks);
                let want: Vec<u8> = pt.iter().zip(&ks).map(|(p, k)| p ^ k).collect();
                assert_eq!(got, want, "XChaCha20-IETF len={len}");
            } else {
                XChaCha20Djb::new(&key, &nonce).xor_keystream(&mut got).unwrap();
                let sub = ref_hchacha20(&key, nonce[..16].try_into().unwrap());
                let n: [u8; 8] = nonce[16..24].try_into().unwrap();
                ref_keystream(20, &ref_state_djb(&sub, &n), false, 0, &mut ks);
                let want: Vec<u8> = pt.iter().zip(&ks).map(|(p, k)| p ^ k).collect();
                assert_eq!(got, want, "XChaCha20-DJB len={len}");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// ChaCha: differential length sweep (generic path and SIMD dispatch)
// ---------------------------------------------------------------------------

#[test]
fn chacha_differential_all_lengths_and_rounds() {
    let key = key32();
    for &rounds in &[8usize, 12, 20] {
        for len in 0..=600usize {
            // DJB layout, counter 0.
            let mut got = vec![0u8; len];
            match rounds {
                8 => ChaCha8Djb::new(&key, &nonce8()).xor_keystream(&mut got).unwrap(),
                12 => crypto::chacha::ChaCha12Djb::new(&key, &nonce8())
                    .xor_keystream(&mut got)
                    .unwrap(),
                _ => ChaCha20Djb::new(&key, &nonce8()).xor_keystream(&mut got).unwrap(),
            }
            let mut want = vec![0u8; len];
            ref_keystream(rounds, &ref_state_djb(&key, &nonce8()), false, 0, &mut want);
            assert_eq!(got, want, "DJB rounds={rounds} len={len}");

            // IETF layout, counter 0 (only ChaCha20 has an IETF alias).
            if rounds == 20 {
                let mut got = vec![0u8; len];
                ChaCha20Ietf::new(&key, &nonce12()).xor_keystream(&mut got).unwrap();
                let mut want = vec![0u8; len];
                ref_keystream(20, &ref_state_ietf(&key, &nonce12()), true, 0, &mut want);
                assert_eq!(got, want, "IETF len={len}");
            }
        }
    }
}

#[test]
fn chacha_differential_high_counters() {
    let key = key32_random();

    // IETF: finish exactly at the reserved counter value.
    for len in 0..=512usize {
        let blocks = len.div_ceil(64) as u32;
        let counter = u32::MAX - blocks; // last used counter is u32::MAX - 1
        let mut got = vec![0u8; len];
        let mut c = ChaCha20Ietf::new(&key, &nonce12());
        c.set_counter(counter);
        c.xor_keystream(&mut got).unwrap();
        let mut want = vec![0u8; len];
        ref_keystream(20, &ref_state_ietf(&key, &nonce12()), true, counter as u64, &mut want);
        assert_eq!(got, want, "IETF high counter={counter} len={len}");
    }

    // DJB: cross the 32-bit counter word boundary.
    for &counter in &[0xffff_fffeu64, 0xffff_ffff, 0x1_0000_0000, 0x1_0000_0001] {
        for len in [1usize, 63, 64, 65, 128, 129, 256] {
            let mut got = vec![0u8; len];
            let mut c = ChaCha20Djb::new(&key, &nonce8());
            c.set_counter(counter);
            c.xor_keystream(&mut got).unwrap();
            let mut want = vec![0u8; len];
            ref_keystream(20, &ref_state_djb(&key, &nonce8()), false, counter, &mut want);
            assert_eq!(got, want, "DJB counter={counter:#x} len={len}");
        }
    }

    // DJB: near the top of the 64-bit space.
    for &counter in &[u64::MAX - 8, u64::MAX - 2] {
        let len = 64 * 2; // two blocks: counters counter, counter+1
        let mut got = vec![0u8; len];
        let mut c = ChaCha20Djb::new(&key, &nonce8());
        c.set_counter(counter);
        c.xor_keystream(&mut got).unwrap();
        let mut want = vec![0u8; len];
        ref_keystream(20, &ref_state_djb(&key, &nonce8()), false, counter, &mut want);
        assert_eq!(got, want, "DJB high counter={counter:#x}");
    }
}

#[test]
fn djb_and_ietf_layouts_agree_when_word13_is_zero() {
    // For a counter that fits in 32 bits, DJB's state[13] is 0 and word 12 holds
    // the counter; IETF only differs in that state[13] carries the first nonce
    // word. Hence DJB(key, nonce8) == IETF(key, 0^4 || nonce8) while the counter
    // is below 2^32.
    let key = key32();
    let n8 = nonce8();
    let mut n12 = [0u8; 12];
    n12[4..].copy_from_slice(&n8);

    for len in [1usize, 63, 64, 65, 127, 128, 129, 300, 512] {
        let mut a = vec![0u8; len];
        ChaCha20Djb::new(&key, &n8).xor_keystream(&mut a).unwrap();
        let mut b = vec![0u8; len];
        ChaCha20Ietf::new(&key, &n12).xor_keystream(&mut b).unwrap();
        assert_eq!(a, b, "layout mismatch len={len}");
    }

    // A non-zero word 13 must change the keystream, i.e. the two layouts are
    // genuinely distinct when the nonce prefix is non-zero.
    let mut n12_nz = n12;
    n12_nz[0] = 1;
    let mut a = vec![0u8; 64];
    ChaCha20Djb::new(&key, &n8).xor_keystream(&mut a).unwrap();
    let mut b = vec![0u8; 64];
    ChaCha20Ietf::new(&key, &n12_nz).xor_keystream(&mut b).unwrap();
    assert_ne!(a, b, "IETF nonce prefix must affect the keystream");
}

// ---------------------------------------------------------------------------
// ChaCha: chunked / leftover state machine
// ---------------------------------------------------------------------------

#[test]
fn chunked_equals_one_shot_every_split_pattern() {
    let key = key32();
    let split_points = [
        0usize, 1, 15, 16, 17, 31, 32, 33, 47, 48, 62, 63, 64, 65, 126, 127, 128, 129, 130, 191, 192, 255,
    ];

    for len in 0..=512usize {
        let pt = pattern(len);

        // Two-way splits.
        for &a in &split_points {
            if a > len {
                continue;
            }
            // IETF
            let mut want = pt.clone();
            ChaCha20Ietf::new(&key, &nonce12()).xor_keystream(&mut want).unwrap();
            let mut got = pt.clone();
            let mut c = ChaCha20Ietf::new(&key, &nonce12());
            c.xor_keystream(&mut got[..a]).unwrap();
            c.xor_keystream(&mut got[a..]).unwrap();
            assert_eq!(got, want, "IETF len={len} a={a}");

            // DJB
            let mut want = pt.clone();
            ChaCha20Djb::new(&key, &nonce8()).xor_keystream(&mut want).unwrap();
            let mut got = pt.clone();
            let mut c = ChaCha20Djb::new(&key, &nonce8());
            c.xor_keystream(&mut got[..a]).unwrap();
            c.xor_keystream(&mut got[a..]).unwrap();
            assert_eq!(got, want, "DJB len={len} a={a}");
        }

        // Three-way splits, emphasizing the leftover boundaries.
        for &a in &[1usize, 17, 63, 64, 65, 127, 128, 129] {
            for &b in &[1usize, 17, 63, 64, 65, 127, 128, 129] {
                if a + b > len {
                    continue;
                }
                let mut want = pt.clone();
                ChaCha20Ietf::new(&key, &nonce12()).xor_keystream(&mut want).unwrap();
                let mut got = pt.clone();
                let mut c = ChaCha20Ietf::new(&key, &nonce12());
                c.xor_keystream(&mut got[..a]).unwrap();
                c.xor_keystream(&mut got[a..a + b]).unwrap();
                c.xor_keystream(&mut got[a + b..]).unwrap();
                assert_eq!(got, want, "IETF len={len} splits=({a},{b})");
            }
        }
    }
}

#[test]
fn chunked_equals_one_shot_random_splits() {
    let key = key32_random();
    let mut rng = ChaChaRng::seed_from_u64(0x0bad_c0de_1234_5678);

    for _ in 0..400 {
        let len = rng.random_range(0..1025);
        let pt = pattern(len);
        let mut want = pt.clone();
        ChaCha20Djb::new(&key, &nonce8()).xor_keystream(&mut want).unwrap();

        // 1..=24 random chunk sizes, including tiny (1 byte) and > block chunks.
        let nchunks = 1 + rng.random_range(0..24);
        let splits: Vec<usize> = (0..nchunks)
            .map(|_| match rng.random_range(0..6) {
                0 => 1,
                1 => 63,
                2 => 64,
                3 => 65,
                4 => 7,
                _ => 1 + rng.random_range(0usize..140),
            })
            .collect();

        let mut got = pt.clone();
        let mut c = ChaCha20Djb::new(&key, &nonce8());
        xor_chunked(&mut c, &mut got, &splits);
        assert_eq!(got, want, "random splits len={len} splits={splits:?}");

        // The same split pattern with many explicit empty calls interleaved.
        let mut got = pt.clone();
        let mut c = ChaCha20Djb::new(&key, &nonce8());
        let mut offset = 0;
        let mut i = 0;
        while offset < got.len() {
            c.xor_keystream(&mut []).unwrap();
            let take = splits[i % splits.len()];
            let end = (offset + take).min(got.len());
            c.xor_keystream(&mut got[offset..end]).unwrap();
            offset = end;
            i += 1;
        }
        c.xor_keystream(&mut []).unwrap();
        assert_eq!(got, want, "random splits+empties len={len}");
    }
}

#[test]
fn empty_input_is_a_noop() {
    let mut c = ChaCha20Djb::new(&key32(), &nonce8());
    // Empty calls must not disturb the counter or the leftover state.
    c.xor_keystream(&mut []).unwrap();
    let mut a = vec![0u8; 200];
    c.xor_keystream(&mut a).unwrap();
    let mut b = vec![0u8; 200];
    ChaCha20Djb::new(&key32(), &nonce8()).xor_keystream(&mut b).unwrap();
    assert_eq!(a, b);

    // Empty call in the middle of a partial block.
    let mut c = ChaCha20Djb::new(&key32(), &nonce8());
    let mut got = vec![0u8; 200];
    c.xor_keystream(&mut got[..10]).unwrap();
    c.xor_keystream(&mut []).unwrap();
    c.xor_keystream(&mut got[10..]).unwrap();
    let mut want = vec![0u8; 200];
    ChaCha20Djb::new(&key32(), &nonce8()).xor_keystream(&mut want).unwrap();
    assert_eq!(got, want);
}

#[test]
fn set_counter_resets_leftover() {
    for &variant in &[0u8, 1] {
        let pt = pattern(150);
        if variant == 0 {
            let mut want = pt.clone();
            ChaCha20Ietf::new(&key32(), &nonce12())
                .xor_keystream(&mut want)
                .unwrap();

            // Partial call leaves 54 bytes of buffered keystream. set_counter(0)
            // must discard it so the second pass restarts at block 0.
            let mut got = pt.clone();
            let mut c = ChaCha20Ietf::new(&key32(), &nonce12());
            c.xor_keystream(&mut got[..10]).unwrap();
            c.set_counter(0);
            got.copy_from_slice(&pt); // restart the buffer, keep the cipher state
            c.xor_keystream(&mut got).unwrap();
            assert_eq!(got, want, "IETF set_counter must clear the leftover");
        } else {
            let mut want = pt.clone();
            ChaCha20Djb::new(&key32(), &nonce8()).xor_keystream(&mut want).unwrap();

            let mut got = pt.clone();
            let mut c = ChaCha20Djb::new(&key32(), &nonce8());
            c.xor_keystream(&mut got[..10]).unwrap();
            c.set_counter(0);
            got.copy_from_slice(&pt);
            c.xor_keystream(&mut got).unwrap();
            assert_eq!(got, want, "DJB set_counter must clear the leftover");

            // Moving backwards/forwards by an explicit counter must be exact.
            let mut c = ChaCha20Djb::new(&key32(), &nonce8());
            let mut seeked = vec![0u8; 64];
            c.set_counter(5);
            c.xor_keystream(&mut seeked).unwrap();
            let state = ref_state_djb(&key32(), &nonce8());
            let mut want = vec![0u8; 64];
            ref_keystream(20, &state, false, 5, &mut want);
            assert_eq!(seeked, want, "DJB seek to block 5");
        }
    }
}

// ---------------------------------------------------------------------------
// ChaCha: counter exhaustion
// ---------------------------------------------------------------------------

#[test]
fn ietf_counter_exhaustion_returns_error_without_wrapping() {
    let key = key32();
    let nonce = nonce12();

    // 1 block at MAX-1 is the last legal block.
    let mut c = ChaCha20Ietf::new(&key, &nonce);
    c.set_counter(u32::MAX - 1);
    let mut buf = [0u8; 64];
    c.xor_keystream(&mut buf).unwrap();
    let mut want = [0u8; 64];
    ref_keystream(20, &ref_state_ietf(&key, &nonce), true, (u32::MAX - 1) as u64, &mut want);
    assert_eq!(buf, want);

    // The next byte would need the reserved counter value u32::MAX.
    assert_eq!(c.xor_keystream(&mut [0u8; 1]), Err(StreamCipherError::CounterExhausted));

    // Requesting 2 blocks from MAX-1 returns an error (not wrap to 0).
    let mut c = ChaCha20Ietf::new(&key, &nonce);
    c.set_counter(u32::MAX - 1);
    assert_eq!(c.xor_keystream(&mut [0u8; 128]), Err(StreamCipherError::CounterExhausted));

    // Setting the reserved counter explicitly and using one block returns an
    // error.
    let mut c = ChaCha20Ietf::new(&key, &nonce);
    c.set_counter(u32::MAX);
    assert_eq!(c.xor_keystream(&mut [0u8; 1]), Err(StreamCipherError::CounterExhausted));

    // XChaCha20-IETF inherits the same limit.
    let mut c = XChaCha20Ietf::new(&key, &nonce24());
    c.set_counter(u32::MAX);
    assert_eq!(c.xor_keystream(&mut [0u8; 1]), Err(StreamCipherError::CounterExhausted));

    // Documented maximum AEAD payload: counter 0 is the Poly1305 key, the
    // payload starts at counter 1, and the final counter is reserved.
    assert_eq!(crypto::chacha::CHACHA20_IETF_MAX_LEN, (u32::MAX as u64 - 1) * 64);
    assert_eq!(crypto::chacha::CHACHA20_IETF_MAX_LEN, 274_877_906_816);
}

#[test]
fn ietf_leftover_at_counter_end_is_consumed_without_panic() {
    let key = key32();
    let nonce = nonce12();

    let mut c = ChaCha20Ietf::new(&key, &nonce);
    c.set_counter(u32::MAX - 1);
    let mut first = [0u8; 10];
    c.xor_keystream(&mut first).unwrap(); // block MAX-1, 54 bytes buffered
    let mut second = [0u8; 54];
    c.xor_keystream(&mut second).unwrap(); // exactly drains the buffer: no new block
    // No error above. Verify the keystream is the expected one.
    let state = ref_state_ietf(&key, &nonce);
    let mut want = vec![0u8; 64];
    ref_keystream(20, &state, true, (u32::MAX - 1) as u64, &mut want);
    assert_eq!(&first[..], &want[..10]);
    assert_eq!(&second[..], &want[10..64]);

    // One more byte needs the reserved block and returns an error.
    let mut third = [0u8; 1];
    assert_eq!(
        c.xor_keystream(&mut third),
        Err(StreamCipherError::CounterExhausted),
        "IETF must error once the leftover is drained at the end of the counter space"
    );
}

#[test]
fn counter_exhaustion_leaves_the_buffer_unchanged() {
    let key = key32();
    let nonce = nonce12();

    let mut c = ChaCha20Ietf::new(&key, &nonce);
    c.set_counter(u32::MAX);
    let mut buf = [0xa5u8; 64];
    assert_eq!(c.xor_keystream(&mut buf), Err(StreamCipherError::CounterExhausted));
    assert_eq!(buf, [0xa5u8; 64], "failed call must not touch the buffer");

    let mut c = ChaCha20Djb::new(&key, &nonce8());
    c.set_counter(u64::MAX);
    let mut buf = [0xa5u8; 64];
    assert_eq!(c.xor_keystream(&mut buf), Err(StreamCipherError::CounterExhausted));
    assert_eq!(buf, [0xa5u8; 64], "failed call must not touch the buffer");
}

#[test]
fn djb_counter_exhaustion_returns_error_without_wrapping() {
    let key = key32();
    let nonce = nonce8();

    // One block at u64::MAX-1 is allowed and matches the reference.
    let mut c = ChaCha20Djb::new(&key, &nonce);
    c.set_counter(u64::MAX - 1);
    let mut buf = [0u8; 64];
    c.xor_keystream(&mut buf).unwrap();
    let mut want = [0u8; 64];
    ref_keystream(20, &ref_state_djb(&key, &nonce), false, u64::MAX - 1, &mut want);
    assert_eq!(buf, want);

    // Two blocks would use the reserved final counter value.
    let mut c = ChaCha20Djb::new(&key, &nonce);
    c.set_counter(u64::MAX - 1);
    assert_eq!(
        c.xor_keystream(&mut [0u8; 128]),
        Err(StreamCipherError::CounterExhausted),
        "DJB 2-block request at u64::MAX-1 must error"
    );

    // Setting the reserved counter explicitly errors on use.
    let mut c = ChaCha20Djb::new(&key, &nonce);
    c.set_counter(u64::MAX);
    assert_eq!(
        c.xor_keystream(&mut [0u8; 1]),
        Err(StreamCipherError::CounterExhausted),
        "DJB counter == u64::MAX must be rejected"
    );

    // XChaCha20-DJB too.
    let mut c = XChaCha20Djb::new(&key, &nonce24());
    c.set_counter(u64::MAX);
    assert_eq!(
        c.xor_keystream(&mut [0u8; 1]),
        Err(StreamCipherError::CounterExhausted),
        "XChaCha20-DJB must inherit the exhaustion limit"
    );
}

// ---------------------------------------------------------------------------
// ChaCha: memory safety around the output slice
// ---------------------------------------------------------------------------

#[test]
fn chacha_guard_canaries_and_offsets() {
    let key = key32();
    let nonce = nonce12();
    const CANARY: usize = 32;

    for &offset in &[0usize, 1, 3, 7, 15, 63, 64, 65, 127] {
        for &len in &[
            0usize, 1, 15, 16, 17, 63, 64, 65, 127, 128, 129, 130, 255, 256, 257, 511,
        ] {
            let pt = pattern(len);
            let mut buf = vec![0xa5u8; offset + len + 2 * CANARY];
            buf[offset..offset + len].copy_from_slice(&pt);
            {
                let slice = &mut buf[offset..offset + len];
                ChaCha20Ietf::new(&key, &nonce).xor_keystream(slice).unwrap();
            }
            assert!(
                buf[..offset].iter().all(|b| *b == 0xa5),
                "prefix canary clobbered offset={offset} len={len}"
            );
            assert!(
                buf[offset + len..].iter().all(|b| *b == 0xa5),
                "suffix canary clobbered offset={offset} len={len}"
            );

            let mut want = pt;
            let mut ks = vec![0u8; len];
            ref_keystream(20, &ref_state_ietf(&key, &nonce), true, 0, &mut ks);
            for (w, k) in want.iter_mut().zip(&ks) {
                *w ^= *k;
            }
            assert_eq!(&buf[offset..offset + len], &want[..], "offset={offset} len={len}");
        }
    }
}

// ---------------------------------------------------------------------------
// Poly1305
// ---------------------------------------------------------------------------

#[test]
fn poly1305_matches_rfc_and_reference_all_lengths() {
    // RFC 8439 §2.5.2
    let key: [u8; 32] = hex::decode("85d6be7857556d337f4452fe42d506a80103808afb0db2fd4abff6af4149f51b")
        .unwrap()
        .try_into()
        .unwrap();
    assert_eq!(
        hex::encode(Poly1305::mac(&key, b"Cryptographic Forum Research Group")),
        "a8061dc1305136c6c22b8baf0c0127a9"
    );

    // Empty message: tag is just `s`.
    let k: [u8; 32] = hex::decode("00000000000000000000000000000000ffffffffffffffffffffffffffffffff")
        .unwrap()
        .try_into()
        .unwrap();
    assert_eq!(hex::encode(Poly1305::mac(&k, b"")), "ffffffffffffffffffffffffffffffff");
    assert_eq!(hex::encode(poly1305_reference(&k, b"")), "ffffffffffffffffffffffffffffffff");

    for key in [key32(), key32_high(), key32_random(), [0u8; 32], k] {
        for len in 0..=512usize {
            let msg = pattern(len);
            let got = Poly1305::mac(&key, &msg);
            let want = poly1305_reference(&key, &msg);
            assert_eq!(got, want, "poly1305 key={} len={len}", hex::encode(key));

            // Every length 1..=16 exercises a distinct partial-block padding
            // position; the reference handles it independently.
        }
    }
}

#[test]
fn poly1305_partial_block_padding_is_not_length_confused() {
    let key = key32_high();
    // Messages that are prefixes of one another must not collide, and the
    // 0x01 padding byte must be placed after the message, not at a fixed spot.
    for len in 1..=64usize {
        let msg = vec![0xffu8; len];
        let tag = Poly1305::mac(&key, &msg);
        assert_eq!(tag, poly1305_reference(&key, &msg), "len={len}");
        for l in 0..len {
            assert_ne!(tag, Poly1305::mac(&key, &msg[..l]), "prefix collision len={len} prefix={l}");
        }
        // A zero message of the same length has a different tag.
        assert_ne!(tag, Poly1305::mac(&key, &vec![0u8; len]), "len={len}");
    }
}

#[test]
fn poly1305_key_clamping_ignores_clamped_bits() {
    // Flipping any bit that RFC 8439 §2.5.1 clamps away must not change the tag.
    let base = key32_random();
    let clamp_mask: [u8; 16] = [
        0xff, 0xff, 0xff, 0x0f, // r0: bits 28..31 of byte 3 masked
        0xfc, 0xff, 0xff, 0x0f, // r1
        0xfc, 0xff, 0xff, 0x0f, // r2
        0xfc, 0xff, 0xff, 0x0f, // r3
    ];
    let msg = pattern(97);
    let want = Poly1305::mac(&base, &msg);

    for i in 0..16 {
        for bit in 0..8 {
            if clamp_mask[i] & (1 << bit) == 0 {
                let mut k = base;
                k[i] ^= 1 << bit;
                assert_eq!(
                    Poly1305::mac(&k, &msg),
                    want,
                    "clamped bit {} of r byte {} changed the tag",
                    bit,
                    i
                );
            }
        }
    }

    // The full 0xff key and the fully clamped equivalent must agree.
    let mut clamped = [0xffu8; 32];
    for i in 0..16 {
        clamped[i] &= clamp_mask[i];
    }
    assert_eq!(Poly1305::mac(&[0xffu8; 32], &msg), Poly1305::mac(&clamped, &msg));

    // The `s` half is used verbatim: flipping any bit of `s` changes the tag.
    for bit in 0..8 {
        let mut k = base;
        k[16] ^= 1 << bit;
        assert_ne!(Poly1305::mac(&k, &msg), want, "s bit {bit} must affect the tag");
    }
}

#[test]
fn poly1305_chunked_update_matches_one_shot() {
    let mut rng = ChaChaRng::seed_from_u64(0xfeed_face_0000_1111);
    for iter in 0..500 {
        let mut key = key32();
        for b in &mut key {
            *b = rng.next_u64() as u8;
        }
        let len = rng.random_range(0..700);
        let msg = pattern(len);
        let want = Poly1305::mac(&key, &msg);

        // Random chunking, including empty updates and 1-byte updates.
        let mut mac = Poly1305::new(&key);
        let mut offset = 0;
        while offset < len {
            let take = match rng.random_range(0..5) {
                0 => 1,
                1 => 15,
                2 => 16,
                3 => 17,
                _ => 1 + rng.random_range(0usize..70),
            };
            let end = (offset + take).min(len);
            mac.update(&msg[offset..end]);
            offset = end;
            if rng.random_range(0..8) == 0 {
                mac.update(&[]);
            }
        }
        assert_eq!(mac.finalize(), want, "iter={iter} len={len}");
    }

    // Exhaustive two-way splits at the block boundaries.
    let key = key32_high();
    for len in 0..=200usize {
        let msg = pattern(len);
        let want = Poly1305::mac(&key, &msg);
        for &a in &[0usize, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65] {
            if a > len {
                continue;
            }
            let mut mac = Poly1305::new(&key);
            mac.update(&msg[..a]);
            mac.update(&msg[a..]);
            assert_eq!(mac.finalize(), want, "len={len} a={a}");
        }
    }
}

#[test]
fn poly1305_extreme_message_values() {
    // All-ones blocks, blocks equal to 2^130-5 and neighbours, long runs of
    // 0xff; these stress the carry/reduction paths.
    let keys = [key32(), key32_high(), key32_random()];
    let messages: Vec<Vec<u8>> = vec![
        vec![0u8; 16],
        vec![0xffu8; 16],
        vec![0xffu8; 32],
        vec![0u8; 64],
        vec![0xffu8; 256],
        hex::decode("fffffffffffffffffffffffffffffffffbfefefefefefefefefefefefefefefe01010101010101010101010101010101")
            .unwrap(),
        {
            // 2^130 - 5 encoded little-endian in 17 bytes.
            let mut v = vec![0u8; 17];
            v[..16].copy_from_slice(&[
                0xfb, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            ]);
            v[16] = 0x03;
            v
        },
    ];
    for key in keys {
        for msg in &messages {
            assert_eq!(Poly1305::mac(&key, msg), poly1305_reference(&key, msg));
        }
    }
}

// ---------------------------------------------------------------------------
// ChaCha20-Poly1305
// ---------------------------------------------------------------------------

#[test]
fn aead_rfc8439_section_2_8_2() {
    let key: [u8; 32] = hex::decode("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f")
        .unwrap()
        .try_into()
        .unwrap();
    let nonce: [u8; 12] = hex::decode("070000004041424344454647").unwrap().try_into().unwrap();
    let aad = hex::decode("50515253c0c1c2c3c4c5c6c7").unwrap();
    let plaintext = hex::decode(concat!(
        "4c616469657320616e642047656e746c656d656e206f662074686520636c6173",
        "73206f66202739393a204966204920636f756c64206f6666657220796f75206f",
        "6e6c79206f6e652074697020666f7220746865206675747572652c2073756e73",
        "637265656e20776f756c642062652069742e"
    ))
    .unwrap();
    let expected_ct = hex::decode(concat!(
        "d31a8d34648e60db7b86afbc53ef7ec2a4aded51296e08fea9e2b5a736ee62d6",
        "3dbea45e8ca9671282fafb69da92728b1a71de0a9e060b2905d6a5b67ecd3b36",
        "92ddbd7f2d778b8c9803aee328091b58fab324e4fad675945585808b4831d7bc",
        "3ff4def08e4b7a9de576d26586cec64b6116"
    ))
    .unwrap();
    let expected_tag = hex::decode("1ae10b594f09e26a7e902ecbd0600691").unwrap();

    let cipher = ChaCha20Poly1305::new(&key);
    let mut buf = plaintext.clone();
    let tag = cipher.encrypt_in_place(&mut buf, &nonce, &aad).unwrap();
    assert_eq!(buf, expected_ct);
    assert_eq!(tag.as_ref(), &expected_tag[..]);

    let mut decrypted = expected_ct.clone();
    cipher
        .decrypt_in_place(&mut decrypted, &nonce, &aad, &expected_tag)
        .unwrap();
    assert_eq!(decrypted, plaintext);
}

#[test]
fn aead_rfc8439_appendix_a5_decrypt() {
    let key: [u8; 32] = hex::decode("1c9240a5eb55d38af333888604f6b5f0473917c1402b80099dca5cbc207075c0")
        .unwrap()
        .try_into()
        .unwrap();
    let nonce: [u8; 12] = hex::decode("000000000102030405060708").unwrap().try_into().unwrap();
    let aad = hex::decode("f33388860000000000004e91").unwrap();
    let ciphertext = hex::decode(concat!(
        "64a0861575861af460f062c79be643bd5e805cfd345cf389f108670ac76c8cb2",
        "4c6cfc18755d43eea09ee94e382d26b0bdb7b73c321b0100d4f03b7f355894cf",
        "332f830e710b97ce98c8a84abd0b948114ad176e008d33bd60f982b1ff37c855",
        "9797a06ef4f0ef61c186324e2b3506383606907b6a7c02b0f9f6157b53c867e4",
        "b9166c767b804d46a59b5216cde7a4e99040c5a40433225ee282a1b0a06c523e",
        "af4534d7f83fa1155b0047718cbc546a0d072b04b3564eea1b422273f548271a",
        "0bb2316053fa76991955ebd63159434ecebb4e466dae5a1073a6727627097a10",
        "49e617d91d361094fa68f0ff77987130305beaba2eda04df997b714d6c6f2c29",
        "a6ad5cb4022b02709b"
    ))
    .unwrap();
    let tag = hex::decode("eead9d67890cbb22392336fea1851f38").unwrap();
    let expected_plaintext = hex::decode(concat!(
        "496e7465726e65742d4472616674732061726520647261667420646f63756d65",
        "6e74732076616c696420666f722061206d6178696d756d206f6620736978206d",
        "6f6e74687320616e64206d617920626520757064617465642c207265706c6163",
        "65642c206f72206f62736f6c65746564206279206f7468657220646f63756d65",
        "6e747320617420616e792074696d652e20497420697320696e617070726f7072",
        "6961746520746f2075736520496e7465726e65742d4472616674732061732072",
        "65666572656e6365206d6174657269616c206f7220746f206369746520746865",
        "6d206f74686572207468616e206173202fe2809c776f726b20696e2070726f67",
        "726573732e2fe2809d"
    ))
    .unwrap();

    let cipher = ChaCha20Poly1305::new(&key);
    let mut buf = ciphertext.clone();
    cipher.decrypt_in_place(&mut buf, &nonce, &aad, &tag).unwrap();
    assert_eq!(buf, expected_plaintext);
}

#[test]
fn aead_differential_against_reference() {
    let key = key32_random();
    let nonce = nonce12();
    let cipher = ChaCha20Poly1305::new(&key);

    let lengths: Vec<usize> = (0..=80)
        .chain([
            95, 96, 97, 127, 128, 129, 191, 192, 193, 255, 256, 257, 511, 512, 513, 1024,
        ])
        .collect();
    let aad_lengths = [0usize, 1, 2, 15, 16, 17, 31, 32, 33, 63, 64, 65];

    for &len in &lengths {
        for &alen in &aad_lengths {
            // Keep the total work bounded for the long lengths.
            if len > 200 && alen > 33 {
                continue;
            }
            let pt = pattern(len);
            let aad = pattern(alen);
            let (want_ct, want_tag) = ref_aead_with_key(&key, &nonce, &pt, &aad);

            let mut buf = pt.clone();
            let tag = cipher.encrypt_in_place(&mut buf, &nonce, &aad).unwrap();
            assert_eq!(buf, want_ct, "ciphertext len={len} aad={alen}");
            assert_eq!(tag.as_ref(), &want_tag[..], "tag len={len} aad={alen}");

            let mut dec = buf.clone();
            cipher.decrypt_in_place(&mut dec, &nonce, &aad, tag.as_ref()).unwrap();
            assert_eq!(dec, pt, "roundtrip len={len} aad={alen}");
        }
    }
}

#[test]
fn aead_empty_plaintext_and_aad() {
    let key = key32();
    let nonce = nonce12();
    let cipher = ChaCha20Poly1305::new(&key);

    for aad in [&b""[..], &b"x"[..], &[0u8; 16][..]] {
        let mut buf: Vec<u8> = Vec::new();
        let tag = cipher.encrypt_in_place(&mut buf, &nonce, aad).unwrap();
        let (want_ct, want_tag) = ref_aead_with_key(&key, &nonce, &[], aad);
        assert!(want_ct.is_empty());
        assert_eq!(tag.as_ref(), &want_tag[..], "empty plaintext aad={}", aad.len());
        assert!(buf.is_empty());

        let mut dec: Vec<u8> = Vec::new();
        cipher.decrypt_in_place(&mut dec, &nonce, aad, tag.as_ref()).unwrap();
        assert!(dec.is_empty());

        // A tag that authenticates empty plaintext must not authenticate a
        // one-byte plaintext.
        let mut one = vec![0u8];
        cipher
            .decrypt_in_place(&mut one, &nonce, aad, tag.as_ref())
            .unwrap_err();
    }
}

#[test]
fn aead_boundary_lengths() {
    let key = key32();
    let nonce = nonce12();
    let cipher = ChaCha20Poly1305::new(&key);

    for &len in &[
        15usize, 16, 17, 31, 32, 33, 47, 48, 49, 62, 63, 64, 65, 66, 127, 128, 129, 130,
    ] {
        for &alen in &[0usize, 15, 16, 17] {
            let pt = pattern(len);
            let aad = pattern(alen);
            let (want_ct, want_tag) = ref_aead_with_key(&key, &nonce, &pt, &aad);
            let mut buf = pt.clone();
            let tag = cipher.encrypt_in_place(&mut buf, &nonce, &aad).unwrap();
            assert_eq!(buf, want_ct, "len={len} aad={alen}");
            assert_eq!(tag.as_ref(), &want_tag[..], "len={len} aad={alen}");
        }
    }
}

/// Every AAD length 0..=33 (both partial-block edges 0/1/15/16/17/31/32/33)
/// for every partial-block plaintext length and exact block multiples, for
/// both AEADs. The existing boundary test only samples a subset of AAD
/// lengths; the padding decision changes at *every* length modulo 16, so this
/// checks all of them against the independent AEAD reference.
#[test]
fn aead_all_aad_lengths_0_to_33() {
    let key = key32_random();
    let nonce = nonce12();
    let nonce24 = nonce24();
    let c20 = ChaCha20Poly1305::new(&key);
    let cx = XChaCha20Poly1305::new(&key);

    for len in [
        0usize, 1, 2, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257,
    ] {
        let pt = pattern(len);
        for alen in 0..=33usize {
            let aad = pattern(alen);

            let (want_ct, want_tag) = ref_aead_with_key(&key, &nonce, &pt, &aad);
            let mut buf = pt.clone();
            let tag = c20.encrypt_in_place(&mut buf, &nonce, &aad).unwrap();
            assert_eq!(buf, want_ct, "chacha ct len={len} aad={alen}");
            assert_eq!(tag.as_ref(), &want_tag[..], "chacha tag len={len} aad={alen}");
            let mut dec = buf.clone();
            c20.decrypt_in_place(&mut dec, &nonce, &aad, tag.as_ref()).unwrap();
            assert_eq!(dec, pt, "chacha roundtrip len={len} aad={alen}");

            let (want_ct, want_tag) = ref_xaead(&key, &nonce24, &pt, &aad);
            let mut buf = pt.clone();
            let tag = cx.encrypt_in_place(&mut buf, &nonce24, &aad).unwrap();
            assert_eq!(buf, want_ct, "xchacha ct len={len} aad={alen}");
            assert_eq!(tag.as_ref(), &want_tag[..], "xchacha tag len={len} aad={alen}");
            let mut dec = buf.clone();
            cx.decrypt_in_place(&mut dec, &nonce24, &aad, tag.as_ref()).unwrap();
            assert_eq!(dec, pt, "xchacha roundtrip len={len} aad={alen}");
        }
    }
}

#[test]
fn aead_tamper_rejection() {
    let key = key32();
    let nonce = nonce12();
    let aad = b"associated data".to_vec();
    let cipher = ChaCha20Poly1305::new(&key);

    let pt = pattern(70);
    let mut ct = pt.clone();
    let tag = cipher.encrypt_in_place(&mut ct, &nonce, &aad).unwrap();
    let good_tag: Vec<u8> = tag.as_ref().to_vec();

    // Every single-bit flip of the tag must be rejected.
    for byte in 0..16 {
        for bit in 0..8 {
            let mut bad = good_tag.clone();
            bad[byte] ^= 1 << bit;
            let mut buf = ct.clone();
            assert_eq!(
                cipher.decrypt_in_place(&mut buf, &nonce, &aad, &bad),
                Err(AeadError::InvalidCiphertext),
                "tag bit {byte}:{bit} accepted"
            );
        }
    }

    // Every single-byte ciphertext mutation must be rejected.
    for i in 0..ct.len() {
        let mut bad = ct.clone();
        bad[i] ^= 0x01;
        let mut buf = bad;
        assert!(
            cipher.decrypt_in_place(&mut buf, &nonce, &aad, &good_tag).is_err(),
            "ct byte {i}"
        );
    }

    // AAD and nonce changes must be rejected.
    let mut buf = ct.clone();
    assert!(
        cipher
            .decrypt_in_place(&mut buf, &nonce, b"associated datA", &good_tag)
            .is_err()
    );
    let mut other_nonce = nonce;
    other_nonce[0] ^= 1;
    assert!(
        cipher
            .decrypt_in_place(&mut buf, &other_nonce, &aad, &good_tag)
            .is_err()
    );

    // Wrong tag lengths are rejected rather than compared loosely.
    for bad_len in [0usize, 1, 8, 15, 17, 32] {
        let mut buf = ct.clone();
        let bad = vec![0u8; bad_len];
        assert_eq!(
            cipher.decrypt_in_place(&mut buf, &nonce, &aad, &bad),
            Err(AeadError::InvalidCiphertext),
            "tag length {bad_len}"
        );
    }

    // Wrong nonce length is an explicit error on both decrypt and encrypt.
    for bad_len in [0usize, 8, 11, 13, 24] {
        let mut buf = ct.clone();
        assert_eq!(
            cipher.decrypt_in_place(&mut buf, &vec![0u8; bad_len], &aad, &good_tag),
            Err(AeadError::InvalidNonce),
            "nonce length {bad_len}"
        );
    }
    let mut buf = pattern(16);
    let before = buf.clone();
    assert_eq!(
        cipher.encrypt_in_place(&mut buf, &[0u8; 11], &aad).err(),
        Some(AeadError::InvalidNonce)
    );
    assert_eq!(buf, before, "buffer modified on a nonce error");
}

#[test]
fn aead_owned_buffer_api() {
    let key = key32_random();
    let nonce = nonce12();
    let aad = b"aad".to_vec();
    let cipher = ChaCha20Poly1305::new(&key);

    for len in [0usize, 1, 15, 16, 17, 63, 64, 65, 100, 256] {
        let pt = pattern(len);
        let sealed = cipher.encrypt(&pt, &nonce, &aad).unwrap();
        assert_eq!(sealed.len(), len + 16);
        let (want_ct, want_tag) = ref_aead_with_key(&key, &nonce, &pt, &aad);
        assert_eq!(&sealed[..len], &want_ct[..]);
        assert_eq!(&sealed[len..], &want_tag[..]);

        let opened = cipher.decrypt(&sealed, &nonce, &aad).unwrap();
        assert_eq!(opened, pt);

        // Tampered input yields an error, never partial plaintext.
        let mut bad = sealed.clone();
        bad[len / 2] ^= 1;
        assert!(cipher.decrypt(&bad, &nonce, &aad).is_err());

        // Too short (< TAG_SIZE) is rejected up front.
        assert_eq!(
            cipher.decrypt(&sealed[..15.min(sealed.len())], &nonce, &aad),
            Err(AeadError::InvalidCiphertext)
        );
    }
}

#[test]
fn aead_decrypt_failure_never_exposes_plaintext() {
    let key = key32();
    let nonce = nonce12();
    let aad = b"ad";

    // RFC 8439 / trait contract: buffer contents are unspecified after a failed
    // tag check, but no caller may use them. Still, verify that no
    // unauthenticated *plaintext* is put there and that the returned value is
    // an error for both AEADs.
    for &which in &[0u8, 1] {
        let pt = pattern(100);
        let mut ct;
        let tag;
        if which == 0 {
            let cipher = ChaCha20Poly1305::new(&key);
            ct = pt.clone();
            tag = cipher.encrypt_in_place(&mut ct, &nonce, aad).unwrap();
            let mut bad_tag: Vec<u8> = tag.as_ref().to_vec();
            bad_tag[0] ^= 0x80;
            let mut buf = ct.clone();
            assert_eq!(
                cipher.decrypt_in_place(&mut buf, &nonce, aad, &bad_tag),
                Err(AeadError::InvalidCiphertext)
            );
            // Observed behavior (not a guarantee): untouched ciphertext.
            assert_eq!(buf, ct, "ChaCha20Poly1305 left unexpected data on failure");
            assert_ne!(buf, pt, "ChaCha20Poly1305 must not emit plaintext on failure");
        } else {
            let cipher = XChaCha20Poly1305::new(&key);
            ct = pt.clone();
            tag = cipher.encrypt_in_place(&mut ct, &nonce24(), aad).unwrap();
            let mut bad_tag: Vec<u8> = tag.as_ref().to_vec();
            bad_tag[0] ^= 0x80;
            let mut buf = ct.clone();
            assert_eq!(
                cipher.decrypt_in_place(&mut buf, &nonce24(), aad, &bad_tag),
                Err(AeadError::InvalidCiphertext)
            );
            // Observed behavior (not a guarantee): buffer zeroed.
            assert!(buf.iter().all(|b| *b == 0), "XChaCha20Poly1305 should zero on failure");
            assert_ne!(buf, pt);
        }
    }
}

#[test]
fn xchacha20_poly1305_matches_draft_and_reference() {
    // draft-irtf-cfrg-xchacha-03 §A.3.1
    let key: [u8; 32] = hex::decode("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f")
        .unwrap()
        .try_into()
        .unwrap();
    let nonce: [u8; 24] = hex::decode("404142434445464748494a4b4c4d4e4f5051525354555657")
        .unwrap()
        .try_into()
        .unwrap();
    let aad = hex::decode("50515253c0c1c2c3c4c5c6c7").unwrap();
    let pt = hex::decode(concat!(
        "4c616469657320616e642047656e746c656d656e206f662074686520636c6173",
        "73206f66202739393a204966204920636f756c64206f6666657220796f75206f",
        "6e6c79206f6e652074697020666f7220746865206675747572652c2073756e73",
        "637265656e20776f756c642062652069742e"
    ))
    .unwrap();
    let expected_tag = hex::decode("c0875924c1c7987947deafd8780acf49").unwrap();
    let expected_ct = hex::decode(concat!(
        "bd6d179d3e83d43b9576579493c0e939572a1700252bfaccbed2902c21396cbb",
        "731c7f1b0b4aa6440bf3a82f4eda7e39ae64c6708c54c216cb96b72e1213b452",
        "2f8c9ba40db5d945b11b69b982c1bb9e3f3fac2bc369488f76b2383565d3fff9",
        "21f9664c97637da9768812f615c68b13b52e"
    ))
    .unwrap();

    let cipher = XChaCha20Poly1305::new(&key);
    let mut buf = pt.clone();
    let tag = cipher.encrypt_in_place(&mut buf, &nonce, &aad).unwrap();
    assert_eq!(buf, expected_ct);
    assert_eq!(tag.as_ref(), &expected_tag[..]);

    // Differential sweep including AAD boundaries and empty inputs.
    let key = key32_random();
    let nonce = nonce24();
    let cipher = XChaCha20Poly1305::new(&key);
    for len in [0usize, 1, 15, 16, 17, 63, 64, 65, 127, 128, 129, 200] {
        for alen in [0usize, 1, 15, 16, 17, 32, 64] {
            let pt = pattern(len);
            let aad = pattern(alen);
            let (want_ct, want_tag) = ref_xaead(&key, &nonce, &pt, &aad);
            let mut buf = pt.clone();
            let tag = cipher.encrypt_in_place(&mut buf, &nonce, &aad).unwrap();
            assert_eq!(buf, want_ct, "xchacha len={len} aad={alen}");
            assert_eq!(tag.as_ref(), &want_tag[..], "xchacha len={len} aad={alen}");
            let mut dec = buf.clone();
            cipher.decrypt_in_place(&mut dec, &nonce, &aad, tag.as_ref()).unwrap();
            assert_eq!(dec, pt);
        }
    }

    // A nonce of the wrong size is rejected.
    for bad_len in [0usize, 12, 23, 25] {
        let mut buf = vec![0u8; 1];
        assert_eq!(
            cipher.decrypt_in_place(&mut buf, &vec![0u8; bad_len], b"", &[0u8; 16]),
            Err(AeadError::InvalidNonce)
        );
    }
}

#[test]
fn aead_guard_canaries() {
    let key = key32();
    let nonce = nonce12();
    let aad = b"canary";
    let cipher = ChaCha20Poly1305::new(&key);
    const CANARY: usize = 32;

    for &offset in &[0usize, 1, 7, 15, 63, 64, 65] {
        for &len in &[0usize, 1, 15, 16, 17, 63, 64, 65, 127, 128, 129] {
            let pt = pattern(len);
            let mut buf = vec![0x5au8; offset + len + 2 * CANARY];
            buf[offset..offset + len].copy_from_slice(&pt);

            let tag = {
                let slice = &mut buf[offset..offset + len];
                cipher.encrypt_in_place(slice, &nonce, aad).unwrap()
            };
            assert!(buf[..offset].iter().all(|b| *b == 0x5a), "prefix offset={offset} len={len}");
            assert!(
                buf[offset + len..].iter().all(|b| *b == 0x5a),
                "suffix offset={offset} len={len}"
            );

            let (want_ct, want_tag) = ref_aead_with_key(&key, &nonce, &pt, aad);
            assert_eq!(&buf[offset..offset + len], &want_ct[..]);
            assert_eq!(tag.as_ref(), &want_tag[..]);

            {
                let slice = &mut buf[offset..offset + len];
                cipher.decrypt_in_place(slice, &nonce, aad, tag.as_ref()).unwrap();
            }
            assert_eq!(&buf[offset..offset + len], &pt[..]);
            assert!(buf[..offset].iter().all(|b| *b == 0x5a));
            assert!(buf[offset + len..].iter().all(|b| *b == 0x5a));
        }
    }
}

#[test]
fn aead_rejects_absurd_length_without_touching_buffer() {
    // The declared maximum is > 256 GiB, so it cannot be allocated here. Only
    // the cheap pre-checks are exercised: an empty buffer plus a valid tag of
    // the wrong length/content must fail cleanly.
    let cipher = ChaCha20Poly1305::new(&key32());
    let mut empty: Vec<u8> = Vec::new();
    assert!(
        cipher
            .decrypt_in_place(&mut empty, &nonce12(), b"", &[0u8; 16])
            .is_err()
    );
    assert!(empty.is_empty());
}

// ---------------------------------------------------------------------------
// Cross-cutting: same keystream length limit for ChaCha20-Poly1305 and
// consistency between the stream cipher and the AEAD.
// ---------------------------------------------------------------------------

#[test]
fn aead_uses_ietf_counter_one_for_the_payload() {
    let key = key32();
    let nonce = nonce12();
    let pt = pattern(300);

    let mut aead_ct = pt.clone();
    let _tag = ChaCha20Poly1305::new(&key)
        .encrypt_in_place(&mut aead_ct, &nonce, b"")
        .unwrap();

    // The AEAD payload keystream is ChaCha20-IETF starting at counter 1, so
    // XORing with the raw stream cipher must give the ciphertext back.
    let mut raw = pt.clone();
    let mut stream = ChaCha20Ietf::new(&key, &nonce);
    stream.set_counter(1);
    stream.xor_keystream(&mut raw).unwrap();
    assert_eq!(raw, aead_ct);

    // Block 0 must not be used for the payload: counter 0 keystream differs.
    let mut zero = pt.clone();
    ChaCha20Ietf::new(&key, &nonce).xor_keystream(&mut zero).unwrap();
    assert_ne!(zero, aead_ct);
}

// ---------------------------------------------------------------------------
// Miri-only subset
// ---------------------------------------------------------------------------

/// Paths Miri can execute: the generic (non-SIMD) ChaCha path, Poly1305 and the
/// AEAD construction on top of them.
///
/// Every buffer is kept below the 128-byte SIMD dispatch threshold so that
/// `chacha_neon` (aarch64) or the AVX2/AVX-512 kernels are never entered:
/// Miri does not implement those intrinsics. The SIMD kernels are covered by
/// `audit_chacha_simd.rs` under the native runner instead.
#[cfg(miri)]
#[test]
fn miri_generic_chacha_poly1305_and_aead() {
    let key = key32();

    // Generic ChaCha path, every length up to the dispatch threshold.
    for len in 0..=127usize {
        let mut got = vec![0u8; len];
        ChaCha20Djb::new(&key, &nonce8()).xor_keystream(&mut got).unwrap();
        let mut want = vec![0u8; len];
        ref_keystream(20, &ref_state_djb(&key, &nonce8()), false, 0, &mut want);
        assert_eq!(got, want, "miri DJB len={len}");

        let mut got = vec![0u8; len];
        ChaCha20Ietf::new(&key, &nonce12()).xor_keystream(&mut got).unwrap();
        let mut want = vec![0u8; len];
        ref_keystream(20, &ref_state_ietf(&key, &nonce12()), true, 0, &mut want);
        assert_eq!(got, want, "miri IETF len={len}");
    }

    // Chunked calls that exercise the 63-byte leftover buffer.
    for &(len, a, b) in &[(127usize, 1usize, 63usize), (120, 63, 1), (100, 17, 64), (127, 126, 0)] {
        let pt = pattern(len);
        let mut want = pt.clone();
        ChaCha20Djb::new(&key, &nonce8()).xor_keystream(&mut want).unwrap();
        let mut got = pt.clone();
        let mut c = ChaCha20Djb::new(&key, &nonce8());
        c.xor_keystream(&mut got[..a]).unwrap();
        c.xor_keystream(&mut got[a..a + b]).unwrap();
        c.xor_keystream(&mut got[a + b..]).unwrap();
        assert_eq!(got, want, "miri chunked len={len}");
    }

    // Guard canaries around a non-zero offset slice (checks the raw pointer
    // arithmetic in `chacha_generic` does not step outside the slice).
    for &offset in &[0usize, 1, 7] {
        for &len in &[0usize, 1, 63, 64, 65, 127] {
            let mut buf = vec![0x3cu8; offset + len + 16];
            ChaCha20Ietf::new(&key, &nonce12())
                .xor_keystream(&mut buf[offset..offset + len])
                .unwrap();
            assert!(buf[..offset].iter().all(|x| *x == 0x3c));
            assert!(buf[offset + len..].iter().all(|x| *x == 0x3c));
        }
    }

    // Poly1305: empty, partial-block, whole-block and multi-block messages.
    for len in 0..=80usize {
        let msg = pattern(len);
        assert_eq!(Poly1305::mac(&key, &msg), poly1305_reference(&key, &msg), "miri poly len={len}");
    }
    let mut mac = Poly1305::new(&key);
    mac.update(&pattern(37));
    assert_eq!(mac.finalize(), Poly1305::mac(&key, &pattern(37)));

    // AEAD: small plaintexts and AADs stay on the generic path.
    let cipher = ChaCha20Poly1305::new(&key);
    for len in [0usize, 1, 15, 16, 17, 63, 64, 65, 100] {
        for alen in [0usize, 1, 16, 17] {
            let pt = pattern(len);
            let aad = pattern(alen);
            let (want_ct, want_tag) = ref_aead_with_key(&key, &nonce12(), &pt, &aad);
            let mut buf = pt.clone();
            let tag = cipher.encrypt_in_place(&mut buf, &nonce12(), &aad).unwrap();
            assert_eq!(buf, want_ct, "miri aead len={len} aad={alen}");
            assert_eq!(tag.as_ref(), &want_tag[..]);
            cipher
                .decrypt_in_place(&mut buf, &nonce12(), &aad, tag.as_ref())
                .unwrap();
            assert_eq!(buf, pt);
        }
    }
}
