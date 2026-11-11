use super::keccak::KeccakSponge;

/// Reader returned by `finalize_xof` on the SHA-3 XOFs.
///
/// Produces the output stream of a finalized [`Shake128`], [`Shake256`],
/// [`CShake256`], [`Kmac256`] or [`ParallelHash256`] instance. Squeezing is
/// the only operation available: once a reader exists, the input phase is
/// over.
///
/// [`squeeze`](Self::squeeze) may be called any number of times. For a given
/// reader, a shorter output is always a prefix of a longer one.
///
/// [`Shake128`]: super::Shake128
/// [`Shake256`]: super::Shake256
/// [`CShake256`]: super::CShake256
/// [`Kmac256`]: super::Kmac256
/// [`ParallelHash256`]: super::ParallelHash256
#[derive(Clone)]
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub struct XofReader {
    keccak: KeccakSponge<24>,
}

impl XofReader {
    #[inline]
    pub(crate) fn new(keccak: KeccakSponge<24>) -> Self {
        return XofReader {
            keccak,
        };
    }

    /// Squeezes the next `out.len()` bytes of the output stream.
    ///
    /// `out` may be empty. Shorter outputs are prefixes of longer ones for the
    /// same reader.
    #[inline]
    pub fn squeeze(&mut self, out: &mut [u8]) {
        self.keccak.squeeze(out);
    }
}

#[cfg(feature = "std")]
impl std::io::Read for XofReader {
    /// Fills `buf` with the next `buf.len()` bytes of the output stream.
    ///
    /// This stream has no end: a non-empty `buf` always advances the reader.
    /// Use [`std::io::Read::take`] or [`std::io::Read::read_exact`] with a
    /// fixed-size buffer. In particular, [`std::io::Read::read_to_end`] and
    /// [`std::io::Read::read_to_string`] never return on this reader.
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.squeeze(buf);
        return Ok(buf.len());
    }
}

#[cfg(test)]
mod tests {
    use crate::{Xof, sha3::Shake256};

    #[test]
    fn squeeze_is_chunk_independent() {
        let mut hasher = Shake256::new();
        hasher.absorb(b"abc");
        let mut one_shot = [0u8; 64];
        hasher.finalize_xof().squeeze(&mut one_shot);

        let mut hasher = Shake256::new();
        hasher.absorb(b"abc");
        let mut reader = hasher.finalize_xof();
        let mut chunked = [0u8; 64];
        let mut offset = 0usize;
        for take in [1, 7, 55, 1] {
            reader.squeeze(&mut chunked[offset..offset + take]);
            offset += take;
        }
        assert_eq!(offset, chunked.len());
        assert_eq!(chunked, one_shot);
    }

    #[test]
    fn short_output_is_a_prefix() {
        let mut hasher = Shake256::new();
        hasher.absorb(b"abc");
        let mut short = [0u8; 16];
        hasher.finalize_xof().squeeze(&mut short);

        let mut hasher = Shake256::new();
        hasher.absorb(b"abc");
        let mut long = [0u8; 64];
        hasher.finalize_xof().squeeze(&mut long);

        assert_eq!(short, long[..16]);
    }

    #[test]
    fn empty_squeeze_does_not_advance() {
        let mut hasher = Shake256::new();
        hasher.absorb(b"abc");
        let mut expected = [0u8; 64];
        hasher.finalize_xof().squeeze(&mut expected);

        let mut hasher = Shake256::new();
        hasher.absorb(b"abc");
        let mut reader = hasher.finalize_xof();
        reader.squeeze(&mut []);
        let mut actual = [0u8; 64];
        reader.squeeze(&mut actual);
        assert_eq!(actual, expected);
    }

    #[cfg(feature = "std")]
    #[test]
    fn implements_std_read() {
        use std::io::Read;

        let mut hasher = Shake256::new();
        hasher.absorb(b"abc");
        let mut reader = hasher.finalize_xof();

        let mut out = [0u8; 64];
        reader.read_exact(&mut out).unwrap();
        assert_eq!(
            hex::encode(out),
            "483366601360a8771c6863080cc4114d8db44530f8f1e1ee4f94ea37e78b5739d5a15bef186a5386c75744c0527e1faa9f8726e462a12a4feb06bd8801e751e4"
        );

        // `read` on an empty buffer is a no-op.
        assert_eq!(reader.read(&mut []).unwrap(), 0);

        // A bounded read through `Take` produces the same stream.
        let mut hasher = Shake256::new();
        hasher.absorb(b"abc");
        let mut bounded = hasher.finalize_xof().take(16);
        let mut short = [0u8; 16];
        bounded.read_exact(&mut short).unwrap();
        assert_eq!(short, out[..16]);
    }
}
