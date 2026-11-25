//! Tests for SHA-2 (`crypto::sha2`).
//!
//! Differential tests of the public `Sha256` / `Sha384` / `Sha512` API against an
//! independent, straightforward reference implementation, plus padding/length
//! boundary sweeps, incremental re-chunking, unaligned inputs, canaries and a
//! read-guard page test.
//!
//! Hardware-specific kernel comparisons (hardware vs `compress_block_software`)
//! live in the crate-internal kernel test modules of `src/sha2/*.rs`, because
//! they need access to private items.

use crypto::{
    Hasher,
    sha2::{Sha256, Sha384, Sha512},
};
use random::{RngExt, SeedableRng, rngs::ChaChaRng};

////////////////////////////////////////////////////////////////////////////////
// Independent reference implementations (FIPS 180-4), written from the spec.
////////////////////////////////////////////////////////////////////////////////
const K256: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98,
    0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
    0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8,
    0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819,
    0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
    0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
    0xc67178f2,
];

const K512: [u64; 80] = [
    0x428a2f98d728ae22,
    0x7137449123ef65cd,
    0xb5c0fbcfec4d3b2f,
    0xe9b5dba58189dbbc,
    0x3956c25bf348b538,
    0x59f111f1b605d019,
    0x923f82a4af194f9b,
    0xab1c5ed5da6d8118,
    0xd807aa98a3030242,
    0x12835b0145706fbe,
    0x243185be4ee4b28c,
    0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f,
    0x80deb1fe3b1696b1,
    0x9bdc06a725c71235,
    0xc19bf174cf692694,
    0xe49b69c19ef14ad2,
    0xefbe4786384f25e3,
    0x0fc19dc68b8cd5b5,
    0x240ca1cc77ac9c65,
    0x2de92c6f592b0275,
    0x4a7484aa6ea6e483,
    0x5cb0a9dcbd41fbd4,
    0x76f988da831153b5,
    0x983e5152ee66dfab,
    0xa831c66d2db43210,
    0xb00327c898fb213f,
    0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2,
    0xd5a79147930aa725,
    0x06ca6351e003826f,
    0x142929670a0e6e70,
    0x27b70a8546d22ffc,
    0x2e1b21385c26c926,
    0x4d2c6dfc5ac42aed,
    0x53380d139d95b3df,
    0x650a73548baf63de,
    0x766a0abb3c77b2a8,
    0x81c2c92e47edaee6,
    0x92722c851482353b,
    0xa2bfe8a14cf10364,
    0xa81a664bbc423001,
    0xc24b8b70d0f89791,
    0xc76c51a30654be30,
    0xd192e819d6ef5218,
    0xd69906245565a910,
    0xf40e35855771202a,
    0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8,
    0x1e376c085141ab53,
    0x2748774cdf8eeb99,
    0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63,
    0x4ed8aa4ae3418acb,
    0x5b9cca4f7763e373,
    0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc,
    0x78a5636f43172f60,
    0x84c87814a1f0ab72,
    0x8cc702081a6439ec,
    0x90befffa23631e28,
    0xa4506cebde82bde9,
    0xbef9a3f7b2c67915,
    0xc67178f2e372532b,
    0xca273eceea26619c,
    0xd186b8c721c0c207,
    0xeada7dd6cde0eb1e,
    0xf57d4f7fee6ed178,
    0x06f067aa72176fba,
    0x0a637dc5a2c898a6,
    0x113f9804bef90dae,
    0x1b710b35131c471b,
    0x28db77f523047d84,
    0x32caab7b40c72493,
    0x3c9ebe0a15c9bebc,
    0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6,
    0x597f299cfc657e2a,
    0x5fcb6fab3ad6faec,
    0x6c44198c4a475817,
];

fn ref_compress256(state: &mut [u32; 8], block: &[u8]) {
    let mut w = [0u32; 64];
    for (i, word) in w.iter_mut().take(16).enumerate() {
        *word = u32::from_be_bytes([block[4 * i], block[4 * i + 1], block[4 * i + 2], block[4 * i + 3]]);
    }
    for i in 16..64 {
        let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
        let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
        w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
    }

    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;
    for i in 0..64 {
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch = (e & f) ^ (!e & g);
        let t1 = h
            .wrapping_add(s1)
            .wrapping_add(ch)
            .wrapping_add(K256[i])
            .wrapping_add(w[i]);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let t2 = s0.wrapping_add(maj);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }
    for (s, v) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
        *s = s.wrapping_add(v);
    }
}

fn ref_sha256(msg: &[u8]) -> [u8; 32] {
    let mut state: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    let (blocks, rem) = msg.as_chunks::<64>();
    for block in blocks {
        ref_compress256(&mut state, block);
    }

    let mut tail = [0u8; 128];
    tail[..rem.len()].copy_from_slice(rem);
    tail[rem.len()] = 0x80;
    let len_off = if rem.len() < 56 { 56 } else { 120 };
    let bit_len = (msg.len() as u64).wrapping_mul(8);
    tail[len_off..len_off + 8].copy_from_slice(&bit_len.to_be_bytes());
    for block in tail[..len_off + 8].as_chunks::<64>().0 {
        ref_compress256(&mut state, block);
    }

    let mut out = [0u8; 32];
    for (i, word) in state.iter().enumerate() {
        out[4 * i..4 * i + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

fn ref_compress512(state: &mut [u64; 8], block: &[u8]) {
    let mut w = [0u64; 80];
    for (i, word) in w.iter_mut().take(16).enumerate() {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&block[8 * i..8 * i + 8]);
        *word = u64::from_be_bytes(bytes);
    }
    for i in 16..80 {
        let s0 = w[i - 15].rotate_right(1) ^ w[i - 15].rotate_right(8) ^ (w[i - 15] >> 7);
        let s1 = w[i - 2].rotate_right(19) ^ w[i - 2].rotate_right(61) ^ (w[i - 2] >> 6);
        w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
    }

    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;
    for i in 0..80 {
        let s1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
        let ch = (e & f) ^ (!e & g);
        let t1 = h
            .wrapping_add(s1)
            .wrapping_add(ch)
            .wrapping_add(K512[i])
            .wrapping_add(w[i]);
        let s0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let t2 = s0.wrapping_add(maj);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }
    for (s, v) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
        *s = s.wrapping_add(v);
    }
}

/// Reference SHA-512 core, used for both SHA-512 and (truncated) SHA-384.
fn ref_sha512(msg: &[u8], iv: [u64; 8]) -> [u8; 64] {
    let mut state = iv;
    let (blocks, rem) = msg.as_chunks::<128>();
    for block in blocks {
        ref_compress512(&mut state, block);
    }

    let mut tail = [0u8; 256];
    tail[..rem.len()].copy_from_slice(rem);
    tail[rem.len()] = 0x80;
    let len_off = if rem.len() < 112 { 112 } else { 240 };
    let bit_len = (msg.len() as u128).wrapping_mul(8);
    tail[len_off..len_off + 16].copy_from_slice(&bit_len.to_be_bytes());
    for block in tail[..len_off + 16].as_chunks::<128>().0 {
        ref_compress512(&mut state, block);
    }

    let mut out = [0u8; 64];
    for (i, word) in state.iter().enumerate() {
        out[8 * i..8 * i + 8].copy_from_slice(&word.to_be_bytes());
    }
    out
}

const IV512: [u64; 8] = [
    0x6a09e667f3bcc908,
    0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b,
    0xa54ff53a5f1d36f1,
    0x510e527fade682d1,
    0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b,
    0x5be0cd19137e2179,
];

const IV384: [u64; 8] = [
    0xcbbb9d5dc1059ed8,
    0x629a292a367cd507,
    0x9159015a3070dd17,
    0x152fecd8f70e5939,
    0x67332667ffc00b31,
    0x8eb44a8768581511,
    0xdb0c2e0d64f98fa7,
    0x47b5481dbefa4fa4,
];

fn ref_sha512_of(msg: &[u8]) -> [u8; 64] {
    ref_sha512(msg, IV512)
}

fn ref_sha384_of(msg: &[u8]) -> [u8; 48] {
    let full = ref_sha512(msg, IV384);
    let mut out = [0u8; 48];
    out.copy_from_slice(&full[..48]);
    out
}

////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////
/// Deterministic seeded `ChaChaRng` (deterministic and reproducible).
fn sha256_chunked(data: &[u8], chunk: usize) -> [u8; 32] {
    let mut h = Sha256::new();
    for part in data.chunks(chunk) {
        h.update(part);
    }
    let out = h.sum();
    let mut digest = [0u8; 32];
    digest.copy_from_slice(out.as_ref());
    assert_eq!(out.as_ref().len(), Sha256::OUTPUT_SIZE);
    digest
}

fn sha384_chunked(data: &[u8], chunk: usize) -> [u8; 48] {
    let mut h = Sha384::new();
    for part in data.chunks(chunk) {
        h.update(part);
    }
    let out = h.sum();
    let mut digest = [0u8; 48];
    digest.copy_from_slice(out.as_ref());
    assert_eq!(out.as_ref().len(), Sha384::OUTPUT_SIZE);
    digest
}

fn sha512_chunked(data: &[u8], chunk: usize) -> [u8; 64] {
    let mut h = Sha512::new();
    for part in data.chunks(chunk) {
        h.update(part);
    }
    let out = h.sum();
    let mut digest = [0u8; 64];
    digest.copy_from_slice(out.as_ref());
    assert_eq!(out.as_ref().len(), Sha512::OUTPUT_SIZE);
    digest
}

////////////////////////////////////////////////////////////////////////////////
// Reference self-check + NIST/FIPS vectors
////////////////////////////////////////////////////////////////////////////////
#[test]
fn reference_self_check() {
    assert_eq!(
        hex::encode(ref_sha256(b"abc")),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(
        hex::encode(ref_sha512_of(b"abc")),
        "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a\
         2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
    );
    assert_eq!(
        hex::encode(ref_sha384_of(b"abc")),
        "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed\
         8086072ba1e7cc2358baeca134c825a7"
    );
}

#[test]
fn nist_vectors_one_shot() {
    let vectors256: &[(&[u8], &str)] = &[
        (b"", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
        (b"abc", "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"),
        (
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
        ),
    ];
    for (msg, expected) in vectors256 {
        let one = Sha256::hash(msg);
        assert_eq!(hex::encode(one.as_ref()), *expected);
        assert_eq!(ref_sha256(msg), one.as_ref());
    }

    let vectors512: &[(&[u8], &str)] = &[
        (
            b"",
            "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e",
        ),
        (
            b"abc",
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f",
        ),
    ];
    for (msg, expected) in vectors512 {
        let one = Sha512::hash(msg);
        assert_eq!(hex::encode(one.as_ref()), *expected);
        assert_eq!(ref_sha512_of(msg), one.as_ref());
    }

    let vectors384: &[(&[u8], &str)] = &[
        (
            b"",
            "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b",
        ),
        (
            b"abc",
            "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7",
        ),
    ];
    for (msg, expected) in vectors384 {
        let one = Sha384::hash(msg);
        assert_eq!(hex::encode(one.as_ref()), *expected);
        assert_eq!(ref_sha384_of(msg), one.as_ref());
    }
}

/// FIPS 180-4 "one million a" vectors, one-shot and re-chunked.
#[test]
fn million_a_vectors() {
    let msg = vec![b'a'; 1_000_000];
    assert_eq!(
        hex::encode(Sha256::hash(&msg).as_ref()),
        "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
    );
    assert_eq!(
        hex::encode(Sha512::hash(&msg).as_ref()),
        "e718483d0ce769644e2e42c7bc15b4638e1f98b13b2044285632a803afa973ebde0ff244877ea60a4cb0432ce577c31beb009c5c2c49aa2e4eadb217ad8cc09b"
    );
    assert_eq!(
        hex::encode(Sha384::hash(&msg).as_ref()),
        "9d0e1809716474cb086e834e310a4a1ced149e9c00f248527972cec5704c2a5b07b8b3dc38ecc4ebae97ddd87f3d8985"
    );
    // Chunked with a size that is coprime with both block sizes (64 and 128).
    assert_eq!(sha256_chunked(&msg, 997), Sha256::hash(&msg).as_ref());
    assert_eq!(sha512_chunked(&msg, 997), Sha512::hash(&msg).as_ref());
    assert_eq!(sha384_chunked(&msg, 997), Sha384::hash(&msg).as_ref());
}

////////////////////////////////////////////////////////////////////////////////
// Length sweeps
////////////////////////////////////////////////////////////////////////////////
/// Every length `0..=300` crosses all SHA-256 (55/56/57, 63/64/65, 119/120/121,
/// 127/128/129, 239/240) and SHA-512 (111/112/113, 127/128/129, 239/240)
/// padding/block boundaries, and covers `2 * 128 + 32`.
#[test]
fn length_sweep_matches_reference() {
    let mut rng = ChaChaRng::seed_from_u64(0xC12_5A256);
    for len in 0..=300usize {
        let data = {
            let mut v = vec![0u8; len];
            rng.fill(&mut v);
            v
        };

        let r256 = ref_sha256(&data);
        let r512 = ref_sha512_of(&data);
        let r384 = ref_sha384_of(&data);

        assert_eq!(sha256_chunked(&data, len.max(1)), r256, "sha256 one-shot len={len}");
        assert_eq!(sha512_chunked(&data, len.max(1)), r512, "sha512 one-shot len={len}");
        assert_eq!(sha384_chunked(&data, len.max(1)), r384, "sha384 one-shot len={len}");

        for chunk in [1usize, 2, 7, 63, 64, 65, 127, 128, 129] {
            assert_eq!(sha256_chunked(&data, chunk), r256, "sha256 len={len} chunk={chunk}");
            assert_eq!(sha512_chunked(&data, chunk), r512, "sha512 len={len} chunk={chunk}");
            assert_eq!(sha384_chunked(&data, chunk), r384, "sha384 len={len} chunk={chunk}");
        }
    }
}

/// Split a message at *every* possible position for lengths around the
/// padding and block boundaries.
#[test]
fn every_split_point() {
    let mut rng = ChaChaRng::seed_from_u64(0x5A2_1234);
    for len in [
        0usize, 1, 54, 55, 56, 57, 63, 64, 65, 110, 111, 112, 113, 119, 120, 121, 126, 127, 128, 129, 191, 192, 193,
        239, 240, 241, 255, 256, 257, 288,
    ] {
        let data = {
            let mut v = vec![0u8; len];
            rng.fill(&mut v);
            v
        };
        let r256 = ref_sha256(&data);
        let r512 = ref_sha512_of(&data);
        let r384 = ref_sha384_of(&data);

        for split in 0..=len {
            let mut h = Sha256::new();
            h.update(&data[..split]);
            h.update(&data[split..]);
            assert_eq!(h.sum().as_ref(), r256, "sha256 len={len} split={split}");

            let mut h = Sha512::new();
            h.update(&data[..split]);
            h.update(&data[split..]);
            assert_eq!(h.sum().as_ref(), r512, "sha512 len={len} split={split}");

            let mut h = Sha384::new();
            h.update(&data[..split]);
            h.update(&data[split..]);
            assert_eq!(h.sum().as_ref(), r384, "sha384 len={len} split={split}");
        }
    }
}

/// Three-way and many-way splits, including zero-length updates in between.
#[test]
fn interleaved_zero_length_updates() {
    let mut rng = ChaChaRng::seed_from_u64(0x0BEE_FC12);
    for len in [0usize, 1, 63, 64, 65, 127, 128, 129, 255, 256, 257, 1000] {
        let data = {
            let mut v = vec![0u8; len];
            rng.fill(&mut v);
            v
        };
        let mut h = Sha256::new();
        let mut pos = 0;
        h.update(&[]);
        while pos < len {
            h.update(&[]);
            let take = (pos + 17).min(len);
            h.update(&data[pos..take]);
            h.update(&[]);
            pos = take;
        }
        assert_eq!(h.sum().as_ref(), ref_sha256(&data), "len={len}");
    }
}

/// 1 MiB of pseudo-random data against the independent reference.
#[test]
fn large_input_matches_reference() {
    let data = {
        let mut rng = ChaChaRng::seed_from_u64(0xC12_C12_C12);
        let mut v = vec![0u8; 1024 * 1024];
        rng.fill(&mut v);
        v
    };
    assert_eq!(sha256_chunked(&data, data.len()), ref_sha256(&data));
    assert_eq!(sha512_chunked(&data, data.len()), ref_sha512_of(&data));
    assert_eq!(sha384_chunked(&data, data.len()), ref_sha384_of(&data));
    // And chunked into odd-sized pieces.
    assert_eq!(sha256_chunked(&data, 1000), ref_sha256(&data));
    assert_eq!(sha512_chunked(&data, 1000), ref_sha512_of(&data));
    assert_eq!(sha384_chunked(&data, 1000), ref_sha384_of(&data));
}

////////////////////////////////////////////////////////////////////////////////
// Unaligned / canary-guarded inputs
////////////////////////////////////////////////////////////////////////////////
/// Exercise inputs at every misalignment 0..=8 inside a canary-framed buffer,
/// and check the hasher never writes to its input.
#[test]
fn unaligned_inputs_with_canaries() {
    const CANARY: u8 = 0xA5;
    let mut rng = ChaChaRng::seed_from_u64(0xA11_6ED);
    for len in [
        0usize, 1, 3, 55, 56, 57, 63, 64, 65, 111, 112, 113, 127, 128, 129, 200, 256,
    ] {
        let msg = {
            let mut v = vec![0u8; len];
            rng.fill(&mut v);
            v
        };
        let expected256 = ref_sha256(&msg);
        let expected512 = ref_sha512_of(&msg);
        let expected384 = ref_sha384_of(&msg);

        for offset in 0..=8usize {
            let mut buf = vec![CANARY; offset + len + 16];
            buf[offset..offset + len].copy_from_slice(&msg);
            let before = buf.clone();
            let input = &buf[offset..offset + len];

            assert_eq!(sha256_chunked(input, 5), expected256, "len={len} off={offset}");
            assert_eq!(sha512_chunked(input, 5), expected512, "len={len} off={offset}");
            assert_eq!(sha384_chunked(input, 5), expected384, "len={len} off={offset}");

            assert_eq!(buf, before, "input buffer modified: len={len} off={offset}");
            assert!(buf[..offset].iter().all(|&b| b == CANARY));
            assert!(buf[offset + len..].iter().all(|&b| b == CANARY));
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Read guard page: no read past the end of the input for full blocks
////////////////////////////////////////////////////////////////////////////////
#[cfg(target_os = "linux")]
mod guard_page {
    use core::ffi::c_void;

    use super::*;

    const PROT_NONE: i32 = 0;
    const PROT_READ: i32 = 1;
    const PROT_WRITE: i32 = 2;
    const MAP_PRIVATE: i32 = 0x02;
    const MAP_ANONYMOUS: i32 = 0x20;

    unsafe extern "C" {
        fn mmap(addr: *mut c_void, len: usize, prot: i32, flags: i32, fd: i32, off: i64) -> *mut c_void;
        fn mprotect(addr: *mut c_void, len: usize, prot: i32) -> i32;
        fn munmap(addr: *mut c_void, len: usize) -> i32;
        fn sysconf(name: i32) -> i64;
    }

    fn page_size() -> usize {
        // _SC_PAGESIZE == 30 on Linux (glibc and musl).
        let v = unsafe { sysconf(30) };
        assert!(v >= 4096 && (v as usize).is_power_of_two(), "unexpected page size {v}");
        v as usize
    }

    /// Hash inputs that end exactly at an unmapped page boundary. Any read past
    /// the last byte (e.g. a hardware kernel processing one block too many)
    /// faults instead of silently succeeding.
    #[test]
    fn input_ends_at_unmapped_page() {
        let page = page_size();
        let base = unsafe {
            mmap(
                core::ptr::null_mut(),
                2 * page,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            )
        };
        assert_ne!(base as isize, -1, "mmap failed");
        assert_eq!(
            unsafe { mprotect(base.cast::<u8>().add(page).cast::<c_void>(), page, PROT_NONE) },
            0
        );

        let region = base.cast::<u8>();
        let end = unsafe { region.add(page) };

        let lengths = [
            1usize, 3, 7, 63, 64, 65, 127, 128, 129, 192, 255, 256, 257, 320, 512, 1024, 2048, 4096,
        ];
        let mut rng = ChaChaRng::seed_from_u64(0x0600_DC12);
        for &len in &lengths {
            let msg = {
                let mut v = vec![0u8; len];
                rng.fill(&mut v);
                v
            };
            let start = unsafe { end.sub(len) };
            unsafe { core::ptr::copy_nonoverlapping(msg.as_ptr(), start, len) };
            let input = unsafe { core::slice::from_raw_parts(start, len) };

            let d256 = sha256_chunked(input, len.max(1));
            let d512 = sha512_chunked(input, len.max(1));
            let d384 = sha384_chunked(input, len.max(1));
            assert_eq!(d256, ref_sha256(&msg), "sha256 len={len}");
            assert_eq!(d512, ref_sha512_of(&msg), "sha512 len={len}");
            assert_eq!(d384, ref_sha384_of(&msg), "sha384 len={len}");

            // Incremental updates over the same guarded buffer, to make sure the
            // buffered path never pre-reads the next block.
            let mut h = Sha256::new();
            for piece in input.chunks(13) {
                h.update(piece);
            }
            assert_eq!(h.sum().as_ref(), ref_sha256(&msg), "sha256 incremental len={len}");
        }

        assert_eq!(unsafe { munmap(base, 2 * page) }, 0);
    }
}
