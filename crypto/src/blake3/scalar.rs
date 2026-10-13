//! Portable (scalar) BLAKE3 kernels.
//!
//! This is the fallback used on targets without a SIMD backend, and it is the
//! code path that 32-bit platforms rely on. It is written with 32-bit words
//! throughout, so it maps directly onto 32-bit registers: the 64-bit counter is
//! split into two 32-bit halves only where the compression function needs it,
//! and every arithmetic operation is an explicit `wrapping_*` so that debug
//! builds cannot panic on overflow.

use super::platform::{BLOCK_LEN, IV, IncrementCounter, MSG_SCHEDULE, OUT_LEN, counter_high, counter_low};

/// The BLAKE3 mixing function, G.
#[inline(always)]
fn g(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize, mx: u32, my: u32) {
    state[a] = state[a].wrapping_add(state[b]).wrapping_add(mx);
    state[d] = (state[d] ^ state[a]).rotate_right(16);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] = (state[b] ^ state[c]).rotate_right(12);
    state[a] = state[a].wrapping_add(state[b]).wrapping_add(my);
    state[d] = (state[d] ^ state[a]).rotate_right(8);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] = (state[b] ^ state[c]).rotate_right(7);
}

/// One round, mixing the four columns and then the four diagonals.
///
/// `r` is always a literal at the call sites below, so `MSG_SCHEDULE[r]`
/// constant-folds and the whole function is fully unrolled.
#[inline(always)]
fn round(state: &mut [u32; 16], msg: &[u32; 16], r: usize) {
    let s = &MSG_SCHEDULE[r];

    g(state, 0, 4, 8, 12, msg[s[0]], msg[s[1]]);
    g(state, 1, 5, 9, 13, msg[s[2]], msg[s[3]]);
    g(state, 2, 6, 10, 14, msg[s[4]], msg[s[5]]);
    g(state, 3, 7, 11, 15, msg[s[6]], msg[s[7]]);

    g(state, 0, 5, 10, 15, msg[s[8]], msg[s[9]]);
    g(state, 1, 6, 11, 12, msg[s[10]], msg[s[11]]);
    g(state, 2, 7, 8, 13, msg[s[12]], msg[s[13]]);
    g(state, 3, 4, 9, 14, msg[s[14]], msg[s[15]]);
}

#[inline(always)]
fn words_from_le_bytes_64(block: &[u8; BLOCK_LEN]) -> [u32; 16] {
    let mut words = [0u32; 16];
    for (word, chunk) in words.iter_mut().zip(block.as_chunks::<4>().0) {
        *word = u32::from_le_bytes(*chunk);
    }
    words
}

#[inline(always)]
fn compress_pre(cv: &[u32; 8], block: &[u8; BLOCK_LEN], block_len: u8, counter: u64, flags: u8) -> [u32; 16] {
    let msg = words_from_le_bytes_64(block);

    #[rustfmt::skip]
    let mut state = [
        cv[0], cv[1], cv[2], cv[3], cv[4], cv[5], cv[6], cv[7],
        IV[0], IV[1], IV[2], IV[3],
        counter_low(counter), counter_high(counter), block_len as u32, flags as u32,
    ];

    round(&mut state, &msg, 0);
    round(&mut state, &msg, 1);
    round(&mut state, &msg, 2);
    round(&mut state, &msg, 3);
    round(&mut state, &msg, 4);
    round(&mut state, &msg, 5);
    round(&mut state, &msg, 6);

    state
}

#[inline]
pub(crate) fn compress_in_place(cv: &mut [u32; 8], block: &[u8; BLOCK_LEN], block_len: u8, counter: u64, flags: u8) {
    let state = compress_pre(cv, block, block_len, counter, flags);
    for i in 0..8 {
        cv[i] = state[i] ^ state[i + 8];
    }
}

#[inline]
pub(crate) fn compress_xof(cv: &[u32; 8], block: &[u8; BLOCK_LEN], block_len: u8, counter: u64, flags: u8) -> [u8; 64] {
    let mut state = compress_pre(cv, block, block_len, counter, flags);
    for i in 0..8 {
        state[i] ^= state[i + 8];
        state[i + 8] ^= cv[i];
    }

    let mut out = [0u8; 64];
    for (word, chunk) in state.iter().zip(out.as_chunks_mut::<4>().0) {
        chunk.copy_from_slice(&word.to_le_bytes());
    }
    out
}

/// Hash one input (a whole number of full blocks) into a chaining value.
fn hash1(input: &[u8], key: &[u32; 8], counter: u64, flags: u8, flags_start: u8, flags_end: u8, out: &mut [u8]) {
    debug_assert_eq!(input.len() % BLOCK_LEN, 0, "uneven blocks");
    debug_assert!(out.len() >= OUT_LEN);

    let mut cv = *key;
    let mut slice = input;
    let mut block_flags = flags | flags_start;
    while !slice.is_empty() {
        if slice.len() == BLOCK_LEN {
            block_flags |= flags_end;
        }
        let block: &[u8; BLOCK_LEN] = slice[..BLOCK_LEN].try_into().unwrap();
        compress_in_place(&mut cv, block, BLOCK_LEN as u8, counter, block_flags);
        block_flags = flags;
        slice = &slice[BLOCK_LEN..];
    }

    for (word, chunk) in cv.iter().zip(out.as_chunks_mut::<4>().0) {
        chunk.copy_from_slice(&word.to_le_bytes());
    }
}

/// Portable `hash_many`: hashes each input independently.
pub(crate) fn hash_many(
    inputs: &[&[u8]],
    key: &[u32; 8],
    mut counter: u64,
    increment_counter: IncrementCounter,
    flags: u8,
    flags_start: u8,
    flags_end: u8,
    out: &mut [u8],
) {
    for (&input, output) in inputs.iter().zip(out.as_chunks_mut::<OUT_LEN>().0) {
        hash1(input, key, counter, flags, flags_start, flags_end, output);
        if increment_counter.yes() {
            counter = counter.wrapping_add(1);
        }
    }
}
