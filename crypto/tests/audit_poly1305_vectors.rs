//! Independent Poly1305 / ChaCha20-Poly1305 edge tests.
//!
//! The oracle below is exact multi-limb integer arithmetic (`2^130 - 5`
//! reduction by repeated shift-and-add of `5 * (x >> 130)`), written from
//! RFC 8439 §2.5. It deliberately does not share the production code's
//! 2×64-bit-limb representation.
//!
//! Coverage focus:
//! * every message length 0..=600, including all lengths that are 15/16/17
//!   modulo 16 and exact multiples of 16;
//! * high-bit and fully saturated keys (all-0xff r and s);
//! * adversarial all-0xff / all-0xaa / `2^130-5`-aligned message bytes;
//! * every two-way `update` split for lengths 0..=300 and every three-way
//!   split for lengths 0..=64;
//! * ChaCha20-Poly1305 tag computation over attacker-chosen all-0xff
//!   ciphertexts, the shape that triggered RUSTSEC-2026-0337 in `ic-cipher`.

use crypto::{Aead, chacha::ChaCha20Poly1305, poly1305::Poly1305};
use random::{RngExt, SeedableRng, rngs::ChaChaRng};

// ---------------------------------------------------------------------------
// Independent reference: exact 128-bit-limb integer arithmetic modulo 2^130-5
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

fn limbs_shr130(x: &Limbs) -> Limbs {
    let mut out = [0u64; 8];
    for i in 0..6 {
        let low = if i + 2 < 8 { x[i + 2] >> 2 } else { 0 };
        let high = if i + 3 < 8 { x[i + 3] << 62 } else { 0 };
        out[i] = low | high;
    }
    out
}

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
    let r = u128::from_le_bytes(key[0..16].try_into().unwrap()) & 0x0fff_fffc_0fff_fffc_0fff_fffc_0fff_ffff;
    let s = u128::from_le_bytes(key[16..32].try_into().unwrap());
    let r = limbs_from_u128(r);

    let mut h: Limbs = [0u64; 8];
    for chunk in msg.chunks(16) {
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

fn pattern(len: usize, mul: u8, add: u8) -> Vec<u8> {
    (0..len)
        .map(|i| (i as u8).wrapping_mul(mul).wrapping_add(add))
        .collect()
}

fn key_bytes(rng: &mut ChaChaRng) -> [u8; 32] {
    let mut k = [0u8; 32];
    rng.fill(&mut k);
    k
}

// ---------------------------------------------------------------------------
// Reference self-check and published vectors
// ---------------------------------------------------------------------------

#[test]
fn reference_is_pinned_to_rfc8439_vectors() {
    // RFC 8439 §2.5.2.
    let key: [u8; 32] = hex::decode("85d6be7857556d337f4452fe42d506a80103808afb0db2fd4abff6af4149f51b")
        .unwrap()
        .try_into()
        .unwrap();
    let msg = b"Cryptographic Forum Research Group";
    assert_eq!(hex::encode(poly1305_reference(&key, msg)), "a8061dc1305136c6c22b8baf0c0127a9");
    assert_eq!(hex::encode(Poly1305::mac(&key, msg)), "a8061dc1305136c6c22b8baf0c0127a9");

    // RFC 8439 A.3 #1..#4.
    let vectors: &[(&str, &str, &str)] = &[
        (
            "0000000000000000000000000000000000000000000000000000000000000000",
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "00000000000000000000000000000000",
        ),
        (
            "0000000000000000000000000000000036e5f6b5c5e06070f0efca96227a863e",
            "416e79207375626d697373696f6e20746f20746865204945544620696e74656e6465642062792074686520436f6e7472696275746f7220666f72207075626c69636174696f6e20617320616c6c206f722070617274206f6620616e204945544620496e7465726e65742d4472616674206f722052464320616e6420616e792073746174656d656e74206d6164652077697468696e2074686520636f6e74657874206f6620616e204945544620616374697669747920697320636f6e7369646572656420616e20224945544620436f6e747269627574696f6e222e20537563682073746174656d656e747320696e636c756465206f72616c2073746174656d656e747320696e20494554462073657373696f6e732c2061732077656c6c206173207772697474656e20616e6420656c656374726f6e696320636f6d6d756e69636174696f6e73206d61646520617420616e792074696d65206f7220706c6163652c207768696368206172652061646472657373656420746f",
            "36e5f6b5c5e06070f0efca96227a863e",
        ),
        (
            "36e5f6b5c5e06070f0efca96227a863e00000000000000000000000000000000",
            "416e79207375626d697373696f6e20746f20746865204945544620696e74656e6465642062792074686520436f6e7472696275746f7220666f72207075626c69636174696f6e20617320616c6c206f722070617274206f6620616e204945544620496e7465726e65742d4472616674206f722052464320616e6420616e792073746174656d656e74206d6164652077697468696e2074686520636f6e74657874206f6620616e204945544620616374697669747920697320636f6e7369646572656420616e20224945544620436f6e747269627574696f6e222e20537563682073746174656d656e747320696e636c756465206f72616c2073746174656d656e747320696e20494554462073657373696f6e732c2061732077656c6c206173207772697474656e20616e6420656c656374726f6e696320636f6d6d756e69636174696f6e73206d61646520617420616e792074696d65206f7220706c6163652c207768696368206172652061646472657373656420746f",
            "f3477e7cd95417af89a6b8794c310cf0",
        ),
        (
            "1c9240a5eb55d38af333888604f6b5f0473917c1402b80099dca5cbc207075c0",
            "2754776173206272696c6c69672c20616e642074686520736c6974687920746f7665730a446964206779726520616e642067696d626c6520696e2074686520776162653a0a416c6c206d696d737920776572652074686520626f726f676f7665732c0a416e6420746865206d6f6d65207261746873206f757467726162652e",
            "4541669a7eaaee61e708dc7cbcc5eb62",
        ),
    ];
    for (key_hex, msg_hex, tag_hex) in vectors {
        let key: [u8; 32] = hex::decode(key_hex).unwrap().try_into().unwrap();
        let msg = hex::decode(msg_hex).unwrap();
        assert_eq!(hex::encode(poly1305_reference(&key, &msg)), *tag_hex);
        assert_eq!(hex::encode(Poly1305::mac(&key, &msg)), *tag_hex);
    }
}

// ---------------------------------------------------------------------------
// Length sweep and adversarial keys/messages
// ---------------------------------------------------------------------------

/// Every length 0..=600 with several key shapes, including fully saturated
/// `r` (after clamping) and `s`. This includes all 15/16/17 residues and every
/// multiple of 16 in the range.
#[test]
fn all_lengths_0_to_600_against_reference() {
    let mut rng = ChaChaRng::seed_from_u64(0x50_4f_4c_59_31_33_30_35);
    let mut keys: Vec<[u8; 32]> = vec![[0x00; 32], [0xff; 32], [0xaa; 32], [0x55; 32]];
    // A key whose clamped `r` has every permitted bit set and whose `s` is
    // all ones: maximum raw product and maximum final addition.
    let mut saturated = [0xffu8; 32];
    saturated[3] = 0x0f;
    saturated[7] = 0x0f;
    saturated[11] = 0x0f;
    saturated[15] = 0x0f;
    keys.push(saturated);
    for _ in 0..4 {
        keys.push(key_bytes(&mut rng));
    }

    for key in &keys {
        for len in 0..=600usize {
            let msg = pattern(len, 37, 11);
            assert_eq!(
                Poly1305::mac(key, &msg),
                poly1305_reference(key, &msg),
                "len={len} key={}",
                hex::encode(key)
            );
        }
    }

    // RUSTSEC-2026-0337 shape: all-0xff / all-0xaa messages at and beyond the
    // 64-byte four-block boundary the vulnerable implementation summed.
    for key in &keys {
        for len in [
            0usize, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257, 512, 513, 600,
        ] {
            for fill in [0xffu8, 0xaau8, 0xfeu8, 0x01u8] {
                let msg = vec![fill; len];
                assert_eq!(
                    Poly1305::mac(key, &msg),
                    poly1305_reference(key, &msg),
                    "adversarial fill={fill:#x} len={len} key={}",
                    hex::encode(key)
                );
            }
        }
    }

    // Blocks whose numeric value is exactly 2^130 - 5 or just below/above it;
    // the high bit comes from the implicit 0x01 byte on full blocks.
    let key = [0xffu8; 32];
    let p_bytes = [
        0xfb, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x03,
    ];
    let mut msg = Vec::new();
    for _ in 0..80 {
        msg.extend_from_slice(&p_bytes);
    }
    for len in [17usize, 34, 51, 68, 170, 340, 680, 1360] {
        let m = &msg[..len];
        assert_eq!(Poly1305::mac(&key, m), poly1305_reference(&key, m), "2^130-5 pattern len={len}");
    }
}

// ---------------------------------------------------------------------------
// Incremental updates: every split
// ---------------------------------------------------------------------------

#[test]
fn every_two_way_split_up_to_300_bytes() {
    let key = [0xffu8; 32];
    for len in 0..=300usize {
        let msg = pattern(len, 29, 7);
        let want = Poly1305::mac(&key, &msg);
        for a in 0..=len {
            let mut mac = Poly1305::new(&key);
            mac.update(&msg[..a]);
            mac.update(&msg[a..]);
            assert_eq!(mac.finalize(), want, "len={len} split={a}");
        }
    }
}

#[test]
fn every_three_way_split_up_to_64_bytes() {
    let key = [0x3cu8; 32];
    for len in 0..=64usize {
        let msg = pattern(len, 11, 3);
        let want = Poly1305::mac(&key, &msg);
        for a in 0..=len {
            for b in a..=len {
                let mut mac = Poly1305::new(&key);
                mac.update(&msg[..a]);
                mac.update(&msg[a..b]);
                mac.update(&msg[b..]);
                assert_eq!(mac.finalize(), want, "len={len} a={a} b={b}");
            }
        }
    }
}

#[test]
fn byte_at_a_time_and_empty_updates() {
    let key = [0x77u8; 32];
    for len in 0..=300usize {
        let msg = pattern(len, 53, 9);
        let want = Poly1305::mac(&key, &msg);
        let mut mac = Poly1305::new(&key);
        for byte in &msg {
            mac.update(&[]);
            mac.update(core::slice::from_ref(byte));
        }
        mac.update(&[]);
        assert_eq!(mac.finalize(), want, "len={len}");
    }
}

// ---------------------------------------------------------------------------
// ChaCha20-Poly1305: attacker-chosen ciphertext (RUSTSEC-2026-0337 shape)
// ---------------------------------------------------------------------------

/// Decrypting an attacker-supplied all-0xff ciphertext must compute the tag
/// over those bytes and reject it, never panic (in a build with overflow
/// checks) or accept it. Run with the default test profile: overflow checks
/// are on.
#[test]
fn chacha20_poly1305_handles_saturated_ciphertext_lengths() {
    let mut rng = ChaChaRng::seed_from_u64(0xc0_ff_ee_00_12_34_56_78);
    for len in [
        0usize, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257, 511, 512, 513, 1024,
    ] {
        for _ in 0..8 {
            let key = key_bytes(&mut rng);
            let mut nonce = [0u8; 12];
            rng.fill(&mut nonce);
            let cipher = ChaCha20Poly1305::new(&key);
            for fill in [0xffu8, 0xaau8, 0x00u8] {
                let mut buf = vec![fill; len];
                let tag = [0u8; 16];
                assert!(
                    cipher.decrypt_in_place(&mut buf, &nonce, b"", &tag).is_err(),
                    "all-{fill:#x} ciphertext len={len} was accepted"
                );
            }
        }
    }
}

/// AEAD composition must agree with an independent Poly1305 over the
/// (fresh-key) ChaCha20 ciphertext for boundary AAD and plaintext lengths.
#[test]
fn aead_tag_matches_independent_poly1305() {
    let mut rng = ChaChaRng::seed_from_u64(0xae_ad_5e_ed_00_00_00_01);
    for len in 0..=65usize {
        let plain = pattern(len, 17, 13);
        let aad = pattern(len % 34, 7, 5);
        let key = key_bytes(&mut rng);
        let mut nonce = [0u8; 12];
        rng.fill(&mut nonce);
        let cipher = ChaCha20Poly1305::new(&key);

        let mut buf = plain.clone();
        let tag = cipher.encrypt_in_place(&mut buf, &nonce, &aad).unwrap();

        // Recompute the RFC 8439 tag from the one-time key and the MAC input.
        let (otk, _) = {
            use crypto::{StreamCipher, chacha::ChaCha20Ietf};
            let mut c = ChaCha20Ietf::new(&key, &nonce);
            c.set_counter(0);
            let mut block = [0u8; 64];
            c.xor_keystream(&mut block).unwrap();
            let mut k = [0u8; 32];
            k.copy_from_slice(&block[..32]);
            (k, ())
        };
        let mut mac_input = Vec::new();
        mac_input.extend_from_slice(&aad);
        mac_input.resize(mac_input.len() + (16 - aad.len() % 16) % 16, 0);
        mac_input.extend_from_slice(&buf);
        mac_input.resize(mac_input.len() + (16 - buf.len() % 16) % 16, 0);
        mac_input.extend_from_slice(&(aad.len() as u64).to_le_bytes());
        mac_input.extend_from_slice(&(buf.len() as u64).to_le_bytes());
        let want = poly1305_reference(&otk, &mac_input);
        assert_eq!(tag.as_ref(), &want[..], "aead tag len={len}");

        let mut dec = buf.clone();
        cipher.decrypt_in_place(&mut dec, &nonce, &aad, tag.as_ref()).unwrap();
        assert_eq!(dec, plain);
    }
}
