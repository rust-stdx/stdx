/// Pure-Rust AES block cipher (128-bit and 256-bit keys).
///
/// Design notes:
/// - Favours speed over constant-time execution (table-driven S-box lookups).
/// - Uses T-tables (Te0..Te3) combining SubBytes+ShiftRows+MixColumns into a
///   single 32-bit lookup per byte per round, matching Go's software AES approach.
#[cfg(test)]
use super::ghash::gf128_mul;

// ── AES constants ─────────────────────────────────────────────────────────────

pub const GCM_MAX_LEN: usize = (u32::MAX as usize - 1) * 16;

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

#[rustfmt::skip]
const SBOX_INV: [u8; 256] = [
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

#[inline(always)]
fn sub_word(w: [u8; 4]) -> [u8; 4] {
    [
        SBOX[w[0] as usize],
        SBOX[w[1] as usize],
        SBOX[w[2] as usize],
        SBOX[w[3] as usize],
    ]
}

/// Expand a key into AES round keys (FIPS 197 §5.2).
///
/// - `N = 11` -> AES-128 (Nk=4, Nr=10, 44 words -> 11 round keys)
/// - `N = 15` -> AES-256 (Nk=8, Nr=14, 60 words -> 15 round keys)
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

/// Const-time gf128 xtime
#[inline(always)]
const fn xtime(a: u8) -> u8 {
    let hi = a & 0x80;
    let b = a << 1;
    if hi != 0 { b ^ 0x1b } else { b }
}

// ── T-tables: combine SubBytes + ShiftRows + MixColumns ─────────────────────
//
// Each Teᵢ[x] = S[x] multiplied by a column of the MixColumns matrix
// (rotated left by i positions). A u32 holds row 0..row 3 in LE byte order.

pub(crate) const TE0: [u32; 256] = {
    let mut t = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        let s = SBOX[i] as u32;
        let s2 = xtime(SBOX[i]) as u32;
        let s3 = (s ^ s2) as u32;
        // Column: [2·S, 1·S, 1·S, 3·S] in rows 0..3
        t[i] = (s3 << 24) | (s << 16) | (s << 8) | s2;
        i += 1;
    }
    t
};

pub(crate) const TE1: [u32; 256] = {
    let mut t = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        let s = SBOX[i] as u32;
        let s2 = xtime(SBOX[i]) as u32;
        let s3 = (s ^ s2) as u32;
        // Column: [3·S, 2·S, 1·S, 1·S]  -> LE bytes [3S,2S,1S,1S]
        t[i] = (s << 24) | (s << 16) | (s2 << 8) | s3;
        i += 1;
    }
    t
};

pub(crate) const TE2: [u32; 256] = {
    let mut t = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        let s = SBOX[i] as u32;
        let s2 = xtime(SBOX[i]) as u32;
        let s3 = (s ^ s2) as u32;
        // Column: [1·S, 3·S, 2·S, 1·S]
        t[i] = (s << 24) | (s2 << 16) | (s3 << 8) | s;
        i += 1;
    }
    t
};

pub(crate) const TE3: [u32; 256] = {
    let mut t = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        let s = SBOX[i] as u32;
        let s2 = xtime(SBOX[i]) as u32;
        let s3 = (s ^ s2) as u32;
        // Column: [1·S, 1·S, 3·S, 2·S]
        t[i] = (s2 << 24) | (s3 << 16) | (s << 8) | s;
        i += 1;
    }
    t
};

// Inverse T-tables for decryption (InvSubBytes + InvShiftRows + InvMixColumns).

const TD0: [u32; 256] = {
    let mut t = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        let s = SBOX_INV[i] as u32;
        let s2 = xtime(s as u8) as u32;
        let s4 = xtime(s2 as u8) as u32;
        let s8 = xtime(s4 as u8) as u32;
        let c0 = (s8 ^ s4 ^ s2) as u32; // 0e·S
        let c1 = (s8 ^ s2 ^ s) as u32; // 0b·S
        let c2 = (s8 ^ s4 ^ s) as u32; // 0d·S
        let c3 = (s8 ^ s) as u32; // 09·S
        // Column: [0e·S, 09·S, 0d·S, 0b·S]  -> LE bytes [c0,c3,c2,c1]
        t[i] = (c1 << 24) | (c2 << 16) | (c3 << 8) | c0;
        i += 1;
    }
    t
};

const TD1: [u32; 256] = {
    let mut t = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        let s = SBOX_INV[i] as u32;
        let s2 = xtime(s as u8) as u32;
        let s4 = xtime(s2 as u8) as u32;
        let s8 = xtime(s4 as u8) as u32;
        let c0 = (s8 ^ s4 ^ s2) as u32;
        let c1 = (s8 ^ s2 ^ s) as u32;
        let c2 = (s8 ^ s4 ^ s) as u32;
        let c3 = (s8 ^ s) as u32;
        // Column: [0b·S, 0e·S, 09·S, 0d·S]
        t[i] = (c2 << 24) | (c3 << 16) | (c0 << 8) | c1;
        i += 1;
    }
    t
};

const TD2: [u32; 256] = {
    let mut t = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        let s = SBOX_INV[i] as u32;
        let s2 = xtime(s as u8) as u32;
        let s4 = xtime(s2 as u8) as u32;
        let s8 = xtime(s4 as u8) as u32;
        let c0 = (s8 ^ s4 ^ s2) as u32;
        let c1 = (s8 ^ s2 ^ s) as u32;
        let c2 = (s8 ^ s4 ^ s) as u32;
        let c3 = (s8 ^ s) as u32;
        // Column: [0d·S, 0b·S, 0e·S, 09·S]
        t[i] = (c3 << 24) | (c0 << 16) | (c1 << 8) | c2;
        i += 1;
    }
    t
};

const TD3: [u32; 256] = {
    let mut t = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        let s = SBOX_INV[i] as u32;
        let s2 = xtime(s as u8) as u32;
        let s4 = xtime(s2 as u8) as u32;
        let s8 = xtime(s4 as u8) as u32;
        let c0 = (s8 ^ s4 ^ s2) as u32;
        let c1 = (s8 ^ s2 ^ s) as u32;
        let c2 = (s8 ^ s4 ^ s) as u32;
        let c3 = (s8 ^ s) as u32;
        // Column: [09·S, 0d·S, 0b·S, 0e·S]
        t[i] = (c0 << 24) | (c1 << 16) | (c2 << 8) | c3;
        i += 1;
    }
    t
};

/// Encrypt one 16-byte block using T-table accelerated routine.
///
/// Combines SubBytes, ShiftRows, and MixColumns into four 32-bit table lookups
/// per column, matching the approach used by Go and OpenSSL for software AES.
///
/// `N = 11` for AES-128 (10 rounds), `N = 15` for AES-256 (14 rounds).
pub fn encrypt_block<const N: usize>(round_keys: &RoundKeysSoftware<N>, block: &[u8; 16]) -> [u8; 16] {
    const {
        assert!(N == 11 || N == 15);
    }

    let mut s = *block;

    // Round 0: AddRoundKey
    for i in 0..16 {
        s[i] ^= round_keys[0][i];
    }

    // Rounds 1..N-2: SubBytes + ShiftRows + MixColumns + AddRoundKey via T-tables
    let round_keys_pointer = round_keys.as_ptr();
    for round in 1..N - 1 {
        let round_keys = unsafe { &*round_keys_pointer.add(round) };
        let t0 = TE0[s[0] as usize]
            ^ TE1[s[5] as usize]
            ^ TE2[s[10] as usize]
            ^ TE3[s[15] as usize]
            ^ u32::from_ne_bytes(round_keys[0..4].try_into().unwrap());
        let t1 = TE0[s[4] as usize]
            ^ TE1[s[9] as usize]
            ^ TE2[s[14] as usize]
            ^ TE3[s[3] as usize]
            ^ u32::from_ne_bytes(round_keys[4..8].try_into().unwrap());
        let t2 = TE0[s[8] as usize]
            ^ TE1[s[13] as usize]
            ^ TE2[s[2] as usize]
            ^ TE3[s[7] as usize]
            ^ u32::from_ne_bytes(round_keys[8..12].try_into().unwrap());
        let t3 = TE0[s[12] as usize]
            ^ TE1[s[1] as usize]
            ^ TE2[s[6] as usize]
            ^ TE3[s[11] as usize]
            ^ u32::from_ne_bytes(round_keys[12..16].try_into().unwrap());

        s[0..4].copy_from_slice(&t0.to_ne_bytes());
        s[4..8].copy_from_slice(&t1.to_ne_bytes());
        s[8..12].copy_from_slice(&t2.to_ne_bytes());
        s[12..16].copy_from_slice(&t3.to_ne_bytes());
    }

    // Final round (N-1): SubBytes + ShiftRows + AddRoundKey (no MixColumns)
    let rk_last = unsafe { &*round_keys_pointer.add(N - 1) };
    s = [
        SBOX[s[0] as usize] ^ rk_last[0],
        SBOX[s[5] as usize] ^ rk_last[1],
        SBOX[s[10] as usize] ^ rk_last[2],
        SBOX[s[15] as usize] ^ rk_last[3],
        SBOX[s[4] as usize] ^ rk_last[4],
        SBOX[s[9] as usize] ^ rk_last[5],
        SBOX[s[14] as usize] ^ rk_last[6],
        SBOX[s[3] as usize] ^ rk_last[7],
        SBOX[s[8] as usize] ^ rk_last[8],
        SBOX[s[13] as usize] ^ rk_last[9],
        SBOX[s[2] as usize] ^ rk_last[10],
        SBOX[s[7] as usize] ^ rk_last[11],
        SBOX[s[12] as usize] ^ rk_last[12],
        SBOX[s[1] as usize] ^ rk_last[13],
        SBOX[s[6] as usize] ^ rk_last[14],
        SBOX[s[11] as usize] ^ rk_last[15],
    ];
    s
}

/// Decrypt one 16-byte block using inverse T-tables.
///
/// Note: the inverse T-tables (TD0..TD3) combine InvSubBytes + InvMixColumns.
/// Because AddRoundKey falls between InvSubBytes and InvMixColumns in the
/// decryption round, each round key rk[1]..rk[N-2] must have InvMixColumns
/// applied to it before the XOR.
///
/// `N = 11` for AES-128 (10 rounds), `N = 15` for AES-256 (14 rounds).
pub fn decrypt_block<const N: usize>(round_keys: &RoundKeysSoftware<N>, block: &[u8; 16]) -> [u8; 16] {
    const {
        assert!(N == 11 || N == 15);
    }

    let mut s = *block;

    // Round N-1 reversed: AddRoundKey
    let p_last = unsafe { &*round_keys.as_ptr().add(N - 1) };
    for i in 0..16 {
        s[i] ^= p_last[i];
    }

    // Rounds N-2..1: InvShiftRows + InvSubBytes + AddRoundKey + InvMixColumns via inverse T-tables
    let round_keys_pointer = round_keys.as_ptr();
    for round in (1..N - 1).rev() {
        // Apply InvMixColumns to round key columns for correct T-table XOR
        let mut rk_adj = unsafe { *round_keys_pointer.add(round) };
        inv_mix_columns(&mut rk_adj);

        let t0 = TD0[s[0] as usize]
            ^ TD1[s[13] as usize]
            ^ TD2[s[10] as usize]
            ^ TD3[s[7] as usize]
            ^ u32::from_ne_bytes(rk_adj[0..4].try_into().unwrap());
        let t1 = TD0[s[4] as usize]
            ^ TD1[s[1] as usize]
            ^ TD2[s[14] as usize]
            ^ TD3[s[11] as usize]
            ^ u32::from_ne_bytes(rk_adj[4..8].try_into().unwrap());
        let t2 = TD0[s[8] as usize]
            ^ TD1[s[5] as usize]
            ^ TD2[s[2] as usize]
            ^ TD3[s[15] as usize]
            ^ u32::from_ne_bytes(rk_adj[8..12].try_into().unwrap());
        let t3 = TD0[s[12] as usize]
            ^ TD1[s[9] as usize]
            ^ TD2[s[6] as usize]
            ^ TD3[s[3] as usize]
            ^ u32::from_ne_bytes(rk_adj[12..16].try_into().unwrap());

        s[0..4].copy_from_slice(&t0.to_ne_bytes());
        s[4..8].copy_from_slice(&t1.to_ne_bytes());
        s[8..12].copy_from_slice(&t2.to_ne_bytes());
        s[12..16].copy_from_slice(&t3.to_ne_bytes());
    }

    // Round 0: InvShiftRows + InvSubBytes + AddRoundKey (no InvMixColumns)
    s = [
        SBOX_INV[s[0] as usize] ^ round_keys[0][0],
        SBOX_INV[s[13] as usize] ^ round_keys[0][1],
        SBOX_INV[s[10] as usize] ^ round_keys[0][2],
        SBOX_INV[s[7] as usize] ^ round_keys[0][3],
        SBOX_INV[s[4] as usize] ^ round_keys[0][4],
        SBOX_INV[s[1] as usize] ^ round_keys[0][5],
        SBOX_INV[s[14] as usize] ^ round_keys[0][6],
        SBOX_INV[s[11] as usize] ^ round_keys[0][7],
        SBOX_INV[s[8] as usize] ^ round_keys[0][8],
        SBOX_INV[s[5] as usize] ^ round_keys[0][9],
        SBOX_INV[s[2] as usize] ^ round_keys[0][10],
        SBOX_INV[s[15] as usize] ^ round_keys[0][11],
        SBOX_INV[s[12] as usize] ^ round_keys[0][12],
        SBOX_INV[s[9] as usize] ^ round_keys[0][13],
        SBOX_INV[s[6] as usize] ^ round_keys[0][14],
        SBOX_INV[s[3] as usize] ^ round_keys[0][15],
    ];
    s
}

// ── helpers ────────────────────────────────────────────────────────

/// Multiply in GF(2^8) mod 0x11b.
#[inline(always)]
fn gmul(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0u8;
    for _ in 0..8 {
        if b & 1 != 0 {
            p ^= a;
        }
        let carry = a & 0x80;
        a <<= 1;
        if carry != 0 {
            a ^= 0x1b;
        }
        b >>= 1;
    }
    p
}

#[inline(always)]
fn inv_mix_col(s: &mut [u8; 16], col: usize) {
    let i = col * 4;
    let s0 = s[i];
    let s1 = s[i + 1];
    let s2 = s[i + 2];
    let s3 = s[i + 3];
    s[i] = gmul(0x0e, s0) ^ gmul(0x0b, s1) ^ gmul(0x0d, s2) ^ gmul(0x09, s3);
    s[i + 1] = gmul(0x09, s0) ^ gmul(0x0e, s1) ^ gmul(0x0b, s2) ^ gmul(0x0d, s3);
    s[i + 2] = gmul(0x0d, s0) ^ gmul(0x09, s1) ^ gmul(0x0e, s2) ^ gmul(0x0b, s3);
    s[i + 3] = gmul(0x0b, s0) ^ gmul(0x0d, s1) ^ gmul(0x09, s2) ^ gmul(0x0e, s3);
}

#[inline(always)]
fn inv_mix_columns(state: &mut [u8; 16]) {
    inv_mix_col(state, 0);
    inv_mix_col(state, 1);
    inv_mix_col(state, 2);
    inv_mix_col(state, 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add tests:
    // https://www.tuhs.org/cgi-bin/utree.pl?file=OpenBSD-4.6/regress/sys/crypto/aes/vectors/ecbnk48.txt
    // https://android.googlesource.com/platform/libcore/+/1db6bf619611525020518a180f0ee82c8cd50af2/luni/src/test/resources/crypto/aes-cbc.csv

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
