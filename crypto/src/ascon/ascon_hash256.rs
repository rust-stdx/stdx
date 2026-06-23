use super::*;
use crate::{Bytes, Hash, Hasher};

/// ASCHB implicit helper const: Ascon-Hash256 initialization vector (NIST SP 800-232).
const IV: u64 = 0x0000_0801_00cc_0002;

/// Ascon-Hash256 cryptographic hash function (NIST SP 800-232 §5.1).
///
/// Produces a 256-bit (32-byte) digest. Rate = 64 bits (8 bytes), capacity = 256 bits.
///
/// Implements the [`Hasher`] trait.
///
/// # One-shot API
///
/// ```ignore
/// use crypto::{Hasher, ascon::AsconHash256};
///
/// let hash = AsconHash256::hash(b"hello world");
/// ```
///
/// # Incremental API
///
/// ```ignore
/// use crypto::{Hasher, ascon::AsconHash256};
///
/// let mut hasher = AsconHash256::new();
/// hasher.update(b"hello ");
/// hasher.update(b"world");
/// let hash = hasher.sum();
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct AsconHash256 {
    state: State,
    buf: [u8; 8],
    buf_len: usize,
}

impl AsconHash256 {
    #[inline]
    pub fn new() -> Self {
        let mut state = State::init_hash(IV);
        p12(&mut state);
        AsconHash256 {
            state,
            buf: [0u8; 8],
            buf_len: 0,
        }
    }

    fn process_buffer(&mut self) {
        debug_assert_eq!(self.buf_len, 8);
        self.state.absorb_block(&self.buf);
        p12(&mut self.state);
        self.buf_len = 0;
    }

    fn pad_and_finalize(&mut self) {
        let mut padded = [0u8; 8];
        padded[..self.buf_len].copy_from_slice(&self.buf[..self.buf_len]);
        padded[self.buf_len] = 0x01;
        self.state.absorb_block(&padded);
        p12(&mut self.state);
    }
}

impl Hasher for AsconHash256 {
    const BLOCK_SIZE: usize = 8;
    const OUTPUT_SIZE: usize = 32;

    #[inline]
    fn new() -> Self {
        AsconHash256::new()
    }

    fn update(&mut self, mut data: &[u8]) {
        if self.buf_len > 0 {
            let to_fill = (8 - self.buf_len).min(data.len());
            self.buf[self.buf_len..self.buf_len + to_fill].copy_from_slice(&data[..to_fill]);
            self.buf_len += to_fill;
            data = &data[to_fill..];

            if self.buf_len == 8 {
                self.process_buffer();
            }
        }

        let mut chunks = data.chunks_exact(8);
        for chunk in &mut chunks {
            self.state.absorb_block(chunk);
            p12(&mut self.state);
        }

        let remainder = chunks.remainder();
        if !remainder.is_empty() {
            self.buf[..remainder.len()].copy_from_slice(remainder);
            self.buf_len = remainder.len();
        }
    }

    fn sum(mut self) -> Hash {
        self.pad_and_finalize();

        let mut hash = Bytes::<64>::with_length(32);
        for i in 0..4 {
            hash.as_mut()[i * 8..(i + 1) * 8].copy_from_slice(&self.state.squeeze_byte());
            if i < 3 {
                p12(&mut self.state);
            }
        }

        Hash(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Hasher;

    #[test]
    fn empty_input() {
        let h = AsconHash256::hash(b"");
        let expected = hex::decode("0B3BE5850F2F6B98CAF29F8FDEA89B64A1FA70AA249B8F839BD53BAA304D92B2").unwrap();
        assert_eq!(h.as_ref(), expected.as_slice());
    }

    #[test]
    fn one_byte() {
        let h = AsconHash256::hash(b"\x00");
        let expected = hex::decode("0728621035AF3ED2BCA03BF6FDE900F9456F5330E4B5EE23E7F6A1E70291BC80").unwrap();
        assert_eq!(h.as_ref(), expected.as_slice());
    }

    #[test]
    fn two_bytes() {
        let h = AsconHash256::hash(b"\x00\x01");
        let expected = hex::decode("6115E7C9C4081C2797FC8FE1BC57A836AFA1C5381E556DD583860CA2DFB48DD2").unwrap();
        assert_eq!(h.as_ref(), expected.as_slice());
    }

    #[test]
    fn exactly_one_block() {
        let h = AsconHash256::hash(b"\x00\x01\x02\x03\x04\x05\x06\x07");
        let expected = hex::decode("B88E497AE8E6FB641B87EF622EB8F2FCA0ED95383F7FFEBE167ACF1099BA764F").unwrap();
        assert_eq!(h.as_ref(), expected.as_slice());
    }

    #[test]
    fn incremental() {
        let msg = b"hello world";
        let one_shot = AsconHash256::hash(msg);
        let mut h = AsconHash256::new();
        for byte in msg {
            h.update(&[*byte]);
        }
        assert_eq!(one_shot.as_ref(), h.sum().as_ref());
    }

    #[test]
    fn block_boundaries() {
        for len in [1usize, 7, 8, 9, 15, 16, 17, 63, 64, 65] {
            let input = vec![b'a'; len];
            let one_shot = AsconHash256::hash(&input);
            let mut h = AsconHash256::new();
            for chunk in input.chunks(3) {
                h.update(chunk);
            }
            assert_eq!(one_shot.as_ref(), h.sum().as_ref(), "len={len}");
        }
    }

    #[test]
    fn kat_vectors() {
        let data = include_str!("../../testdata/ascon/LWC_HASH_KAT_128_256.txt");
        let mut count = 0u64;
        let mut msg_hex = String::new();

        for line in data.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with("Count = ") {
                count = line["Count = ".len()..].parse().unwrap();
                msg_hex.clear();
                continue;
            }
            if line.starts_with("Msg = ") {
                msg_hex = line[6..].to_string();
                continue;
            }
            if line.starts_with("MD = ") {
                let expected_md: &str = &line[5..];
                let msg = hex::decode(&msg_hex).unwrap();
                let expected = hex::decode(expected_md).unwrap();
                let hash = AsconHash256::hash(&msg);
                assert_eq!(hash.as_ref(), expected.as_slice(), "KAT Hash Count={count} mismatch");
                let mut h = AsconHash256::new();
                for chunk in msg.chunks(3) {
                    h.update(chunk);
                }
                assert_eq!(
                    h.sum().as_ref(),
                    expected.as_slice(),
                    "KAT Hash Count={count} incremental mismatch"
                );
                msg_hex.clear();
                continue;
            }
        }
    }
}
