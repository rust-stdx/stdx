use super::*;
use crate::Xof;

/// Ascon-XOF128 initialization vector (NIST SP 800-232).
const IV: u64 = 0x0000_0800_00cc_0003;

/// Ascon-XOF128 extensible-output function (NIST SP 800-232 §5.2).
///
/// Produces variable-length output. Rate = 64 bits (8 bytes), capacity = 256 bits.
/// Shorter outputs are prefixes of longer ones.
///
/// Implements the [`Xof`] trait.
///
/// # One-shot API
///
/// ```ignore
/// use crypto::ascon::AsconXof128;
///
/// let mut output = [0u8; 32];
/// AsconXof128::hash(b"hello world", &mut output);
/// ```
///
/// # Incremental API
///
/// ```ignore
/// use crypto::{ascon::AsconXof128, Xof};
///
/// let mut xof = AsconXof128::new();
/// xof.absorb(b"hello ");
/// xof.absorb(b"world");
/// let mut out = [0u8; 32];
/// xof.squeeze(&mut out);
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct AsconXof128 {
    state: State,
    buf: [u8; 8],
    buf_len: usize,
    squeezing: bool,
    squeeze_pos: usize,
    current_block: [u8; 8],
}

impl AsconXof128 {
    #[inline]
    pub fn new() -> Self {
        let mut state = State::init_hash(IV);
        p12(&mut state);
        AsconXof128 {
            state,
            buf: [0u8; 8],
            buf_len: 0,
            squeezing: false,
            squeeze_pos: 0,
            current_block: [0u8; 8],
        }
    }

    #[inline]
    pub fn hash(data: &[u8], output: &mut [u8]) {
        let mut xof = Self::new();
        xof.absorb(data);
        xof.squeeze(output);
    }
}

impl Xof for AsconXof128 {
    fn absorb(&mut self, mut data: &[u8]) {
        assert!(!self.squeezing, "absorb cannot be called after squeeze");

        if self.buf_len > 0 {
            let to_fill = (8 - self.buf_len).min(data.len());
            self.buf[self.buf_len..self.buf_len + to_fill].copy_from_slice(&data[..to_fill]);
            self.buf_len += to_fill;
            data = &data[to_fill..];

            if self.buf_len == 8 {
                self.state.absorb_block(&self.buf);
                p12(&mut self.state);
                self.buf_len = 0;
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

    fn squeeze(&mut self, out: &mut [u8]) {
        if !self.squeezing {
            let mut padded = [0u8; 8];
            padded[..self.buf_len].copy_from_slice(&self.buf[..self.buf_len]);
            padded[self.buf_len] = 0x01;
            self.state.absorb_block(&padded);
            p12(&mut self.state);
            self.squeezing = true;
        }

        let mut remaining = out;
        while !remaining.is_empty() {
            if self.squeeze_pos == 0 {
                self.current_block = self.state.squeeze_byte();
            }
            let n = remaining.len().min(8 - self.squeeze_pos);
            remaining[..n].copy_from_slice(&self.current_block[self.squeeze_pos..self.squeeze_pos + n]);
            self.squeeze_pos += n;
            remaining = &mut remaining[n..];

            if self.squeeze_pos == 8 && !remaining.is_empty() {
                self.squeeze_pos = 0;
                p12(&mut self.state);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Xof;

    #[test]
    fn empty_xof() {
        let mut out = [0u8; 64];
        AsconXof128::hash(b"", &mut out);
        let expected = hex::decode("473D5E6164F58B39DFD84AACDB8AE42EC2D91FED33388EE0D960D9B3993295C6AD77855A5D3B13FE6AD9E6098988373AF7D0956D05A8F1665D2C67D1A3AD10FF").unwrap();
        assert_eq!(out.as_slice(), expected.as_slice());
    }

    #[test]
    fn one_byte_xof() {
        let mut out = [0u8; 64];
        AsconXof128::hash(b"\x00", &mut out);
        let expected = hex::decode("51430E0438ECDF642B393630D977625F5F337656BA58AB1E960784AC32A16E0D446405551F5469384F8EA283CF12E64FA72C426BFEBAEA3AA1529E2C4AB23A2F").unwrap();
        assert_eq!(out.as_slice(), expected.as_slice());
    }

    #[test]
    fn incremental_xof() {
        let mut one_shot = [0u8; 64];
        AsconXof128::hash(b"", &mut one_shot);

        let mut xof = AsconXof128::new();
        xof.absorb(b"");
        let mut first = [0u8; 32];
        let mut second = [0u8; 32];
        xof.squeeze(&mut first);
        xof.squeeze(&mut second);

        assert_eq!(first.as_slice(), &one_shot[..32]);
        assert_eq!(second.as_slice(), &one_shot[32..]);
    }

    #[test]
    fn prefix_property() {
        let mut out64 = [0u8; 64];
        AsconXof128::hash(b"data", &mut out64);

        let mut out32 = [0u8; 32];
        AsconXof128::hash(b"data", &mut out32);

        assert_eq!(out32.as_slice(), &out64[..32]);
    }

    #[test]
    fn kat_vectors() {
        let data = include_str!("../../testdata/ascon/LWC_XOF_KAT_128_512.txt");
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
                let mut out = vec![0u8; expected.len()];
                AsconXof128::hash(&msg, &mut out);
                assert_eq!(out.as_slice(), expected.as_slice(), "KAT XOF Count={count} mismatch");

                // Test incremental
                let mut xof = AsconXof128::new();
                xof.absorb(&msg);
                let mut out_inc = vec![0u8; expected.len()];
                xof.squeeze(&mut out_inc);
                assert_eq!(
                    out_inc.as_slice(),
                    expected.as_slice(),
                    "KAT XOF Count={count} incremental mismatch"
                );
                msg_hex.clear();
                continue;
            }
        }
    }
}
