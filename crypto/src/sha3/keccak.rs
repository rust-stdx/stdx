use core::cmp::min;

#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

pub const RHO: [u32; 24] = [
    1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 2, 14, 27, 41, 56, 8, 25, 43, 62, 18, 39, 61, 20, 44,
];

pub const PI: [usize; 24] = [
    10, 7, 11, 17, 18, 3, 5, 16, 8, 21, 24, 4, 15, 23, 19, 13, 12, 2, 20, 14, 22, 9, 6, 1,
];

pub const ROUND_CONSTANTS: [u64; 24] = [
    0x0000000000000001,
    0x0000000000008082,
    0x800000000000808a,
    0x8000000080008000,
    0x000000000000808b,
    0x0000000080000001,
    0x8000000080008081,
    0x8000000000008009,
    0x000000000000008a,
    0x0000000000000088,
    0x0000000080008009,
    0x000000008000000a,
    0x000000008000808b,
    0x800000000000008b,
    0x8000000000008089,
    0x8000000000008003,
    0x8000000000008002,
    0x8000000000000080,
    0x000000000000800a,
    0x800000008000000a,
    0x8000000080008081,
    0x8000000000008080,
    0x0000000080000001,
    0x8000000080008008,
];

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
enum SpongeMode {
    Absorbing,
    Squeezing,
}

/// The single-state permutation implementation a sponge uses.
///
/// With `std`, the x86-64 backend is detected once per process and cached
/// again in each sponge; other targets use their compile-time or cached feature
/// detection. Neither the permutation nor the sponge construction probes the
/// CPU after the first detection. This mirrors
/// [`super::keccak_batch::Backend`], which does the same for the batched
/// permutation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "zeroize", derive(Zeroize))]
pub(crate) enum PermuteBackend {
    /// Portable 64-bit implementation.
    Soft,
    /// x86-64 single-state AVX2, in the CRYPTOGAMS "magic" lane order.
    #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
    Avx2,
    /// x86-64 single-state AVX-512F, two rounds per iteration.
    #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
    Avx512,
    /// Armv8.4-A with FEAT_SHA3.
    #[cfg(all(target_arch = "aarch64", any(feature = "std", target_feature = "sha3")))]
    Sha3,
}

impl PermuteBackend {
    /// Detects the single-state backend once per process.
    ///
    /// With `std` on x86-64 the result is cached in the `keccak_amd64` module,
    /// so the vendor/family `cpuid` probe runs at most once instead of once
    /// per sponge. `no_std` builds trust the compile-time `target_feature`s,
    /// since there is no runtime probe.
    #[allow(unreachable_code)]
    #[inline]
    pub(crate) fn detect() -> Self {
        #[cfg(all(target_arch = "x86_64", feature = "std"))]
        {
            return super::keccak_amd64::detect_amd64();
        }

        #[cfg(target_arch = "x86_64")]
        {
            #[cfg(all(not(feature = "std"), target_feature = "avx512f"))]
            return PermuteBackend::Avx512;

            #[cfg(all(not(feature = "std"), target_feature = "avx2"))]
            return PermuteBackend::Avx2;
        }

        #[cfg(target_arch = "aarch64")]
        {
            #[cfg(feature = "std")]
            if std::arch::is_aarch64_feature_detected!("sha3") {
                return PermuteBackend::Sha3;
            }

            #[cfg(all(not(feature = "std"), target_feature = "sha3"))]
            return PermuteBackend::Sha3;
        }

        PermuteBackend::Soft
    }

    /// Human-readable name of this backend, for diagnostics.
    pub(crate) const fn name(self) -> &'static str {
        match self {
            PermuteBackend::Soft => "scalar",
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
            PermuteBackend::Avx2 => "avx2",
            #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
            PermuteBackend::Avx512 => "avx512",
            #[cfg(all(target_arch = "aarch64", any(feature = "std", target_feature = "sha3")))]
            PermuteBackend::Sha3 => "armv8.4-sha3",
        }
    }
}

/// A sponge construction based on Keccak p1600
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
// `state` is transmuted to `&mut [u64; 25]` on little-endian targets, which
// requires the field itself (not just the struct) to be 8-byte aligned.
// `repr(C)` guarantees that `state`, the first field, has offset zero.
#[repr(C, align(8))]
pub struct KeccakSponge<const ROUNDS: usize> {
    state: [u8; 200],
    rate: usize,
    padding: u8,
    /// the current position in the state buffer
    pos: usize,
    mode: SpongeMode,
    /// cached at construction so the permutation does not probe the CPU
    backend: PermuteBackend,
}

const _: () = assert!(
    core::mem::offset_of!(KeccakSponge<24>, state) % 8 == 0,
    "the Keccak sponge state must be 8-byte aligned for the u64 view"
);

/// Loads the 25 little-endian lanes of a serialized Keccak state.
///
/// Keccak interprets each eight-byte lane in little-endian order regardless of
/// the host byte order. This helper is only needed on big-endian targets and by
/// tests, but keeping it target-independent makes the conversion testable on
/// little-endian machines.
#[cfg(any(target_endian = "big", test))]
#[inline]
fn load_state_le(bytes: &[u8; 200]) -> [u64; 25] {
    let mut lanes = [0u64; 25];
    for (i, lane) in lanes.iter_mut().enumerate() {
        let mut lane_bytes = [0u8; 8];
        lane_bytes.copy_from_slice(&bytes[8 * i..8 * i + 8]);
        *lane = u64::from_le_bytes(lane_bytes);
    }
    return lanes;
}

/// Serializes 25 lanes as little-endian bytes.
#[cfg(any(target_endian = "big", test))]
#[inline]
fn store_state_le(lanes: &[u64; 25], bytes: &mut [u8; 200]) {
    for (i, lane) in lanes.iter().enumerate() {
        bytes[8 * i..8 * i + 8].copy_from_slice(&lane.to_le_bytes());
    }
}

/// Applies the permutation to a byte-serialized state.
#[inline]
fn permute_state_bytes<const ROUNDS: usize>(state: &mut [u8; 200], backend: PermuteBackend) {
    #[cfg(target_endian = "little")]
    {
        // SAFETY: `state` is 8-byte aligned (see the `repr(C, align(8))` and
        // the compile-time assertion above), has the same size as `[u64; 25]`,
        // and no other reference to it exists for the duration of the call.
        let state: &mut [u64; 25] = unsafe { core::mem::transmute(state) };
        p1600_with::<ROUNDS>(state, backend);
    }

    #[cfg(target_endian = "big")]
    {
        let mut lanes = load_state_le(state);
        p1600_with::<ROUNDS>(&mut lanes, backend);
        store_state_le(&lanes, state);
    }
}

impl<const ROUNDS: usize> KeccakSponge<ROUNDS> {
    /// Creates a sponge with the given rate (in bytes) and padding delimiter.
    ///
    /// `rate` must satisfy `0 < rate < 200`; it is the number of state bytes
    /// between two permutations and must leave room for the padding byte.
    ///
    /// # Panics
    ///
    /// Panics if `rate` is zero or `>= 200`.
    #[inline]
    pub fn new(rate: usize, delimiter: u8) -> Self {
        assert!(rate > 0 && rate < 200, "the Keccak rate must be in 1..200 bytes");
        return KeccakSponge {
            state: [0u8; 200],
            rate,
            padding: delimiter,
            pos: 0,
            mode: SpongeMode::Absorbing,
            backend: PermuteBackend::detect(),
        };
    }

    /// Replaces the domain-separation byte used by the next padding step.
    ///
    /// This is only valid while the sponge is absorbing. It lets a caller
    /// that has already absorbed a prefix decide the final domain separator
    /// afterwards, without buffering the prefix.
    ///
    /// # Panics
    ///
    /// Panics if the sponge has already started squeezing.
    #[inline]
    pub(crate) fn set_delimiter(&mut self, delimiter: u8) {
        assert_eq!(
            self.mode,
            SpongeMode::Absorbing,
            "the delimiter can only be changed while absorbing"
        );
        self.padding = delimiter;
    }

    #[inline]
    pub fn absorb(&mut self, data: &[u8]) {
        assert_eq!(self.mode, SpongeMode::Absorbing, "absorb can't be called after squeezing");

        // we first need to prevent `data` to overflow into `capacity`
        let rate_remainder = min(self.rate - self.pos, data.len());
        self.absorb_chunk(&data[..rate_remainder]);

        // then we can absorbe `RATE`-sized chunks
        for chunk in data[rate_remainder..].chunks(self.rate) {
            self.absorb_chunk(chunk);
        }
    }

    #[inline]
    pub fn squeeze(&mut self, out: &mut [u8]) {
        // if we're still absorbing, pad and apply the permutation
        if self.mode == SpongeMode::Absorbing {
            self.pad_and_permute();
            self.mode = SpongeMode::Squeezing;
        }

        // we first need to prevent `out` to overflow into `capacity`
        let rate_remainder = min(self.rate - self.pos, out.len());
        self.squeeze_chunk(&mut out[..rate_remainder]);

        // then we can squeeze `RATE`-sized chunks
        for chunk in out[rate_remainder..].chunks_mut(self.rate) {
            self.squeeze_chunk(chunk);
        }
    }

    #[inline]
    fn absorb_chunk(&mut self, chunk: &[u8]) {
        xor(&mut self.state[self.pos..self.pos + chunk.len()], chunk);
        self.pos += chunk.len();

        // if the sponge is full, apply the permutation
        if self.pos == self.rate {
            self.permute_and_reset_pos();
        }
    }

    #[inline]
    fn squeeze_chunk(&mut self, out: &mut [u8]) {
        if self.pos == self.rate {
            self.permute_and_reset_pos();
        }

        out.copy_from_slice(&self.state[self.pos..self.pos + out.len()]);
        self.pos += out.len();
    }

    #[inline]
    fn pad_and_permute(&mut self) {
        self.state[self.pos] ^= self.padding;
        self.state[self.rate - 1] ^= 0x80;
        self.permute_and_reset_pos();
    }

    #[inline]
    fn permute_and_reset_pos(&mut self) {
        permute_state_bytes::<ROUNDS>(&mut self.state, self.backend);
        self.pos = 0;
    }
}

/// The Keccak-p permutation of a 1600-bit state.
///
/// With `std`, the best implementation the CPU supports is selected at run
/// time: AVX-512F or AVX2 on x86-64 (detected once per process and cached; see
/// [`PermuteBackend::detect`]) and FEAT_SHA3 on aarch64. Without `std`, the
/// compile-time `target_feature`s are used instead. Everywhere else the
/// portable implementation is used.
///
/// Callers that permute more than one block should detect once with
/// [`PermuteBackend::detect`] and call [`p1600_with`] instead; [`KeccakSponge`]
/// already does.
///
/// Instantiating this function with `ROUNDS > 24` fails to compile.
pub fn p1600<const ROUNDS: usize>(state: &mut [u64; 25]) {
    p1600_with::<ROUNDS>(state, PermuteBackend::detect());
}

/// Applies Keccak-p[1600, `ROUNDS`] with an already-detected `backend`.
///
/// Instantiating this function with `ROUNDS > 24` fails to compile.
pub(crate) fn p1600_with<const ROUNDS: usize>(state: &mut [u64; 25], backend: PermuteBackend) {
    const {
        assert!(ROUNDS <= 24, "A round_count greater than 24 is not supported.");
    }

    match backend {
        PermuteBackend::Soft => p1600_soft::<ROUNDS>(state),
        #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
        // SAFETY: the backend was detected at construction.
        PermuteBackend::Avx2 => unsafe { super::keccak_amd64::p1600_avx2::<ROUNDS>(state) },
        #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
        // SAFETY: the backend was detected at construction.
        PermuteBackend::Avx512 => unsafe { super::keccak_amd64::p1600_avx512::<ROUNDS>(state) },
        #[cfg(all(target_arch = "aarch64", any(feature = "std", target_feature = "sha3")))]
        // SAFETY: FEAT_SHA3 was detected at construction.
        PermuteBackend::Sha3 => unsafe { super::keccak_arm64::p1600_armv8::<ROUNDS>(state) },
    }
}

/// Portable implementation of the Keccak-p permutation of a 1600-bit state.
#[inline]
pub(super) fn p1600_soft<const ROUNDS: usize>(state: &mut [u64; 25]) {
    // https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf#page=25
    // "the rounds of KECCAK-p[b, nr] match the last rounds of KECCAK-f[b]"
    let round_consts: &[u64] = &ROUND_CONSTANTS[(24 - ROUNDS)..];

    // not unrolling this loop may results in a smaller function, plus
    // it may positively influences performance due to the smaller number of instructions
    for &rc in round_consts {
        let mut array = [0u64; 5];

        // Theta
        for x in 0..5 {
            for y in 0..5 {
                array[x] ^= state[5 * y + x];
            }
        }

        for x in 0..5 {
            let t1 = array[(x + 4) % 5];
            let t2 = array[(x + 1) % 5].rotate_left(1);
            for y in 0..5 {
                state[5 * y + x] ^= t1 ^ t2;
            }
        }

        // Rho and pi
        let mut last = state[1];
        for x in 0..24 {
            array[0] = state[PI[x]];
            state[PI[x]] = last.rotate_left(RHO[x]);
            last = array[0];
        }

        // Chi
        for y_step in 0..5 {
            let y = 5 * y_step;

            array.copy_from_slice(&state[y..][..5]);

            for x in 0..5 {
                let t1 = !array[(x + 1) % 5];
                let t2 = array[(x + 2) % 5];
                state[y + x] = array[x] ^ (t1 & t2);
            }
        }

        // Iota
        state[0] ^= rc;
    }
}

/// xor dest with source. source is not modified.
#[inline(always)]
fn xor(dest: &mut [u8], source: &[u8]) {
    dest.iter_mut()
        .zip(source.iter())
        .for_each(|(dest, source)| *dest ^= *source);
}

/// Independent, test-only reference implementations of the sponge and the
/// SP 800-185 encodings.
///
/// These are deliberately built from the FIPS 202/SP 800-185 definitions
/// instead of the production sponge, so a bug in one layer cannot silently
/// become the oracle for another layer. They still use `p1600_soft`, which is
/// pinned by the external XKCP test vectors.
#[cfg(test)]
pub(crate) mod test_util {
    use super::p1600_soft;

    /// Encodes a `usize` as the SP 800-185 `left_encode` bit string.
    ///
    /// Returns the buffer and the number of valid leading bytes.
    pub(crate) fn left_encode(x: usize) -> ([u8; 9], usize) {
        let bytes = x.to_be_bytes();
        let mut first = 0;
        while first < bytes.len() - 1 && bytes[first] == 0 {
            first += 1;
        }
        let n = bytes.len() - first;

        let mut out = [0u8; 9];
        out[0] = n as u8;
        out[1..1 + n].copy_from_slice(&bytes[first..]);
        return (out, 1 + n);
    }

    /// Encodes a `usize` as the SP 800-185 `right_encode` bit string.
    pub(crate) fn right_encode(x: usize) -> ([u8; 9], usize) {
        let (left, len) = left_encode(x);
        let n = left[0] as usize;
        debug_assert_eq!(len, n + 1);

        let mut out = [0u8; 9];
        out[..n].copy_from_slice(&left[1..1 + n]);
        out[n] = n as u8;
        return (out, n + 1);
    }

    /// Incremental reference sponge built directly on `p1600_soft`.
    pub(crate) struct RefSponge<const ROUNDS: usize> {
        state: [u64; 25],
        rate: usize,
        delimiter: u8,
        pos: usize,
        squeezing: bool,
    }

    impl<const ROUNDS: usize> RefSponge<ROUNDS> {
        pub(crate) fn new(rate: usize, delimiter: u8) -> Self {
            assert!(rate > 0 && rate < 200);
            return RefSponge {
                state: [0u64; 25],
                rate,
                delimiter,
                pos: 0,
                squeezing: false,
            };
        }

        pub(crate) fn absorb(&mut self, data: &[u8]) {
            assert!(!self.squeezing);
            for &byte in data {
                self.state[self.pos / 8] ^= (byte as u64) << (8 * (self.pos % 8));
                self.pos += 1;
                if self.pos == self.rate {
                    p1600_soft::<ROUNDS>(&mut self.state);
                    self.pos = 0;
                }
            }
        }

        /// Pads and starts squeezing.
        pub(crate) fn finish_absorb(&mut self) {
            if self.squeezing {
                return;
            }
            self.state[self.pos / 8] ^= (self.delimiter as u64) << (8 * (self.pos % 8));
            let last = self.rate - 1;
            self.state[last / 8] ^= 0x80u64 << (8 * (last % 8));
            p1600_soft::<ROUNDS>(&mut self.state);
            self.pos = 0;
            self.squeezing = true;
        }

        pub(crate) fn squeeze(&mut self, out: &mut [u8]) {
            self.finish_absorb();
            let mut written = 0usize;
            while written < out.len() {
                if self.pos == self.rate {
                    p1600_soft::<ROUNDS>(&mut self.state);
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

    /// One-shot sponge with `ROUNDS` Keccak-p rounds.
    pub(crate) fn sponge<const ROUNDS: usize>(data: &[u8], rate: usize, delimiter: u8, out: &mut [u8]) {
        let mut sponge = RefSponge::<ROUNDS>::new(rate, delimiter);
        sponge.absorb(data);
        sponge.squeeze(out);
    }

    /// One-shot cSHAKE256 reference.
    pub(crate) fn cshake256(data: &[u8], function_name: &[u8], customization: &[u8], out: &mut [u8]) {
        if function_name.is_empty() && customization.is_empty() {
            sponge::<24>(data, 136, 0x1f, out);
            return;
        }

        let mut sponge = RefSponge::<24>::new(136, 0x04);

        // bytepad(encode_string(N) || encode_string(S), 136)
        let (w, w_len) = left_encode(136);
        sponge.absorb(&w[..w_len]);
        let (n_bits, n_bits_len) = left_encode(function_name.len() * 8);
        sponge.absorb(&n_bits[..n_bits_len]);
        sponge.absorb(function_name);
        let (s_bits, s_bits_len) = left_encode(customization.len() * 8);
        sponge.absorb(&s_bits[..s_bits_len]);
        sponge.absorb(customization);

        let total = w_len + n_bits_len + function_name.len() + s_bits_len + customization.len();
        let pad = (136 - (total % 136)) % 136;
        sponge.absorb(&[0u8; 136][..pad]);

        sponge.absorb(data);
        sponge.squeeze(out);
    }
}

#[cfg(test)]
mod tests {
    use super::{p1600, p1600_soft};

    fn keccak_f(permute: impl Fn(&mut [u64; 25]), state_first: [u64; 25], state_second: [u64; 25]) {
        let mut state = [0u64; 25];

        permute(&mut state);
        assert_eq!(state, state_first);

        permute(&mut state);
        assert_eq!(state, state_second);
    }

    // Test vectors are copied from XKCP (eXtended Keccak Code Package)
    // https://github.com/XKCP/XKCP/blob/master/tests/TestVectors/KeccakF-1600-IntermediateValues.txt
    fn keccak_f1600_vectors() -> ([u64; 25], [u64; 25]) {
        let state_first = [
            0xF1258F7940E1DDE7,
            0x84D5CCF933C0478A,
            0xD598261EA65AA9EE,
            0xBD1547306F80494D,
            0x8B284E056253D057,
            0xFF97A42D7F8E6FD4,
            0x90FEE5A0A44647C4,
            0x8C5BDA0CD6192E76,
            0xAD30A6F71B19059C,
            0x30935AB7D08FFC64,
            0xEB5AA93F2317D635,
            0xA9A6E6260D712103,
            0x81A57C16DBCF555F,
            0x43B831CD0347C826,
            0x01F22F1A11A5569F,
            0x05E5635A21D9AE61,
            0x64BEFEF28CC970F2,
            0x613670957BC46611,
            0xB87C5A554FD00ECB,
            0x8C3EE88A1CCF32C8,
            0x940C7922AE3A2614,
            0x1841F924A2C509E4,
            0x16F53526E70465C2,
            0x75F644E97F30A13B,
            0xEAF1FF7B5CECA249,
        ];
        let state_second = [
            0x2D5C954DF96ECB3C,
            0x6A332CD07057B56D,
            0x093D8D1270D76B6C,
            0x8A20D9B25569D094,
            0x4F9C4F99E5E7F156,
            0xF957B9A2DA65FB38,
            0x85773DAE1275AF0D,
            0xFAF4F247C3D810F7,
            0x1F1B9EE6F79A8759,
            0xE4FECC0FEE98B425,
            0x68CE61B6B9CE68A1,
            0xDEEA66C4BA8F974F,
            0x33C43D836EAFB1F5,
            0xE00654042719DBD9,
            0x7CF8A9F009831265,
            0xFD5449A6BF174743,
            0x97DDAD33D8994B40,
            0x48EAD5FC5D0BE774,
            0xE3B8C8EE55B7B03C,
            0x91A0226E649E42E9,
            0x900E3129E7BADD7B,
            0x202A9EC5FAA3CCE8,
            0x5B3402464E1C3DB6,
            0x609F4E62A44C1059,
            0x20D06CD26A8FBF5C,
        ];

        (state_first, state_second)
    }

    #[test]
    fn keccak_f1600() {
        let (state_first, state_second) = keccak_f1600_vectors();
        keccak_f(p1600::<24>, state_first, state_second);
    }

    #[test]
    fn keccak_f1600_soft() {
        let (state_first, state_second) = keccak_f1600_vectors();
        keccak_f(p1600_soft::<24>, state_first, state_second);
    }

    #[test]
    #[should_panic(expected = "the Keccak rate must be in 1..200 bytes")]
    fn sponge_rejects_zero_rate() {
        let _ = super::KeccakSponge::<24>::new(0, 0x06);
    }

    #[test]
    #[should_panic(expected = "the Keccak rate must be in 1..200 bytes")]
    fn sponge_rejects_oversized_rate() {
        let _ = super::KeccakSponge::<24>::new(201, 0x06);
    }

    /// Deterministic pseudo-random state, adequate for differential tests.
    fn random_state(seed: u64) -> [u64; 25] {
        let mut rng = seed;
        let mut state = [0u64; 25];
        for word in state.iter_mut() {
            rng ^= rng >> 12;
            rng ^= rng << 25;
            rng ^= rng >> 27;
            *word = rng.wrapping_mul(0x2545_f491_4f6c_dd1d);
        }
        return state;
    }

    /// The FEAT_SHA3 permutation must agree with the portable one for every
    /// round count, not just for the 24 rounds covered by the XKCP vectors.
    #[cfg(all(target_arch = "aarch64", feature = "std"))]
    #[test]
    fn keccak_f1600_armv8_matches_soft() {
        use crate::sha3::keccak_arm64::p1600_armv8;

        if !std::arch::is_aarch64_feature_detected!("sha3") {
            return;
        }

        let state = random_state(0x0123_4567_89ab_cdef);
        macro_rules! check {
            ($($rounds:literal),* $(,)?) => {
                $({
                    let mut hardware = state;
                    let mut portable = state;
                    // SAFETY: FEAT_SHA3 was detected above.
                    unsafe { p1600_armv8::<$rounds>(&mut hardware) };
                    p1600_soft::<$rounds>(&mut portable);
                    assert_eq!(hardware, portable, "ROUNDS = {}", $rounds);
                })*
            };
        }

        check!(
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        );
    }

    /// The single-state AVX2 permutation must agree with the portable one for
    /// every round count.
    #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx2")))]
    #[test]
    fn keccak_f1600_avx2_matches_soft() {
        use crate::sha3::keccak_amd64::p1600_avx2;

        #[cfg(feature = "std")]
        if !std::arch::is_x86_feature_detected!("avx2") {
            return;
        }

        let state = random_state(0x0123_4567_89ab_cdef);
        macro_rules! check {
            ($($rounds:literal),* $(,)?) => {
                $({
                    let mut hardware = state;
                    let mut portable = state;
                    // SAFETY: AVX2 was detected above.
                    unsafe { p1600_avx2::<$rounds>(&mut hardware) };
                    p1600_soft::<$rounds>(&mut portable);
                    assert_eq!(hardware, portable, "ROUNDS = {}", $rounds);
                })*
            };
        }

        check!(
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        );
    }

    /// The single-state AVX-512 permutation must agree with the portable one
    /// for every round count.
    #[cfg(all(target_arch = "x86_64", any(feature = "std", target_feature = "avx512f")))]
    #[test]
    fn keccak_f1600_avx512_matches_soft() {
        use crate::sha3::keccak_amd64::p1600_avx512;

        #[cfg(feature = "std")]
        if !std::arch::is_x86_feature_detected!("avx512f") {
            return;
        }

        let state = random_state(0x0123_4567_89ab_cdef);
        macro_rules! check {
            ($($rounds:literal),* $(,)?) => {
                $({
                    let mut hardware = state;
                    let mut portable = state;
                    // SAFETY: AVX-512F was detected above.
                    unsafe { p1600_avx512::<$rounds>(&mut hardware) };
                    p1600_soft::<$rounds>(&mut portable);
                    assert_eq!(hardware, portable, "ROUNDS = {}", $rounds);
                })*
            };
        }

        check!(
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        );
    }

    /// The backend selected by [`super::PermuteBackend::detect`] must agree with
    /// the portable implementation for every round count.
    #[test]
    fn dispatched_backend_matches_soft() {
        let backend = super::PermuteBackend::detect();
        let state = random_state(0x0123_4567_89ab_cdef);
        macro_rules! check {
            ($($rounds:literal),* $(,)?) => {
                $({
                    let mut hardware = state;
                    let mut portable = state;
                    super::p1600_with::<$rounds>(&mut hardware, backend);
                    p1600_soft::<$rounds>(&mut portable);
                    assert_eq!(hardware, portable, "ROUNDS = {}", $rounds);
                })*
            };
        }

        check!(
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        );
    }

    /// The round constants are generated by the FIPS 202 LFSR, not copied from
    /// a table. This catches a typo even if all backends share the same table.
    #[test]
    fn round_constants_match_lfsr() {
        fn rc(t: usize) -> u64 {
            let t = t % 255;
            if t == 0 {
                return 1;
            }
            let mut r: u16 = 1;
            for _ in 1..=t {
                r <<= 1;
                if r & 0x100 != 0 {
                    r ^= 0x171;
                }
            }
            return (r & 1) as u64;
        }

        for (i, &expected) in super::ROUND_CONSTANTS.iter().enumerate() {
            let mut actual = 0u64;
            for j in 0..7 {
                if rc(7 * i + j) != 0 {
                    actual |= 1 << ((1 << j) - 1);
                }
            }
            assert_eq!(actual, expected, "RC[{i}]");
        }
    }

    /// The rho rotation offsets and pi lane mapping are generated by the
    /// FIPS 202 formulas. This pins `RHO`/`PI` independently of the table.
    #[test]
    fn rho_pi_tables_match_formulas() {
        let mut x = 1usize;
        let mut y = 0usize;
        for t in 0..24 {
            let expected_rotation = ((t + 1) * (t + 2) / 2) % 64;
            assert_eq!(super::RHO[t] as usize, expected_rotation, "RHO[{t}]");

            let expected_destination = y + 5 * ((2 * x + 3 * y) % 5);
            assert_eq!(super::PI[t], expected_destination, "PI[{t}]");

            let next_x = y;
            let next_y = (2 * x + 3 * y) % 5;
            x = next_x;
            y = next_y;
        }
    }

    /// The byte-serialized permutation must use the FIPS 202 little-endian
    /// lane convention on every host.
    #[test]
    fn state_byte_conversion_is_little_endian() {
        let (state_first, state_second) = keccak_f1600_vectors();

        let mut bytes = [0u8; 200];
        super::store_state_le(&state_first, &mut bytes);
        let lanes = super::load_state_le(&bytes);
        assert_eq!(lanes, state_first, "load/store round-trip");

        super::permute_state_bytes::<24>(&mut bytes, super::PermuteBackend::Soft);
        let mut expected = [0u8; 200];
        super::store_state_le(&state_second, &mut expected);
        assert_eq!(bytes, expected, "serialized Keccak-f[1600] output");
    }

    fn sponge_reference<const ROUNDS: usize>(data: &[u8], rate: usize, delimiter: u8, out: &mut [u8]) {
        super::test_util::sponge::<ROUNDS>(data, rate, delimiter, out);
    }

    /// Directly differential-tests the sponge construction, including rates
    /// that are not a multiple of eight and both padding bytes landing in the
    /// same position.
    #[test]
    fn sponge_matches_reference_across_rates_and_boundaries() {
        let rates = [1usize, 7, 8, 71, 72, 135, 136, 137, 167, 168, 199];
        let delimiters = [0x00u8, 0x01, 0x06, 0x07, 0x0b, 0x1f, 0x7f, 0x80, 0xff];

        for &rate in &rates {
            let lengths = [
                0usize,
                1,
                rate.saturating_sub(2),
                rate - 1,
                rate,
                rate + 1,
                2 * rate - 1,
                2 * rate,
                2 * rate + 1,
            ];
            let output_len = 2 * rate + 7;

            for &delimiter in &delimiters {
                for &len in &lengths {
                    let data: Vec<u8> = (0..len).map(|i| (i % 251) as u8).collect();
                    let mut expected = vec![0u8; output_len];
                    sponge_reference::<24>(&data, rate, delimiter, &mut expected);

                    // One-shot.
                    let mut sponge = super::KeccakSponge::<24>::new(rate, delimiter);
                    sponge.absorb(&data);
                    let mut actual = vec![0u8; output_len];
                    sponge.squeeze(&mut actual);
                    assert_eq!(actual, expected, "one-shot rate={rate} delimiter={delimiter:#04x} len={len}");

                    // Chunked absorb + chunked squeeze.
                    let chunks = [1usize, 2, 3, 7, rate.saturating_sub(1).max(1), rate, rate + 1];
                    let squeeze_chunks = [1usize, 7, rate.saturating_sub(1).max(1), rate, rate + 1];
                    for &chunk in &chunks {
                        let mut sponge = super::KeccakSponge::<24>::new(rate, delimiter);
                        for part in data.chunks(chunk) {
                            sponge.absorb(part);
                        }
                        let mut actual = vec![0u8; output_len];
                        let mut offset = 0usize;
                        let mut squeeze_index = 0usize;
                        while offset < actual.len() {
                            let squeeze_chunk = squeeze_chunks[squeeze_index % squeeze_chunks.len()];
                            let take = core::cmp::min(squeeze_chunk, actual.len() - offset);
                            sponge.squeeze(&mut actual[offset..offset + take]);
                            offset += take;
                            squeeze_index += 1;
                        }
                        assert_eq!(
                            actual, expected,
                            "chunked rate={rate} delimiter={delimiter:#04x} len={len} chunk={chunk}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn sponge_zero_length_output_then_squeeze() {
        let rate = 136usize;
        let delimiter = 0x1fu8;
        let data: Vec<u8> = (0..300).map(|i| (i % 251) as u8).collect();

        let mut expected = vec![0u8; 2 * rate + 3];
        sponge_reference::<24>(&data, rate, delimiter, &mut expected);

        let mut sponge = super::KeccakSponge::<24>::new(rate, delimiter);
        sponge.absorb(&data);
        sponge.squeeze(&mut []);
        let mut actual = vec![0u8; expected.len()];
        sponge.squeeze(&mut actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sponge_delimiter_switch_matches_fresh_sponge() {
        for &len in &[0usize, 1, 135, 136, 137, 500] {
            let data: Vec<u8> = (0..len).map(|i| (i % 251) as u8).collect();

            let mut switched = super::KeccakSponge::<24>::new(136, 0x06);
            switched.absorb(&data);
            switched.set_delimiter(0x07);
            let mut a = [0u8; 64];
            switched.squeeze(&mut a);

            let mut fresh = super::KeccakSponge::<24>::new(136, 0x07);
            fresh.absorb(&data);
            let mut b = [0u8; 64];
            fresh.squeeze(&mut b);

            assert_eq!(a, b, "len={len}");
        }
    }

    #[test]
    #[should_panic(expected = "absorb can't be called after squeezing")]
    fn sponge_rejects_absorb_after_squeeze() {
        let mut sponge = super::KeccakSponge::<24>::new(136, 0x1f);
        sponge.absorb(b"prefix");
        sponge.squeeze(&mut [0u8; 1]);
        sponge.absorb(b"suffix");
    }

    #[test]
    #[should_panic(expected = "the delimiter can only be changed while absorbing")]
    fn sponge_rejects_delimiter_change_after_squeeze() {
        let mut sponge = super::KeccakSponge::<24>::new(136, 0x1f);
        sponge.absorb(b"prefix");
        sponge.squeeze(&mut [0u8; 1]);
        sponge.set_delimiter(0x06);
    }
}
