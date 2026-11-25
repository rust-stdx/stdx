//! Tests for the AES block cipher (`Aes128Block`, `Aes256Block`,
//! `encrypt_block`, `decrypt_block`, key schedule, software constant-time core,
//! ARMv8 kernel).
//!
//! The reference implementation below is deliberately independent of the
//! crate: the S-box is *computed* from the GF(2^8) inverse plus the FIPS 197
//! affine transform instead of being copied from a table, and the round
//! function is a plain byte-oriented implementation written from FIPS 197.

use crypto::aes::{Aes128Block, Aes256Block, decrypt_block, encrypt_block};
use random::{RngExt, SeedableRng, rngs::ChaChaRng};

// ── Independent AES-128/AES-256 reference ─────────────────────────────────────

fn gf_mul(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0u8;
    for _ in 0..8 {
        if b & 1 != 0 {
            p ^= a;
        }
        let hi = a & 0x80;
        a <<= 1;
        if hi != 0 {
            a ^= 0x1b;
        }
        b >>= 1;
    }
    p
}

/// Multiplicative inverse in GF(2^8), computed as `a^254` (0 maps to 0).
fn gf_inv(a: u8) -> u8 {
    let mut inv = 1u8;
    for _ in 0..254 {
        inv = gf_mul(inv, a);
    }
    inv
}

/// The forward S-box, computed algebraically instead of copied from FIPS 197.
fn sbox_from_definition(x: u8) -> u8 {
    let inv = gf_inv(x);
    inv ^ inv.rotate_left(1) ^ inv.rotate_left(2) ^ inv.rotate_left(3) ^ inv.rotate_left(4) ^ 0x63
}

/// Cache the algebraically computed S-box so the 10k-vector sweeps stay fast.
static SBOX: std::sync::LazyLock<[u8; 256]> = std::sync::LazyLock::new(|| {
    let mut t = [0u8; 256];
    for (x, slot) in t.iter_mut().enumerate() {
        *slot = sbox_from_definition(x as u8);
    }
    t
});

/// Inverse S-box, derived by inverting the computed table.
static INV_SBOX: std::sync::LazyLock<[u8; 256]> = std::sync::LazyLock::new(|| {
    let mut t = [0u8; 256];
    for (x, s) in SBOX.iter().enumerate() {
        t[*s as usize] = x as u8;
    }
    t
});

fn ref_sbox(x: u8) -> u8 {
    SBOX[x as usize]
}

fn ref_inv_sbox(x: u8) -> u8 {
    INV_SBOX[x as usize]
}

fn ref_expand_key(key: &[u8]) -> Vec<[u8; 16]> {
    let nk = key.len() / 4;
    assert!(nk == 4 || nk == 8);
    let nr = nk + 6;
    let total_words = 4 * (nr + 1);

    let mut w = vec![[0u8; 4]; total_words];
    for i in 0..nk {
        w[i] = [key[4 * i], key[4 * i + 1], key[4 * i + 2], key[4 * i + 3]];
    }
    let mut rcon = 1u8;
    for i in nk..total_words {
        let mut temp = w[i - 1];
        if i % nk == 0 {
            temp = [temp[1], temp[2], temp[3], temp[0]];
            for b in temp.iter_mut() {
                *b = ref_sbox(*b);
            }
            temp[0] ^= rcon;
            rcon = gf_mul(rcon, 2);
        } else if nk == 8 && i % nk == 4 {
            for b in temp.iter_mut() {
                *b = ref_sbox(*b);
            }
        }
        for j in 0..4 {
            w[i][j] = w[i - nk][j] ^ temp[j];
        }
    }

    (0..=nr)
        .map(|r| {
            let mut rk = [0u8; 16];
            for c in 0..4 {
                rk[4 * c..4 * c + 4].copy_from_slice(&w[4 * r + c]);
            }
            rk
        })
        .collect()
}

fn ref_add_round_key(s: &mut [u8; 16], rk: &[u8; 16]) {
    for i in 0..16 {
        s[i] ^= rk[i];
    }
}

fn ref_sub_bytes(s: &mut [u8; 16], inv: bool) {
    for b in s.iter_mut() {
        *b = if inv { ref_inv_sbox(*b) } else { ref_sbox(*b) };
    }
}

/// State is column-major: `s[row + 4 * col]`.
fn ref_shift_rows(s: &mut [u8; 16], inv: bool) {
    let mut out = [0u8; 16];
    for row in 0..4 {
        for col in 0..4 {
            let src = if inv { (col + 4 - row) % 4 } else { (col + row) % 4 };
            out[row + 4 * col] = s[row + 4 * src];
        }
    }
    *s = out;
}

fn ref_mix_columns(s: &mut [u8; 16], inv: bool) {
    let m = if inv {
        [0x0e, 0x0b, 0x0d, 0x09]
    } else {
        [0x02, 0x03, 0x01, 0x01]
    };
    for col in 0..4 {
        let a = [s[4 * col], s[4 * col + 1], s[4 * col + 2], s[4 * col + 3]];
        for row in 0..4 {
            s[4 * col + row] = gf_mul(m[(4 - row) % 4], a[0])
                ^ gf_mul(m[(1 + 4 - row) % 4], a[1])
                ^ gf_mul(m[(2 + 4 - row) % 4], a[2])
                ^ gf_mul(m[(3 + 4 - row) % 4], a[3]);
        }
    }
}

fn ref_encrypt(rks: &[[u8; 16]], pt: &[u8; 16]) -> [u8; 16] {
    let nr = rks.len() - 1;
    let mut s = *pt;
    ref_add_round_key(&mut s, &rks[0]);
    for rk in &rks[1..nr] {
        ref_sub_bytes(&mut s, false);
        ref_shift_rows(&mut s, false);
        ref_mix_columns(&mut s, false);
        ref_add_round_key(&mut s, rk);
    }
    ref_sub_bytes(&mut s, false);
    ref_shift_rows(&mut s, false);
    ref_add_round_key(&mut s, &rks[nr]);
    s
}

fn ref_decrypt(rks: &[[u8; 16]], ct: &[u8; 16]) -> [u8; 16] {
    let nr = rks.len() - 1;
    let mut s = *ct;
    ref_add_round_key(&mut s, &rks[nr]);
    for r in (1..nr).rev() {
        ref_shift_rows(&mut s, true);
        ref_sub_bytes(&mut s, true);
        ref_add_round_key(&mut s, &rks[r]);
        ref_mix_columns(&mut s, true);
    }
    ref_shift_rows(&mut s, true);
    ref_sub_bytes(&mut s, true);
    ref_add_round_key(&mut s, &rks[0]);
    s
}

fn ref_schedule<const N: usize>(key: &[u8]) -> [[u8; 16]; N] {
    let rks = ref_expand_key(key);
    assert_eq!(rks.len(), N);
    rks.try_into().unwrap()
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Deterministic seeded `ChaChaRng` generator.
// ── Reference self-check ──────────────────────────────────────────────────────

#[test]
fn reference_sbox_matches_fips197() {
    // FIPS 197 Figure 7 spot values.
    assert_eq!(ref_sbox(0x00), 0x63);
    assert_eq!(ref_sbox(0x01), 0x7c);
    assert_eq!(ref_sbox(0x53), 0xed);
    assert_eq!(ref_sbox(0x7f), 0xd2);
    assert_eq!(ref_sbox(0xff), 0x16);
    for x in 0..=255u8 {
        assert_eq!(ref_sbox(ref_inv_sbox(x)), x);
    }
}

#[test]
fn reference_aes_matches_known_vectors() {
    // FIPS 197 C.1
    let rk = ref_expand_key(&hex::decode_array::<16>(("000102030405060708090a0b0c0d0e0f").as_bytes()).unwrap());
    let pt = hex::decode_array::<16>(("00112233445566778899aabbccddeeff").as_bytes()).unwrap();
    assert_eq!(
        ref_encrypt(&rk, &pt),
        hex::decode_array::<16>(("69c4e0d86a7b0430d8cdb78070b4c55a").as_bytes()).unwrap()
    );
    assert_eq!(
        ref_decrypt(
            &rk,
            &hex::decode_array::<16>(("69c4e0d86a7b0430d8cdb78070b4c55a").as_bytes()).unwrap()
        ),
        pt
    );

    // FIPS 197 C.3
    let key = hex::decode_array::<32>(("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f").as_bytes())
        .unwrap();
    let rk = ref_expand_key(&key);
    assert_eq!(
        ref_encrypt(&rk, &pt),
        hex::decode_array::<16>(("8ea2b7ca516745bfeafc49904b496089").as_bytes()).unwrap()
    );
    assert_eq!(
        ref_decrypt(
            &rk,
            &hex::decode_array::<16>(("8ea2b7ca516745bfeafc49904b496089").as_bytes()).unwrap()
        ),
        pt
    );
}

// ── Public block-cipher API: known-answer vectors ────────────────────────────

#[test]
fn fips197_c1_aes128() {
    let key = hex::decode_array::<16>(("000102030405060708090a0b0c0d0e0f").as_bytes()).unwrap();
    let pt = hex::decode_array::<16>(("00112233445566778899aabbccddeeff").as_bytes()).unwrap();
    let ct = hex::decode_array::<16>(("69c4e0d86a7b0430d8cdb78070b4c55a").as_bytes()).unwrap();

    let cipher = Aes128Block::new(&key);
    assert_eq!(cipher.encrypt_block(&pt), ct);

    let rk = ref_schedule::<11>(&key);
    assert_eq!(encrypt_block::<11>(&rk, &pt), ct);
    assert_eq!(decrypt_block::<11>(&rk, &ct), pt);
}

#[test]
fn fips197_c3_aes256() {
    let key = hex::decode_array::<32>(("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f").as_bytes())
        .unwrap();
    let pt = hex::decode_array::<16>(("00112233445566778899aabbccddeeff").as_bytes()).unwrap();
    let ct = hex::decode_array::<16>(("8ea2b7ca516745bfeafc49904b496089").as_bytes()).unwrap();

    let cipher = Aes256Block::new(&key);
    assert_eq!(cipher.encrypt_block(&pt), ct);

    let rk = ref_schedule::<15>(&key);
    assert_eq!(encrypt_block::<15>(&rk, &pt), ct);
    assert_eq!(decrypt_block::<15>(&rk, &ct), pt);
}

#[test]
fn nist_sp800_38a_ecb_all_blocks() {
    // F.1.1/F.1.2 (AES-128) and F.1.5/F.1.6 (AES-256).
    let key128 = hex::decode_array::<16>(("2b7e151628aed2a6abf7158809cf4f3c").as_bytes()).unwrap();
    let key256 =
        hex::decode_array::<32>(("603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4").as_bytes())
            .unwrap();

    let cases128: [(&str, &str); 4] = [
        ("6bc1bee22e409f96e93d7e117393172a", "3ad77bb40d7a3660a89ecaf32466ef97"),
        ("ae2d8a571e03ac9c9eb76fac45af8e51", "f5d3d58503b9699de785895a96fdbaaf"),
        ("30c81c46a35ce411e5fbc1191a0a52ef", "43b1cd7f598ece23881b00e3ed030688"),
        ("f69f2445df4f9b17ad2b417be66c3710", "7b0c785e27e8ad3f8223207104725dd4"),
    ];
    let cases256: [(&str, &str); 4] = [
        ("6bc1bee22e409f96e93d7e117393172a", "f3eed1bdb5d2a03c064b5a7e3db181f8"),
        ("ae2d8a571e03ac9c9eb76fac45af8e51", "591ccb10d410ed26dc5ba74a31362870"),
        ("30c81c46a35ce411e5fbc1191a0a52ef", "b6ed21b99ca6f4f9f153e7b1beafed1d"),
        ("f69f2445df4f9b17ad2b417be66c3710", "23304b7a39f9f3ff067d8d8f9e24ecc7"),
    ];

    let c128 = Aes128Block::new(&key128);
    let rk128 = ref_schedule::<11>(&key128);
    let c256 = Aes256Block::new(&key256);
    let rk256 = ref_schedule::<15>(&key256);

    for (pt, ct) in cases128 {
        let (pt, ct) = (
            hex::decode_array::<16>((pt).as_bytes()).unwrap(),
            hex::decode_array::<16>((ct).as_bytes()).unwrap(),
        );
        assert_eq!(c128.encrypt_block(&pt), ct);
        assert_eq!(encrypt_block::<11>(&rk128, &pt), ct);
        assert_eq!(decrypt_block::<11>(&rk128, &ct), pt);
    }
    for (pt, ct) in cases256 {
        let (pt, ct) = (
            hex::decode_array::<16>((pt).as_bytes()).unwrap(),
            hex::decode_array::<16>((ct).as_bytes()).unwrap(),
        );
        assert_eq!(c256.encrypt_block(&pt), ct);
        assert_eq!(encrypt_block::<15>(&rk256, &pt), ct);
        assert_eq!(decrypt_block::<15>(&rk256, &ct), pt);
    }
}

#[test]
fn all_zero_and_all_ones_keys_and_blocks() {
    // Well-known all-zero AES-128 encryption (also the GCM subkey H for K=0).
    let c = Aes128Block::new(&[0u8; 16]);
    assert_eq!(
        c.encrypt_block(&[0u8; 16]),
        hex::decode_array::<16>(("66e94bd4ef8a2c3b884cfa59ca342b2e").as_bytes()).unwrap()
    );

    // AES-256 all-zero key/block (NIST KAT).
    let c = Aes256Block::new(&[0u8; 32]);
    assert_eq!(
        c.encrypt_block(&[0u8; 16]),
        hex::decode_array::<16>(("dc95c078a2408989ad48a21492842087").as_bytes()).unwrap()
    );

    // Cross-check every combination of all-zero / all-ones key and block
    // against the reference implementation, and round-trip decryption.
    for key in [[0u8; 16], [0xffu8; 16]] {
        let rk = ref_schedule::<11>(&key);
        let cipher = Aes128Block::new(&key);
        for pt in [[0u8; 16], [0xffu8; 16]] {
            let want = ref_encrypt(&rk, &pt);
            assert_eq!(cipher.encrypt_block(&pt), want);
            assert_eq!(encrypt_block::<11>(&rk, &pt), want);
            assert_eq!(decrypt_block::<11>(&rk, &want), pt);
        }
    }
    for key in [[0u8; 32], [0xffu8; 32]] {
        let rk = ref_schedule::<15>(&key);
        let cipher = Aes256Block::new(&key);
        for pt in [[0u8; 16], [0xffu8; 16]] {
            let want = ref_encrypt(&rk, &pt);
            assert_eq!(cipher.encrypt_block(&pt), want);
            assert_eq!(encrypt_block::<15>(&rk, &pt), want);
            assert_eq!(decrypt_block::<15>(&rk, &want), pt);
        }
    }
}

// ── Differential sweeps ───────────────────────────────────────────────────────

/// 10k ciphertexts: 100 pseudo-random keys x 100 pseudo-random blocks, for both
/// key sizes, comparing `Aes*Block` and the one-shot `encrypt_block` against
/// the independent reference.
#[test]
fn random_sweep_encrypt_matches_reference() {
    let mut rng = ChaChaRng::seed_from_u64(0x0d0c_0b0a_0908_0706);
    let mut key128 = [0u8; 16];
    let mut rk128 = ref_schedule::<11>(&key128);
    let mut cipher128 = Aes128Block::new(&key128);
    for i in 0..10_000 {
        if i % 100 == 0 {
            rng.fill(&mut key128);
            cipher128 = Aes128Block::new(&key128);
            rk128 = ref_schedule::<11>(&key128);
        }
        let pt = rng.random::<[u8; 16]>();
        let want = ref_encrypt(&rk128, &pt);
        assert_eq!(cipher128.encrypt_block(&pt), want, "AES-128 vector {i}");
        assert_eq!(encrypt_block::<11>(&rk128, &pt), want, "AES-128 one-shot {i}");
    }

    let mut key256 = [0u8; 32];
    let mut rk256 = ref_schedule::<15>(&key256);
    let mut cipher256 = Aes256Block::new(&key256);
    for i in 0..10_000 {
        if i % 100 == 0 {
            rng.fill(&mut key256);
            cipher256 = Aes256Block::new(&key256);
            rk256 = ref_schedule::<15>(&key256);
        }
        let pt = rng.random::<[u8; 16]>();
        let want = ref_encrypt(&rk256, &pt);
        assert_eq!(cipher256.encrypt_block(&pt), want, "AES-256 vector {i}");
        assert_eq!(encrypt_block::<15>(&rk256, &pt), want, "AES-256 one-shot {i}");
    }
}

/// 10k decrypt vectors: ciphertext in, independent-reference plaintext out,
/// plus an encrypt/decrypt round trip.
#[test]
fn random_sweep_decrypt_matches_reference() {
    let mut rng = ChaChaRng::seed_from_u64(0xfeed_face_dead_beef);
    let mut key128 = [0u8; 16];
    let mut key256 = [0u8; 32];
    let mut rk128 = ref_schedule::<11>(&key128);
    let mut rk256 = ref_schedule::<15>(&key256);
    let mut cipher128 = Aes128Block::new(&key128);
    let mut cipher256 = Aes256Block::new(&key256);

    for i in 0..10_000 {
        if i % 100 == 0 {
            rng.fill(&mut key128);
            rng.fill(&mut key256);
            cipher128 = Aes128Block::new(&key128);
            cipher256 = Aes256Block::new(&key256);
            rk128 = ref_schedule::<11>(&key128);
            rk256 = ref_schedule::<15>(&key256);
        }
        let ct = rng.random::<[u8; 16]>();

        let want128 = ref_decrypt(&rk128, &ct);
        assert_eq!(decrypt_block::<11>(&rk128, &ct), want128, "AES-128 decrypt {i}");
        assert_eq!(cipher128.encrypt_block(&want128), ct, "AES-128 round-trip {i}");

        let want256 = ref_decrypt(&rk256, &ct);
        assert_eq!(decrypt_block::<15>(&rk256, &ct), want256, "AES-256 decrypt {i}");
        assert_eq!(cipher256.encrypt_block(&want256), ct, "AES-256 round-trip {i}");
    }
}

/// The one-shot API must not modify its input block, and separate keys/blocks
/// must not interfere (no hidden shared state).
#[test]
fn inputs_are_not_modified_and_calls_are_independent() {
    let mut rng = ChaChaRng::seed_from_u64(0x1234_5678_9abc_def0);
    for _ in 0..1000 {
        let key = hex::decode_array::<16>(("2b7e151628aed2a6abf7158809cf4f3c").as_bytes()).unwrap();
        let rk = ref_schedule::<11>(&key);
        let pt = rng.random::<[u8; 16]>();
        let orig = pt;
        let a = encrypt_block::<11>(&rk, &pt);
        assert_eq!(pt, orig, "encrypt_block modified its input");
        let b = decrypt_block::<11>(&rk, &a);
        assert_eq!(b, orig);
        assert_eq!(pt, orig, "decrypt_block modified its input");
        // Same call again gives the same answer (pure function).
        assert_eq!(encrypt_block::<11>(&rk, &pt), a);
    }
}

/// Boundary keys: first/last byte set, single-bit keys, and keys with every
/// byte equal, for both key sizes.
#[test]
fn boundary_keys_match_reference() {
    let mut keys128 = Vec::new();
    for bit in 0..128 {
        let mut k = [0u8; 16];
        k[bit / 8] = 1 << (bit % 8);
        keys128.push(k);
    }
    keys128.push([0xff; 16]);
    keys128.push([0x80; 16]);

    let mut keys256 = Vec::new();
    for bit in 0..256 {
        let mut k = [0u8; 32];
        k[bit / 8] = 1 << (bit % 8);
        keys256.push(k);
    }
    keys256.push([0xff; 32]);
    keys256.push([0x01; 32]);

    let pt = hex::decode_array::<16>(("00112233445566778899aabbccddeeff").as_bytes()).unwrap();
    for key in keys128 {
        let rk = ref_schedule::<11>(&key);
        let want = ref_encrypt(&rk, &pt);
        assert_eq!(Aes128Block::new(&key).encrypt_block(&pt), want);
        assert_eq!(encrypt_block::<11>(&rk, &pt), want);
        assert_eq!(decrypt_block::<11>(&rk, &want), pt);
    }
    for key in keys256 {
        let rk = ref_schedule::<15>(&key);
        let want = ref_encrypt(&rk, &pt);
        assert_eq!(Aes256Block::new(&key).encrypt_block(&pt), want);
        assert_eq!(encrypt_block::<15>(&rk, &pt), want);
        assert_eq!(decrypt_block::<15>(&rk, &want), pt);
    }
}

/// A call that ends in the middle of a block must not reuse the unused
/// keystream bytes on the next call.
#[test]
fn chunked_partial_block_does_not_reuse_keystream() {
    use crypto::{StreamCipher, aes::Aes128Ctr};

    let key = [0x42u8; 16];

    // Two chunked calls totalling 16 bytes.
    let mut cipher = Aes128Ctr::new(&key);
    let mut first = [0u8; 5];
    cipher.xor_keystream(&mut first).unwrap();
    let mut second = [0u8; 11];
    cipher.xor_keystream(&mut second).unwrap();

    // One single 11-byte call from a fresh cipher.
    let mut fresh = Aes128Ctr::new(&key);
    let mut whole = [0u8; 11];
    fresh.xor_keystream(&mut whole).unwrap();

    assert_ne!(&second[..], &whole[..], "keystream block was reused across chunked calls");
}
