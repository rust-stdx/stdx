/// Pure-Rust AES block cipher (128-bit and 256-bit keys).
///
/// The software implementation is **constant-time**: it uses no lookup tables
/// and no secret-dependent branches, so it does not leak key or data bytes
/// through cache or timing side channels (see [`super::aes_ct`]). Hardware
/// paths (AES-NI / ARMv8) are used when available and are used in preference
/// to this fallback.
#[cfg(test)]
use super::ghash::gf128_mul;

// ── AES constants ─────────────────────────────────────────────────────────────

pub const GCM_MAX_LEN: u64 = (u32::MAX as u64 - 1) * 16;

#[cfg(test)]
#[rustfmt::skip]
pub(crate) const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

#[cfg(test)]
#[rustfmt::skip]
pub(crate) const SBOX_INV: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

/// Round constants for AES key expansion (RCON[1..10]).
pub(crate) const RCON: [u8; 11] = [0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];

// ── AES key schedule ───────────────────────────────────────────────────────────

/// AES expanded key: `N` round keys x 16 bytes.
/// N = 11 for AES-128 and 15 for AES-256
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub(crate) enum RoundKeys<const N: usize> {
    #[cfg(target_arch = "x86_64")]
    /// The platform supports x86 AES-NI instructions
    X86_64([core::arch::x86_64::__m128i; N]),

    #[cfg(target_arch = "aarch64")]
    /// The platform supports ARMv8 AES instructions
    Armv8([core::arch::aarch64::uint8x16_t; N]),

    /// The platform doesn't support AES hardware acceleration
    Software([[u8; 16]; N]),
}

pub(crate) type RoundKeysSoftware<const N: usize> = [[u8; 16]; N];

#[inline(always)]
fn rot_word(w: [u8; 4]) -> [u8; 4] {
    [w[1], w[2], w[3], w[0]]
}

/// AES `SubWord`: apply the S-box to each byte of a word in constant time.
#[inline(always)]
fn sub_word(w: [u8; 4]) -> [u8; 4] {
    super::aes_ct::sub_word(u32::from_le_bytes(w)).to_le_bytes()
}

/// Expand a key into AES round keys (FIPS 197 §5.2).
///
/// - `N = 11` -> AES-128 (Nk=4, Nr=10, 44 words -> 11 round keys)
/// - `N = 15` -> AES-256 (Nk=8, Nr=14, 60 words -> 15 round keys)
///
/// The schedule is computed with a constant-time S-box (no table lookups).
pub fn expand_key<const N: usize>(key: &[u8]) -> RoundKeysSoftware<N> {
    const {
        assert!(N == 11 || N == 15);
    }
    let nk = key.len() / 4;
    let words = N * 4;

    // Max 60 words (AES-256). Use a fixed-size buffer for both key sizes.
    let mut w = [[0u8; 4]; 60];
    for i in 0..nk {
        w[i] = [key[4 * i], key[4 * i + 1], key[4 * i + 2], key[4 * i + 3]];
    }

    for i in nk..words {
        let mut temp = w[i - 1];
        if i % nk == 0 {
            temp = sub_word(rot_word(temp));
            temp[0] ^= RCON[i / nk];
        } else if N == 15 && i % nk == 4 {
            // AES-256 only: SubWord every 4 words within the Nk=8 group
            temp = sub_word(temp);
        }
        w[i] = [
            w[i - nk][0] ^ temp[0],
            w[i - nk][1] ^ temp[1],
            w[i - nk][2] ^ temp[2],
            w[i - nk][3] ^ temp[3],
        ];
    }

    let mut rk = [[0u8; 16]; N];
    for i in 0..N {
        for j in 0..4 {
            rk[i][4 * j..4 * j + 4].copy_from_slice(&w[4 * i + j]);
        }
    }
    rk
}

// ── AES block cipher (encrypt / decrypt) ─────────────────────────────────────

/// Encrypt one 16-byte block.
///
/// `N = 11` for AES-128 (10 rounds), `N = 15` for AES-256 (14 rounds).
///
/// This is the constant-time software path; it performs no data-dependent
/// memory accesses or branches.
pub fn encrypt_block<const N: usize>(round_keys: &RoundKeysSoftware<N>, block: &[u8; 16]) -> [u8; 16] {
    const {
        assert!(N == 11 || N == 15);
    }
    let sched = super::aes_ct::keysched(round_keys);
    super::aes_ct::encrypt_block_sched(&sched, block)
}

/// Decrypt one 16-byte block.
///
/// `N = 11` for AES-128 (10 rounds), `N = 15` for AES-256 (14 rounds).
///
/// This is the constant-time software path; it performs no data-dependent
/// memory accesses or branches.
pub fn decrypt_block<const N: usize>(round_keys: &RoundKeysSoftware<N>, block: &[u8; 16]) -> [u8; 16] {
    const {
        assert!(N == 11 || N == 15);
    }
    let sched = super::aes_ct::keysched(round_keys);
    super::aes_ct::decrypt_block_sched(&sched, block)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── AES-256 block cipher (FIPS 197 Appendix B + C) ────────────────────────

    /// NIST FIPS 197 Appendix B – AES-128 vectors (re-confirmed in AES-256 test)
    /// These come from FIPS 197 Appendix C.3 (AES-256).
    #[test]
    fn fips197_aes256_encrypt() {
        // FIPS 197 Appendix C.3
        let key: [u8; 32] =
            hex::decode_array::<32>(b"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f").unwrap();
        let pt: [u8; 16] = hex::decode_array::<16>(b"00112233445566778899aabbccddeeff").unwrap();
        let ct_expected: [u8; 16] = hex::decode_array::<16>(b"8ea2b7ca516745bfeafc49904b496089").unwrap();

        let rk = expand_key::<15>(&key);
        let ct = encrypt_block(&rk, &pt);
        assert_eq!(ct, ct_expected);
    }

    #[test]
    fn fips197_aes256_decrypt() {
        let key: [u8; 32] =
            hex::decode_array::<32>(b"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f").unwrap();
        let ct: [u8; 16] = hex::decode_array::<16>(b"8ea2b7ca516745bfeafc49904b496089").unwrap();
        let pt_expected: [u8; 16] = hex::decode_array::<16>(b"00112233445566778899aabbccddeeff").unwrap();

        let rk = expand_key::<15>(&key);
        let pt = decrypt_block(&rk, &ct);
        assert_eq!(pt, pt_expected);
    }

    /// NIST SP 800-38A ECB-AES256 vectors (F.1.5 / F.1.6).
    #[test]
    fn nist_sp800_38a_aes256_ecb() {
        let key: [u8; 32] =
            hex::decode_array::<32>(b"603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4").unwrap();

        let blocks: &[([u8; 16], [u8; 16])] = &[
            (
                hex::decode_array::<16>(b"6bc1bee22e409f96e93d7e117393172a").unwrap(),
                hex::decode_array::<16>(b"f3eed1bdb5d2a03c064b5a7e3db181f8").unwrap(),
            ),
            (
                hex::decode_array::<16>(b"ae2d8a571e03ac9c9eb76fac45af8e51").unwrap(),
                hex::decode_array::<16>(b"591ccb10d410ed26dc5ba74a31362870").unwrap(),
            ),
            (
                hex::decode_array::<16>(b"30c81c46a35ce411e5fbc1191a0a52ef").unwrap(),
                hex::decode_array::<16>(b"b6ed21b99ca6f4f9f153e7b1beafed1d").unwrap(),
            ),
            (
                hex::decode_array::<16>(b"f69f2445df4f9b17ad2b417be66c3710").unwrap(),
                hex::decode_array::<16>(b"23304b7a39f9f3ff067d8d8f9e24ecc7").unwrap(),
            ),
        ];

        let rk = expand_key::<15>(&key);
        for (pt, ct) in blocks {
            assert_eq!(encrypt_block(&rk, pt), *ct);
            assert_eq!(decrypt_block(&rk, ct), *pt);
        }
    }

    /// NIST Known Answer Test (KAT) – a few AES-256 single-block KATs.
    #[test]
    fn aes256_kat_vectors() {
        // key, plaintext, ciphertext
        let vectors: &[([u8; 32], [u8; 16], [u8; 16])] = &[
            // All-zero key and plaintext
            (
                [0u8; 32],
                [0u8; 16],
                hex::decode_array::<16>(b"dc95c078a2408989ad48a21492842087").unwrap(),
            ),
            // Key = 0x01..0x20, PT = 0
            (
                hex::decode_array::<32>(b"0101010101010101010101010101010101010101010101010101010101010101").unwrap(),
                [0u8; 16],
                hex::decode_array::<16>(b"7298caa565031eadc6ce23d23ea66378").unwrap(),
            ),
            // Key = 0xff..0xff
            (
                [0xff; 32],
                [0u8; 16],
                hex::decode_array::<16>(b"4bf85f1b5d54adbc307b0a048389adcb").unwrap(),
            ),
        ];

        for (key, pt, ct_expected) in vectors {
            let rk = expand_key::<15>(key);
            let ct = encrypt_block(&rk, pt);
            assert_eq!(ct, *ct_expected, "key={}", hex::encode(key));
            let pt2 = decrypt_block(&rk, &ct);
            assert_eq!(pt2, *pt, "round-trip failed");
        }
    }

    #[test]
    fn encrypt_decrypt_roundtrip_random() {
        let key: [u8; 32] =
            hex::decode_array::<32>(b"deadbeefcafebabedeadbeefcafebabe0011223344556677deadbeefcafebabe").unwrap();
        let rk = expand_key::<15>(&key);
        for seed in 0u8..=255 {
            let pt = [seed; 16];
            let ct = encrypt_block(&rk, &pt);
            let pt2 = decrypt_block(&rk, &ct);
            assert_eq!(pt2, pt);
        }
    }

    // ── GF(2^128) multiplication ───────────────────────────────────────────────

    #[test]
    fn gf128_mul_zero() {
        let h = [
            0x66, 0xe9, 0x4b, 0xd4, 0xef, 0x8a, 0x2c, 0x3b, 0x88, 0x4c, 0xfa, 0x59, 0xca, 0x34, 0x2b, 0x2e,
        ];
        let zero = [0u8; 16];
        assert_eq!(gf128_mul(&zero, &h), zero);
        assert_eq!(gf128_mul(&h, &zero), zero);
    }

    #[test]
    fn gf128_mul_commutativity() {
        let a: [u8; 16] = hex::decode_array::<16>(b"66e94bd4ef8a2c3b884cfa59ca342b2e").unwrap();
        let b: [u8; 16] = hex::decode_array::<16>(b"feedfacedeadbeeffeedfacedeadbeef").unwrap();
        assert_eq!(gf128_mul(&a, &b), gf128_mul(&b, &a));
    }

    // H and X from GCM Test Case 2 (NIST SP 800-38D Appendix B).
    #[test]
    fn gf128_mul_nist_tv2() {
        // H = AES_K(0) for K = all-zeros 128-bit key -> irrelevant for 256-bit here,
        // but we test the raw GF multiplication with known values from NIST test vectors.
        // From TC2: H = 66e94bd4ef8a2c3b884cfa59ca342b2e
        //           X (first GHASH input) = feedfacedeadbeeffeedfacedeadbeef
        // Expected product from the NIST spec reference implementation.
        let h: [u8; 16] = hex::decode_array::<16>(b"66e94bd4ef8a2c3b884cfa59ca342b2e").unwrap();
        let x: [u8; 16] = hex::decode_array::<16>(b"feedfacedeadbeeffeedfacedeadbeef").unwrap();
        // Computed offline with a reference implementation.
        let expected: [u8; 16] = hex::decode_array::<16>(b"88eddca9968dec8b9c952d6ae0290a82").unwrap();
        assert_eq!(gf128_mul(&x, &h), expected);
    }
}
