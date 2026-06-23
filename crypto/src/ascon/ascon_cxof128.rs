use super::*;
use crate::Xof;

/// Ascon-CXOF128 initialization vector (NIST SP 800-232).
const IV: u64 = 0x0000_0800_00cc_0004;

/// Ascon-CXOF128 customizable extensible-output function (NIST SP 800-232 §5.3).
///
/// Extends Ascon-XOF128 with a customization string `Z` (up to 256 bytes).
/// The customization is absorbed before the message, so `new_with_customization(b"")`
/// produces different output than [`AsconXof128`](super::AsconXof128) on the same input.
///
/// Implements the [`Xof`] trait.
///
/// # Panics
///
/// Panics if the customization string exceeds 256 bytes.
///
/// # Incremental API
///
/// ```ignore
/// use crypto::{ascon::AsconCxof128, Xof};
///
/// let mut cxof = AsconCxof128::new_with_customization(b"my-app-v1");
/// cxof.absorb(b"hello world");
/// let mut out = [0u8; 32];
/// cxof.squeeze(&mut out);
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct AsconCxof128 {
    state: State,
    buf: [u8; 8],
    buf_len: usize,
    squeezing: bool,
    squeeze_pos: usize,
    current_block: [u8; 8],
}

impl AsconCxof128 {
    /// Creates a new Ascon-CXOF128 with an empty customization string.
    #[inline]
    pub fn new() -> Self {
        Self::new_with_customization(&[])
    }

    /// Creates a new Ascon-CXOF128 with the given customization string.
    ///
    /// # Panics
    ///
    /// Panics if `z.len() > 256`.
    pub fn new_with_customization(z: &[u8]) -> Self {
        assert!(z.len() <= 256, "CXOF customization string must be at most 256 bytes");
        let mut state = State::init_hash(IV);
        p12(&mut state);

        // Absorb the bit-length of Z as a 64-bit little-endian integer
        let z_bits = (z.len() as u64).wrapping_mul(8);
        state.absorb_block(&z_bits.to_le_bytes());
        p12(&mut state);

        // Absorb Z itself
        if !z.is_empty() {
            let mut chunks = z.chunks_exact(8);
            for chunk in &mut chunks {
                state.absorb_block(chunk);
                p12(&mut state);
            }
            let remainder = chunks.remainder();
            if !remainder.is_empty() {
                let mut padded = [0u8; 8];
                padded[..remainder.len()].copy_from_slice(remainder);
                padded[remainder.len()] = 0x01;
                state.absorb_block(&padded);
                p12(&mut state);
            } else {
                // Z was a multiple of 8 bytes - add a pad block
                let mut padded = [0u8; 8];
                padded[0] = 0x01;
                state.absorb_block(&padded);
                p12(&mut state);
            }
        } else {
            // Empty Z: add a pad block [0x01, 0x00, ...]
            let mut padded = [0u8; 8];
            padded[0] = 0x01;
            state.absorb_block(&padded);
            p12(&mut state);
        }

        AsconCxof128 {
            state,
            buf: [0u8; 8],
            buf_len: 0,
            squeezing: false,
            squeeze_pos: 0,
            current_block: [0u8; 8],
        }
    }
}

impl Xof for AsconCxof128 {
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
    fn empty_cxof() {
        let mut out = [0u8; 64];
        let mut cxof = AsconCxof128::new();
        cxof.absorb(b"");
        cxof.squeeze(&mut out);
        let expected = hex::decode("4F50159EF70BB3DAD8807E034EAEBD44C4FA2CBBC8CF1F05511AB66CDCC529905CA12083FC186AD899B270B1473DC5F7EC88D1052082DCDFE69FB75D269E7B74").unwrap();
        assert_eq!(out.as_slice(), expected.as_slice());
    }

    #[test]
    fn cxof_with_customization() {
        let mut out = [0u8; 64];
        let mut cxof = AsconCxof128::new_with_customization(b"\x10");
        cxof.absorb(b"");
        cxof.squeeze(&mut out);
        let expected = hex::decode("0C93A483E7D574D49FE52CCE03EE646117977D57A8AA57704AB4DAF44B501430FF6AC11A5D1FD6F2154B5C65728268270C8BB578508487B8965718ADA6272FD6").unwrap();
        assert_eq!(out.as_slice(), expected.as_slice());
    }

    #[test]
    fn cxof_differs_from_xof() {
        let mut xof_out = [0u8; 32];
        {
            let mut xof = crate::ascon::AsconXof128::new();
            xof.absorb(b"test");
            xof.squeeze(&mut xof_out);
        }

        let mut cxof_out = [0u8; 32];
        let mut cxof = AsconCxof128::new_with_customization(b"test");
        cxof.absorb(b"");
        cxof.squeeze(&mut cxof_out);

        assert_ne!(xof_out, cxof_out, "CXOF with non-empty customization should differ from XOF");
    }

    #[test]
    #[should_panic(expected = "CXOF customization string must be at most 256 bytes")]
    fn cxof_customization_too_long() {
        AsconCxof128::new_with_customization(&[0u8; 257]);
    }

    #[test]
    fn kat_vectors() {
        let data = include_str!("../../testdata/ascon/LWC_CXOF_KAT_128_512.txt");
        let mut count = 0u64;
        let mut msg_hex = String::new();
        let mut z_hex = String::new();

        for line in data.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with("Count = ") {
                count = line["Count = ".len()..].parse().unwrap();
                msg_hex.clear();
                z_hex.clear();
                continue;
            }
            if line.starts_with("Msg = ") {
                msg_hex = line[6..].to_string();
                continue;
            }
            if line.starts_with("Z = ") {
                z_hex = line[4..].to_string();
                continue;
            }
            if line.starts_with("MD = ") {
                let expected_md: &str = &line[5..];
                let msg = hex::decode(&msg_hex).unwrap();
                let z = hex::decode(&z_hex).unwrap();
                let expected = hex::decode(expected_md).unwrap();
                let mut cxof = AsconCxof128::new_with_customization(&z);
                cxof.absorb(&msg);
                let mut out = vec![0u8; expected.len()];
                cxof.squeeze(&mut out);
                assert_eq!(out.as_slice(), expected.as_slice(), "KAT CXOF Count={count} mismatch");
                msg_hex.clear();
                z_hex.clear();
                continue;
            }
        }
    }
}
