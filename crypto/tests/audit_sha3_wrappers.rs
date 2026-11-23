//! Tests for the SHA-3 / XOF wrapper layer.
//!
//! The wrappers in `sha3/{mod,xof,sha3_256,sha3_512,shake128,shake256,
//! turboshake,kmac,kangaroo_twelve}.rs` contain no `unsafe`; they drive the
//! Keccak core. These tests therefore check the contracts and
//! edge cases those safe wrappers are responsible for: padding delimiters,
//! rates, SP 800-185 length encodings, the KangarooTwelve tree layout, output
//! length handling, absorb/squeeze state transitions and chunk-independent
//! squeezing.
//!
//! Differential oracle: `reference` below is an independent, literal
//! transcription of FIPS 202 / SP 800-185 / RFC 9861 built on its own scalar
//! Keccak-p implementation. It does not call the crate's sponge.

use std::collections::HashSet;
#[cfg(not(target_arch = "wasm32"))]
use std::panic::{AssertUnwindSafe, catch_unwind};

use crypto::{
    Hasher, Xof,
    sha3::{
        CShake256, HopMac256, KT256_CHUNK_SIZE, Kmac256, Kt256, Sha3_256, Sha3_512, Shake128, Shake256,
        TURBOSHAKE256_DEFAULT_DOMAIN_SEPARATOR, TurboShake256, TurboShakeError,
    },
};
use rand::{Rng, RngExt, SeedableRng, rngs::StdRng};

// ---------------------------------------------------------------------------
// Independent reference implementation
// ---------------------------------------------------------------------------

mod reference {
    const ROUND_CONSTANTS: [u64; 24] = [
        0x0000_0000_0000_0001,
        0x0000_0000_0000_8082,
        0x8000_0000_0000_808a,
        0x8000_0000_8000_8000,
        0x0000_0000_0000_808b,
        0x0000_0000_8000_0001,
        0x8000_0000_8000_8081,
        0x8000_0000_0000_8009,
        0x0000_0000_0000_008a,
        0x0000_0000_0000_0088,
        0x0000_0000_8000_8009,
        0x0000_0000_8000_000a,
        0x0000_0000_8000_808b,
        0x8000_0000_0000_008b,
        0x8000_0000_0000_8089,
        0x8000_0000_0000_8003,
        0x8000_0000_0000_8002,
        0x8000_0000_0000_0080,
        0x0000_0000_0000_800a,
        0x8000_0000_8000_000a,
        0x8000_0000_8000_8081,
        0x8000_0000_0000_8080,
        0x0000_0000_8000_0001,
        0x8000_0000_8000_8008,
    ];

    // Rho offsets and pi destinations in the standard compact traversal.
    const RHO: [u32; 24] = [
        1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 2, 14, 27, 41, 56, 8, 25, 43, 62, 18, 39, 61, 20, 44,
    ];
    const PI: [usize; 24] = [
        10, 7, 11, 17, 18, 3, 5, 16, 8, 21, 24, 4, 15, 23, 19, 13, 12, 2, 20, 14, 22, 9, 6, 1,
    ];

    fn round(state: &mut [u64; 25], rc: u64) {
        // Theta
        let mut c = [0u64; 5];
        for (x, lane) in c.iter_mut().enumerate() {
            *lane = state[x] ^ state[x + 5] ^ state[x + 10] ^ state[x + 15] ^ state[x + 20];
        }
        for x in 0..5 {
            let d = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
            for y in 0..5 {
                state[5 * y + x] ^= d;
            }
        }

        // Rho and Pi
        let mut last = state[1];
        for i in 0..24 {
            let tmp = state[PI[i]];
            state[PI[i]] = last.rotate_left(RHO[i]);
            last = tmp;
        }

        // Chi
        for y in 0..5 {
            let base = 5 * y;
            let row = [
                state[base],
                state[base + 1],
                state[base + 2],
                state[base + 3],
                state[base + 4],
            ];
            for x in 0..5 {
                state[base + x] = row[x] ^ ((!row[(x + 1) % 5]) & row[(x + 2) % 5]);
            }
        }

        // Iota
        state[0] ^= rc;
    }

    fn permutation<const ROUNDS: usize>(state: &mut [u64; 25]) {
        for &rc in &ROUND_CONSTANTS[24 - ROUNDS..] {
            round(state, rc);
        }
    }

    /// Literal byte-oriented sponge: absorb one byte at a time, pad with
    /// `delimiter` and `0x80`, then squeeze.
    pub struct Sponge<const ROUNDS: usize> {
        state: [u64; 25],
        rate: usize,
        delimiter: u8,
        pos: usize,
        squeezing: bool,
    }

    impl<const ROUNDS: usize> Sponge<ROUNDS> {
        pub fn new(rate: usize, delimiter: u8) -> Self {
            assert!(rate > 0 && rate < 200);
            Self {
                state: [0u64; 25],
                rate,
                delimiter,
                pos: 0,
                squeezing: false,
            }
        }

        pub fn absorb(&mut self, data: &[u8]) {
            assert!(!self.squeezing, "absorb after squeeze");
            for &byte in data {
                self.state[self.pos / 8] ^= (byte as u64) << (8 * (self.pos % 8));
                self.pos += 1;
                if self.pos == self.rate {
                    permutation::<ROUNDS>(&mut self.state);
                    self.pos = 0;
                }
            }
        }

        fn finish_absorb(&mut self) {
            if self.squeezing {
                return;
            }
            self.state[self.pos / 8] ^= (self.delimiter as u64) << (8 * (self.pos % 8));
            let last = self.rate - 1;
            self.state[last / 8] ^= 0x80u64 << (8 * (last % 8));
            permutation::<ROUNDS>(&mut self.state);
            self.pos = 0;
            self.squeezing = true;
        }

        pub fn squeeze(&mut self, out: &mut [u8]) {
            self.finish_absorb();
            let mut written = 0;
            while written < out.len() {
                if self.pos == self.rate {
                    permutation::<ROUNDS>(&mut self.state);
                    self.pos = 0;
                }
                let take = core::cmp::min(self.rate - self.pos, out.len() - written);
                for i in 0..take {
                    out[written + i] = (self.state[(self.pos + i) / 8] >> (8 * ((self.pos + i) % 8))) as u8;
                }
                written += take;
                self.pos += take;
            }
        }
    }

    pub fn sponge<const ROUNDS: usize>(data: &[u8], rate: usize, delimiter: u8, out: &mut [u8]) {
        let mut sponge = Sponge::<ROUNDS>::new(rate, delimiter);
        sponge.absorb(data);
        sponge.squeeze(out);
    }

    pub fn sha3_256(data: &[u8], out: &mut [u8]) {
        sponge::<24>(data, 136, 0x06, out);
    }

    pub fn sha3_512(data: &[u8], out: &mut [u8]) {
        sponge::<24>(data, 72, 0x06, out);
    }

    pub fn shake128(data: &[u8], out: &mut [u8]) {
        sponge::<24>(data, 168, 0x1f, out);
    }

    pub fn shake256(data: &[u8], out: &mut [u8]) {
        sponge::<24>(data, 136, 0x1f, out);
    }

    pub fn turboshake256(data: &[u8], domain: u8, out: &mut [u8]) {
        sponge::<12>(data, 136, domain, out);
    }

    pub fn left_encode(x: usize) -> Vec<u8> {
        let bytes = x.to_be_bytes();
        let mut first = 0;
        while first < bytes.len() - 1 && bytes[first] == 0 {
            first += 1;
        }
        let mut out = vec![(bytes.len() - first) as u8];
        out.extend_from_slice(&bytes[first..]);
        out
    }

    pub fn right_encode(x: usize) -> Vec<u8> {
        let left = left_encode(x);
        let n = left[0] as usize;
        let mut out = left[1..].to_vec();
        out.push(n as u8);
        out
    }

    pub fn encode_string(data: &[u8]) -> Vec<u8> {
        let mut out = left_encode(data.len() * 8);
        out.extend_from_slice(data);
        out
    }

    pub fn bytepad(data: &[u8], w: usize) -> Vec<u8> {
        let mut out = left_encode(w);
        out.extend_from_slice(data);
        let pad = (w - (out.len() % w)) % w;
        out.resize(out.len() + pad, 0);
        out
    }

    pub fn cshake256(data: &[u8], function_name: &[u8], customization: &[u8], out: &mut [u8]) {
        if function_name.is_empty() && customization.is_empty() {
            sponge::<24>(data, 136, 0x1f, out);
            return;
        }

        let mut sponge = Sponge::<24>::new(136, 0x04);
        let mut names = encode_string(function_name);
        names.extend_from_slice(&encode_string(customization));
        sponge.absorb(&bytepad(&names, 136));
        sponge.absorb(data);
        sponge.squeeze(out);
    }

    pub fn kmac256(key: &[u8], data: &[u8], customization: &[u8], xof: bool, out: &mut [u8]) {
        let mut sponge = Sponge::<24>::new(136, 0x04);
        let mut names = encode_string(b"KMAC");
        names.extend_from_slice(&encode_string(customization));
        sponge.absorb(&bytepad(&names, 136));
        sponge.absorb(&bytepad(&encode_string(key), 136));
        sponge.absorb(data);
        let bits = if xof { 0 } else { out.len() * 8 };
        sponge.absorb(&right_encode(bits));
        sponge.squeeze(out);
    }

    pub fn length_encode(mut x: usize) -> Vec<u8> {
        if x == 0 {
            return vec![0];
        }
        let mut digits = [0u8; 8];
        let mut n = 0;
        while x > 0 {
            digits[digits.len() - 1 - n] = (x & 0xff) as u8;
            x >>= 8;
            n += 1;
        }
        let mut out = digits[digits.len() - n..].to_vec();
        out.push(n as u8);
        out
    }

    pub const KT_CHUNK: usize = 8192;

    /// RFC 9861 appendix A.5, transcribed literally.
    pub fn kt256(message: &[u8], customization: &[u8], out: &mut [u8]) {
        let mut s = message.to_vec();
        s.extend_from_slice(customization);
        s.extend_from_slice(&length_encode(customization.len()));

        if s.len() <= KT_CHUNK {
            sponge::<12>(&s, 136, 0x07, out);
            return;
        }

        let mut final_node = s[..KT_CHUNK].to_vec();
        final_node.extend_from_slice(&[0x03, 0, 0, 0, 0, 0, 0, 0]);

        let mut chunks = 0usize;
        for chunk in s[KT_CHUNK..].chunks(KT_CHUNK) {
            let mut cv = [0u8; 64];
            sponge::<12>(chunk, 136, 0x0b, &mut cv);
            final_node.extend_from_slice(&cv);
            chunks += 1;
        }

        final_node.extend_from_slice(&length_encode(chunks));
        final_node.extend_from_slice(&[0xff, 0xff]);
        sponge::<12>(&final_node, 136, 0x06, out);
    }

    pub fn hopmac256(key: &[u8], message: &[u8], customization: &[u8], out: &mut [u8]) {
        let mut digest = [0u8; 64];
        kt256(message, customization, &mut digest);
        kt256(key, &digest, out);
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn pattern(len: usize) -> Vec<u8> {
    (0..len).map(|i| (i % 251) as u8).collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn panics<F: FnOnce()>(f: F) -> bool {
    catch_unwind(AssertUnwindSafe(f)).is_err()
}

/// Squeezes `total` bytes in a deliberately awkward sequence of chunk sizes,
/// including empty calls, and returns the stream.
fn squeeze_chunked<F>(mut squeeze: F, total: usize, rate: usize) -> Vec<u8>
where
    F: FnMut(&mut [u8]),
{
    let sizes = [1usize, 0, 7, rate - 1, rate, 1, rate + 1, 3, 2 * rate + 1, 5];
    let mut out = vec![0u8; total];
    let mut offset = 0;
    let mut i = 0;
    while offset < total {
        let n = sizes[i % sizes.len()].min(total - offset);
        squeeze(&mut out[offset..offset + n]);
        offset += n;
        i += 1;
    }
    out
}

/// Runs `f` on a subslice surrounded by canary bytes and checks that neither
/// canary was touched.
fn with_canaries<F: FnOnce(&mut [u8])>(len: usize, f: F) {
    const GUARD: usize = 32;
    let mut buffer = vec![0xa5u8; GUARD + len + GUARD];
    let (before, rest) = buffer.split_at_mut(GUARD);
    let (middle, after) = rest.split_at_mut(len);
    f(middle);
    assert!(before.iter().all(|&b| b == 0xa5), "write before output buffer");
    assert!(after.iter().all(|&b| b == 0xa5), "write after output buffer");
}

// ---------------------------------------------------------------------------
// SHA3-256 / SHA3-512
// ---------------------------------------------------------------------------

#[test]
fn sha3_published_vectors() {
    assert_eq!(
        hex::encode(<Sha3_256 as Hasher>::hash(b"")),
        "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a"
    );
    assert_eq!(
        hex::encode(<Sha3_256 as Hasher>::hash(b"abc")),
        "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"
    );
    assert_eq!(
        hex::encode(<Sha3_512 as Hasher>::hash(b"")),
        "a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a6\
         15b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26"
    );
    assert_eq!(
        hex::encode(<Sha3_512 as Hasher>::hash(b"abc")),
        "b751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e\
         10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0"
    );
}

#[test]
fn sha3_constants_and_output_lengths() {
    assert_eq!(<Sha3_256 as Hasher>::BLOCK_SIZE, 136);
    assert_eq!(<Sha3_256 as Hasher>::OUTPUT_SIZE, 32);
    assert_eq!(<Sha3_512 as Hasher>::BLOCK_SIZE, 72);
    assert_eq!(<Sha3_512 as Hasher>::OUTPUT_SIZE, 64);

    assert_eq!(<Sha3_256 as Hasher>::hash(b"x").len(), 32);
    assert_eq!(<Sha3_512 as Hasher>::hash(b"x").len(), 64);

    // Empty output is not possible for the fixed-size `Hasher` API, but an
    // empty input must still produce the full digest.
    assert_eq!(<Sha3_256 as Hasher>::hash(b"").len(), 32);
    assert_eq!(<Sha3_512 as Hasher>::hash(b"").len(), 64);
}

fn check_sha3_incremental<H: Hasher>(data: &[u8], expected: &[u8]) {
    let chunk_sizes = [1usize, 3, 7, 31, 71, 72, 73, 135, 136, 137, 257];
    let mut hasher = H::new();
    let mut offset = 0;
    let mut i = 0;
    while offset < data.len() {
        let take = chunk_sizes[i % chunk_sizes.len()].min(data.len() - offset);
        hasher.update(&data[offset..offset + take]);
        offset += take;
        i += 1;
    }
    assert_eq!(hasher.sum().as_ref(), expected);
}

#[test]
fn sha3_reference_sweep_and_incremental() {
    for len in 0..=1024 {
        let data = pattern(len);

        let mut expected256 = [0u8; 32];
        reference::sha3_256(&data, &mut expected256);
        assert_eq!(<Sha3_256 as Hasher>::hash(&data).as_ref(), expected256, "sha3-256 len={len}");
        check_sha3_incremental::<Sha3_256>(&data, &expected256);

        let mut expected512 = [0u8; 64];
        reference::sha3_512(&data, &mut expected512);
        assert_eq!(<Sha3_512 as Hasher>::hash(&data).as_ref(), expected512, "sha3-512 len={len}");
        check_sha3_incremental::<Sha3_512>(&data, &expected512);
    }
}

// ---------------------------------------------------------------------------
// SHAKE128 / SHAKE256
// ---------------------------------------------------------------------------

#[test]
fn shake_published_vectors() {
    let mut out = [0u8; 32];
    Shake128::hash(b"", &mut out);
    assert_eq!(
        hex::encode(out),
        "7f9c2ba4e88f827d616045507605853ed73b8093f6efbc88eb1a6eacfa66ef26"
    );

    let mut out = [0u8; 64];
    Shake128::hash(b"", &mut out);
    assert_eq!(
        hex::encode(out),
        "7f9c2ba4e88f827d616045507605853ed73b8093f6efbc88eb1a6eacfa66ef26\
         3cb1eea988004b93103cfb0aeefd2a686e01fa4a58e8a3639ca8a1e3f9ae57e2"
    );

    let mut out = [0u8; 32];
    Shake128::hash(b"abc", &mut out);
    assert_eq!(
        hex::encode(out),
        "5881092dd818bf5cf8a3ddb793fbcba74097d5c526a6d35f97b83351940f2cc8"
    );

    let mut out = [0u8; 64];
    Shake256::hash(b"", &mut out);
    assert_eq!(
        hex::encode(out),
        "46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762f\
         d75dc4ddd8c0f200cb05019d67b592f6fc821c49479ab48640292eacb3b7c4be"
    );

    let mut out = [0u8; 64];
    Shake256::hash(b"abc", &mut out);
    assert_eq!(
        hex::encode(out),
        "483366601360a8771c6863080cc4114d8db44530f8f1e1ee4f94ea37e78b5739\
         d5a15bef186a5386c75744c0527e1faa9f8726e462a12a4feb06bd8801e751e4"
    );
}

#[test]
fn shake_one_shot_matches_reference_across_input_lengths() {
    for len in 0..=1024 {
        let data = pattern(len);
        for out_len in [32usize, 64, 168, 169] {
            let mut actual = vec![0u8; out_len];
            Shake128::hash(&data, &mut actual);
            let mut expected = vec![0u8; out_len];
            reference::shake128(&data, &mut expected);
            assert_eq!(actual, expected, "shake128 input={len} output={out_len}");

            Shake256::hash(&data, &mut actual);
            reference::shake256(&data, &mut expected);
            assert_eq!(actual, expected, "shake256 input={len} output={out_len}");
        }
    }
}

fn check_shake_lengths<X: Xof>(
    mut make: impl FnMut() -> X,
    data: &[u8],
    out_len: usize,
    rate: usize,
    expected: &[u8],
    name: &str,
) {
    let mut actual = vec![0u8; out_len];
    let mut hasher = make();
    hasher.absorb(data);
    hasher.squeeze(&mut actual);
    assert_eq!(actual, expected, "{name} output={out_len}");

    // Chunked squeeze, including empty calls, must be identical.
    let mut hasher = make();
    hasher.absorb(data);
    hasher.squeeze(&mut []);
    let chunked = squeeze_chunked(|chunk| hasher.squeeze(chunk), out_len, rate);
    assert_eq!(chunked, expected, "{name} chunked output={out_len}");

    // Prefix property.
    let prefix_len = out_len / 2;
    let mut prefix = vec![0u8; prefix_len];
    let mut short = make();
    short.absorb(data);
    short.squeeze(&mut prefix);
    assert_eq!(prefix.as_slice(), &expected[..prefix_len], "{name} prefix output={out_len}");
}

#[test]
fn shake_output_length_sweep_and_chunking() {
    let inputs = [0usize, 1, 135, 136, 137, 167, 168, 169, 271, 272, 273, 500, 1024];

    for &input_len in &inputs {
        let data = pattern(input_len);
        for out_len in (0..=256).chain([2 * 168 + 1, 4 * 168 + 7]) {
            let mut expected = vec![0u8; out_len];
            reference::shake128(&data, &mut expected);
            check_shake_lengths(Shake128::new, &data, out_len, 168, &expected, "shake128");

            let mut expected = vec![0u8; out_len];
            reference::shake256(&data, &mut expected);
            check_shake_lengths(Shake256::new, &data, out_len, 136, &expected, "shake256");
        }
    }
}

#[test]
fn xof_full_input_output_matrix() {
    // Deterministic "fuzz" sweep: every input length 0..=1024 crossed with
    // every output length 0..=256 for the three Xof types. Heavy; run this
    // file with `--release` if the scalar reference is too slow in debug.
    let mut rng = StdRng::seed_from_u64(0x1234_5678_9abc_def0);

    for input_len in 0..=1024 {
        let data = pattern(input_len);

        for out_len in 0..=256 {
            let mut expected = vec![0u8; out_len];
            let mut actual = vec![0u8; out_len];

            // SHAKE128
            reference::shake128(&data, &mut expected);
            Shake128::hash(&data, &mut actual);
            assert_eq!(actual, expected, "shake128 {input_len}/{out_len}");

            // SHAKE256
            reference::shake256(&data, &mut expected);
            Shake256::hash(&data, &mut actual);
            assert_eq!(actual, expected, "shake256 {input_len}/{out_len}");

            // TurboSHAKE256 with a pseudo-random domain byte.
            let domain = 1 + (rng.next_u64() % 0x7f) as u8;
            reference::turboshake256(&data, domain, &mut expected);
            let mut hasher = TurboShake256::new(domain).unwrap();
            hasher.absorb(&data);
            hasher.squeeze(&mut actual);
            assert_eq!(actual, expected, "turboshake256 D={domain:#04x} {input_len}/{out_len}");
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
macro_rules! assert_absorb_after_squeeze_panics {
    ($hasher:expr) => {{
        let mut hasher = $hasher;
        hasher.squeeze(&mut [0u8; 1]);
        assert!(panics(move || hasher.absorb(b"more")), "absorb after squeeze did not panic");
    }};
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn xof_absorb_after_squeeze_panics() {
    assert_absorb_after_squeeze_panics!(Shake128::new());
    assert_absorb_after_squeeze_panics!(Shake256::new());
    assert_absorb_after_squeeze_panics!(CShake256::new(b"N", b"S"));
    assert_absorb_after_squeeze_panics!(TurboShake256::default());
}

#[test]
fn empty_squeeze_starts_the_output_phase_but_advances_nothing() {
    // `XofReader`/`Xof` document that `out` may be empty. This pins the
    // current behaviour: an empty squeeze is a no-op on the output stream but
    // still finalizes the sponge (a later absorb therefore panics).
    let data = pattern(300);

    let mut expected = [0u8; 136];
    Shake256::hash(&data, &mut expected);

    let mut hasher = Shake256::new();
    hasher.absorb(&data);
    hasher.squeeze(&mut []);
    let mut actual = [0u8; 136];
    hasher.squeeze(&mut actual);
    assert_eq!(actual, expected);

    let mut hasher = Shake256::new();
    hasher.absorb(&data);
    hasher.squeeze(&mut []);
    #[cfg(not(target_arch = "wasm32"))]
    assert!(panics(|| hasher.absorb(b"x")));
}

#[test]
fn xof_reader_behaviour_and_clone() {
    use std::io::Read;

    let mut hasher = Shake256::new();
    hasher.absorb(b"abc");
    let mut reader = hasher.finalize_xof();
    let mut out = [0u8; 64];
    reader.read_exact(&mut out).unwrap();
    assert_eq!(
        hex::encode(out),
        "483366601360a8771c6863080cc4114d8db44530f8f1e1ee4f94ea37e78b5739\
         d5a15bef186a5386c75744c0527e1faa9f8726e462a12a4feb06bd8801e751e4"
    );
    assert_eq!(reader.read(&mut []).unwrap(), 0);

    // A clone continues the same stream independently.
    let mut hasher = Shake256::new();
    hasher.absorb(b"abc");
    let mut reader = hasher.finalize_xof();
    let mut first = [0u8; 32];
    reader.squeeze(&mut first);
    let mut clone = reader.clone();
    let mut a = [0u8; 32];
    let mut b = [0u8; 32];
    reader.squeeze(&mut a);
    clone.squeeze(&mut b);
    assert_eq!(a, b);

    let mut hasher = Shake128::new();
    hasher.absorb(b"abc");
    let mut reader = hasher.finalize_xof();
    let mut limited = (&mut reader).take(16);
    let mut short = [0u8; 16];
    limited.read_exact(&mut short).unwrap();
    let mut full = [0u8; 16];
    let mut hasher = Shake128::new();
    hasher.absorb(b"abc");
    hasher.finalize_xof().squeeze(&mut full);
    assert_eq!(short, full);
}

// ---------------------------------------------------------------------------
// TurboSHAKE256
// ---------------------------------------------------------------------------

const RFC_TURBOSHAKE: &[(usize, u8, &str)] = &[
    (
        0,
        0x1f,
        "367a329dafea871c7802ec67f905ae13c57695dc2c6663c61035f59a18f8e7db\
         11edc0e12e91ea60eb6b32df06dd7f002fbafabb6e13ec1cc20d995547600db0",
    ),
    (
        1,
        0x1f,
        "3e1712f928f8eaf1054632b2aa0a246ed8b0c378728f60bc970410155c28820e\
         90cc90d8a3006aa2372c5c5ea176b0682bf22bae7467ac94f74d43d39b0482e2",
    ),
    (
        17,
        0x1f,
        "b3bab0300e6a191fbe6137939835923578794ea54843f5011090fa2f3780a9e5\
         cb22c59d78b40a0fbff9e672c0fbe0970bd2c845091c6044d687054da5d8e9c7",
    ),
    (
        289,
        0x1f,
        "66b810db8e90780424c0847372fdc95710882fde31c6df75beb9d4cd9305cfca\
         e35e7b83e8b7e6eb4b78605880116316fe2c078a09b94ad7b8213c0a738b65c0",
    ),
    (
        3,
        0x01,
        "d21c6fbbf587fa2282f29aea620175fb0257413af78a0b1b2a87419ce031d933\
         ae7a4d383327a8a17641a34f8a1d1003ad7da6b72dba84bb62fef28f62f12424",
    ),
    (
        1,
        0x06,
        "738d7b4e37d18b7f22ad1b5313e357e3dd7d07056a26a303c433fa3533455280\
         f4f5a7d4f700efb437fe6d281405e07be32a0a972e22e63adc1b090daefe004b",
    ),
    (
        3,
        0x07,
        "18b3b5b7061c2e67c1753a00e6ad7ed7ba1c906cf93efb7092eaf27fbeebb755\
         ae6e292493c110e48d260028492b8e09b5500612b8f2578985ded5357d00ec67",
    ),
    (
        7,
        0x0b,
        "bb36764951ec97e9d85f7ee9a67a7718fc005cf42556be79ce12c0bde50e5736\
         d6632b0d0dfb202d1bbb8ffe3dd74cb00834fa756cb03471bab13a1e2c16b3c0",
    ),
    (
        1,
        0x30,
        "f3fe12873d34bcbb2e608779d6b70e7f86bec7e90bf113cbd4fdd0c4e2f4625e\
         148dd7ee1a52776cf77f240514d9ccfc3b5ddab8ee255e39ee389072962c111a",
    ),
    (
        3,
        0x7f,
        "abe569c1f77ec340f02705e7d37c9ab7e155516e4a6a150021d70b6fac0bb40c\
         069f9a9828a0d575cd99f9bae435ab1acf7ed9110ba97ce0388d074bac768776",
    ),
];

#[test]
fn turboshake_rfc_9861_vectors() {
    for &(input_len, domain, expected) in RFC_TURBOSHAKE {
        let input = if input_len == 3 && domain == 0x01 {
            vec![0xff; 3]
        } else if input_len == 1 && domain == 0x06 {
            vec![0xff]
        } else if input_len == 3 && domain == 0x07 {
            vec![0xff; 3]
        } else if input_len == 7 && domain == 0x0b {
            vec![0xff; 7]
        } else if input_len == 1 && domain == 0x30 {
            vec![0xff]
        } else if input_len == 3 && domain == 0x7f {
            vec![0xff; 3]
        } else {
            pattern(input_len)
        };

        let mut out = [0u8; 64];
        let mut hasher = TurboShake256::new(domain).unwrap();
        hasher.absorb(&input);
        hasher.squeeze(&mut out);
        assert_eq!(hex::encode(out), expected, "D={domain:#04x} len={input_len}");

        // Chunked squeeze must match.
        let mut hasher = TurboShake256::new(domain).unwrap();
        hasher.absorb(&input);
        let chunked = squeeze_chunked(|chunk| hasher.squeeze(chunk), 64, 136);
        assert_eq!(chunked.as_slice(), out.as_slice());

        // `hash` and `Default` use the default domain separator.
        if domain == TURBOSHAKE256_DEFAULT_DOMAIN_SEPARATOR {
            let mut via_hash = [0u8; 64];
            TurboShake256::hash(&input, &mut via_hash);
            assert_eq!(via_hash, out);

            let mut via_default = TurboShake256::default();
            via_default.absorb(&input);
            let mut default_out = [0u8; 64];
            via_default.squeeze(&mut default_out);
            assert_eq!(default_out, out);
        }
    }

    // RFC 9861 long-output vector: last 32 bytes of TurboSHAKE256("", 0x1f,
    // 10032), and the 64-byte prefix above.
    let mut long = vec![0u8; 10032];
    TurboShake256::hash(&[], &mut long);
    assert_eq!(
        hex::encode(&long[10032 - 32..]),
        "abefa11630c661269249742685ec082f207265dccf2f43534e9c61ba0c9d1d75"
    );
}

#[test]
fn turboshake_domain_separator_sweep() {
    let data = pattern(300);
    let mut seen = HashSet::new();

    for domain in 0x01..=0x7f {
        let mut hasher = TurboShake256::new(domain).unwrap();
        hasher.absorb(&data);
        let chunked = squeeze_chunked(|chunk| hasher.squeeze(chunk), 2 * 136 + 17, 136);

        let mut expected = vec![0u8; chunked.len()];
        reference::turboshake256(&data, domain, &mut expected);

        assert_eq!(chunked, expected, "D={domain:#04x}");

        let mut tag = [0u8; 32];
        tag.copy_from_slice(&expected[..32]);
        assert!(seen.insert(tag), "domain bytes {domain:#04x} collided");
    }
    assert_eq!(seen.len(), 0x7f);
}

#[test]
fn turboshake_rejects_invalid_domain_separators() {
    for domain in [0x00u8, 0x80, 0xff] {
        assert!(matches!(
            TurboShake256::new(domain),
            Err(TurboShakeError::InvalidDomainSeparator)
        ));
    }
    assert!(TurboShake256::new(0x01).is_ok());
    assert!(TurboShake256::new(0x7f).is_ok());
}

#[test]
fn turboshake_output_length_sweep() {
    let inputs = [0usize, 1, 135, 136, 137, 271, 272, 273, 500, 1024];
    for &input_len in &inputs {
        let data = pattern(input_len);
        for out_len in 0..=272 {
            let mut expected = vec![0u8; out_len];
            reference::turboshake256(&data, 0x1f, &mut expected);

            let mut actual = vec![0u8; out_len];
            let mut hasher = TurboShake256::default();
            hasher.absorb(&data);
            hasher.squeeze(&mut actual);
            assert_eq!(actual, expected, "input={input_len} output={out_len}");

            let mut hasher = TurboShake256::default();
            hasher.absorb(&data);
            let chunked = squeeze_chunked(|chunk| hasher.squeeze(chunk), out_len, 136);
            assert_eq!(chunked, expected, "chunked input={input_len} output={out_len}");
        }
    }
}

#[test]
fn turboshake_hasher_trait_and_zero_output() {
    assert_eq!(<TurboShake256 as Hasher>::BLOCK_SIZE, 136);
    assert_eq!(<TurboShake256 as Hasher>::OUTPUT_SIZE, 64);

    let digest = <TurboShake256 as Hasher>::hash(b"");
    assert_eq!(
        hex::encode(digest),
        "367a329dafea871c7802ec67f905ae13c57695dc2c6663c61035f59a18f8e7db\
         11edc0e12e91ea60eb6b32df06dd7f002fbafabb6e13ec1cc20d995547600db0"
    );

    with_canaries(0, |out| TurboShake256::hash(b"data", out));
}

// ---------------------------------------------------------------------------
// KMAC256 / KMACXOF256
// ---------------------------------------------------------------------------

const KMAC_KEY: [u8; 32] = [
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52,
    0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f,
];
const KMAC_CUSTOM: &[u8] = b"My Tagged Application";

#[test]
fn kmac_nist_sp_800_185_vectors() {
    let mut out = [0u8; 64];
    Kmac256::mac(&KMAC_KEY, &[0x00, 0x01, 0x02, 0x03], KMAC_CUSTOM, &mut out);
    assert_eq!(
        hex::encode(out),
        "20c570c31346f703c9ac36c61c03cb64c3970d0cfc787e9b79599d273a68d2f7\
         f69d4cc3de9d104a351689f27cf6f5951f0103f33f4f24871024d9c27773a8dd"
    );

    let input: Vec<u8> = (0u8..200).collect();
    let mut out = [0u8; 64];
    Kmac256::mac(&KMAC_KEY, &input, b"", &mut out);
    assert_eq!(
        hex::encode(out),
        "75358cf39e41494e949707927cee0af20a3ff553904c86b08f21cc414bcfd691\
         589d27cf5e15369cbbff8b9a4c2eb17800855d0235ff635da82533ec6b759b69"
    );

    let mut kmac = Kmac256::new(&KMAC_KEY, KMAC_CUSTOM);
    for chunk in input.chunks(1) {
        kmac.update(chunk);
    }
    let mut out = [0u8; 64];
    kmac.finalize_into(&mut out);
    assert_eq!(
        hex::encode(out),
        "b58618f71f92e1d56c1b8c55ddd7cd188b97b4ca4d99831eb2699a837da2e4d9\
         70fbacfde50033aea585f1a2708510c32d07880801bd182898fe476876fc8965"
    );

    let mut kmac = Kmac256::new(&KMAC_KEY, KMAC_CUSTOM);
    kmac.update(&[0x00, 0x01, 0x02, 0x03]);
    let mut out = [0u8; 64];
    kmac.finalize_xof().squeeze(&mut out);
    assert_eq!(
        hex::encode(out),
        "1755133f1534752aad0748f2c706fb5c784512cab835cd15676b16c0c6647fa9\
         6faa7af634a0bf8ff6df39374fa00fad9a39e322a7c92065a64eb1fb0801eb2b"
    );

    let mut kmac = Kmac256::new(&KMAC_KEY, b"");
    kmac.update(&input);
    let mut out = [0u8; 64];
    kmac.finalize_xof().squeeze(&mut out);
    assert_eq!(
        hex::encode(out),
        "ff7b171f1e8a2b24683eed37830ee797538ba8dc563f6da1e667391a75edc02c\
         a633079f81ce12a25f45615ec89972031d18337331d24ceb8f8ca8e6a19fd98b"
    );
}

#[test]
fn kmac_reference_sweep() {
    let key_lengths = [0usize, 1, 2, 31, 32, 33, 130, 131, 132, 135, 136, 137, 200];
    let custom_lengths = [0usize, 1, 135, 136, 137, 255, 256];
    let message_lengths = [0usize, 1, 135, 136, 137, 200];
    let output_lengths = [0usize, 1, 16, 32, 64, 136, 137];

    for &key_len in &key_lengths {
        let key = pattern(key_len);
        for &custom_len in &custom_lengths {
            let custom = pattern(custom_len);
            for &message_len in &message_lengths {
                let message = pattern(message_len);
                for &output_len in &output_lengths {
                    let mut actual = vec![0u8; output_len];
                    Kmac256::mac(&key, &message, &custom, &mut actual);

                    let mut expected = vec![0u8; output_len];
                    reference::kmac256(&key, &message, &custom, false, &mut expected);

                    assert_eq!(
                        actual, expected,
                        "KMAC256 key={key_len} custom={custom_len} msg={message_len} out={output_len}"
                    );

                    // Incremental update must match the one-shot call.
                    let mut kmac = Kmac256::new(&key, &custom);
                    for chunk in message.chunks(7) {
                        kmac.update(chunk);
                    }
                    let mut incremental = vec![0u8; output_len];
                    kmac.finalize_into(&mut incremental);
                    assert_eq!(
                        incremental, expected,
                        "KMAC256 incremental key={key_len} custom={custom_len} msg={message_len}"
                    );
                }
            }
        }
    }
}

#[test]
fn kmacxof_reference_and_chunking() {
    let key32 = pattern(32);
    let key136 = pattern(136);
    let key200 = pattern(200);
    let custom135 = pattern(135);
    let custom136 = pattern(136);
    let custom256 = pattern(256);
    let msg136 = pattern(136);
    let msg300 = pattern(300);

    let keys: [&[u8]; 5] = [&[], b"k", &key32, &key136, &key200];
    let customs: [&[u8]; 5] = [&[], b"c", &custom135, &custom136, &custom256];
    let messages: [&[u8]; 4] = [&[], b"m", &msg136, &msg300];

    for key in keys {
        for custom in customs {
            for message in messages {
                for out_len in [0usize, 1, 135, 136, 137, 256, 2 * 136 + 7] {
                    let mut expected = vec![0u8; out_len];
                    reference::kmac256(key, message, custom, true, &mut expected);

                    let mut kmac = Kmac256::new(key, custom);
                    kmac.update(message);
                    let mut actual = vec![0u8; out_len];
                    kmac.finalize_xof().squeeze(&mut actual);
                    assert_eq!(actual, expected, "KMACXOF key={} msg={}", key.len(), message.len());

                    let mut kmac = Kmac256::new(key, custom);
                    kmac.update(message);
                    let mut reader = kmac.finalize_xof();
                    let chunked = squeeze_chunked(|chunk| reader.squeeze(chunk), out_len, 136);
                    assert_eq!(chunked, expected);
                }
            }
        }
    }
}

#[test]
fn kmac_output_length_is_bound_into_the_tag() {
    let key = pattern(32);
    let msg = pattern(100);
    let custom = b"domain";

    // Fixed-length KMAC binds L = 8*out.len(), so tags of different lengths
    // must not be prefixes of one another.
    let lengths = [0usize, 1, 2, 16, 31, 32, 33, 63, 64, 135, 136, 137];
    let mut tags: Vec<Vec<u8>> = Vec::new();
    for &len in &lengths {
        let mut tag = vec![0u8; len];
        Kmac256::mac(&key, &msg, custom, &mut tag);
        tags.push(tag);
    }

    // Equal-length KMAC must be identical, and a 64-byte XOF stream is not a
    // fixed-length 64-byte KMAC tag.
    let mut again = vec![0u8; 64];
    Kmac256::mac(&key, &msg, custom, &mut again);
    assert_eq!(again, tags[8]);

    let mut xof = vec![0u8; 64];
    let mut kmac = Kmac256::new(&key, custom);
    kmac.update(&msg);
    kmac.finalize_xof().squeeze(&mut xof);
    assert_ne!(xof, again);

    // KMACXOF output is a prefix stream independent of the squeezed length.
    let mut long = vec![0u8; 300];
    let mut kmac = Kmac256::new(&key, custom);
    kmac.update(&msg);
    kmac.finalize_xof().squeeze(&mut long);
    let mut short = vec![0u8; 64];
    let mut kmac = Kmac256::new(&key, custom);
    kmac.update(&msg);
    kmac.finalize_xof().squeeze(&mut short);
    assert_eq!(short.as_slice(), &long[..64]);
}

#[test]
fn kmac_clone_continues_the_same_stream() {
    let key = pattern(32);
    let customization = pattern(41);
    let first = pattern(137);
    let second = pattern(600);

    let mut expected = [0u8; 64];
    Kmac256::mac(
        &key,
        &[first.as_slice(), second.as_slice()].concat(),
        &customization,
        &mut expected,
    );

    let mut hasher = Kmac256::new(&key, &customization);
    hasher.update(&first);
    let mut clone = hasher.clone();
    hasher.update(&second);
    clone.update(&second);

    let mut from_original = [0u8; 64];
    hasher.finalize_into(&mut from_original);
    let mut from_clone = [0u8; 64];
    clone.finalize_into(&mut from_clone);
    assert_eq!(from_original, expected);
    assert_eq!(from_clone, expected);
}

#[test]
fn kmac_empty_key_and_customization() {
    let mut empty = [0u8; 32];
    Kmac256::mac(&[], b"", &[], &mut empty);
    let mut expected = [0u8; 32];
    reference::kmac256(&[], b"", &[], false, &mut expected);
    assert_eq!(empty, expected);

    let mut nonempty_key = [0u8; 32];
    Kmac256::mac(b"k", b"", &[], &mut nonempty_key);
    assert_ne!(empty, nonempty_key);

    let mut nonempty_custom = [0u8; 32];
    Kmac256::mac(&[], b"", b"c", &mut nonempty_custom);
    assert_ne!(empty, nonempty_custom);

    with_canaries(0, |out| Kmac256::mac(b"key", b"msg", b"custom", out));
}

#[test]
fn kmac_wycheproof_vectors() {
    let data: serde_json::Value = serde_json::from_str(include_str!(
        "../testdata/wycheproof/testvectors_v1/kmac256_no_customization_test.json"
    ))
    .unwrap();

    let mut valid = 0usize;
    for group in data["testGroups"].as_array().unwrap() {
        for test in group["tests"].as_array().unwrap() {
            let result = test["result"].as_str().unwrap();
            if result != "valid" && result != "acceptable" {
                continue;
            }

            let key = hex::decode(test["key"].as_str().unwrap()).unwrap();
            let msg = hex::decode(test["msg"].as_str().unwrap()).unwrap();
            let expected = hex::decode(test["tag"].as_str().unwrap()).unwrap();

            let mut out = vec![0u8; expected.len()];
            Kmac256::mac(&key, &msg, b"", &mut out);
            if result == "acceptable" && out != expected {
                continue;
            }
            assert_eq!(out, expected, "wycheproof tcId={}", test["tcId"]);
            valid += 1;
        }
    }
    assert!(valid > 0, "no valid KMAC256 Wycheproof cases");
}

// ---------------------------------------------------------------------------
// KangarooTwelve-256 / HopMAC256
// ---------------------------------------------------------------------------

#[test]
fn kt256_rfc_9861_vectors() {
    let vectors: &[(Vec<u8>, Vec<u8>, &str)] = &[
        (
            vec![],
            vec![],
            "b23d2e9cea9f4904e02bec06817fc10ce38ce8e93ef4c89e6537076af8646404\
             e3e8b68107b8833a5d30490aa33482353fd4adc7148ecb782855003aaebde4a9",
        ),
        (
            pattern(1),
            vec![],
            "0d005a194085360217128cf17f91e1f71314efa5564539d444912e3437efa17f\
             82db6f6ffe76e781eaa068bce01f2bbf81eacb983d7230f2fb02834a21b1ddd0",
        ),
        (
            pattern(17),
            vec![],
            "1ba3c02b1fc514474f06c8979978a9056c8483f4a1b63d0dccefe3a28a2f323e\
             1cdcca40ebf006ac76ef0397152346837b1277d3e7faa9c9653b19075098527b",
        ),
        (
            pattern(17 * 17),
            vec![],
            "de8ccbc63e0f133ebb4416814d4c66f691bbf8b6a61ec0a7700f836b086cb029\
             d54f12ac7159472c72db118c35b4e6aa213c6562caaa9dcc518959e69b10f3ba",
        ),
        (
            pattern(17 * 17 * 17),
            vec![],
            "647efb49fe9d717500171b41e7f11bd491544443209997ce1c2530d15eb1ffbb\
             598935ef954528ffc152b1e4d731ee2683680674365cd191d562bae753b84aa5",
        ),
        (
            pattern(17 * 17 * 17 * 17),
            vec![],
            "b06275d284cd1cf205bcbe57dccd3ec1ff6686e3ed15776383e1f2fa3c6ac8f0\
             8bf8a162829db1a44b2a43ff83dd89c3cf1ceb61ede659766d5ccf817a62ba8d",
        ),
        (
            vec![],
            pattern(1),
            "9280f5cc39b54a5a594ec63de0bb99371e4609d44bf845c2f5b8c316d72b1598\
             11f748f23e3fabbe5c3226ec96c62186df2d33e9df74c5069ceecbb4dd10eff6",
        ),
        (
            vec![0xff],
            pattern(41),
            "47ef96dd616f200937aa7847e34ec2feae8087e3761dc0f8c1a154f51dc9ccf8\
             45d7adbce57ff64b639722c6a1672e3bf5372d87e00aff89be97240756998853",
        ),
        (
            vec![0xff; 3],
            pattern(41 * 41),
            "3b48667a5051c5966c53c5d42b95de451e05584e7806e2fb765eda959074172c\
             b438a9e91dde337c98e9c41bed94c4e0aef431d0b64ef2324f7932caa6f54969",
        ),
        (
            vec![0xff; 7],
            pattern(41 * 41 * 41),
            "e0911cc00025e1540831e266d94add9b98712142b80d2629e643aac4efaf5a3a\
             30a88cbf4ac2a91a2432743054fbcc9897670e86ba8cec2fc2ace9c966369724",
        ),
        (
            pattern(8191),
            vec![],
            "3081434d93a4108d8d8a3305b89682cebedc7ca4ea8a3ce869fbb73cbe4a58ee\
             f6f24de38ffc170514c70e7ab2d01f03812616e863d769afb3753193ba045b20",
        ),
        (
            pattern(8192),
            vec![],
            "c6ee8e2ad3200c018ac87aaa031cdac22121b412d07dc6e0dccbb53423747e9a\
             1c18834d99df596cf0cf4b8dfafb7bf02d139d0c9035725adc1a01b7230a41fa",
        ),
        (
            pattern(8192),
            pattern(8189),
            "74e47879f10a9c5d11bd2da7e194fe57e86378bf3c3f7448eff3c576a0f18c5c\
             aae0999979512090a7f348af4260d4de3c37f1ecaf8d2c2c96c1d16c64b12496",
        ),
        (
            pattern(8192),
            pattern(8190),
            "f4b5908b929ffe01e0f79ec2f21243d41a396b2e7303a6af1d6399cd6c7a0a2d\
             d7c4f607e8277f9c9b1cb4ab9ddc59d4b92d1fc7558441f1832c3279a4241b8b",
        ),
    ];

    for (i, (message, customization, expected)) in vectors.iter().enumerate() {
        let mut out = [0u8; 64];
        Kt256::hash(message, customization, &mut out);
        assert_eq!(
            hex::encode(out),
            *expected,
            "vector {i}: message={} customization={}",
            message.len(),
            customization.len()
        );
    }
}

#[test]
fn kt256_chunk_boundaries_match_reference() {
    let message_lengths = [
        0usize,
        1,
        135,
        136,
        137,
        255,
        256,
        KT256_CHUNK_SIZE - 1,
        KT256_CHUNK_SIZE,
        KT256_CHUNK_SIZE + 1,
        2 * KT256_CHUNK_SIZE - 1,
        2 * KT256_CHUNK_SIZE,
        2 * KT256_CHUNK_SIZE + 1,
        3 * KT256_CHUNK_SIZE,
        4 * KT256_CHUNK_SIZE + 1,
        8 * KT256_CHUNK_SIZE - 1,
        8 * KT256_CHUNK_SIZE,
        8 * KT256_CHUNK_SIZE + 1,
    ];
    let customization_lengths = [0usize, 1, 255, 256, 8189, 8190, 8191, 8192];

    for &message_len in &message_lengths {
        let message = pattern(message_len);
        for &customization_len in &customization_lengths {
            let customization = pattern(customization_len);

            let mut expected = [0u8; 64];
            reference::kt256(&message, &customization, &mut expected);

            let mut actual = [0u8; 64];
            Kt256::hash(&message, &customization, &mut actual);
            assert_eq!(
                actual, expected,
                "KT256 message={message_len} customization={customization_len}"
            );
        }
    }
}

#[test]
fn kt256_chunk_count_encoding_boundaries() {
    // `length_encode(n)` for the number of chaining values changes width at
    // n = 255 and n = 256. A message of n whole chunks plus the empty
    // customization's `length_encode(0)` byte produces n leaves.
    for chunks in [1usize, 2, 254, 255, 256] {
        let message = pattern(chunks * KT256_CHUNK_SIZE);
        let mut expected = [0u8; 64];
        reference::kt256(&message, &[], &mut expected);

        let mut actual = [0u8; 64];
        Kt256::hash(&message, &[], &mut actual);
        assert_eq!(actual, expected, "chunks={chunks}");

        // Feed the same message in one-chunk updates so the batched and
        // sequential leaf paths both run.
        let mut hasher = Kt256::new(&[]);
        for chunk in message.chunks(KT256_CHUNK_SIZE) {
            hasher.update(chunk);
        }
        let mut streamed = [0u8; 64];
        hasher.finalize_into(&mut streamed);
        assert_eq!(streamed, expected, "chunks={chunks} streaming");
    }
}

#[test]
fn kt256_s_boundary_picks_single_node_or_tree() {
    // The single-node/tree decision is made on |S| = |M| + |C| +
    // |length_encode(|C|)|, not on |M|. These cases straddle |S| = 8192.
    // `length_encode(n)` is 1 byte for 0, 2 bytes for 1..=255 and 3 bytes for
    // 256..=65535, so each pair below has a known |S|.
    let cases = [
        (8189usize, 0usize, 8190usize),
        (8189, 1, 8192),  // 8189 + 1 + 2
        (8188, 2, 8192),  // 8188 + 2 + 2
        (8180, 10, 8192), // 8180 + 10 + 2
        (8190, 0, 8191),
        (8191, 0, 8192),
        (8191, 1, 8194), // tree
        (8192, 0, 8193), // tree
        (8190, 1, 8193), // tree
    ];

    for (message_len, custom_len, s_len) in cases {
        let message = pattern(message_len);
        let customization = pattern(custom_len);
        assert_eq!(
            message_len + custom_len + reference::length_encode(custom_len).len(),
            s_len,
            "test case arithmetic"
        );

        let mut expected = [0u8; 64];
        reference::kt256(&message, &customization, &mut expected);

        let mut actual = [0u8; 64];
        Kt256::hash(&message, &customization, &mut actual);
        assert_eq!(actual, expected, "|S|={s_len}");

        let mut hasher = Kt256::new(&customization);
        for chunk in message.chunks(7) {
            hasher.update(chunk);
        }
        let mut streamed = [0u8; 64];
        hasher.finalize_into(&mut streamed);
        assert_eq!(streamed, expected, "|S|={s_len} streaming");
    }
}

#[test]
fn kt256_streaming_matches_one_shot() {
    let chunk_sizes = [1usize, 2, 3, 7, 135, 136, 137, 8191, 8192, 8193, 2 * 8192 + 1];
    let lengths = [
        0usize,
        1,
        8191,
        8192,
        8193,
        2 * 8192,
        2 * 8192 + 1,
        3 * 8192 + 7,
        8 * 8192 + 1,
    ];
    let customization = pattern(41);

    for &len in &lengths {
        let data = pattern(len);
        let mut expected = [0u8; 64];
        Kt256::hash(&data, &customization, &mut expected);

        for &chunk_size in &chunk_sizes {
            let mut hasher = Kt256::new(&customization);
            for chunk in data.chunks(chunk_size) {
                hasher.update(chunk);
            }
            let mut actual = [0u8; 64];
            hasher.finalize_into(&mut actual);
            assert_eq!(actual, expected, "len={len} chunk={chunk_size}");
        }
    }
}

#[test]
fn kt256_customization_lengths() {
    let message = pattern(2 * KT256_CHUNK_SIZE + 1);

    for custom_len in [0usize, 1, 255, 256, 8191, 8192, 8193] {
        let customization = pattern(custom_len);
        let mut expected = [0u8; 64];
        reference::kt256(&message, &customization, &mut expected);

        let mut actual = [0u8; 64];
        Kt256::hash(&message, &customization, &mut actual);
        assert_eq!(actual, expected, "customization={custom_len}");
    }

    // Different customization strings must be bound into the hash, including
    // the empty string versus a one-byte string.
    let mut a = [0u8; 64];
    let mut b = [0u8; 64];
    Kt256::hash(&message, &[], &mut a);
    Kt256::hash(&message, b"\x00", &mut b);
    assert_ne!(a, b);
}

#[test]
fn kt256_output_length_sweep_and_prefix() {
    let message = pattern(2 * KT256_CHUNK_SIZE + 17);
    let customization = pattern(13);

    let mut long = vec![0u8; 4 * 136 + 100];
    Kt256::hash(&message, &customization, &mut long);

    for out_len in 0..=300 {
        let mut expected = vec![0u8; out_len];
        reference::kt256(&message, &customization, &mut expected);
        assert_eq!(&long[..out_len], expected.as_slice(), "output={out_len}");

        let mut actual = vec![0u8; out_len];
        Kt256::hash(&message, &customization, &mut actual);
        assert_eq!(actual, expected, "output={out_len}");
    }

    // Incremental reader, chunked squeeze.
    let mut hasher = Kt256::new(&customization);
    hasher.update(&message);
    let mut reader = hasher.finalize_xof();
    let chunked = squeeze_chunked(|chunk| reader.squeeze(chunk), long.len(), 136);
    assert_eq!(chunked, long);

    with_canaries(0, |out| Kt256::hash(&message, &customization, out));
}

#[test]
fn kt256_deterministic_random_sweep() {
    let mut rng = StdRng::seed_from_u64(0x0ddc_0ffe_e123_4567);

    for _ in 0..100 {
        let message_len = rng.random_range(0..4 * KT256_CHUNK_SIZE + 1);
        let custom_len = rng.random_range(0..300);
        let out_len = rng.random_range(0..400);
        let message = pattern(message_len);
        let customization = pattern(custom_len);

        let mut expected = vec![0u8; out_len];
        reference::kt256(&message, &customization, &mut expected);

        let mut first = vec![0u8; out_len];
        Kt256::hash(&message, &customization, &mut first);
        assert_eq!(first, expected, "message={message_len} custom={custom_len} out={out_len}");

        let mut second = vec![0u8; out_len];
        Kt256::hash(&message, &customization, &mut second);
        assert_eq!(first, second, "non-deterministic output");
    }
}

#[test]
fn kt256_reader_and_streaming_clone() {
    let data = pattern(3 * KT256_CHUNK_SIZE + 7);
    let customization = pattern(13);

    let mut expected = [0u8; 64];
    Kt256::hash(&data, &customization, &mut expected);

    for split in [0usize, 8191, 8192, 8193, 2 * 8192 + 5] {
        let mut hasher = Kt256::new(&customization);
        hasher.update(&data[..split]);
        let clone = hasher.clone();

        let mut from_original = [0u8; 64];
        hasher.update(&data[split..]);
        hasher.finalize_into(&mut from_original);

        let mut from_clone = [0u8; 64];
        let mut clone = clone;
        clone.update(&data[split..]);
        clone.finalize_into(&mut from_clone);

        assert_eq!(from_original, expected, "split={split}");
        assert_eq!(from_clone, expected, "split={split}");
    }
}

#[test]
fn hopmac256_matches_spec_composition() {
    let key_lengths = [0usize, 1, 32, 200];
    let message_lengths = [0usize, 1, 8191, 8192, 8193, 2 * 8192 + 1];
    let custom_lengths = [0usize, 1, 255, 256];
    let output_lengths = [0usize, 1, 32, 64, 257];

    for &key_len in &key_lengths {
        let key = pattern(key_len);
        for &message_len in &message_lengths {
            let message = pattern(message_len);
            for &custom_len in &custom_lengths {
                let customization = pattern(custom_len);
                for &output_len in &output_lengths {
                    let mut expected = vec![0u8; output_len];
                    reference::hopmac256(&key, &message, &customization, &mut expected);

                    let mut actual = vec![0u8; output_len];
                    HopMac256::mac(&key, &message, &customization, &mut actual);
                    assert_eq!(
                        actual, expected,
                        "HopMAC key={key_len} msg={message_len} custom={custom_len} out={output_len}"
                    );
                }
            }
        }
    }
}

#[test]
fn hopmac256_streaming_and_xof() {
    let key = pattern(32);
    let customization = pattern(41);

    for &len in &[0usize, 1, 8191, 8192, 8193, 2 * 8192, 2 * 8192 + 1] {
        let data = pattern(len);
        let mut expected = [0u8; 64];
        HopMac256::mac(&key, &data, &customization, &mut expected);

        for &chunk_size in &[1usize, 3, 7, 136, 8192, 2 * 8192 + 1] {
            let mut hasher = HopMac256::new(&key, &customization);
            for chunk in data.chunks(chunk_size) {
                hasher.update(chunk);
            }
            let mut streamed = [0u8; 64];
            hasher.finalize_into(&mut streamed);
            assert_eq!(streamed, expected, "len={len} chunk={chunk_size}");
        }

        // Chunk-independent incremental output.
        let mut short = [0u8; 64];
        HopMac256::mac(&key, &data, &customization, &mut short);

        let mut hasher = HopMac256::new(&key, &customization);
        hasher.update(&data);
        let mut reader = hasher.finalize_xof();
        let long = squeeze_chunked(|chunk| reader.squeeze(chunk), 1000, 136);
        assert_eq!(&long[..64], short.as_slice());

        let mut hasher = HopMac256::new(&key, &customization);
        hasher.update(&data);
        let mut long_oneshot = vec![0u8; 1000];
        hasher.finalize_into(&mut long_oneshot);
        assert_eq!(long_oneshot, long);
    }
}

#[test]
fn hopmac256_known_answer_vector() {
    let key = pattern(32);
    let mut out = [0u8; 32];
    HopMac256::mac(&key, &[], &[], &mut out);
    assert_eq!(
        hex::encode(out),
        "137d48ca2fc05854533ddfce3c5e0acb1888488f651cbe5940b0f1e87321c52c"
    );

    let mut full = [0u8; 64];
    HopMac256::mac(&key, &pattern(17), &[], &mut full);
    assert_eq!(
        hex::encode(full),
        "398819a1b1a2bc5d809713fa22c21284e360562a6ee17840816e66be84bc7dbd\
         d028886d493a57a33f38d6596afc5cf57ad7224650374af90d9fff65506af127"
    );
}

// ---------------------------------------------------------------------------
// SP 800-185 encodings and boundary/guard checks
// ---------------------------------------------------------------------------

#[test]
fn sp_800_185_encodings_are_used_consistently() {
    // The reference encoder itself is pinned against the SP 800-185 examples.
    assert_eq!(reference::left_encode(0).as_slice(), &[0x01, 0x00][..]);
    assert_eq!(reference::left_encode(1).as_slice(), &[0x01, 0x01][..]);
    assert_eq!(reference::left_encode(255).as_slice(), &[0x01, 0xff][..]);
    assert_eq!(reference::left_encode(256).as_slice(), &[0x02, 0x01, 0x00][..]);
    assert_eq!(reference::left_encode(65535).as_slice(), &[0x02, 0xff, 0xff][..]);
    assert_eq!(reference::left_encode(65536).as_slice(), &[0x03, 0x01, 0x00, 0x00][..]);

    assert_eq!(reference::right_encode(0).as_slice(), &[0x00, 0x01][..]);
    assert_eq!(reference::right_encode(1).as_slice(), &[0x01, 0x01][..]);
    assert_eq!(reference::right_encode(255).as_slice(), &[0xff, 0x01][..]);
    assert_eq!(reference::right_encode(256).as_slice(), &[0x01, 0x00, 0x02][..]);
    assert_eq!(reference::right_encode(65535).as_slice(), &[0xff, 0xff, 0x02][..]);
    assert_eq!(reference::right_encode(65536).as_slice(), &[0x01, 0x00, 0x00, 0x03][..]);

    // cSHAKE with empty N and S is SHAKE256; KMAC uses cSHAKE256("KMAC", S).
    let data = pattern(129);
    let mut cshake = [0u8; 64];
    CShake256::hash(&data, b"", b"", &mut cshake);
    let mut shake = [0u8; 64];
    Shake256::hash(&data, &mut shake);
    assert_eq!(cshake, shake);
}

#[test]
fn cshake256_reference_sweep() {
    let name_lengths = [0usize, 1, 2, 135, 136, 137, 200];
    let custom_lengths = [0usize, 1, 2, 125, 126, 127, 129, 135, 136, 137, 255, 256, 300];
    let message_lengths = [0usize, 1, 135, 136, 137, 200, 300];

    for &name_len in &name_lengths {
        let name = pattern(name_len);
        for &custom_len in &custom_lengths {
            let custom = pattern(custom_len);
            for &message_len in &message_lengths {
                let message = pattern(message_len);

                let mut expected = vec![0u8; 2 * 136 + 7];
                reference::cshake256(&message, &name, &custom, &mut expected);

                let mut actual = vec![0u8; 2 * 136 + 7];
                CShake256::hash(&message, &name, &custom, &mut actual);
                assert_eq!(actual, expected, "cSHAKE name={name_len} custom={custom_len} msg={message_len}");

                // Incremental absorption and empty squeeze.
                let mut hasher = CShake256::new(&name, &custom);
                for chunk in message.chunks(5) {
                    hasher.absorb(chunk);
                }
                hasher.squeeze(&mut []);
                let mut incremental = vec![0u8; 2 * 136 + 7];
                hasher.squeeze(&mut incremental);
                assert_eq!(incremental, expected);
            }
        }
    }
}

#[test]
fn cshake256_nist_samples() {
    let mut out = [0u8; 64];
    CShake256::hash(&[0x00, 0x01, 0x02, 0x03], b"", b"Email Signature", &mut out);
    assert_eq!(
        hex::encode(out),
        "d008828e2b80ac9d2218ffee1d070c48b8e4c87bff32c9699d5b6896eee0edd1\
         64020e2be0560858d9c00c037e34a96937c561a74c412bb4c746469527281c8c"
    );

    let input: Vec<u8> = (0u8..200).collect();
    let mut out = [0u8; 64];
    CShake256::hash(&input, b"", b"Email Signature", &mut out);
    assert_eq!(
        hex::encode(out),
        "07dc27b11e51fbac75bc7b3c1d983e8b4b85fb1defaf218912ac864302730917\
         27f42b17ed1df63e8ec118f04b23633c1dfb1574c8fb55cb45da8e25afb092bb"
    );
}

#[test]
fn large_inputs_match_reference() {
    let data = pattern(1_000_000);

    let mut expected256 = [0u8; 32];
    reference::sha3_256(&data, &mut expected256);
    assert_eq!(<Sha3_256 as Hasher>::hash(&data).as_ref(), expected256);

    let mut expected512 = [0u8; 64];
    reference::sha3_512(&data, &mut expected512);
    assert_eq!(<Sha3_512 as Hasher>::hash(&data).as_ref(), expected512);

    let mut expected = [0u8; 64];
    reference::shake128(&data, &mut expected);
    let mut actual = [0u8; 64];
    Shake128::hash(&data, &mut actual);
    assert_eq!(actual, expected);

    reference::shake256(&data, &mut expected);
    Shake256::hash(&data, &mut actual);
    assert_eq!(actual, expected);

    reference::turboshake256(&data, 0x1f, &mut expected);
    TurboShake256::hash(&data, &mut actual);
    assert_eq!(actual, expected);

    reference::kmac256(b"large-key", &data, b"large-custom", false, &mut expected);
    Kmac256::mac(b"large-key", &data, b"large-custom", &mut actual);
    assert_eq!(actual, expected);

    reference::kt256(&data, b"large-custom", &mut expected);
    Kt256::hash(&data, b"large-custom", &mut actual);
    assert_eq!(actual, expected);
}

#[test]
fn output_buffers_are_not_overwritten() {
    let data = pattern(300);

    with_canaries(32, |out| Shake128::hash(&data, out));
    with_canaries(64, |out| Shake256::hash(&data, out));
    with_canaries(17, |out| TurboShake256::hash(&data, out));
    with_canaries(64, |out| CShake256::hash(&data, b"n", b"s", out));
    with_canaries(64, |out| Kmac256::mac(b"key", &data, b"custom", out));
    with_canaries(64, |out| Kt256::hash(&data, b"custom", out));
    with_canaries(64, |out| HopMac256::mac(b"key", &data, b"custom", out));

    // Fixed-size outputs.
    let mut digest = <Sha3_256 as Hasher>::hash(&data);
    assert_eq!(digest.len(), 32);
    digest.as_mut()[0] ^= 0;
}

#[test]
fn backend_names_are_reported() {
    let keccak = crypto::sha3::keccak_backend_name();
    let kt256 = crypto::sha3::kt256_backend_name();
    println!("keccak backend: {keccak}; kt256 backend: {kt256}");
    assert!(!keccak.is_empty());
    assert!(!kt256.is_empty());
}

#[test]
fn zero_length_outputs_never_panic() {
    let data = pattern(200);

    let mut shake128 = Shake128::new();
    shake128.absorb(&data);
    shake128.squeeze(&mut []);

    let mut shake256 = Shake256::new();
    shake256.absorb(&data);
    shake256.squeeze(&mut []);

    let mut turbo = TurboShake256::default();
    turbo.absorb(&data);
    turbo.squeeze(&mut []);

    let mut cshake = CShake256::new(b"n", b"s");
    cshake.absorb(&data);
    cshake.squeeze(&mut []);

    Kmac256::mac(b"key", &data, b"custom", &mut []);
    Kt256::hash(&data, b"custom", &mut []);
    HopMac256::mac(b"key", &data, b"custom", &mut []);
}
