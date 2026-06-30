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
pub(crate) enum SpongeMode {
    Absorbing,
    Squeezing,
}

/// A sponge construction based on Keccak p1600
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
#[repr(align(8))]
// the struct requires 8-byte alignment for `ptr::copy_nonoverlapping` UB check on wasm32:
// `state` is transmuted to
// `&mut [u64; 25]` in `permute_and_reset_pos`, which needs 8-byte alignment.
pub(crate) struct Keccak<const ROUNDS: usize> {
    state: [u8; 200],
    rate: usize,
    padding: u8,
    /// the current position in the state buffer
    pos: usize,
    mode: SpongeMode,
}

impl<const ROUNDS: usize> Keccak<ROUNDS> {
    #[inline]
    pub(crate) fn new(rate: usize, delimiter: u8) -> Self {
        debug_assert!(rate > 0 && rate < 200);
        return Keccak {
            state: [0u8; 200],
            rate,
            padding: delimiter,
            pos: 0,
            mode: SpongeMode::Absorbing,
        };
    }

    #[inline]
    pub(crate) fn absorb(&mut self, data: &[u8]) {
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
    fn absorb_chunk(&mut self, chunk: &[u8]) {
        // xor(&mut self.state[self.pos..RATE], &chunk);
        xor(&mut self.state[self.pos..self.pos + chunk.len()], &chunk);
        self.pos += chunk.len();

        // if the sponge is full, apply the permutation
        if self.pos == self.rate {
            self.permute_and_reset_pos();
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
        for mut chunk in out[rate_remainder..].chunks_mut(self.rate) {
            self.squeeze_chunk(&mut chunk);
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
        // this is totally safe as long as state.len() == 200 and state remains a [u8]. We are just
        // playing with the memory representation of the array, from [u8] to [u64].
        let mut state: &mut [u64; 200 / 8] = unsafe { core::mem::transmute(&mut self.state) };
        p1600::<ROUNDS>(&mut state);
        self.pos = 0;
    }
}

/// The Keccak-p sponge function for a 1600-bit state
#[allow(unreachable_code)]
pub fn p1600<const ROUNDS: usize>(state: &mut [u64; 25]) {
    const {
        assert!(ROUNDS <= 24, "A round_count greater than 24 is not supported.");
    }

    // we assume that the SHA-3 instructions are always preseent for aarch64
    #[cfg(target_arch = "aarch64")]
    unsafe {
        super::keccak_arm64::p1600_armv8::<ROUNDS>(state);
        return;
    }

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
pub fn xor(dest: &mut [u8], source: &[u8]) {
    dest.iter_mut()
        .zip(source.iter())
        .for_each(|(dest, source)| *dest ^= *source);
}

#[cfg(test)]
mod tests {
    use super::p1600;

    fn keccak_f(state_first: [u64; 25], state_second: [u64; 25]) {
        let mut state = [0u64; 25];

        p1600::<24>(&mut state);
        assert_eq!(state, state_first);

        p1600::<24>(&mut state);
        assert_eq!(state, state_second);
    }

    #[test]
    fn keccak_f1600() {
        // Test vectors are copied from XKCP (eXtended Keccak Code Package)
        // https://github.com/XKCP/XKCP/blob/master/tests/TestVectors/KeccakF-1600-IntermediateValues.txt
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

        keccak_f(state_first, state_second);
    }
}
