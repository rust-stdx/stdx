/// HChaCha20 is a hash function derived from ChaCha20.
///
/// It takes a 256-bit key and a 128-bit input (nonce) and produces a 256-bit
/// hash. It is used internally by XChaCha20 to derive a subkey.
///
/// # Algorithm
///
/// 1. Initialize a ChaCha20 state with the constant, key, and 128-bit nonce
///    (placed in the counter and nonce positions).
/// 2. Perform 20 rounds (10 double rounds) of ChaCha quarter rounds.
/// 3. Output words 0-3 and 12-15 of the final state, serialized as
///    little-endian bytes (without adding the initial state back).
pub fn hchacha20(key: &[u8; 32], input: &[u8; 16]) -> [u8; 32] {
    use super::{CONSTANT, STATE_WORDS, quarter_round};

    let mut state = [0u32; STATE_WORDS];

    state[..4].copy_from_slice(&CONSTANT);

    for (state_word, key_chunk) in state[4..12].iter_mut().zip(key.as_chunks::<4>().0) {
        *state_word = u32::from_le_bytes(*key_chunk);
    }

    state[12] = u32::from_le_bytes(input[0..4].try_into().unwrap());
    state[13] = u32::from_le_bytes(input[4..8].try_into().unwrap());
    state[14] = u32::from_le_bytes(input[8..12].try_into().unwrap());
    state[15] = u32::from_le_bytes(input[12..16].try_into().unwrap());

    for _ in 0..10 {
        quarter_round(&mut state, 0, 4, 8, 12);
        quarter_round(&mut state, 1, 5, 9, 13);
        quarter_round(&mut state, 2, 6, 10, 14);
        quarter_round(&mut state, 3, 7, 11, 15);

        quarter_round(&mut state, 0, 5, 10, 15);
        quarter_round(&mut state, 1, 6, 11, 12);
        quarter_round(&mut state, 2, 7, 8, 13);
        quarter_round(&mut state, 3, 4, 9, 14);
    }

    let mut output = [0u8; 32];
    for i in 0..4 {
        output[i * 4..(i + 1) * 4].copy_from_slice(&state[i].to_le_bytes());
    }
    for i in 0..4 {
        output[16 + i * 4..16 + (i + 1) * 4].copy_from_slice(&state[12 + i].to_le_bytes());
    }

    return output;
}

#[cfg(test)]
mod test {
    use super::hchacha20;

    /// Test vector from draft-irtf-cfrg-xchacha-03, Section 2.2.1.
    #[test]
    fn hchacha20_test_vector_1() {
        let key: [u8; 32] =
            hex::decode_array(b"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f").unwrap();
        let input: [u8; 16] = hex::decode_array(b"000000090000004a0000000031415927").unwrap();
        let expected: [u8; 32] =
            hex::decode_array(b"82413b4227b27bfed30e42508a877d73a0f9e4d58a74a853c12ec41326d3ecdc").unwrap();

        assert_eq!(hchacha20(&key, &input), expected);
    }

    /// Test with all-zero key and input.
    #[test]
    fn hchacha20_all_zeros() {
        let key = [0u8; 32];
        let input = [0u8; 16];
        let expected: [u8; 32] =
            hex::decode_array(b"1140704c328d1d5d0e30086cdf209dbd6a43b8f41518a11cc387b669b2ee6586").unwrap();

        assert_eq!(hchacha20(&key, &input), expected);
    }
}
