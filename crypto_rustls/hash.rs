//! Hash algorithm implementations for rustls.
//!
//! Provides the [`rustls::crypto::hash::Hash`] implementations used by the
//! TLS 1.3 cipher suites: SHA-256 and SHA-384, backed by [`crypto::sha2`].

use crypto::Hasher;
use rustls::crypto::hash::{Context, Hash, HashAlgorithm, Output};

const SHA256_OUTPUT: usize = 32;
const SHA384_OUTPUT: usize = 48;

/// SHA-256, mapped to [`HashAlgorithm::SHA256`].
#[derive(Debug)]
pub(crate) struct Sha256;

impl Hash for Sha256 {
    fn start(&self) -> Box<dyn Context> {
        Box::new(Sha256Context(crypto::sha2::Sha256::new()))
    }

    fn hash(&self, data: &[u8]) -> Output {
        Output::new(crypto::sha2::Sha256::hash(data).as_ref())
    }

    fn output_len(&self) -> usize {
        SHA256_OUTPUT
    }

    fn algorithm(&self) -> HashAlgorithm {
        HashAlgorithm::SHA256
    }
}

struct Sha256Context(crypto::sha2::Sha256);

impl Context for Sha256Context {
    fn fork_finish(&self) -> Output {
        Output::new(self.0.clone().sum().as_ref())
    }

    fn fork(&self) -> Box<dyn Context> {
        Box::new(Self(self.0.clone()))
    }

    fn finish(self: Box<Self>) -> Output {
        Output::new(self.0.sum().as_ref())
    }

    fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }
}

/// SHA-384, mapped to [`HashAlgorithm::SHA384`].
#[derive(Debug)]
pub(crate) struct Sha384;

impl Hash for Sha384 {
    fn start(&self) -> Box<dyn Context> {
        Box::new(Sha384Context(crypto::sha2::Sha384::new()))
    }

    fn hash(&self, data: &[u8]) -> Output {
        Output::new(crypto::sha2::Sha384::hash(data).as_ref())
    }

    fn output_len(&self) -> usize {
        SHA384_OUTPUT
    }

    fn algorithm(&self) -> HashAlgorithm {
        HashAlgorithm::SHA384
    }
}

struct Sha384Context(crypto::sha2::Sha384);

impl Context for Sha384Context {
    fn fork_finish(&self) -> Output {
        Output::new(self.0.clone().sum().as_ref())
    }

    fn fork(&self) -> Box<dyn Context> {
        Box::new(Self(self.0.clone()))
    }

    fn finish(self: Box<Self>) -> Output {
        Output::new(self.0.sum().as_ref())
    }

    fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_known_vector() {
        // FIPS 180-4 example: SHA-256("abc")
        let expected = [
            0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae, 0x22, 0x23, 0xb0, 0x03,
            0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61, 0xf2, 0x00, 0x15, 0xad,
        ];
        assert_eq!(Sha256.hash(b"abc").as_ref(), expected);
    }

    #[test]
    fn sha384_known_vector() {
        // FIPS 180-4 example: SHA-384("abc")
        let expected = hex::decode(
            "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed\
             8086072ba1e7cc2358baeca134c825a7",
        )
        .unwrap();
        assert_eq!(Sha384.hash(b"abc").as_ref(), expected);
    }

    #[test]
    fn incremental_matches_one_shot() {
        let data = b"hello incremental world";
        let mut ctx = Sha256.start();
        ctx.update(&data[..5]);

        // A fork sees the same prefix but can diverge independently.
        let mut fork = ctx.fork();
        ctx.update(&data[5..]);
        fork.update(&data[5..]);

        let from_ctx = ctx.finish();
        let from_fork = fork.finish();
        assert_eq!(from_ctx.as_ref(), from_fork.as_ref());
        assert_eq!(from_ctx.as_ref(), Sha256.hash(data).as_ref());
    }

    #[test]
    fn sha384_incremental_matches_one_shot() {
        let data = b"another message to hash in one go";
        let mut ctx = Sha384.start();
        for chunk in data.chunks(7) {
            ctx.update(chunk);
        }
        assert_eq!(ctx.finish().as_ref(), Sha384.hash(data).as_ref());
    }
}
