#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::StreamCipher;

/// The number of 32-bit words that compose ChaCha's state.
pub(crate) const STATE_WORDS: usize = 16;

/// The size of a ChaCha block in bytes which is the size of the state in bytes
pub(crate) const BLOCK_SIZE: usize = 64;

/// The "sigma" constant which is the value of the first row of ChaCha's state.
pub(crate) const CONSTANT: [u32; 4] = [
    0x61707865, // "expa"
    0x3320646e, // "nd 3"
    0x79622d32, // "2-by"
    0x6b206574, // "te k"
];

pub type ChaCha8Djb = ChaCha<8, false>;
pub type ChaCha12Djb = ChaCha<12, false>;
pub type ChaCha20Djb = ChaCha<20, false>;
pub type ChaCha20Ietf = ChaCha<20, true>;
pub type XChaCha20 = XChaCha<20>;

/// ChaCha stream cipher.
///
/// `IS_IETF` selects the nonce/counter layout:
/// - `false` (DJB original): 64-bit counter at words 12–13, 64-bit nonce at words 14–15.
/// - `true` (IETF / RFC 8439): 32-bit counter at word 12, 96-bit nonce at words 13–15.
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct ChaCha<const ROUNDS: usize, const IS_IETF: bool> {
    state: [u32; STATE_WORDS],
    /// ChaCha is a stream cipher that works with 64-byte blocks.
    /// It means that consumers of this packages should be able to call `xor_keystream` multiple
    /// times even if there input is not aligned with ChaCha blocks.
    /// Thus calling multiple times `xor_keystream`:
    /// xor_keystream(plaintext[0..3]), xor_keystream(plaintext[3..50]), xor_keystream(plaintext[50..150]);
    /// Should be equal to calling it only once:
    /// xor_keystream(plaintext[0..150]);
    /// For that, we keep the last computed keystream block, as well as an offset indicating where
    /// the unconsumed tail starts.
    /// The full leftover block is stored in `keystream_leftover` minus 1 byte because if there is leftover
    /// it means that the leftover is <= (BLOCK_SIZE - 1).
    /// When `keystream_leftover_offset == (BLOCK_SIZE - 1)`, there is no leftover (empty slice).
    /// NOTE: the `keystream_leftover` buffer is valid only if the previous call to `xor_keystream` had
    /// an `input.len() % 64 != 0`, Otherwise there is no need to preserve the last keystream block.
    keystream_leftover: [u8; BLOCK_SIZE - 1],
    keystream_leftover_offset: u8,
}

impl<const ROUNDS: usize, const IS_IETF: bool> ChaCha<ROUNDS, IS_IETF> {
    #[inline(always)]
    fn extract_counter(state: &[u32; STATE_WORDS]) -> u64 {
        if IS_IETF {
            state[12] as u64
        } else {
            ((state[13] as u64) << 32) | (state[12] as u64)
        }
    }

    #[inline(always)]
    fn inject_counter(state: &mut [u32; STATE_WORDS], counter: u64) {
        state[12] = counter as u32;
        if !IS_IETF {
            state[13] = (counter >> 32) as u32;
        }
    }
}

impl<const ROUNDS: usize> ChaCha<ROUNDS, false> {
    /// Create a new ChaCha instance with the DJB nonce layout (8-byte nonce, 64-bit counter).
    pub fn new(key: &[u8; 32], nonce: &[u8; 8]) -> ChaCha<ROUNDS, false> {
        let mut state = [0u32; STATE_WORDS];

        state[..4].copy_from_slice(&CONSTANT);

        for (state_word, key_chunk) in state[4..12].iter_mut().zip(key.chunks_exact(4)) {
            *state_word = u32::from_le_bytes(key_chunk.try_into().unwrap());
        }

        state[14] = u32::from_le_bytes(nonce[0..4].try_into().unwrap());
        state[15] = u32::from_le_bytes(nonce[4..8].try_into().unwrap());

        return ChaCha {
            state,
            keystream_leftover: [0u8; BLOCK_SIZE - 1],
            keystream_leftover_offset: (BLOCK_SIZE - 1) as u8,
        };
    }

    /// Set the ChaCha counter (words 12 and 13). It can be used to move forward and backward in the
    /// keystream.
    #[inline(always)]
    pub fn set_counter(&mut self, counter: u64) {
        Self::inject_counter(&mut self.state, counter);
        self.keystream_leftover_offset = (BLOCK_SIZE - 1) as u8;
    }
}

impl<const ROUNDS: usize> ChaCha<ROUNDS, true> {
    /// Create a new ChaCha instance with the IETF (RFC 8439) nonce layout (12-byte nonce, 32-bit counter).
    pub fn new(key: &[u8; 32], nonce: &[u8; 12]) -> ChaCha<ROUNDS, true> {
        let mut state = [0u32; STATE_WORDS];

        state[..4].copy_from_slice(&CONSTANT);

        for (state_word, key_chunk) in state[4..12].iter_mut().zip(key.chunks_exact(4)) {
            *state_word = u32::from_le_bytes(key_chunk.try_into().unwrap());
        }

        state[13] = u32::from_le_bytes(nonce[0..4].try_into().unwrap());
        state[14] = u32::from_le_bytes(nonce[4..8].try_into().unwrap());
        state[15] = u32::from_le_bytes(nonce[8..12].try_into().unwrap());

        return ChaCha {
            state,
            keystream_leftover: [0u8; BLOCK_SIZE - 1],
            keystream_leftover_offset: (BLOCK_SIZE - 1) as u8,
        };
    }

    /// Set the ChaCha counter (word 12). The counter is a u32. It can be used to move forward
    /// and backward in the keystream.
    #[inline(always)]
    pub fn set_counter(&mut self, counter: u32) {
        Self::inject_counter(&mut self.state, counter as u64);
        self.keystream_leftover_offset = (BLOCK_SIZE - 1) as u8;
    }
}

impl<const ROUNDS: usize, const IS_IETF: bool> StreamCipher for ChaCha<ROUNDS, IS_IETF> {
    /// XOR `plaintext` with the ChaCha keystream.
    fn xor_keystream(&mut self, mut in_out: &mut [u8]) {
        if in_out.len() == 0 {
            return;
        }

        // first, consume the keystream leftover, if any
        if self.keystream_leftover_offset < (BLOCK_SIZE - 1) as u8 {
            let keystream_leftover = &self.keystream_leftover[(self.keystream_leftover_offset as usize)..];

            in_out
                .iter_mut()
                .zip(keystream_leftover)
                .for_each(|(plaintext, keystream)| *plaintext ^= *keystream);

            if in_out.len() > keystream_leftover.len() {
                in_out = &mut in_out[keystream_leftover.len()..];
            } else if in_out.len() < keystream_leftover.len() {
                self.keystream_leftover_offset += in_out.len() as u8;
                return;
            } else {
                // in_out.len() == keystream_leftover.len() -> in_out has consumed exactly all the
                // leftover keystream
                self.keystream_leftover_offset = (BLOCK_SIZE - 1) as u8;
                return;
            }
        }
        // at this point, we already know how many bytes of leftover there will be
        self.keystream_leftover_offset = ((in_out.len() + BLOCK_SIZE - 1) % BLOCK_SIZE) as u8;

        // aarch64 assumes that NEON is always available
        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        if in_out.len() >= 128 {
            use super::chacha_neon::chacha_neon;
            // SAFETY: the cfg attribute above ensures that the required CPU feature(s) are available
            unsafe {
                chacha_neon::<ROUNDS, IS_IETF>(&mut self.state, in_out, &mut self.keystream_leftover);
            }
            return;
        }

        // wasm32 only supports compile-time features detection
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        if in_out.len() >= 128 {
            use super::chacha_wasm_simd128::chacha_wasm_simd128;
            chacha_wasm_simd128::<ROUNDS, IS_IETF>(&mut self.state, in_out, &mut self.keystream_leftover);
            return;
        }

        // runtime detection of CPU features for x86 and x86_64 when the "std" feature is enabled
        #[cfg(feature = "std")]
        {
            #[cfg(target_arch = "x86_64")]
            if is_x86_feature_detected!("avx512f") && in_out.len() >= 128 {
                use super::chacha_avx512::chacha_avx512;
                unsafe {
                    chacha_avx512::<ROUNDS, IS_IETF>(&mut self.state, in_out, &mut self.keystream_leftover);
                }
                return;
            }

            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            if is_x86_feature_detected!("avx2") && in_out.len() >= 128 {
                use super::chacha_avx2::chacha_avx2;
                unsafe {
                    chacha_avx2::<ROUNDS, IS_IETF>(&mut self.state, in_out, &mut self.keystream_leftover);
                }
                return;
            }
        }

        // compile-time CPU features detection for x86 and x86_64 when the "std" feature is not enabled
        #[cfg(not(feature = "std"))]
        {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
            if in_out.len() >= 128 {
                use super::chacha_avx512::chacha_avx512;
                unsafe { chacha_avx512::<ROUNDS, IS_IETF>(&mut self.state, in_out, &mut self.keystream_leftover) };
                return;
            }

            #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"))]
            if in_out.len() >= 128 {
                use super::chacha_avx2::chacha_avx2;
                unsafe { chacha_avx2::<ROUNDS, IS_IETF>(&mut self.state, in_out, &mut self.keystream_leftover) };
                return;
            }
        }

        // fallback for when SIMD acceleration is not available
        chacha_generic::<ROUNDS, IS_IETF>(&mut self.state, &mut self.keystream_leftover, in_out);
    }
}

#[inline]
fn chacha_generic<const ROUNDS: usize, const IS_IETF: bool>(
    mut state: &mut [u32; STATE_WORDS],
    keystream_leftover: &mut [u8; BLOCK_SIZE - 1],
    plaintext: &mut [u8],
) {
    let mut keystream = [0u8; BLOCK_SIZE];
    let keystream_ptr = keystream.as_mut_ptr();
    let mut counter = ChaCha::<ROUNDS, IS_IETF>::extract_counter(state);

    // process the input by blocks of 64 bytes
    for plaintext_block in plaintext.chunks_mut(BLOCK_SIZE) {
        ChaCha::<ROUNDS, IS_IETF>::inject_counter(&mut state, counter);

        // prepare temporary (working) state
        let mut tmp_state = *state;

        // perform the ROUNDS / 2 double rounds e.g. 10 double rounds for ChaCha20
        for _ in 0..(ROUNDS / 2) {
            // column rounds
            quarter_round(&mut tmp_state, 0, 4, 8, 12);
            quarter_round(&mut tmp_state, 1, 5, 9, 13);
            quarter_round(&mut tmp_state, 2, 6, 10, 14);
            quarter_round(&mut tmp_state, 3, 7, 11, 15);

            // diagonal rounds
            quarter_round(&mut tmp_state, 0, 5, 10, 15);
            quarter_round(&mut tmp_state, 1, 6, 11, 12);
            quarter_round(&mut tmp_state, 2, 7, 8, 13);
            quarter_round(&mut tmp_state, 3, 4, 9, 14);
        }

        // add initial state to tmp_state to generate the keystream and "serialize" it to little endian
        // for (tmp_word, state_word) in tmp_state.iter_mut().zip(state.iter()) {
        //     *tmp_word = tmp_word.wrapping_add(*state_word).to_le();
        // }
        for word_index in 0..STATE_WORDS {
            // first we add the initial state to the working state to get the keystream
            tmp_state[word_index] = tmp_state[word_index].wrapping_add(state[word_index]);

            // then we serialize the keystream
            // SAFETY: this is safe because `tmp_state` and `keystream` both have fixed, known size.
            // We are just merely converting [u32; STATE_WORDS] to [u8; BLOCK_SIZE] with the correct
            // endianness.
            unsafe {
                core::ptr::copy_nonoverlapping(
                    tmp_state[word_index].to_le_bytes().as_ptr(),
                    keystream_ptr.add(word_index * 4),
                    4,
                );
            }
        }

        // XOR plaintext with keystream
        plaintext_block
            .iter_mut()
            .zip(keystream)
            .for_each(|(plaintext, keystream)| *plaintext ^= keystream);

        counter = counter.wrapping_add(1);
    }

    ChaCha::<ROUNDS, IS_IETF>::inject_counter(state, counter);

    if plaintext.len() % BLOCK_SIZE != 0 {
        // copy the last 63 bytes of the leftover keystream block
        keystream_leftover.copy_from_slice(&keystream[1..]);
    }
}

#[inline(always)]
pub(crate) const fn quarter_round(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    // a += b; d ^= a; d <<<= 16
    state[a] = state[a].wrapping_add(state[b]);
    state[d] ^= state[a];
    state[d] = state[d].rotate_left(16);

    // c += d; b ^= c; b <<<= 12
    state[c] = state[c].wrapping_add(state[d]);
    state[b] ^= state[c];
    state[b] = state[b].rotate_left(12);

    // a += b; d ^= a; d <<<= 8
    state[a] = state[a].wrapping_add(state[b]);
    state[d] ^= state[a];
    state[d] = state[d].rotate_left(8);

    // c += d; b ^= c; b <<<= 7
    state[c] = state[c].wrapping_add(state[d]);
    state[b] ^= state[c];
    state[b] = state[b].rotate_left(7);
}

/// XChaCha20 stream cipher with 24-byte (192-bit) nonce (draft-irtf-cfrg-xchacha-03).
///
/// Internally uses HChaCha20 to derive a subkey from the first 16 bytes of the nonce,
/// then uses the IETF ChaCha20 variant with the remaining 8 nonce bytes.
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct XChaCha<const ROUNDS: usize> {
    inner: ChaCha<ROUNDS, true>,
}

impl<const ROUNDS: usize> XChaCha<ROUNDS> {
    /// Creates a new XChaCha stream cipher from a 32-byte key and a 24-byte nonce.
    ///
    /// The first 16 bytes of the nonce are used with HChaCha20 to derive a subkey.
    /// The last 8 bytes of the nonce become the ChaCha20 IETF nonce (prefixed with 4 zero bytes).
    pub fn new(key: &[u8; 32], nonce: &[u8; 24]) -> XChaCha<ROUNDS> {
        let subkey = super::hchacha20(key, nonce[..16].try_into().unwrap());
        let mut ietf_nonce = [0u8; 12];
        ietf_nonce[4..12].copy_from_slice(&nonce[16..24]);
        return XChaCha {
            inner: ChaCha::<ROUNDS, true>::new(&subkey, &ietf_nonce),
        };
    }

    /// Sets the ChaCha20 block counter. Counter is a u32.
    pub fn set_counter(&mut self, counter: u32) {
        self.inner.set_counter(counter);
    }
}

impl<const ROUNDS: usize> StreamCipher for XChaCha<ROUNDS> {
    fn xor_keystream(&mut self, in_out: &mut [u8]) {
        self.inner.xor_keystream(in_out);
    }
}

#[cfg(test)]
mod test {
    use super::{ChaCha8Djb, ChaCha12Djb, ChaCha20Djb, ChaCha20Ietf, XChaCha20};
    use crate::StreamCipher;

    struct TestDjb {
        key: [u8; 32],
        nonce: [u8; 8],
        initial_counter: u64,
        plaintext: Vec<u8>,
        expected_ciphertext: Vec<u8>,
    }

    #[test]
    fn chacha20_test_vectors_djb() {
        let tests = vec![
            // https://www.rfc-editor.org/rfc/rfc8439#section-2.4.2
            TestDjb {
                key: hex::decode("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f")
                    .unwrap()
                    .try_into()
                    .unwrap(),
                nonce: hex::decode("0000004a00000000").unwrap().try_into().unwrap(),
                initial_counter: 1,
                plaintext: hex::decode(
                    "4c616469657320616e642047656e746c\
656d656e206f662074686520636c6173\
73206f66202739393a20496620492063\
6f756c64206f6666657220796f75206f\
6e6c79206f6e652074697020666f7220\
746865206675747572652c2073756e73\
637265656e20776f756c642062652069\
742e",
                )
                .unwrap(),
                expected_ciphertext: hex::decode(
                    "6e2e359a2568f98041ba0728dd0d6981\
e97e7aec1d4360c20a27afccfd9fae0b\
f91b65c5524733ab8f593dabcd62b357\
1639d624e65152ab8f530c359f0861d8\
07ca0dbf500d6a6156a38e088a22b65e\
52bc514d16ccf806818ce91ab7793736\
5af90bbf74a35be6b40b8eedf2785e42\
874d",
                )
                .unwrap(),
            },
            // https://www.rfc-editor.org/rfc/rfc8439#appendix-A.2 Test vector #1
            TestDjb {
                key: [0u8; 32],
                nonce: [0u8; 8],
                initial_counter: 0,
                plaintext: [0u8; 64].to_vec(),
                expected_ciphertext: hex::decode(
                    "76b8e0ada0f13d90405d6ae55386bd28\
bdd219b8a08ded1aa836efcc8b770dc7\
da41597c5157488d7724e03fb8d84a37\
6a43b8f41518a11cc387b669b2ee6586",
                )
                .unwrap(),
            },
            // https://www.rfc-editor.org/rfc/rfc8439#appendix-A.2 Test Vector #2
            TestDjb {
                key: hex::decode("0000000000000000000000000000000000000000000000000000000000000001")
                    .unwrap()
                    .try_into()
                    .unwrap(),
                nonce: hex::decode("0000000000000002").unwrap().try_into().unwrap(),
                initial_counter: 1,
                plaintext: hex::decode(
                    "416e79207375626d697373696f6e2074\
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
7373656420746f",
                )
                .unwrap(),
                expected_ciphertext: hex::decode(
                    "a3fbf07df3fa2fde4f376ca23e827370\
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
c4fd806c22f221",
                )
                .unwrap(),
            },
            // https://www.rfc-editor.org/rfc/rfc8439#appendix-A.2 Test Vector #3
            TestDjb {
                key: hex::decode("1c9240a5eb55d38af333888604f6b5f0473917c1402b80099dca5cbc207075c0")
                    .unwrap()
                    .try_into()
                    .unwrap(),
                nonce: hex::decode("0000000000000002").unwrap().try_into().unwrap(),
                initial_counter: 42,
                plaintext: hex::decode(
                    "2754776173206272696c6c69672c2061\
6e642074686520736c6974687920746f\
7665730a446964206779726520616e64\
2067696d626c6520696e207468652077\
6162653a0a416c6c206d696d73792077\
6572652074686520626f726f676f7665\
732c0a416e6420746865206d6f6d6520\
7261746873206f757467726162652e",
                )
                .unwrap(),
                expected_ciphertext: hex::decode(
                    "62e6347f95ed87a45ffae7426f27a1df\
5fb69110044c0d73118effa95b01e5cf\
166d3df2d721caf9b21e5fb14c616871\
fd84c54f9d65b283196c7fe4f60553eb\
f39c6402c42234e32a356b3e764312a6\
1a5532055716ead6962568f87d3f3f77\
04c6a8d1bcd1bf4d50d6154b6da731b1\
87b58dfd728afa36757a797ac188d1",
                )
                .unwrap(),
            },
        ];

        for (i, test) in tests.into_iter().enumerate() {
            let mut cipher = ChaCha20Djb::new(&test.key, &test.nonce);
            cipher.set_counter(test.initial_counter);

            let mut plaintext = test.plaintext.clone();
            cipher.xor_keystream(&mut plaintext);

            assert_eq!(
                plaintext,
                test.expected_ciphertext,
                "test [{i}] failed
Got ciphertext: {}
Expected ciphertext: {}",
                hex::encode(&plaintext),
                hex::encode(&test.expected_ciphertext),
            );

            let mut cipher = ChaCha20Djb::new(&test.key, &test.nonce);
            cipher.set_counter(test.initial_counter);
            cipher.xor_keystream(&mut plaintext);

            assert_eq!(
                plaintext,
                test.plaintext,
                "test [{i}] failed. Initial plaintext != decrypt(encrypt(plaintext))
Got: {}
Expected: {}",
                hex::encode(&plaintext),
                hex::encode(&test.plaintext),
            );

            // ensure that the encryption is correct even for plaintexts that are not % 64 (block size)
            // thus:
            // cipher.xor_keystream(plaintext[0..10])
            // cipher.xor_keystream(plaintext[10..30])
            // cipher.xor_keystream(plaintext[30..5])
            // should be equal to:
            // cipher.xor_keystream(plaintext[0..35])

            let mut cipher = ChaCha20Djb::new(&test.key, &test.nonce);
            cipher.xor_keystream(&mut plaintext);
            for n in 0..10 {
                let mut partial_plaintext: Vec<u8> = test.plaintext.clone();

                let mut cipher = ChaCha20Djb::new(&test.key, &test.nonce);
                cipher.xor_keystream(&mut partial_plaintext[..n]);
                cipher.xor_keystream(&mut partial_plaintext[n..]);

                assert_eq!(
                    plaintext,
                    partial_plaintext,
                    "test [{i}] failed. partial encryption is not valid for n = {n}
            Got: {}
            Expected: {}",
                    hex::encode(&partial_plaintext),
                    hex::encode(&plaintext),
                )
            }
        }
    }

    #[test]
    fn chacha20_keystream_leftover_multi_call() {
        // Regression: verifies that leftovers are correctly consumed across 3+
        // partial calls. The old length-based approach did not compact the array
        // after partial consumption, causing stale keystream reuse on subsequent calls.
        let key = hex::decode("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f")
            .unwrap()
            .try_into()
            .unwrap();
        let nonce = hex::decode("0000004a00000000").unwrap().try_into().unwrap();
        let plaintext = hex::decode(
            "4c616469657320616e642047656e746c\
656d656e206f662074686520636c6173\
73206f66202739393a20496620492063\
6f756c64206f6666657220796f75206f\
6e6c79206f6e652074697020666f7220\
746865206675747572652c2073756e73\
637265656e20776f756c642062652069\
742e",
        )
        .unwrap();

        let mut expected = plaintext.clone();
        ChaCha20Djb::new(&key, &nonce).xor_keystream(&mut expected);

        // call 1: partial block -> leaves leftover
        // call 2: partially consumes leftover
        // call 3: consumes remaining leftover + fresh blocks
        {
            let mut buf = plaintext.clone();
            let mut cipher = ChaCha20Djb::new(&key, &nonce);
            cipher.xor_keystream(&mut buf[..10]);
            cipher.xor_keystream(&mut buf[10..15]);
            cipher.xor_keystream(&mut buf[15..]);
            assert_eq!(buf, expected, "partial leftover consumption");
        }

        // call 1: partial block -> leaves leftover
        // call 2: exactly exhausts leftover
        // call 3: fresh blocks
        {
            let mut buf = plaintext.clone();
            let mut cipher = ChaCha20Djb::new(&key, &nonce);
            cipher.xor_keystream(&mut buf[..10]);
            cipher.xor_keystream(&mut buf[10..64]);
            cipher.xor_keystream(&mut buf[64..]);
            assert_eq!(buf, expected, "exact leftover exhaustion");
        }

        // call 1: partial block -> leaves leftover
        // call 2 + call 3: two rounds of partial consumption
        // call 4: consumes remaining leftover + fresh blocks
        {
            let mut buf = plaintext.clone();
            let mut cipher = ChaCha20Djb::new(&key, &nonce);
            cipher.xor_keystream(&mut buf[..8]);
            cipher.xor_keystream(&mut buf[8..13]);
            cipher.xor_keystream(&mut buf[13..33]);
            cipher.xor_keystream(&mut buf[33..]);
            assert_eq!(buf, expected, "multiple partial leftover consumptions");
        }
    }

    #[test]
    fn chacha20_ietf_test_vectors() {
        // IETF ChaCha20 test vectors from RFC 8439 Appendix A.2.
        // These use the IETF layout: 32-bit counter (word 12), 96-bit nonce (words 13–15).
        // The nonce is the full 12-byte value (little-endian) that RFC 8439 places in words 13–15.

        struct TestIetf {
            key: [u8; 32],
            nonce: [u8; 12],
            initial_counter: u32,
            plaintext: Vec<u8>,
            expected_ciphertext: Vec<u8>,
        }

        let tests = vec![
            // RFC 8439 section 2.4.2 — the "Ladies and Gentlemen" test vector
            // IETF state: word 12 = counter, words 13-15 = 96-bit nonce LE
            // So state[13]=0, state[14]=0x4a000000, state[15]=0
            TestIetf {
                key: hex::decode("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f")
                    .unwrap()
                    .try_into()
                    .unwrap(),
                nonce: hex::decode("000000000000004a00000000").unwrap().try_into().unwrap(),
                initial_counter: 1,
                plaintext: hex::decode(
                    "4c616469657320616e642047656e746c\
656d656e206f662074686520636c6173\
73206f66202739393a20496620492063\
6f756c64206f6666657220796f75206f\
6e6c79206f6e652074697020666f7220\
746865206675747572652c2073756e73\
637265656e20776f756c642062652069\
742e",
                )
                .unwrap(),
                expected_ciphertext: hex::decode(
                    "6e2e359a2568f98041ba0728dd0d6981\
e97e7aec1d4360c20a27afccfd9fae0b\
f91b65c5524733ab8f593dabcd62b357\
1639d624e65152ab8f530c359f0861d8\
07ca0dbf500d6a6156a38e088a22b65e\
52bc514d16ccf806818ce91ab7793736\
5af90bbf74a35be6b40b8eedf2785e42\
874d",
                )
                .unwrap(),
            },
            // RFC 8439 Appendix A.2 Vector #1 — all zeros key, nonce, counter=0
            TestIetf {
                key: [0u8; 32],
                nonce: [0u8; 12],
                initial_counter: 0,
                plaintext: [0u8; 64].to_vec(),
                expected_ciphertext: hex::decode(
                    "76b8e0ada0f13d90405d6ae55386bd28\
bdd219b8a08ded1aa836efcc8b770dc7\
da41597c5157488d7724e03fb8d84a37\
6a43b8f41518a11cc387b669b2ee6586",
                )
                .unwrap(),
            },
            // RFC 8439 Appendix A.2 Vector #2 — counter=1
            TestIetf {
                key: hex::decode("0000000000000000000000000000000000000000000000000000000000000001")
                    .unwrap()
                    .try_into()
                    .unwrap(),
                nonce: hex::decode("000000000000000000000002").unwrap().try_into().unwrap(),
                initial_counter: 1,
                plaintext: hex::decode(
                    "416e79207375626d697373696f6e2074\
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
7373656420746f",
                )
                .unwrap(),
                expected_ciphertext: hex::decode(
                    "a3fbf07df3fa2fde4f376ca23e827370\
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
c4fd806c22f221",
                )
                .unwrap(),
            },
            // RFC 8439 Appendix A.2 Vector #3 — counter=42
            TestIetf {
                key: hex::decode("1c9240a5eb55d38af333888604f6b5f0473917c1402b80099dca5cbc207075c0")
                    .unwrap()
                    .try_into()
                    .unwrap(),
                nonce: hex::decode("000000000000000000000002").unwrap().try_into().unwrap(),
                initial_counter: 42,
                plaintext: hex::decode(
                    "2754776173206272696c6c69672c2061\
6e642074686520736c6974687920746f\
7665730a446964206779726520616e64\
2067696d626c6520696e207468652077\
6162653a0a416c6c206d696d73792077\
6572652074686520626f726f676f7665\
732c0a416e6420746865206d6f6d6520\
7261746873206f757467726162652e",
                )
                .unwrap(),
                expected_ciphertext: hex::decode(
                    "62e6347f95ed87a45ffae7426f27a1df\
5fb69110044c0d73118effa95b01e5cf\
166d3df2d721caf9b21e5fb14c616871\
fd84c54f9d65b283196c7fe4f60553eb\
f39c6402c42234e32a356b3e764312a6\
1a5532055716ead6962568f87d3f3f77\
04c6a8d1bcd1bf4d50d6154b6da731b1\
87b58dfd728afa36757a797ac188d1",
                )
                .unwrap(),
            },
        ];

        for (i, test) in tests.into_iter().enumerate() {
            let mut cipher = ChaCha20Ietf::new(&test.key, &test.nonce);
            cipher.set_counter(test.initial_counter);

            let mut plaintext = test.plaintext.clone();
            cipher.xor_keystream(&mut plaintext);

            assert_eq!(
                plaintext,
                test.expected_ciphertext,
                "ietf test [{i}] failed
Got ciphertext: {}
Expected ciphertext: {}",
                hex::encode(&plaintext),
                hex::encode(&test.expected_ciphertext),
            );

            // decrypt
            let mut cipher = ChaCha20Ietf::new(&test.key, &test.nonce);
            cipher.set_counter(test.initial_counter);
            cipher.xor_keystream(&mut plaintext);

            assert_eq!(
                plaintext,
                test.plaintext,
                "ietf test [{i}] failed. Initial plaintext != decrypt(encrypt(plaintext))
Got: {}
Expected: {}",
                hex::encode(&plaintext),
                hex::encode(&test.plaintext),
            );

            // partial encryption check
            let mut cipher = ChaCha20Ietf::new(&test.key, &test.nonce);
            cipher.set_counter(test.initial_counter);
            cipher.xor_keystream(&mut plaintext);
            for n in 0..10 {
                let mut partial_plaintext: Vec<u8> = test.plaintext.clone();

                let mut cipher = ChaCha20Ietf::new(&test.key, &test.nonce);
                cipher.set_counter(test.initial_counter);
                cipher.xor_keystream(&mut partial_plaintext[..n]);
                cipher.xor_keystream(&mut partial_plaintext[n..]);

                assert_eq!(
                    plaintext,
                    partial_plaintext,
                    "ietf test [{i}] failed. partial encryption is not valid for n = {n}
            Got: {}
            Expected: {}",
                    hex::encode(&partial_plaintext),
                    hex::encode(&plaintext),
                )
            }
        }
    }

    #[test]
    fn chacha12_case_1() {
        let nonce: &[u8; 8] = &[0xdb, 0x4b, 0x4a, 0x41, 0xd8, 0xdf, 0x18, 0xaa];
        let key: &[u8; 32] = &[
            0x27, 0xfc, 0x12, 0x0b, 0x01, 0x3b, 0x82, 0x9f, 0x1f, 0xae, 0xef, 0xd1, 0xab, 0x41, 0x7e, 0x86, 0x62, 0xf4,
            0x3e, 0x0d, 0x73, 0xf9, 0x8d, 0xe8, 0x66, 0xe3, 0x46, 0x35, 0x31, 0x80, 0xfd, 0xb7,
        ];

        let mut buffer = [0u8; 100];
        ChaCha12Djb::new(key, nonce).xor_keystream(&mut buffer);

        assert_eq!(
            buffer,
            [
                0x5f, 0x3c, 0x8c, 0x19, 0x0a, 0x78, 0xab, 0x7f, 0xe8, 0x08, 0xca, 0xe9, 0xcb, 0xcb, 0x0a, 0x98, 0x37,
                0xc8, 0x93, 0x49, 0x2d, 0x96, 0x3a, 0x1c, 0x2e, 0xda, 0x6c, 0x15, 0x58, 0xb0, 0x2c, 0x83, 0xfc, 0x02,
                0xa4, 0x4c, 0xbb, 0xb7, 0xe6, 0x20, 0x4d, 0x51, 0xd1, 0xc2, 0x43, 0x0e, 0x9c, 0x0b, 0x58, 0xf2, 0x93,
                0x7b, 0xf5, 0x93, 0x84, 0x0c, 0x85, 0x0b, 0xda, 0x90, 0x51, 0xa1, 0xf0, 0x51, 0xdd, 0xf0, 0x9d, 0x2a,
                0x03, 0xeb, 0xf0, 0x9f, 0x01, 0xbd, 0xba, 0x9d, 0xa0, 0xb6, 0xda, 0x79, 0x1b, 0x2e, 0x64, 0x56, 0x41,
                0x04, 0x7d, 0x11, 0xeb, 0xf8, 0x50, 0x87, 0xd4, 0xde, 0x5c, 0x01, 0x5f, 0xdd, 0xd0, 0x44,
            ]
        );
    }

    #[test]
    fn chacha8_case_1() {
        let key = &[
            0x64, 0x1a, 0xea, 0xeb, 0x08, 0x03, 0x6b, 0x61, 0x7a, 0x42, 0xcf, 0x14, 0xe8, 0xc5, 0xd2, 0xd1, 0x15, 0xf8,
            0xd7, 0xcb, 0x6e, 0xa5, 0xe2, 0x8b, 0x9b, 0xfa, 0xf8, 0x3e, 0x03, 0x84, 0x26, 0xa7,
        ];
        let nonce = &[0xa1, 0x4a, 0x11, 0x68, 0x27, 0x1d, 0x45, 0x9b];

        let mut buffer = [0u8; 100];
        ChaCha8Djb::new(key, nonce).xor_keystream(&mut buffer);

        assert_eq!(
            buffer,
            [
                0x17, 0x21, 0xc0, 0x44, 0xa8, 0xa6, 0x45, 0x35, 0x22, 0xdd, 0xdb, 0x31, 0x43, 0xd0, 0xbe, 0x35, 0x12,
                0x63, 0x3c, 0xa3, 0xc7, 0x9b, 0xf8, 0xcc, 0xc3, 0x59, 0x4c, 0xb2, 0xc2, 0xf3, 0x10, 0xf7, 0xbd, 0x54,
                0x4f, 0x55, 0xce, 0x0d, 0xb3, 0x81, 0x23, 0x41, 0x2d, 0x6c, 0x45, 0x20, 0x7d, 0x5c, 0xf9, 0xaf, 0x0c,
                0x6c, 0x68, 0x0c, 0xce, 0x1f, 0x7e, 0x43, 0x38, 0x8d, 0x1b, 0x03, 0x46, 0xb7, 0x13, 0x3c, 0x59, 0xfd,
                0x6a, 0xf4, 0xa5, 0xa5, 0x68, 0xaa, 0x33, 0x4c, 0xcd, 0xc3, 0x8a, 0xf5, 0xac, 0xe2, 0x01, 0xdf, 0x84,
                0xd0, 0xa3, 0xca, 0x22, 0x54, 0x94, 0xca, 0x62, 0x09, 0x34, 0x5f, 0xcf, 0x30, 0x13, 0x2e,
            ]
        );
    }

    #[test]
    fn xchacha20_test_vector_counter_0() {
        // draft-irtf-cfrg-xchacha-03, Appendix A.3.2.1
        let key: [u8; 32] = hex::decode("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f")
            .unwrap()
            .try_into()
            .unwrap();
        let nonce: [u8; 24] = hex::decode("404142434445464748494a4b4c4d4e4f5051525354555658")
            .unwrap()
            .try_into()
            .unwrap();
        let plaintext = hex::decode(concat!(
            "5468652064686f6c65202870726f6e6f756e6365642022646f6c652229206973",
            "20616c736f206b6e6f776e2061732074686520417369617469632077696c6420",
            "646f672c2072656420646f672c20616e642077686973746c696e6720646f672e",
            "2049742069732061626f7574207468652073697a65206f662061204765726d61",
            "6e20736865706865726420627574206c6f6f6b73206d6f7265206c696b652061",
            "206c6f6e672d6c656767656420666f782e205468697320686967686c7920656c",
            "757369766520616e6420736b696c6c6564206a756d70657220697320636c6173",
            "736966696564207769746820776f6c7665732c20636f796f7465732c206a6163",
            "6b616c732c20616e6420666f78657320696e20746865207461786f6e6f6d6963",
            "2066616d696c792043616e696461652e",
        ))
        .unwrap();
        let expected_ciphertext = hex::decode(concat!(
            "4559abba4e48c16102e8bb2c05e6947f50a786de162f9b0b7e592a9b53d0d4e9",
            "8d8d6410d540a1a6375b26d80dace4fab52384c731acbf16a5923c0c48d3575d",
            "4d0d2c673b666faa731061277701093a6bf7a158a8864292a41c48e3a9b4c0da",
            "ece0f8d98d0d7e05b37a307bbb66333164ec9e1b24ea0d6c3ffddcec4f68e744",
            "3056193a03c810e11344ca06d8ed8a2bfb1e8d48cfa6bc0eb4e2464b74814240",
            "7c9f431aee769960e15ba8b96890466ef2457599852385c661f752ce20f9da0c",
            "09ab6b19df74e76a95967446f8d0fd415e7bee2a12a114c20eb5292ae7a349ae",
            "577820d5520a1f3fb62a17ce6a7e68fa7c79111d8860920bc048ef43fe84486c",
            "cb87c25f0ae045f0cce1e7989a9aa220a28bdd4827e751a24a6d5c62d790a663",
            "93b93111c1a55dd7421a10184974c7c5",
        ))
        .unwrap();

        let mut cipher = XChaCha20::new(&key, &nonce);
        let mut buf = plaintext.clone();
        cipher.xor_keystream(&mut buf);
        assert_eq!(buf, expected_ciphertext);

        // Decrypt
        cipher.set_counter(0);
        cipher.xor_keystream(&mut buf);
        assert_eq!(buf, plaintext);
    }

    #[test]
    fn xchacha20_set_counter() {
        let key = [0x55u8; 32];
        let nonce = [0xaau8; 24];
        let plaintext = b"test message for xchacha20";

        let mut cipher1 = XChaCha20::new(&key, &nonce);
        let mut buf1 = plaintext.to_vec();
        cipher1.xor_keystream(&mut buf1);

        // Same cipher but with set_counter(0) should produce same output
        let mut cipher2 = XChaCha20::new(&key, &nonce);
        cipher2.set_counter(0);
        let mut buf2 = plaintext.to_vec();
        cipher2.xor_keystream(&mut buf2);

        assert_eq!(buf1, buf2);

        // Different counter should produce different output
        let mut cipher3 = XChaCha20::new(&key, &nonce);
        cipher3.set_counter(1);
        let mut buf3 = plaintext.to_vec();
        cipher3.xor_keystream(&mut buf3);
        assert_ne!(buf1, buf3);
    }

    // -------------------------------------------------------------------------
    // Edge-case leftover tests
    // -------------------------------------------------------------------------

    /// Helper: encrypt in one shot to get the expected reference.
    fn encrypt_one_shot_djb(key: &[u8; 32], nonce: &[u8; 8], plaintext: &[u8]) -> Vec<u8> {
        let mut buf = plaintext.to_vec();
        ChaCha20Djb::new(key, nonce).xor_keystream(&mut buf);
        buf
    }

    fn encrypt_one_shot_ietf(key: &[u8; 32], nonce: &[u8; 12], plaintext: &[u8]) -> Vec<u8> {
        let mut buf = plaintext.to_vec();
        ChaCha20Ietf::new(key, nonce).xor_keystream(&mut buf);
        buf
    }

    fn encrypt_one_shot_xchacha(key: &[u8; 32], nonce: &[u8; 24], plaintext: &[u8]) -> Vec<u8> {
        let mut buf = plaintext.to_vec();
        XChaCha20::new(key, nonce).xor_keystream(&mut buf);
        buf
    }

    fn make_djb_test(key: &[u8; 32], nonce: &[u8; 8], plaintext: &[u8], split_sizes: &[usize]) {
        let expected = encrypt_one_shot_djb(key, nonce, plaintext);
        let mut buf = plaintext.to_vec();
        let mut cipher = ChaCha20Djb::new(key, nonce);
        let mut offset = 0;
        for &size in split_sizes {
            let end = core::cmp::min(offset + size, buf.len());
            cipher.xor_keystream(&mut buf[offset..end]);
            offset = end;
        }
        assert_eq!(buf, expected, "DJB leftover test failed for splits {split_sizes:?}");
    }

    fn make_ietf_test(key: &[u8; 32], nonce: &[u8; 12], plaintext: &[u8], split_sizes: &[usize]) {
        let expected = encrypt_one_shot_ietf(key, nonce, plaintext);
        let mut buf = plaintext.to_vec();
        let mut cipher = ChaCha20Ietf::new(key, nonce);
        let mut offset = 0;
        for &size in split_sizes {
            let end = core::cmp::min(offset + size, buf.len());
            cipher.xor_keystream(&mut buf[offset..end]);
            offset = end;
        }
        assert_eq!(buf, expected, "IETF leftover test failed for splits {split_sizes:?}");
    }

    fn make_xchacha_test(key: &[u8; 32], nonce: &[u8; 24], plaintext: &[u8], split_sizes: &[usize]) {
        let expected = encrypt_one_shot_xchacha(key, nonce, plaintext);
        let mut buf = plaintext.to_vec();
        let mut cipher = XChaCha20::new(key, nonce);
        let mut offset = 0;
        for &size in split_sizes {
            let end = core::cmp::min(offset + size, buf.len());
            cipher.xor_keystream(&mut buf[offset..end]);
            offset = end;
        }
        assert_eq!(buf, expected, "XChaCha leftover test failed for splits {split_sizes:?}");
    }

    fn test_key_32() -> [u8; 32] {
        let mut k = [0u8; 32];
        for i in 0..32 {
            k[i] = i as u8;
        }
        k
    }

    fn test_nonce_8() -> [u8; 8] {
        [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]
    }

    fn test_nonce_12() -> [u8; 12] {
        [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b]
    }

    fn test_nonce_24() -> [u8; 24] {
        let mut n = [0u8; 24];
        for i in 0..24 {
            n[i] = i as u8;
        }
        n
    }

    fn test_plaintext(len: usize) -> Vec<u8> {
        (0..len).map(|i| (i % 251) as u8).collect()
    }

    /// IETF variant: multi-call leftover consumption (analogous to the DJB test above).
    #[test]
    fn chacha20_ietf_keystream_leftover_multi_call() {
        let key = test_key_32();
        let nonce = test_nonce_12();
        let pt = test_plaintext(300);

        let expected = encrypt_one_shot_ietf(&key, &nonce, &pt);

        // partial block -> leaves leftover, partially consume, then finish
        {
            let mut buf = pt.clone();
            let mut c = ChaCha20Ietf::new(&key, &nonce);
            c.xor_keystream(&mut buf[..10]);
            c.xor_keystream(&mut buf[10..15]);
            c.xor_keystream(&mut buf[15..]);
            assert_eq!(buf, expected, "ietf partial leftover consumption");
        }

        // partial block -> exactly exhaust leftover -> fresh blocks
        {
            let mut buf = pt.clone();
            let mut c = ChaCha20Ietf::new(&key, &nonce);
            c.xor_keystream(&mut buf[..3]);
            c.xor_keystream(&mut buf[3..64]);
            c.xor_keystream(&mut buf[64..]);
            assert_eq!(buf, expected, "ietf exact leftover exhaustion");
        }

        // three rounds of partial leftover consumption
        {
            let mut buf = pt.clone();
            let mut c = ChaCha20Ietf::new(&key, &nonce);
            c.xor_keystream(&mut buf[..5]);
            c.xor_keystream(&mut buf[5..12]);
            c.xor_keystream(&mut buf[12..20]);
            c.xor_keystream(&mut buf[20..]);
            assert_eq!(buf, expected, "ietf multiple partial leftover consumptions");
        }
    }

    /// XChaCha20: multi-call leftover consumption.
    #[test]
    fn xchacha20_keystream_leftover_multi_call() {
        let key = test_key_32();
        let nonce = test_nonce_24();
        let pt = test_plaintext(300);

        let expected = encrypt_one_shot_xchacha(&key, &nonce, &pt);

        // partial block -> leaves leftover, partially consume, then finish
        {
            let mut buf = pt.clone();
            let mut c = XChaCha20::new(&key, &nonce);
            c.xor_keystream(&mut buf[..8]);
            c.xor_keystream(&mut buf[8..20]);
            c.xor_keystream(&mut buf[20..]);
            assert_eq!(buf, expected, "xchacha partial leftover consumption");
        }

        // three rounds of partial consumption
        {
            let mut buf = pt.clone();
            let mut c = XChaCha20::new(&key, &nonce);
            c.xor_keystream(&mut buf[..13]);
            c.xor_keystream(&mut buf[13..27]);
            c.xor_keystream(&mut buf[27..40]);
            c.xor_keystream(&mut buf[40..]);
            assert_eq!(buf, expected, "xchacha multiple partial leftover consumptions");
        }
    }

    /// Very small chunks: 1-byte calls to stress the leftover offset state machine.
    #[test]
    fn chacha_keystream_leftover_tiny_chunks() {
        let key = test_key_32();
        let nonce = test_nonce_8();
        let pt = test_plaintext(200);

        // 10 calls of 1 byte each, then the rest in one shot
        let mut cipher = ChaCha20Djb::new(&key, &nonce);
        let mut buf = pt.clone();
        for n in 0..10 {
            cipher.xor_keystream(&mut buf[n..n + 1]);
        }
        cipher.xor_keystream(&mut buf[10..]);
        let expected = encrypt_one_shot_djb(&key, &nonce, &pt);
        assert_eq!(buf, expected, "tiny-chunk DJB failed");

        // IETF variant
        let nonce12 = test_nonce_12();
        let mut buf = pt.clone();
        let mut cipher = ChaCha20Ietf::new(&key, &nonce12);
        for n in 0..10 {
            cipher.xor_keystream(&mut buf[n..n + 1]);
        }
        cipher.xor_keystream(&mut buf[10..]);
        let expected = encrypt_one_shot_ietf(&key, &nonce12, &pt);
        assert_eq!(buf, expected, "tiny-chunk IETF failed");

        // XChaCha variant
        let nonce24 = test_nonce_24();
        let mut buf = pt.clone();
        let mut cipher = XChaCha20::new(&key, &nonce24);
        for n in 0..10 {
            cipher.xor_keystream(&mut buf[n..n + 1]);
        }
        cipher.xor_keystream(&mut buf[10..]);
        let expected = encrypt_one_shot_xchacha(&key, &nonce24, &pt);
        assert_eq!(buf, expected, "tiny-chunk XChaCha failed");
    }

    /// Boundary sizes: 63, 64, 65, 127, 128, 129 bytes.
    /// Tests the leftover offset formula at exact transition points.
    #[test]
    fn chacha_keystream_leftover_boundary_sizes() {
        let key = test_key_32();
        let nonce = test_nonce_8();

        for &len in &[63usize, 64, 65, 127, 128, 129, 191, 192, 193, 255, 256, 257] {
            let pt = test_plaintext(len);
            let expected = encrypt_one_shot_djb(&key, &nonce, &pt);

            // split into 3 chunks: a, b, c where a+b+c = len
            // Use varying split sizes to exercise different leftover states
            for a in [0usize, 1, 8, 31, 32, 33, 62, 63].iter().copied() {
                if a > len {
                    continue;
                }
                for b in [0usize, 1, 7, 32, 63, 64].iter().copied() {
                    if a + b > len {
                        continue;
                    }
                    let c = len - a - b;
                    let splits = [a, b, c];

                    let mut cipher = ChaCha20Djb::new(&key, &nonce);
                    let mut buf = pt.clone();
                    let mut offset = 0;
                    for &size in &splits {
                        cipher.xor_keystream(&mut buf[offset..offset + size]);
                        offset += size;
                    }
                    assert_eq!(buf, expected, "Boundary DJB failed for len={len} splits={splits:?}",);
                }
            }
        }
    }

    /// Boundary sizes for IETF variant.
    #[test]
    fn chacha_keystream_leftover_boundary_sizes_ietf() {
        let key = test_key_32();
        let nonce = test_nonce_12();

        for &len in &[63usize, 64, 65, 127, 128, 129] {
            let pt = test_plaintext(len);
            let expected = encrypt_one_shot_ietf(&key, &nonce, &pt);

            for &a in &[0usize, 1, 31, 32, 33, 63] {
                if a > len {
                    continue;
                }
                let mut cipher = ChaCha20Ietf::new(&key, &nonce);
                let mut buf = pt.clone();
                cipher.xor_keystream(&mut buf[..a]);
                cipher.xor_keystream(&mut buf[a..]);
                assert_eq!(buf, expected, "Boundary IETF failed for len={len} a={a}",);
            }
        }
    }

    /// Zero-byte intermediate calls: should be no-ops that don't corrupt leftover state.
    #[test]
    fn chacha_keystream_leftover_zero_byte_intermediate() {
        let key = test_key_32();
        let nonce = test_nonce_8();
        let pt = test_plaintext(150);
        let expected = encrypt_one_shot_djb(&key, &nonce, &pt);

        let mut buf = pt.clone();
        let mut cipher = ChaCha20Djb::new(&key, &nonce);

        // first partial call
        cipher.xor_keystream(&mut buf[..10]);
        // zero-byte call in the middle
        cipher.xor_keystream(&mut []);
        // second partial call
        cipher.xor_keystream(&mut buf[10..20]);
        // another zero-byte call
        cipher.xor_keystream(&mut []);
        // final call
        cipher.xor_keystream(&mut buf[20..]);

        assert_eq!(buf, expected, "zero-byte intermediate DJB failed");
    }

    /// set_counter mid-stream, then partial encryption: verifies leftover is cleared
    /// and counter is correctly reset.
    #[test]
    fn chacha_keystream_leftover_set_counter_mid_stream() {
        let key = test_key_32();
        let nonce = test_nonce_8();
        let pt = test_plaintext(200);

        // one-shot reference
        let full_encrypted = encrypt_one_shot_djb(&key, &nonce, &pt);

        // encrypt all at once with a fresh cipher
        let mut buf = pt.clone();
        let mut cipher = ChaCha20Djb::new(&key, &nonce);
        cipher.xor_keystream(&mut buf);
        assert_eq!(buf, full_encrypted, "baseline DJB");

        // encrypt 10 bytes, reset counter to 0, re-encrypt those 10 bytes
        // (this XORs again → back to plaintext), then encrypt the rest.
        let mut buf = pt.clone();
        let mut cipher = ChaCha20Djb::new(&key, &nonce);
        cipher.xor_keystream(&mut buf[..10]);
        cipher.set_counter(0);
        cipher.xor_keystream(&mut buf[..10]); // re-XOR → bytes 0-9 are plaintext again
        cipher.xor_keystream(&mut buf[10..]);

        // first 10 bytes are back to plaintext
        assert_eq!(
            &buf[..10],
            &pt[..10],
            "set_counter mid-stream: first 10 bytes should be plaintext"
        );
        // remaining bytes match the one-shot encryption
        assert_eq!(&buf[10..], &full_encrypted[10..], "set_counter mid-stream DJB failed");

        // IETF variant: same pattern
        let nonce12 = test_nonce_12();
        let full_encrypted = encrypt_one_shot_ietf(&key, &nonce12, &pt);

        let mut buf = pt.clone();
        let mut cipher = ChaCha20Ietf::new(&key, &nonce12);
        cipher.xor_keystream(&mut buf[..10]);
        cipher.set_counter(0);
        cipher.xor_keystream(&mut buf[..10]);
        cipher.xor_keystream(&mut buf[10..]);
        assert_eq!(&buf[..10], &pt[..10], "set_counter mid-stream: IETF first 10 bytes");
        assert_eq!(&buf[10..], &full_encrypted[10..], "set_counter mid-stream IETF failed");
    }

    /// Stress test: many sequential calls with random-sized splits at block boundaries.
    #[test]
    fn chacha_keystream_leftover_stress_random_splits() {
        let key = test_key_32();
        let nonce = test_nonce_8();

        let sizes = [50usize, 127, 128, 129, 200, 256, 300, 400, 512];
        let split_patterns: &[&[usize]] = &[
            &[1, 2, 3, 4, 5],
            &[7, 13, 23, 31],
            &[32, 32, 32],
            &[63, 1],
            &[64, 64],
            &[65, 63],
            &[33, 33, 33, 33],
            &[10, 10, 10, 10, 10],
            &[50, 50, 50],
        ];

        for &len in &sizes {
            let pt = test_plaintext(len);

            // ChaCha20
            let expected20 = {
                let mut b = pt.clone();
                ChaCha20Djb::new(&key, &nonce).xor_keystream(&mut b);
                b
            };
            for &splits in split_patterns {
                let mut buf = pt.clone();
                let mut offset = 0;
                let mut c = ChaCha20Djb::new(&key, &nonce);
                for &size in splits {
                    let end = core::cmp::min(offset + size, buf.len());
                    c.xor_keystream(&mut buf[offset..end]);
                    offset = end;
                    if offset >= buf.len() {
                        break;
                    }
                }
                if offset < buf.len() {
                    c.xor_keystream(&mut buf[offset..]);
                }
                assert_eq!(buf, expected20, "Stress ChaCha20 failed for len={len} splits={splits:?}",);
            }

            // ChaCha12
            let expected12 = {
                let mut b = pt.clone();
                ChaCha12Djb::new(&key, &nonce).xor_keystream(&mut b);
                b
            };
            for &splits in split_patterns {
                let mut buf = pt.clone();
                let mut offset = 0;
                let mut c = ChaCha12Djb::new(&key, &nonce);
                for &size in splits {
                    let end = core::cmp::min(offset + size, buf.len());
                    c.xor_keystream(&mut buf[offset..end]);
                    offset = end;
                    if offset >= buf.len() {
                        break;
                    }
                }
                if offset < buf.len() {
                    c.xor_keystream(&mut buf[offset..]);
                }
                assert_eq!(buf, expected12, "Stress ChaCha12 failed for len={len} splits={splits:?}",);
            }

            // ChaCha8
            let expected8 = {
                let mut b = pt.clone();
                ChaCha8Djb::new(&key, &nonce).xor_keystream(&mut b);
                b
            };
            for &splits in split_patterns {
                let mut buf = pt.clone();
                let mut offset = 0;
                let mut c = ChaCha8Djb::new(&key, &nonce);
                for &size in splits {
                    let end = core::cmp::min(offset + size, buf.len());
                    c.xor_keystream(&mut buf[offset..end]);
                    offset = end;
                    if offset >= buf.len() {
                        break;
                    }
                }
                if offset < buf.len() {
                    c.xor_keystream(&mut buf[offset..]);
                }
                assert_eq!(buf, expected8, "Stress ChaCha8 failed for len={len} splits={splits:?}",);
            }
        }
    }

    /// IETF-specific variant of the stress test.
    #[test]
    fn chacha_keystream_leftover_stress_random_splits_ietf() {
        let key = test_key_32();
        let nonce = test_nonce_12();

        for &len in &[50usize, 127, 128, 129, 200, 256, 300] {
            let pt = test_plaintext(len);
            let expected = encrypt_one_shot_ietf(&key, &nonce, &pt);

            let split_patterns: &[&[usize]] = &[
                &[1, 2, 3, 4, 5],
                &[7, 13, 23, 31],
                &[63, 1],
                &[64, 64],
                &[65, 63],
                &[33, 33, 33, 33],
                &[50, 50, 50],
            ];

            for &splits in split_patterns {
                let mut buf = pt.clone();
                let mut offset = 0;
                let mut cipher = ChaCha20Ietf::new(&key, &nonce);
                for &size in splits {
                    let end = core::cmp::min(offset + size, buf.len());
                    cipher.xor_keystream(&mut buf[offset..end]);
                    offset = end;
                    if offset >= buf.len() {
                        break;
                    }
                }
                if offset < buf.len() {
                    cipher.xor_keystream(&mut buf[offset..]);
                }
                assert_eq!(buf, expected, "Stress IETF failed for len={len} splits={splits:?}",);
            }
        }
    }

    /// XChaCha-specific variant of the stress test.
    #[test]
    fn keystream_leftover_stress_random_splits_xchacha() {
        let key = test_key_32();
        let nonce = test_nonce_24();

        for &len in &[50usize, 127, 128, 129, 200] {
            let pt = test_plaintext(len);
            let expected = encrypt_one_shot_xchacha(&key, &nonce, &pt);

            let split_patterns: &[&[usize]] = &[&[1, 2, 3, 4, 5], &[7, 13, 23], &[63, 1], &[64, 64], &[65, 63]];

            for &splits in split_patterns {
                let mut buf = pt.clone();
                let mut offset = 0;
                let mut cipher = XChaCha20::new(&key, &nonce);
                for &size in splits {
                    let end = core::cmp::min(offset + size, buf.len());
                    cipher.xor_keystream(&mut buf[offset..end]);
                    offset = end;
                    if offset >= buf.len() {
                        break;
                    }
                }
                if offset < buf.len() {
                    cipher.xor_keystream(&mut buf[offset..]);
                }
                assert_eq!(buf, expected, "Stress XChaCha failed for len={len} splits={splits:?}",);
            }
        }
    }
}
