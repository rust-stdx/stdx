//! Tests for the ChaCha SIMD kernels.
//!
//! These tests exercise the *public* stream-cipher API, which dispatches to
//! `chacha_neon` / `chacha_wasm_simd128` / `chacha_avx2` / `chacha_avx512`
//! for inputs of at least 128 bytes and to `chacha_generic` below that
//! threshold.  Every output is compared against an independent, safe,
//! from-the-spec reference implementation (`ref_keystream_*`) and against
//! published RFC 8439 test vectors.
//!
//! The direct kernel contract (counter advance and the 63-byte
//! `keystream_leftover` tail) is additionally checked by `#[cfg(test)]` unit
//! tests living in each kernel module, because the kernels are private.
//!
//! Deterministic pseudo-random data is generated with a seeded `StdRng` so
//! the tests never depend on OS randomness.

use crypto::{
    StreamCipher, StreamCipherError,
    chacha::{ChaCha, ChaCha8Djb, ChaCha12Djb, ChaCha20Djb, ChaCha20Ietf},
};
use rand::{RngExt, SeedableRng, rngs::StdRng};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Independent reference implementation (safe Rust, straight from RFC 8439)
////////////////////////////////////////////////////////////////////////////////////////////////////
const CONSTANT: [u32; 4] = [0x61707865, 0x3320646e, 0x79622d32, 0x6b206574];

#[inline(always)]
fn qr(s: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    s[a] = s[a].wrapping_add(s[b]);
    s[d] = (s[d] ^ s[a]).rotate_left(16);
    s[c] = s[c].wrapping_add(s[d]);
    s[b] = (s[b] ^ s[c]).rotate_left(12);
    s[a] = s[a].wrapping_add(s[b]);
    s[d] = (s[d] ^ s[a]).rotate_left(8);
    s[c] = s[c].wrapping_add(s[d]);
    s[b] = (s[b] ^ s[c]).rotate_left(7);
}

fn ref_block<const ROUNDS: usize>(initial: [u32; 16]) -> [u8; 64] {
    let mut s = initial;
    for _ in 0..ROUNDS / 2 {
        qr(&mut s, 0, 4, 8, 12);
        qr(&mut s, 1, 5, 9, 13);
        qr(&mut s, 2, 6, 10, 14);
        qr(&mut s, 3, 7, 11, 15);
        qr(&mut s, 0, 5, 10, 15);
        qr(&mut s, 1, 6, 11, 12);
        qr(&mut s, 2, 7, 8, 13);
        qr(&mut s, 3, 4, 9, 14);
    }
    let mut out = [0u8; 64];
    for i in 0..16 {
        out[i * 4..i * 4 + 4].copy_from_slice(&s[i].wrapping_add(initial[i]).to_le_bytes());
    }
    out
}

fn key_words(key: &[u8; 32]) -> [u32; 8] {
    let mut w = [0u32; 8];
    for (i, word) in w.iter_mut().enumerate() {
        *word = u32::from_le_bytes(key[i * 4..i * 4 + 4].try_into().unwrap());
    }
    w
}

fn ietf_state(key: &[u8; 32], nonce: &[u8; 12], counter: u32) -> [u32; 16] {
    let mut s = [0u32; 16];
    s[..4].copy_from_slice(&CONSTANT);
    s[4..12].copy_from_slice(&key_words(key));
    s[12] = counter;
    s[13] = u32::from_le_bytes(nonce[0..4].try_into().unwrap());
    s[14] = u32::from_le_bytes(nonce[4..8].try_into().unwrap());
    s[15] = u32::from_le_bytes(nonce[8..12].try_into().unwrap());
    s
}

fn djb_state(key: &[u8; 32], nonce: &[u8; 8], counter: u64) -> [u32; 16] {
    let mut s = [0u32; 16];
    s[..4].copy_from_slice(&CONSTANT);
    s[4..12].copy_from_slice(&key_words(key));
    s[12] = counter as u32;
    s[13] = (counter >> 32) as u32;
    s[14] = u32::from_le_bytes(nonce[0..4].try_into().unwrap());
    s[15] = u32::from_le_bytes(nonce[4..8].try_into().unwrap());
    s
}

fn ref_keystream_ietf<const ROUNDS: usize>(key: &[u8; 32], nonce: &[u8; 12], counter: u32, len: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(len.div_ceil(64) * 64);
    for j in 0..len.div_ceil(64) {
        out.extend_from_slice(&ref_block::<ROUNDS>(ietf_state(key, nonce, counter.wrapping_add(j as u32))));
    }
    out.truncate(len);
    out
}

fn ref_keystream_djb<const ROUNDS: usize>(key: &[u8; 32], nonce: &[u8; 8], counter: u64, len: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(len.div_ceil(64) * 64);
    for j in 0..len.div_ceil(64) {
        out.extend_from_slice(&ref_block::<ROUNDS>(djb_state(key, nonce, counter.wrapping_add(j as u64))));
    }
    out.truncate(len);
    out
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Deterministic seeded `StdRng` byte stream.
fn xor_with(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).map(|(x, y)| x ^ y).collect()
}

fn fresh_ietf<const R: usize>(key: &[u8; 32], nonce: &[u8; 12], counter: u32) -> ChaCha<R, true> {
    let mut c = ChaCha::<R, true>::new(key, nonce);
    c.set_counter(counter);
    c
}

fn fresh_djb<const R: usize>(key: &[u8; 32], nonce: &[u8; 8], counter: u64) -> ChaCha<R, false> {
    let mut c = ChaCha::<R, false>::new(key, nonce);
    c.set_counter(counter);
    c
}

/// Encrypt `plaintext` in one call and compare it with the reference keystream.
fn check_ietf<const R: usize>(key: &[u8; 32], nonce: &[u8; 12], counter: u32, plaintext: &[u8], ctx: &str) {
    let mut buf = plaintext.to_vec();
    fresh_ietf::<R>(key, nonce, counter).xor_keystream(&mut buf).unwrap();
    let expected = xor_with(plaintext, &ref_keystream_ietf::<R>(key, nonce, counter, plaintext.len()));
    assert_eq!(buf, expected, "IETF mismatch ({ctx}), len={}", plaintext.len());
}

fn check_djb<const R: usize>(key: &[u8; 32], nonce: &[u8; 8], counter: u64, plaintext: &[u8], ctx: &str) {
    let mut buf = plaintext.to_vec();
    fresh_djb::<R>(key, nonce, counter).xor_keystream(&mut buf).unwrap();
    let expected = xor_with(plaintext, &ref_keystream_djb::<R>(key, nonce, counter, plaintext.len()));
    assert_eq!(buf, expected, "DJB mismatch ({ctx}), len={}", plaintext.len());
}

fn test_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    for (i, b) in key.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(7).wrapping_add(0x5b);
    }
    key
}

fn test_nonce12() -> [u8; 12] {
    [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98]
}

fn test_nonce8() -> [u8; 8] {
    [0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe, 0xba, 0xbe]
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Test vectors (RFC 8439)
////////////////////////////////////////////////////////////////////////////////////////////////////
const RFC_A1_TV1_KEY: [u8; 32] = [0u8; 32];
const RFC_A1_TV1_NONCE: [u8; 12] = [0u8; 12];
const RFC_A1_TV1_KS: &str = "76b8e0ada0f13d90405d6ae55386bd28\
bdd219b8a08ded1aa836efcc8b770dc7\
da41597c5157488d7724e03fb8d84a37\
6a43b8f41518a11cc387b669b2ee6586";

const RFC_A1_TV2_KS: &str = "9f07e7be5551387a98ba977c732d080d\
cb0f29a048e3656912c6533e32ee7aed\
29b721769ce64e43d57133b074d839d5\
31ed1f28510afb45ace10a1f4b794d6f";

const RFC_A1_TV3_KEY: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
];
const RFC_A1_TV3_KS: &str = "3aeb5224ecf849929b9d828db1ced4dd\
832025e8018b8160b82284f3c949aa5a\
8eca00bbb4a73bdad192b5c42f73f2fd\
4e273644c8b36125a64addeb006c13a0";

const RFC_A1_TV4_KEY: [u8; 32] = [
    0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
const RFC_A1_TV4_KS: &str = "72d54dfbf12ec44b362692df94137f32\
8fea8da73990265ec1bbbea1ae9af0ca\
13b25aa26cb4a648cb9b9d1be65b2c09\
24a66c54d545ec1b7374f4872e99f096";

const RFC_A1_TV5_NONCE: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];
const RFC_A1_TV5_KS: &str = "c2c64d378cd536374ae204b9ef933fcd\
1a8b2288b3dfa49672ab765b54ee27c7\
8a970e0e955c14f3a88e741b97c286f7\
5f8fc299e8148362fa198a39531bed6d";

/// RFC 8439 Appendix A.2 test vector #2 (375-byte message, key ends in 0x01,
/// nonce ends in 0x02, initial counter 1).  Because the first four nonce bytes
/// are zero and the counter fits in 32 bits, the same vectors are valid for
/// the DJB layout when the last 8 nonce bytes are used.
const RFC_A2_TV2_PLAINTEXT: &str = "\
416e79207375626d697373696f6e2074\
6f20746865204945544620696e74656e\
6465642062792074686520436f6e7472\
696275746f7220666f72207075626c69\
636174696f6e20617320616c6c206f72\
2070617274206f6620616e2049455446\
20496e7465726e65742d447261667420\
6f722052464320616e6420616e792073\
746174656d656e74206d616465207769\
7468696e2074686520636f6e74657874\
206f6620616e20494554462061637469\
7669747920697320636f6e7369646572\
656420616e20224945544620436f6e74\
7269627574696f6e222e205375636820\
73746174656d656e747320696e636c75\
6465206f72616c2073746174656d656e\
747320696e2049455446207365737369\
6f6e732c2061732077656c6c20617320\
7772697474656e20616e6420656c6563\
74726f6e696320636f6d6d756e696361\
74696f6e73206d61646520617420616e\
792074696d65206f7220706c6163652c\
20776869636820617265206164647265\
7373656420746f";

const RFC_A2_TV2_CIPHERTEXT: &str = "\
a3fbf07df3fa2fde4f376ca23e827370\
41605d9f4f4f57bd8cff2c1d4b7955ec\
2a97948bd3722915c8f3d337f7d37005\
0e9e96d647b7c39f56e031ca5eb6250d\
4042e02785ececfa4b4bb5e8ead0440e\
20b6e8db09d881a7c6132f420e527950\
42bdfa7773d8a9051447b3291ce1411c\
680465552aa6c405b7764d5e87bea85a\
d00f8449ed8f72d0d662ab052691ca66\
424bc86d2df80ea41f43abf937d3259d\
c4b2d0dfb48a6c9139ddd7f76966e928\
e635553ba76c5c879d7b35d49eb2e62b\
0871cdac638939e25e8a1e0ef9d5280f\
a8ca328b351c3c765989cbcf3daa8b6c\
cc3aaf9f3979c92b3720fc88dc95ed84\
a1be059c6499b9fda236e7e818b04b0b\
c39c1e876b193bfe5569753f88128cc0\
8aaa9b63d1a16f80ef2554d7189c411f\
5869ca52c5b83fa36ff216b9c1d30062\
bebcfd2dc5bce0911934fda79a86f6e6\
98ced759c3ff9b6477338f3da4f9cd85\
14ea9982ccafb341b2384dd902f3d1ab\
7ac61dd29c6f21ba5b862f3730e37cfd\
c4fd806c22f221";

/// RFC 8439 section 2.4.2 ("sunscreen"), 114-byte message.
const RFC_242_PLAINTEXT: &str = "\
4c616469657320616e642047656e746c\
656d656e206f662074686520636c6173\
73206f66202739393a20496620492063\
6f756c64206f6666657220796f75206f\
6e6c79206f6e652074697020666f7220\
746865206675747572652c2073756e73\
637265656e20776f756c642062652069\
742e";

const RFC_242_CIPHERTEXT: &str = "\
6e2e359a2568f98041ba0728dd0d6981\
e97e7aec1d4360c20a27afccfd9fae0b\
f91b65c5524733ab8f593dabcd62b357\
1639d624e65152ab8f530c359f0861d8\
07ca0dbf500d6a6156a38e088a22b65e\
52bc514d16ccf806818ce91ab7793736\
5af90bbf74a35be6b40b8eedf2785e42\
874d";

fn hex_vec(s: &str) -> Vec<u8> {
    hex::decode(s).unwrap()
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Sanity-check the independent reference implementation against the RFC 8439
/// Appendix A.1 single-block vectors, so that a bug in the reference cannot
/// mask a bug in the kernels.
#[test]
fn reference_impl_matches_rfc8439_block_vectors() {
    let cases: [(&[u8; 32], &[u8; 12], u32, &str); 5] = [
        (&RFC_A1_TV1_KEY, &RFC_A1_TV1_NONCE, 0, RFC_A1_TV1_KS),
        (&RFC_A1_TV1_KEY, &RFC_A1_TV1_NONCE, 1, RFC_A1_TV2_KS),
        (&RFC_A1_TV3_KEY, &RFC_A1_TV1_NONCE, 1, RFC_A1_TV3_KS),
        (&RFC_A1_TV4_KEY, &RFC_A1_TV1_NONCE, 2, RFC_A1_TV4_KS),
        (&RFC_A1_TV1_KEY, &RFC_A1_TV5_NONCE, 0, RFC_A1_TV5_KS),
    ];
    for (i, (key, nonce, counter, ks)) in cases.iter().enumerate() {
        let expected = hex_vec(ks);
        let block = ref_block::<20>(ietf_state(key, nonce, *counter));
        assert_eq!(&block[..], &expected[..], "reference block vector #{i} mismatch");
    }
}

/// RFC 8439 Appendix A.2 test vector #2, checked at *every* prefix length from
/// 0 through 375 bytes, for both the IETF and the DJB layout.  Lengths >= 128
/// take the SIMD dispatch path.
#[test]
fn rfc8439_a2_vector_every_prefix_both_layouts() {
    let key: [u8; 32] = {
        let mut k = [0u8; 32];
        k[31] = 1;
        k
    };
    let nonce12: [u8; 12] = {
        let mut n = [0u8; 12];
        n[11] = 2;
        n
    };
    let nonce8: [u8; 8] = {
        let mut n = [0u8; 8];
        n[7] = 2;
        n
    };
    let plaintext = hex_vec(RFC_A2_TV2_PLAINTEXT);
    let ciphertext = hex_vec(RFC_A2_TV2_CIPHERTEXT);
    assert_eq!(plaintext.len(), 375);

    for len in 0..=plaintext.len() {
        let mut ietf = plaintext[..len].to_vec();
        fresh_ietf::<20>(&key, &nonce12, 1).xor_keystream(&mut ietf).unwrap();
        assert_eq!(ietf, ciphertext[..len], "IETF RFC A.2 #2 prefix mismatch at len={len}");

        let mut djb = plaintext[..len].to_vec();
        fresh_djb::<20>(&key, &nonce8, 1).xor_keystream(&mut djb).unwrap();
        assert_eq!(djb, ciphertext[..len], "DJB RFC A.2 #2 prefix mismatch at len={len}");

        // The reference must agree too (guards the reference itself).
        assert_eq!(
            xor_with(&plaintext[..len], &ref_keystream_ietf::<20>(&key, &nonce12, 1, len)),
            ciphertext[..len],
            "reference IETF RFC A.2 #2 prefix mismatch at len={len}"
        );
    }
}

/// RFC 8439 section 2.4.2 ("sunscreen", 114 bytes) and Appendix A.2 vector #3
/// (127 bytes): both are just below the 128-byte SIMD threshold, so they pin
/// down the scalar path at the boundary.
#[test]
fn rfc8439_sub_threshold_vectors() {
    // Section 2.4.2.
    let key: [u8; 32] = (0u8..32).collect::<Vec<_>>().try_into().unwrap();
    let nonce12: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0x4a, 0, 0, 0, 0];
    let mut buf = hex_vec(RFC_242_PLAINTEXT);
    assert_eq!(buf.len(), 114);
    fresh_ietf::<20>(&key, &nonce12, 1).xor_keystream(&mut buf).unwrap();
    assert_eq!(buf, hex_vec(RFC_242_CIPHERTEXT));

    // Appendix A.2 #3: 127 bytes, counter 42.
    let key = hex_vec("1c9240a5eb55d38af333888604f6b5f0473917c1402b80099dca5cbc207075c0")
        .try_into()
        .unwrap();
    let nonce12: [u8; 12] = {
        let mut n = [0u8; 12];
        n[11] = 2;
        n
    };
    let mut buf = hex_vec(
        "2754776173206272696c6c69672c2061\
         6e642074686520736c6974687920746f\
         7665730a446964206779726520616e64\
         2067696d626c6520696e207468652077\
         6162653a0a416c6c206d696d73792077\
         6572652074686520626f726f676f7665\
         732c0a416e6420746865206d6f6d6520\
         7261746873206f757467726162652e",
    );
    let expected = hex_vec(
        "62e6347f95ed87a45ffae7426f27a1df\
         5fb69110044c0d73118effa95b01e5cf\
         166d3df2d721caf9b21e5fb14c616871\
         fd84c54f9d65b283196c7fe4f60553eb\
         f39c6402c42234e32a356b3e764312a6\
         1a5532055716ead6962568f87d3f3f77\
         04c6a8d1bcd1bf4d50d6154b6da731b1\
         87b58dfd728afa36757a797ac188d1",
    );
    assert_eq!(buf.len(), 127);
    fresh_ietf::<20>(&key, &nonce12, 42).xor_keystream(&mut buf).unwrap();
    assert_eq!(buf, expected);
}

/// Exhaustive single-call length sweep.  Every length from 0 to 1400 is
/// checked, which covers 127/128/129, 191/192/193, 255/256/257, all multiples
/// of 64, and the 4-block (256-byte) / 8-block (512-byte) / 16-block (1024-byte)
/// batch boundaries of every kernel.
#[test]
fn single_call_length_sweep_both_layouts() {
    let key = test_key();
    let nonce12 = test_nonce12();
    let nonce8 = test_nonce8();

    for len in 0..=1400usize {
        let mut plaintext = vec![0u8; len];
        StdRng::seed_from_u64(0x1234_5678_9abc_def0 ^ len as u64).fill(&mut plaintext);

        check_ietf::<20>(&key, &nonce12, 0, &plaintext, "sweep counter=0");
        check_ietf::<20>(&key, &nonce12, 1, &plaintext, "sweep counter=1");
        check_ietf::<20>(&key, &nonce12, 0xffff_ff00, &plaintext, "sweep counter=0xffffff00");
        check_djb::<20>(&key, &nonce8, 0, &plaintext, "sweep counter=0");
        check_djb::<20>(&key, &nonce8, 1, &plaintext, "sweep counter=1");
        check_djb::<20>(&key, &nonce8, (1u64 << 32) - 2, &plaintext, "sweep counter=2^32-2");
    }
}

/// Explicit boundary lengths with a bigger payload description.
#[test]
fn boundary_lengths_127_128_129_and_multiples() {
    let key = test_key();
    let nonce12 = test_nonce12();
    let nonce8 = test_nonce8();
    let lengths = [
        63usize, 64, 65, 126, 127, 128, 129, 130, 191, 192, 193, 255, 256, 257, 258, 319, 320, 321, 383, 384, 385, 447,
        448, 449, 510, 511, 512, 513, 514, 575, 576, 577, 1023, 1024, 1025, 4095, 4096, 4097,
    ];
    for len in lengths {
        let mut pt = vec![0u8; len];
        StdRng::seed_from_u64(0xdead_beef_0000_0000 ^ len as u64).fill(&mut pt);
        check_ietf::<20>(&key, &nonce12, 7, &pt, "boundary");
        check_djb::<20>(&key, &nonce8, 0x1_0000_0007, &pt, "boundary");
    }
}

/// The required split patterns: a first call between 1 and 127 bytes followed
/// by a large (>= 128 byte) call.  The first call leaves a partial keystream
/// block behind; the second must consume it before touching the SIMD kernel.
#[test]
fn small_then_large_split_crossing_simd_threshold() {
    let key = test_key();
    let nonce12 = test_nonce12();

    for first in 1..=127usize {
        let mut pt = vec![0u8; first + 512];
        StdRng::seed_from_u64(0x0bad_f00d_0000_0000 ^ first as u64).fill(&mut pt);

        let mut buf = pt.clone();
        let mut c = fresh_ietf::<20>(&key, &nonce12, 3);
        c.xor_keystream(&mut buf[..first]).unwrap();
        c.xor_keystream(&mut buf[first..]).unwrap();

        let expected = xor_with(&pt, &ref_keystream_ietf::<20>(&key, &nonce12, 3, pt.len()));
        assert_eq!(buf, expected, "small({first}) then large(512) split mismatch");
    }
}

/// The reverse pattern: a large call that leaves a 63-byte-byte tail, followed
/// by small calls.  A following call of >= 63 bytes forces the whole stored
/// tail to be consumed.
#[test]
fn large_then_small_split_crossing_simd_threshold() {
    let key = test_key();
    let nonce12 = test_nonce12();

    for &first in &[
        128usize, 129, 130, 191, 192, 193, 255, 256, 257, 320, 511, 512, 513, 1023, 1024, 1025,
    ] {
        for &second in &[1usize, 2, 31, 62, 63, 64, 65, 126, 127, 128, 129, 300] {
            let mut pt = vec![0u8; first + second];
            StdRng::seed_from_u64(0xfeed_face_0000_0000 ^ ((first as u64) << 16) ^ second as u64).fill(&mut pt);

            let mut buf = pt.clone();
            let mut c = fresh_ietf::<20>(&key, &nonce12, 11);
            c.xor_keystream(&mut buf[..first]).unwrap();
            c.xor_keystream(&mut buf[first..]).unwrap();

            let expected = xor_with(&pt, &ref_keystream_ietf::<20>(&key, &nonce12, 11, pt.len()));
            assert_eq!(buf, expected, "large({first}) then small({second}) split mismatch");
        }
    }
}

/// A small first call followed by a second call that exactly consumes the
/// stored tail (no new block) must reproduce the reference keystream; this
/// inspects the tail buffer at full length.
#[test]
fn leftover_tail_round_trip_exact() {
    let key = test_key();
    let nonce12 = test_nonce12();

    for first in 1..=63usize {
        let tail_len = 64 - first;
        let mut pt = vec![0u8; first + tail_len];
        StdRng::seed_from_u64(0x1111_2222_3333_0000 ^ first as u64).fill(&mut pt);

        let mut buf = pt.clone();
        let mut c = fresh_ietf::<20>(&key, &nonce12, 5);
        c.xor_keystream(&mut buf[..first]).unwrap();
        // Exactly finishes the block: no new keystream block may be generated.
        c.xor_keystream(&mut buf[first..]).unwrap();

        let expected = xor_with(&pt, &ref_keystream_ietf::<20>(&key, &nonce12, 5, pt.len()));
        assert_eq!(buf, expected, "tail round-trip mismatch for first={first}");
        assert_eq!(pt.len() % 64, 0);
    }
}

/// Multi-way deterministic splits (2 to 6 pieces) with lengths around the
/// SIMD thresholds, for both layouts and both round counts.
#[test]
fn random_multisplit_against_reference() {
    let key = test_key();
    let nonce12 = test_nonce12();
    let nonce8 = test_nonce8();

    for round in 0..400u64 {
        let mut rng = StdRng::seed_from_u64(0x5eed_0000_0000_0000 ^ round);
        let total = rng.random_range(0..2600) as usize;
        let pieces = 2 + rng.random_range(0..5) as usize;

        let mut splits = Vec::with_capacity(pieces + 1);
        splits.push(0usize);
        while *splits.last().unwrap() < total {
            let remaining = total - splits.last().unwrap();
            let step = (rng.random_range(0..remaining as u64 + 1) as usize)
                .max(1)
                .min(remaining);
            splits.push(*splits.last().unwrap() + step);
        }

        let mut pt = vec![0u8; total];
        rng.fill(&mut pt);

        let mut ietf = pt.clone();
        let mut c = fresh_ietf::<20>(&key, &nonce12, 2);
        for pair in splits.windows(2) {
            c.xor_keystream(&mut ietf[pair[0]..pair[1]]).unwrap();
        }
        assert_eq!(
            ietf,
            xor_with(&pt, &ref_keystream_ietf::<20>(&key, &nonce12, 2, total)),
            "IETF multisplit mismatch (round={round}, total={total}, splits={splits:?})"
        );

        let mut djb = pt.clone();
        let mut c = fresh_djb::<20>(&key, &nonce8, 0xffff_ffff_ffff_0000);
        for pair in splits.windows(2) {
            c.xor_keystream(&mut djb[pair[0]..pair[1]]).unwrap();
        }
        assert_eq!(
            djb,
            xor_with(&pt, &ref_keystream_djb::<20>(&key, &nonce8, 0xffff_ffff_ffff_0000, total)),
            "DJB multisplit mismatch (round={round}, total={total}, splits={splits:?})"
        );
    }
}

/// Guard canaries before/after the encrypted slice, non-zero input offsets,
/// and a slice that ends exactly at the end of its allocation.
#[test]
fn in_place_offsets_and_guard_canaries() {
    let key = test_key();
    let nonce12 = test_nonce12();
    const CANARY: u8 = 0xa7;
    const PAD: usize = 37;

    for &len in &[
        0usize, 1, 63, 64, 65, 127, 128, 129, 192, 255, 256, 257, 320, 511, 512, 513, 777, 1024, 1025,
    ] {
        for &off in &[0usize, 1, 2, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 65] {
            let mut pt = vec![0u8; len];
            StdRng::seed_from_u64(0xcafe_0000_0000_0000 ^ ((len as u64) << 8) ^ off as u64).fill(&mut pt);
            let mut buf = vec![CANARY; off];
            buf.extend_from_slice(&pt);
            buf.extend_from_slice(&[CANARY; PAD]);

            fresh_ietf::<20>(&key, &nonce12, 4)
                .xor_keystream(&mut buf[off..off + len])
                .unwrap();

            assert!(
                buf[..off].iter().all(|&b| b == CANARY),
                "prefix canary modified (len={len}, off={off})"
            );
            assert!(
                buf[off + len..].iter().all(|&b| b == CANARY),
                "suffix canary modified (len={len}, off={off})"
            );
            let expected = xor_with(&pt, &ref_keystream_ietf::<20>(&key, &nonce12, 4, len));
            assert_eq!(&buf[off..off + len], &expected[..], "payload mismatch (len={len}, off={off})");

            // Same thing with the slice ending exactly at the allocation end
            // (`into_boxed_slice` shrinks the allocation to exactly `off+len`).
            let mut exact = vec![CANARY; off];
            exact.extend_from_slice(&pt);
            let mut exact = exact.into_boxed_slice();
            fresh_ietf::<20>(&key, &nonce12, 4)
                .xor_keystream(&mut exact[off..])
                .unwrap();
            assert!(
                exact[..off].iter().all(|&b| b == CANARY),
                "exact-allocation prefix canary modified (len={len}, off={off})"
            );
            assert_eq!(
                &exact[off..],
                &expected[..],
                "exact-allocation payload mismatch (len={len}, off={off})"
            );
        }
    }
}

/// Starting counters around the IETF 32-bit counter space.  Includes the
/// highest values that may still be encrypted (the final counter value is
/// reserved) and checks that the next call panics instead of reusing
/// keystream.
#[test]
fn counter_boundaries_ietf() {
    let key = test_key();
    let nonce12 = test_nonce12();

    let allowed: &[(u32, usize)] = &[
        (0, 4096),
        (1, 1025),
        (u32::MAX - 4, 129),
        (u32::MAX - 3, 128),
        (u32::MAX - 2, 64),
        (u32::MAX - 1, 1),
    ];
    for &(counter, len) in allowed {
        let mut pt = vec![0u8; len];
        StdRng::seed_from_u64(0xaaaa_0000_0000_0000 ^ counter as u64 ^ len as u64).fill(&mut pt);
        check_ietf::<20>(&key, &nonce12, counter, &pt, "ietf counter boundary");
    }

    // Reserved final block / wrap: must panic, never silently reuse the
    // keystream block 0xffff_ffff.
    #[cfg(not(target_arch = "wasm32"))]
    {
        let rejected: &[(u32, usize)] = &[
            (u32::MAX, 1),
            (u32::MAX - 1, 65),
            (u32::MAX - 2, 129),
            (u32::MAX - 4, 257),
        ];
        for &(counter, len) in rejected {
            let mut buf = vec![0u8; len];
            let mut cipher = fresh_ietf::<20>(&key, &nonce12, counter);
            assert_eq!(
                cipher.xor_keystream(&mut buf),
                Err(StreamCipherError::CounterExhausted),
                "expected counter exhaustion for counter={counter}, len={len}"
            );
            assert!(buf.iter().all(|&b| b == 0), "failed call wrote to the buffer");
        }
    }
}

/// Starting counters around the DJB 64-bit counter space, including values
/// that cross the 32-bit boundary between words 12 and 13 while a single SIMD
/// batch is in flight.
#[test]
fn counter_boundaries_djb() {
    let key = test_key();
    let nonce8 = test_nonce8();

    let allowed: &[(u64, usize)] = &[
        (0, 4096),
        (1, 1025),
        ((1u64 << 32) - 2, 512),
        ((1u64 << 32) - 1, 1024),
        (1u64 << 32, 257),
        ((2u64 << 32) - 3, 777),
        (u64::MAX - 3, 128),
        (u64::MAX - 4, 192),
        (u64::MAX - 1, 1),
    ];
    for &(counter, len) in allowed {
        let mut pt = vec![0u8; len];
        StdRng::seed_from_u64(0xbbbb_0000_0000_0000 ^ counter ^ len as u64).fill(&mut pt);
        check_djb::<20>(&key, &nonce8, counter, &pt, "djb counter boundary");
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let rejected: &[(u64, usize)] = &[
            (u64::MAX, 1),
            (u64::MAX - 1, 128),
            (u64::MAX - 2, 129),
            (u64::MAX - 4, 257),
        ];
        for &(counter, len) in rejected {
            let mut buf = vec![0u8; len];
            let mut cipher = fresh_djb::<20>(&key, &nonce8, counter);
            assert_eq!(
                cipher.xor_keystream(&mut buf),
                Err(StreamCipherError::CounterExhausted),
                "expected counter exhaustion for counter={counter}, len={len}"
            );
            assert!(buf.iter().all(|&b| b == 0), "failed call wrote to the buffer");
        }
    }
}

/// The same kernels are instantiated for 8, 12 and 20 rounds.
#[test]
fn round_variants_dispatch() {
    let key = test_key();
    let nonce12 = test_nonce12();
    let nonce8 = test_nonce8();

    for &len in &[
        0usize, 1, 63, 64, 65, 127, 128, 129, 191, 192, 193, 255, 256, 257, 512, 777, 1024, 1025,
    ] {
        let mut pt = vec![0u8; len];
        StdRng::seed_from_u64(0x9999_0000_0000_0000 ^ len as u64).fill(&mut pt);

        check_ietf::<8>(&key, &nonce12, 9, &pt, "rounds=8");
        check_ietf::<12>(&key, &nonce12, 9, &pt, "rounds=12");
        check_ietf::<20>(&key, &nonce12, 9, &pt, "rounds=20");
        check_djb::<8>(&key, &nonce8, 9, &pt, "rounds=8");
        check_djb::<12>(&key, &nonce8, 9, &pt, "rounds=12");
        check_djb::<20>(&key, &nonce8, 9, &pt, "rounds=20");
    }

    // Also exercise the exported aliases to make sure dispatch is wired for
    // them (they are type aliases of the same generic).
    let mut a = vec![0u8; 300];
    let mut b = a.clone();
    ChaCha8Djb::new(&key, &nonce8).xor_keystream(&mut a).unwrap();
    ChaCha12Djb::new(&key, &nonce8).xor_keystream(&mut b).unwrap();
    assert_ne!(a, b);
    let mut c = vec![0u8; 300];
    ChaCha20Djb::new(&key, &nonce8).xor_keystream(&mut c).unwrap();
    let mut d = vec![0u8; 300];
    ChaCha20Ietf::new(&key, &nonce12).xor_keystream(&mut d).unwrap();
    let _ = c;
    let _ = d;
}

/// Repeated calls with a size that always leaves a leftover, verifying that the
/// stored tail is refreshed for every call.
#[test]
fn repeated_partial_calls_do_not_reuse_stale_tail() {
    let key = test_key();
    let nonce12 = test_nonce12();
    let sizes = [1usize, 65, 129, 130, 191, 193, 255, 257, 500, 1023, 1025];

    for &size in &sizes {
        let mut pt = vec![0u8; size * 6];
        StdRng::seed_from_u64(0x7777_0000_0000_0000 ^ size as u64).fill(&mut pt);

        let mut buf = pt.clone();
        let mut c = fresh_ietf::<20>(&key, &nonce12, 13);
        for chunk in buf.chunks_mut(size) {
            c.xor_keystream(chunk).unwrap();
        }
        assert_eq!(
            buf,
            xor_with(&pt, &ref_keystream_ietf::<20>(&key, &nonce12, 13, pt.len())),
            "repeated size={size} mismatch"
        );
    }
}
