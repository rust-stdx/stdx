//! HMAC implementations for rustls.
//!
//! Provides the [`rustls::crypto::hmac::Hmac`] implementations used by the
//! TLS 1.3 key schedule (via `HkdfUsingHmac`): HMAC-SHA256 and HMAC-SHA384,
//! backed by [`crypto::hmac`].

use rustls::crypto::hmac::{Hmac as HmacTrait, Key, Tag};

const SHA256_OUTPUT: usize = 32;
const SHA384_OUTPUT: usize = 48;

/// HMAC-SHA256.
#[derive(Debug)]
pub(crate) struct Sha256Hmac;

impl HmacTrait for Sha256Hmac {
    fn with_key(&self, key: &[u8]) -> Box<dyn Key> {
        Box::new(Sha256HmacKey(crypto::hmac::Hmac::<crypto::sha2::Sha256>::new(key)))
    }

    fn hash_output_len(&self) -> usize {
        SHA256_OUTPUT
    }
}

struct Sha256HmacKey(crypto::hmac::Hmac<crypto::sha2::Sha256>);

impl Key for Sha256HmacKey {
    fn sign_concat(&self, first: &[u8], middle: &[&[u8]], last: &[u8]) -> Tag {
        let mut ctx = self.0.clone();
        ctx.update(first);
        for chunk in middle {
            ctx.update(chunk);
        }
        ctx.update(last);
        Tag::new(ctx.finalize().as_ref())
    }

    fn tag_len(&self) -> usize {
        SHA256_OUTPUT
    }
}

/// HMAC-SHA384.
#[derive(Debug)]
pub(crate) struct Sha384Hmac;

impl HmacTrait for Sha384Hmac {
    fn with_key(&self, key: &[u8]) -> Box<dyn Key> {
        Box::new(Sha384HmacKey(crypto::hmac::Hmac::<crypto::sha2::Sha384>::new(key)))
    }

    fn hash_output_len(&self) -> usize {
        SHA384_OUTPUT
    }
}

struct Sha384HmacKey(crypto::hmac::Hmac<crypto::sha2::Sha384>);

impl Key for Sha384HmacKey {
    fn sign_concat(&self, first: &[u8], middle: &[&[u8]], last: &[u8]) -> Tag {
        let mut ctx = self.0.clone();
        ctx.update(first);
        for chunk in middle {
            ctx.update(chunk);
        }
        ctx.update(last);
        Tag::new(ctx.finalize().as_ref())
    }

    fn tag_len(&self) -> usize {
        SHA384_OUTPUT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // RFC 4231 test case 1: key = 20 x 0x0b, data = "Hi There".
    #[test]
    fn hmac_sha256_rfc4231_case1() {
        let key = [0x0b; 20];
        let tag = Sha256Hmac.with_key(&key).sign(&[b"Hi There"]);
        assert_eq!(
            tag.as_ref(),
            hex::decode("b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7").unwrap()
        );
    }

    // RFC 4231 test case 1 with SHA-384.
    #[test]
    fn hmac_sha384_rfc4231_case1() {
        let key = [0x0b; 20];
        let tag = Sha384Hmac.with_key(&key).sign(&[b"Hi There"]);
        assert_eq!(
            tag.as_ref(),
            hex::decode(
                "afd03944d84895626b0825f4ab46907f15f9dadbe4101ec682aa034c7cebc59c\
                 faea9ea9076ede7f4af152e8b2fa9cb6"
            )
            .unwrap()
        );
    }

    #[test]
    fn sign_concat_matches_sign() {
        let key = b"some key";
        let parts: [&[u8]; 3] = [b"hello", b" ", b"world"];
        let concat = Sha256Hmac.with_key(key).sign(&parts);
        // `sign` delegates to `sign_concat`; check a manual join agrees.
        let joined = [parts[0], parts[1], parts[2]].concat();
        assert_eq!(concat.as_ref(), Sha256Hmac.with_key(key).sign(&[&joined]).as_ref());
    }
}
