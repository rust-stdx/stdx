use alloc::{boxed::Box, format};

use crypto::{
    Aead as CryptoAead, Hasher, StreamCipher, hkdf,
    hmac::Hmac,
    mlkem::{CIPHERTEXT_SIZE_768, PUBLIC_KEY_SIZE_768, PublicKey768, SecretKey768},
    sha2::Sha256,
};
use heapless::Vec;

use crate::{
    Error,
    crypto::{
        Aead, CipherSuite, CryptoProvider, KeyExchangeGroup, KeyExchangeKeyPair, MAX_AEAD_TAG_SIZE, MAX_HASH_OUTPUT,
        MAX_KX_PUBLIC_KEY, MAX_PUBLIC_KEY_BYTES, MAX_SHARED_SECRET, MAX_SIGNATURE_SIZE, SignatureScheme, Signer,
    },
    error::{CertificateValidationFailure, CryptoFailure, InvalidKeyFailure},
};

/// The default [`CryptoProvider`], backed by the `crypto` crate.
///
/// Supports:
/// - Cipher suites: `TLS_CHACHA20_POLY1305_SHA256`, `TLS_AES_128_GCM_SHA256`
/// - Key exchange: X25519, X25519MLKEM768 (post-quantum hybrid)
/// - Signatures: Ed25519, ECDSA P-256 SHA-256, ECDSA P-384 SHA-384, RSA-PSS with SHA-256
#[derive(Debug, Clone)]
pub struct DefaultCryptoProvider {
    cipher_suites: [CipherSuite; 2],
    key_exchange_groups: [KeyExchangeGroup; 2],
    signature_schemes: [SignatureScheme; 5],
}

impl DefaultCryptoProvider {
    pub fn new() -> Self {
        Self {
            cipher_suites: [CipherSuite::TlsAes128GcmSha256, CipherSuite::TlsChaCha20Poly1305Sha256],
            key_exchange_groups: [KeyExchangeGroup::X25519MlKem768, KeyExchangeGroup::X25519],
            signature_schemes: [
                SignatureScheme::Ed25519,
                SignatureScheme::EcdsaP256Sha256,
                SignatureScheme::EcdsaP384Sha384,
                SignatureScheme::RsaPssRsaSha256,
                SignatureScheme::RsaPkcs1Sha256,
            ],
        }
    }
}

impl Default for DefaultCryptoProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl CryptoProvider for DefaultCryptoProvider {
    fn supported_cipher_suites(&self) -> &[CipherSuite] {
        &self.cipher_suites
    }

    fn supported_key_exchange_groups(&self) -> &[KeyExchangeGroup] {
        &self.key_exchange_groups
    }

    fn supported_signature_schemes(&self) -> &[SignatureScheme] {
        &self.signature_schemes
    }

    fn create_aead(&self, suite: CipherSuite, key: &[u8]) -> Result<Box<dyn Aead>, Error> {
        match suite {
            CipherSuite::TlsAes128GcmSha256 => {
                let key: [u8; 16] = key.try_into().map_err(|_| {
                    Error::InvalidKey(InvalidKeyFailure::WrongLength {
                        algorithm: "AES-128-GCM",
                        expected: 16,
                    })
                })?;
                Ok(Box::new(Aes128GcmAead::new(&key)))
            }
            CipherSuite::TlsChaCha20Poly1305Sha256 => {
                let key: [u8; 32] = key.try_into().map_err(|_| {
                    Error::InvalidKey(InvalidKeyFailure::WrongLength {
                        algorithm: "ChaCha20-Poly1305",
                        expected: 32,
                    })
                })?;
                Ok(Box::new(ChaCha20Poly1305Aead::new(&key)))
            }
            ciphersuite @ _ => {
                let msg = alloc::format!("{ciphersuite:?}");
                Err(Error::CryptoError(CryptoFailure::UnsupportedCipherSuite(msg.into())))
            }
        }
    }

    fn create_kx_pair(&self, group: KeyExchangeGroup) -> Result<Box<dyn KeyExchangeKeyPair>, Error> {
        match group {
            KeyExchangeGroup::X25519 => {
                let sk = crypto::curve25519::x25519::SecretKey::generate();
                Ok(Box::new(X25519KxKeyPair(sk)))
            }
            KeyExchangeGroup::X25519MlKem768 => {
                let x25519_sk = crypto::curve25519::x25519::SecretKey::generate();
                let x25519_pk_bytes = x25519_sk.public_key().to_bytes();
                let (mlkem_sk, mlkem_pk) = SecretKey768::generate();
                Ok(Box::new(X25519MlKem768KxKeyPair {
                    x25519_secret: x25519_sk,
                    x25519_public_bytes: x25519_pk_bytes,
                    mlkem_secret: Some(mlkem_sk),
                    mlkem_public: Some(mlkem_pk),
                    peer_x25519_public_key: None,
                    our_public_bytes: None,
                    mlkem_shared_secret: None,
                }))
            }
        }
    }

    fn create_signer(&self, scheme: SignatureScheme, secret_key: &[u8]) -> Result<Box<dyn Signer>, Error> {
        match scheme {
            SignatureScheme::Ed25519 => {
                let seed: [u8; 32] = secret_key.try_into().map_err(|_| {
                    Error::InvalidKey(InvalidKeyFailure::WrongLength {
                        algorithm: "Ed25519",
                        expected: 32,
                    })
                })?;
                let sk = crypto::curve25519::ed25519::SecretKey::from_bytes(&seed);
                let pk = sk.public_key();
                Ok(Box::new(Ed25519Signer {
                    sk,
                    pk_bytes: pk.to_bytes(),
                }))
            }
            SignatureScheme::EcdsaP256Sha256 => {
                let key_bytes: [u8; 32] = secret_key.try_into().map_err(|_| {
                    Error::InvalidKey(InvalidKeyFailure::WrongLength {
                        algorithm: "P-256",
                        expected: 32,
                    })
                })?;
                let pk = crypto::p256::PrivateKey::from_bytes(&key_bytes).map_err(|_| {
                    Error::InvalidKey(InvalidKeyFailure::ParseError("invalid P-256 private key".into()))
                })?;
                let pk_bytes = pk.public_key().to_bytes();
                Ok(Box::new(P256Signer {
                    pk,
                    pk_bytes,
                }))
            }
            SignatureScheme::EcdsaP384Sha384 => {
                let key_bytes: [u8; 48] = secret_key.try_into().map_err(|_| {
                    Error::InvalidKey(InvalidKeyFailure::WrongLength {
                        algorithm: "P-384",
                        expected: 48,
                    })
                })?;
                let pk = crypto::p384::PrivateKey::from_bytes(&key_bytes).map_err(|_| {
                    Error::InvalidKey(InvalidKeyFailure::ParseError("invalid P-384 private key".into()))
                })?;
                let pk_bytes = pk.public_key().to_bytes();
                Ok(Box::new(P384Signer {
                    pk,
                    pk_bytes,
                }))
            }
            scheme @ _ => Err(Error::CryptoError(CryptoFailure::UnsupportedSignatureScheme(
                format!("{scheme:?}").into(),
            ))),
        }
    }

    fn verify_signature(
        &self,
        scheme: SignatureScheme,
        public_key: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> Result<(), Error> {
        match scheme {
            SignatureScheme::Ed25519 => {
                let pk_bytes: [u8; 32] = public_key.try_into().map_err(|_| {
                    Error::InvalidKey(InvalidKeyFailure::WrongLength {
                        algorithm: "Ed25519 public key",
                        expected: 32,
                    })
                })?;
                let pk = crypto::curve25519::ed25519::PublicKey::from_bytes(&pk_bytes).map_err(|_| {
                    Error::InvalidKey(InvalidKeyFailure::ParseError("invalid Ed25519 public key".into()))
                })?;
                let sig: [u8; 64] = signature.try_into().map_err(|_| {
                    Error::InvalidKey(InvalidKeyFailure::WrongLength {
                        algorithm: "Ed25519 signature",
                        expected: 64,
                    })
                })?;
                pk.verify(data, &sig).map_err(|_| {
                    Error::CertificateValidationFailed(CertificateValidationFailure::SignatureVerificationFailed)
                })
            }
            SignatureScheme::EcdsaP256Sha256 => {
                let pk = crypto::p256::PublicKey::from_bytes(public_key)
                    .map_err(|_| Error::InvalidKey(InvalidKeyFailure::ParseError("invalid P-256 public key".into())))?;
                let sig_raw = der_to_raw_p256_signature(signature)?;
                pk.verify(data, &sig_raw).map_err(|_| {
                    Error::CertificateValidationFailed(CertificateValidationFailure::SignatureVerificationFailed)
                })
            }
            SignatureScheme::EcdsaP384Sha384 => {
                let pk = crypto::p384::PublicKey::from_bytes(public_key)
                    .map_err(|_| Error::InvalidKey(InvalidKeyFailure::ParseError("invalid P-384 public key".into())))?;
                let sig_raw = der_to_raw_p384_signature(signature)?;
                pk.verify(data, &sig_raw).map_err(|_| {
                    Error::CertificateValidationFailed(CertificateValidationFailure::SignatureVerificationFailed)
                })
            }
            SignatureScheme::RsaPkcs1Sha256 => {
                crypto::rsa::verify_pkcs1_sha256(public_key, signature, data).map_err(|e| {
                    Error::CryptoError(CryptoFailure::RsaVerification(format!("RSA-PKCS1-SHA256: {e}").into()))
                })
            }
            SignatureScheme::RsaPssRsaSha256 => crypto::rsa::verify_pss_sha256(public_key, signature, data)
                .map_err(|e| Error::CryptoError(CryptoFailure::RsaVerification(format!("RSA-PSS-SHA256: {e}").into()))),
            scheme @ _ => Err(Error::CryptoError(CryptoFailure::UnsupportedSignatureScheme(
                format!("{scheme:?}").into(),
            ))),
        }
    }

    fn hash(&self, suite: CipherSuite, data: &[u8]) -> Vec<u8, MAX_HASH_OUTPUT> {
        let mut out = Vec::new();
        match suite {
            CipherSuite::TlsChaCha20Poly1305Sha256 => {
                out.extend_from_slice(Sha256::hash(data).as_ref()).unwrap();
            }
            CipherSuite::TlsAes128GcmSha256 => {
                out.extend_from_slice(Sha256::hash(data).as_ref()).unwrap();
            }
            ciphersuite @ _ => unreachable!("Ciphersuite {ciphersuite:?} not supported"),
        }
        out
    }

    fn secure_random(&self, buf: &mut [u8]) {
        crypto::random_fill(buf);
    }

    fn hmac(&self, suite: CipherSuite, key: &[u8], data: &[u8]) -> Vec<u8, MAX_HASH_OUTPUT> {
        let mut out = Vec::new();
        match suite {
            CipherSuite::TlsChaCha20Poly1305Sha256 => {
                out.extend_from_slice(Hmac::<Sha256>::mac(key, data).as_ref()).unwrap();
            }
            CipherSuite::TlsAes128GcmSha256 => {
                out.extend_from_slice(Hmac::<Sha256>::mac(key, data).as_ref()).unwrap();
            }
            ciphersuite @ _ => unreachable!("Ciphersuite {ciphersuite:?} not supported"),
        }
        out
    }

    fn hkdf_extract(&self, suite: CipherSuite, salt: &[u8], ikm: &[u8]) -> Vec<u8, MAX_HASH_OUTPUT> {
        let mut out = Vec::new();
        match suite {
            CipherSuite::TlsChaCha20Poly1305Sha256 => {
                out.extend_from_slice(hkdf::extract::<Sha256>(Some(salt), ikm).as_ref())
                    .unwrap();
            }
            CipherSuite::TlsAes128GcmSha256 => {
                out.extend_from_slice(hkdf::extract::<Sha256>(Some(salt), ikm).as_ref())
                    .unwrap();
            }
            ciphersuite @ _ => unreachable!("Ciphersuite {ciphersuite:?} not supported"),
        }
        out
    }

    fn hkdf_expand(&self, suite: CipherSuite, prk: &[u8], info: &[u8], length: usize) -> Vec<u8, MAX_HASH_OUTPUT> {
        fn expand_inner<H: Hasher>(prk: &[u8], info: &[u8], length: usize) -> Vec<u8, MAX_HASH_OUTPUT> {
            let mut out = Vec::new();
            match length {
                12 => {
                    let arr = hkdf::expand::<H, 12>(prk, info).unwrap();
                    out.extend_from_slice(&arr).unwrap();
                }
                16 => {
                    let arr = hkdf::expand::<H, 16>(prk, info).unwrap();
                    out.extend_from_slice(&arr).unwrap();
                }
                32 => {
                    let arr = hkdf::expand::<H, 32>(prk, info).unwrap();
                    out.extend_from_slice(&arr).unwrap();
                }
                48 => {
                    let arr = hkdf::expand::<H, 48>(prk, info).unwrap();
                    out.extend_from_slice(&arr).unwrap();
                }
                _ => unreachable!("hkdf_expand: unsupported output length {length}"),
            }
            out
        }
        match suite {
            CipherSuite::TlsChaCha20Poly1305Sha256 => expand_inner::<Sha256>(prk, info, length),
            CipherSuite::TlsAes128GcmSha256 => expand_inner::<Sha256>(prk, info, length),
            ciphersuite @ _ => unreachable!("Ciphersuite {ciphersuite:?} not supported"),
        }
    }

    fn hkdf_expand_label(
        &self,
        suite: CipherSuite,
        secret: &[u8],
        label: &[u8],
        context: &[u8],
        length: usize,
    ) -> Vec<u8, MAX_HASH_OUTPUT> {
        let mut hkdf_label = Vec::<u8, 128>::new();
        hkdf_label.extend_from_slice(&(length as u16).to_be_bytes()).unwrap();
        hkdf_label.push(label.len() as u8).unwrap();
        hkdf_label.extend_from_slice(label).unwrap();
        hkdf_label.push(context.len() as u8).unwrap();
        hkdf_label.extend_from_slice(context).unwrap();
        self.hkdf_expand(suite, secret, &hkdf_label, length)
    }

    fn header_protection_mask(&self, suite: CipherSuite, hp_key: &[u8], sample: &[u8; 16]) -> Result<[u8; 16], Error> {
        match suite {
            CipherSuite::TlsAes128GcmSha256 => {
                use crypto::aes::{encrypt_block_aes128, key_expand_128};
                let key: &[u8; 16] = hp_key.try_into().map_err(|_| {
                    Error::InvalidKey(InvalidKeyFailure::WrongLength {
                        algorithm: "AES-128 HP key",
                        expected: 16,
                    })
                })?;
                let rk = key_expand_128(key);
                Ok(encrypt_block_aes128(&rk, sample))
            }
            CipherSuite::TlsAes256GcmSha384 => {
                use crypto::aes::{encrypt_block, key_expand};
                let key: &[u8; 32] = hp_key.try_into().map_err(|_| {
                    Error::InvalidKey(InvalidKeyFailure::WrongLength {
                        algorithm: "AES-256 HP key",
                        expected: 32,
                    })
                })?;
                let rk = key_expand(key);
                Ok(encrypt_block(&rk, sample))
            }
            CipherSuite::TlsChaCha20Poly1305Sha256 => {
                use crypto::chacha::ChaCha;
                let key: &[u8; 32] = hp_key.try_into().map_err(|_| {
                    Error::InvalidKey(InvalidKeyFailure::WrongLength {
                        algorithm: "ChaCha20 HP key",
                        expected: 32,
                    })
                })?;
                let mut nonce = [0u8; 12];
                nonce.copy_from_slice(&sample[4..16]);
                let ctr = u32::from_le_bytes([sample[0], sample[1], sample[2], sample[3]]);
                let mut cipher = ChaCha::<20, true>::new(key, &nonce);
                cipher.set_counter(ctr as u32);
                let mut out = [0u8; 64];
                cipher.xor_keystream(&mut out);
                let mut mask = [0u8; 16];
                mask.copy_from_slice(&out[..16]);
                Ok(mask)
            }
        }
    }
}

// ── AEAD wrappers ─────────────────────────────────────────────────────────

struct Aes128GcmAead(crypto::aes::Aes128Gcm);

impl Aes128GcmAead {
    fn new(key: &[u8; 16]) -> Self {
        Self(crypto::aes::Aes128Gcm::new(key))
    }
}

impl Aead for Aes128GcmAead {
    fn encrypt(&self, buf: &mut [u8], nonce: &[u8], aad: &[u8]) -> heapless::Vec<u8, MAX_AEAD_TAG_SIZE> {
        let tag = self.0.encrypt_in_place(buf, nonce, aad);
        tag.as_ref().try_into().unwrap()
    }

    fn decrypt(&self, buf: &mut [u8], nonce: &[u8], aad: &[u8]) -> Result<usize, Error> {
        let tag_size = self.tag_size();
        if buf.len() < tag_size {
            return Err(Error::DecryptFailed);
        }
        let (ciphertext, tag) = buf.split_at_mut(buf.len() - tag_size);
        let plaintext_len = ciphertext.len();
        self.0
            .decrypt_in_place(ciphertext, nonce, aad, tag)
            .map_err(|_| Error::DecryptFailed)?;
        Ok(plaintext_len)
    }

    #[inline]
    fn key_size(&self) -> usize {
        16
    }

    #[inline]
    fn nonce_size(&self) -> usize {
        12
    }

    #[inline]
    fn tag_size(&self) -> usize {
        16
    }
}

// struct Aes256GcmAead(crypto::aes::Aes256Gcm);

// impl Aes256GcmAead {
//     fn new(key: &[u8; 32]) -> Self {
//         Self(crypto::aes::Aes256Gcm::new(key))
//     }
// }

// impl Aead for Aes256GcmAead {
//     fn encrypt(&self, buf: &mut [u8], nonce: &[u8], aad: &[u8]) -> heapless::Vec<u8, MAX_AEAD_TAG_SIZE> {
//         let tag = self.0.encrypt_in_place(buf, nonce, aad);
//         tag.as_ref().try_into().unwrap()
//     }

//     fn decrypt(&self, buf: &mut [u8], nonce: &[u8], aad: &[u8]) -> Result<usize, Error> {
//         let tag_size = self.tag_size();
//         if buf.len() < tag_size {
//             return Err(Error::DecryptFailed);
//         }
//         let (ciphertext, tag) = buf.split_at_mut(buf.len() - tag_size);
//         let plaintext_len = ciphertext.len();
//         self.0
//             .decrypt_in_place(ciphertext, nonce, aad, tag)
//             .map_err(|_| Error::DecryptFailed)?;
//         Ok(plaintext_len)
//     }

//     #[inline]
//     fn key_size(&self) -> usize {
//         32
//     }

//     #[inline]
//     fn nonce_size(&self) -> usize {
//         12
//     }

//     #[inline]
//     fn tag_size(&self) -> usize {
//         16
//     }
// }

struct ChaCha20Poly1305Aead(crypto::chacha::ChaCha20Poly1305);

impl ChaCha20Poly1305Aead {
    fn new(key: &[u8; 32]) -> Self {
        Self(crypto::chacha::ChaCha20Poly1305::new(key))
    }
}

impl Aead for ChaCha20Poly1305Aead {
    fn encrypt(&self, buf: &mut [u8], nonce: &[u8], aad: &[u8]) -> heapless::Vec<u8, MAX_AEAD_TAG_SIZE> {
        let tag = self.0.encrypt_in_place(buf, nonce, aad);
        tag.as_ref().try_into().unwrap()
    }

    fn decrypt(&self, buf: &mut [u8], nonce: &[u8], aad: &[u8]) -> Result<usize, Error> {
        let tag_size = self.tag_size();
        if buf.len() < tag_size {
            return Err(Error::DecryptFailed);
        }
        let (ciphertext, tag) = buf.split_at_mut(buf.len() - tag_size);
        let plaintext_len = ciphertext.len();
        self.0
            .decrypt_in_place(ciphertext, nonce, aad, tag)
            .map_err(|_| Error::DecryptFailed)?;
        Ok(plaintext_len)
    }

    #[inline]
    fn key_size(&self) -> usize {
        32
    }

    #[inline]
    fn nonce_size(&self) -> usize {
        12
    }

    #[inline]
    fn tag_size(&self) -> usize {
        16
    }
}

// ── KX wrapper ────────────────────────────────────────────────────────────

struct X25519KxKeyPair(crypto::curve25519::x25519::SecretKey);

impl KeyExchangeKeyPair for X25519KxKeyPair {
    fn group(&self) -> KeyExchangeGroup {
        KeyExchangeGroup::X25519
    }

    fn public_key_bytes(&self) -> Vec<u8, MAX_KX_PUBLIC_KEY> {
        let arr = self.0.public_key().to_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&arr).unwrap();
        out
    }

    fn shared_secret(&self, peer_public_key: &[u8]) -> Result<Vec<u8, MAX_SHARED_SECRET>, Error> {
        let peer_bytes: [u8; 32] = peer_public_key.try_into().map_err(|_| {
            Error::InvalidKey(InvalidKeyFailure::WrongLength {
                algorithm: "X25519",
                expected: 32,
            })
        })?;
        let peer = crypto::curve25519::x25519::PublicKey::from_bytes(&peer_bytes);
        let arr = self.0.ecdh(&peer);
        let mut out = Vec::new();
        out.extend_from_slice(&arr).unwrap();
        Ok(out)
    }
}

struct X25519MlKem768KxKeyPair {
    x25519_secret: crypto::curve25519::x25519::SecretKey,
    x25519_public_bytes: [u8; 32],
    mlkem_secret: Option<SecretKey768>,
    mlkem_public: Option<PublicKey768>,
    peer_x25519_public_key: Option<crypto::curve25519::x25519::PublicKey>,
    our_public_bytes: Option<Vec<u8, MAX_KX_PUBLIC_KEY>>,
    mlkem_shared_secret: Option<[u8; 32]>,
}

impl KeyExchangeKeyPair for X25519MlKem768KxKeyPair {
    fn group(&self) -> KeyExchangeGroup {
        KeyExchangeGroup::X25519MlKem768
    }

    fn public_key_bytes(&self) -> Vec<u8, MAX_KX_PUBLIC_KEY> {
        if let Some(ref cached) = self.our_public_bytes {
            cached.clone()
        } else {
            let mlkem_pk = self
                .mlkem_public
                .as_ref()
                .expect("mlkem_public must be set for client-side key share")
                .to_bytes();
            let mut buf = Vec::new();
            buf.extend_from_slice(&mlkem_pk).unwrap();
            buf.extend_from_slice(&self.x25519_public_bytes).unwrap();
            buf
        }
    }

    fn set_peer_public_key(&mut self, peer_public_key: &[u8]) -> Result<(), Error> {
        let total_size = PUBLIC_KEY_SIZE_768 + 32;
        if peer_public_key.len() != total_size {
            return Err(Error::InvalidKey(InvalidKeyFailure::Other(
                format!(
                    "X25519MLKEM768 peer (client) key must be {} bytes, got {}",
                    total_size,
                    peer_public_key.len()
                )
                .into(),
            )));
        }

        let peer_mlkem_pk = PublicKey768::from_bytes(peer_public_key[..PUBLIC_KEY_SIZE_768].try_into().unwrap());
        let peer_x25519_pk = crypto::curve25519::x25519::PublicKey::from_bytes(
            peer_public_key[PUBLIC_KEY_SIZE_768..].try_into().unwrap(),
        );

        let (ct, ss) = peer_mlkem_pk.encapsulate();

        let mut buf = Vec::new();
        buf.extend_from_slice(&ct).unwrap();
        buf.extend_from_slice(&self.x25519_public_bytes).unwrap();
        self.our_public_bytes = Some(buf);
        self.peer_x25519_public_key = Some(peer_x25519_pk);
        self.mlkem_shared_secret = Some(ss);

        Ok(())
    }

    fn shared_secret(&self, peer_public_key: &[u8]) -> Result<Vec<u8, MAX_SHARED_SECRET>, Error> {
        let x25519_ss = if let Some(ref peer_pk) = self.peer_x25519_public_key {
            self.x25519_secret.ecdh(peer_pk)
        } else {
            let total_size = CIPHERTEXT_SIZE_768 + 32;
            if peer_public_key.len() != total_size {
                return Err(Error::InvalidKey(InvalidKeyFailure::Other(
                    format!(
                        "X25519MLKEM768 peer (server) key must be {} bytes, got {}",
                        total_size,
                        peer_public_key.len()
                    )
                    .into(),
                )));
            }
            let peer_x25519_pk = crypto::curve25519::x25519::PublicKey::from_bytes(
                peer_public_key[CIPHERTEXT_SIZE_768..].try_into().unwrap(),
            );
            self.x25519_secret.ecdh(&peer_x25519_pk)
        };

        let mlkem_ss = if let Some(ss) = self.mlkem_shared_secret {
            ss
        } else {
            let ct: [u8; CIPHERTEXT_SIZE_768] = peer_public_key[..CIPHERTEXT_SIZE_768].try_into().map_err(|_| {
                Error::InvalidKey(InvalidKeyFailure::ParseError(
                    "X25519MLKEM768 peer key: ML-KEM ciphertext too short".into(),
                ))
            })?;
            self.mlkem_secret
                .as_ref()
                .ok_or_else(|| {
                    Error::InvalidKey(InvalidKeyFailure::Other(
                        "X25519MLKEM768: no ML-KEM secret key available for decapsulation".into(),
                    ))
                })?
                .decapsulate(&ct)
                .map_err(|e| {
                    Error::InvalidKey(InvalidKeyFailure::Other(format!("ML-KEM decapsulation failed: {e}").into()))
                })?
        };

        let mut shared = Vec::new();
        shared.extend_from_slice(&mlkem_ss).unwrap();
        shared.extend_from_slice(&x25519_ss).unwrap();
        Ok(shared)
    }
}

// ── Signer / verifier wrappers ────────────────────────────────────────────

struct Ed25519Signer {
    sk: crypto::curve25519::ed25519::SecretKey,
    pk_bytes: [u8; 32],
}

impl Signer for Ed25519Signer {
    fn scheme(&self) -> SignatureScheme {
        SignatureScheme::Ed25519
    }

    fn public_key_bytes(&self) -> Vec<u8, MAX_PUBLIC_KEY_BYTES> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.pk_bytes).unwrap();
        out
    }

    fn sign(&self, data: &[u8]) -> Result<Vec<u8, MAX_SIGNATURE_SIZE>, Error> {
        let arr = self.sk.sign(data);
        let mut out = Vec::new();
        out.extend_from_slice(&arr).unwrap();
        Ok(out)
    }
}

struct P256Signer {
    pk: crypto::p256::PrivateKey,
    pk_bytes: [u8; 65],
}

impl Signer for P256Signer {
    fn scheme(&self) -> SignatureScheme {
        SignatureScheme::EcdsaP256Sha256
    }

    fn public_key_bytes(&self) -> Vec<u8, MAX_PUBLIC_KEY_BYTES> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.pk_bytes).unwrap();
        out
    }

    fn sign(&self, data: &[u8]) -> Result<Vec<u8, MAX_SIGNATURE_SIZE>, Error> {
        let raw = self
            .pk
            .sign(data)
            .map_err(|_| Error::CryptoError(CryptoFailure::SigningFailed))?;
        Ok(raw_p256_to_der(&raw))
    }
}

struct P384Signer {
    pk: crypto::p384::PrivateKey,
    pk_bytes: [u8; 97],
}

impl Signer for P384Signer {
    fn scheme(&self) -> SignatureScheme {
        SignatureScheme::EcdsaP384Sha384
    }

    fn public_key_bytes(&self) -> Vec<u8, MAX_PUBLIC_KEY_BYTES> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.pk_bytes).unwrap();
        out
    }

    fn sign(&self, data: &[u8]) -> Result<Vec<u8, MAX_SIGNATURE_SIZE>, Error> {
        let raw = self
            .pk
            .sign(data)
            .map_err(|_| Error::CryptoError(CryptoFailure::SigningFailed))?;
        Ok(raw_p384_to_der(&raw))
    }
}

fn raw_p256_to_der(raw: &[u8; 64]) -> Vec<u8, MAX_SIGNATURE_SIZE> {
    let r = trim_leading_zeros(&raw[..32]);
    let s = trim_leading_zeros(&raw[32..]);
    let mut der = Vec::new();
    der.push(0x30).unwrap();
    let len_pos = der.len();
    der.push(0).unwrap();
    der.push(0x02).unwrap();
    der.push(r.len() as u8).unwrap();
    der.extend_from_slice(&r).unwrap();
    der.push(0x02).unwrap();
    der.push(s.len() as u8).unwrap();
    der.extend_from_slice(&s).unwrap();
    der[len_pos] = (der.len() - len_pos - 1) as u8;
    der
}

fn raw_p384_to_der(raw: &[u8; 96]) -> Vec<u8, MAX_SIGNATURE_SIZE> {
    let r = trim_leading_zeros(&raw[..48]);
    let s = trim_leading_zeros(&raw[48..]);
    let mut der = Vec::new();
    der.push(0x30).unwrap();
    let len_pos = der.len();
    der.push(0).unwrap();
    der.push(0x02).unwrap();
    der.push(r.len() as u8).unwrap();
    der.extend_from_slice(&r).unwrap();
    der.push(0x02).unwrap();
    der.push(s.len() as u8).unwrap();
    der.extend_from_slice(&s).unwrap();
    der[len_pos] = (der.len() - len_pos - 1) as u8;
    der
}

fn trim_leading_zeros(bytes: &[u8]) -> Vec<u8, 128> {
    let pos = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
    let trimmed = &bytes[pos..];
    let mut out = Vec::new();
    if trimmed.is_empty() {
        out.push(0).unwrap();
    } else if trimmed[0] & 0x80 != 0 {
        out.push(0).unwrap();
        out.extend_from_slice(trimmed).unwrap();
    } else {
        out.extend_from_slice(trimmed).unwrap();
    }
    out
}

fn der_to_raw_p256_signature(der: &[u8]) -> Result<[u8; 64], Error> {
    if der.len() < 8 {
        return Err(Error::InvalidKey(InvalidKeyFailure::DerError("too short".into())));
    }
    if der[0] != 0x30 {
        return Err(Error::InvalidKey(InvalidKeyFailure::DerError("expected SEQUENCE".into())));
    }
    let mut pos = 2;
    if der.len() < pos + 2 {
        return Err(Error::InvalidKey(InvalidKeyFailure::DerError("truncated".into())));
    }
    if der[pos] != 0x02 {
        return Err(Error::InvalidKey(InvalidKeyFailure::DerError("expected INTEGER r".into())));
    }
    pos += 1;
    let r_len = der[pos] as usize;
    pos += 1;
    if der.len() < pos + r_len {
        return Err(Error::InvalidKey(InvalidKeyFailure::DerError("r truncated".into())));
    }
    let r = normalize_scalar(&der[pos..pos + r_len]);
    pos += r_len;
    if der.len() < pos + 2 {
        return Err(Error::InvalidKey(InvalidKeyFailure::DerError("s header truncated".into())));
    }
    if der[pos] != 0x02 {
        return Err(Error::InvalidKey(InvalidKeyFailure::DerError("expected INTEGER s".into())));
    }
    pos += 1;
    let s_len = der[pos] as usize;
    pos += 1;
    if der.len() < pos + s_len {
        return Err(Error::InvalidKey(InvalidKeyFailure::DerError("s truncated".into())));
    }
    let s = normalize_scalar(&der[pos..pos + s_len]);
    let mut sig = [0u8; 64];
    sig[..32].copy_from_slice(&r);
    sig[32..].copy_from_slice(&s);
    Ok(sig)
}

fn normalize_scalar(bytes: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    let start = bytes.len().saturating_sub(32);
    let copy_len = bytes.len().saturating_sub(start);
    out[32 - copy_len..].copy_from_slice(&bytes[start..]);
    out
}

fn normalize_scalar_48(bytes: &[u8]) -> [u8; 48] {
    let mut out = [0u8; 48];
    let start = bytes.len().saturating_sub(48);
    let copy_len = bytes.len().saturating_sub(start);
    out[48 - copy_len..].copy_from_slice(&bytes[start..]);
    out
}

fn der_to_raw_p384_signature(der: &[u8]) -> Result<[u8; 96], Error> {
    if der.len() < 8 {
        return Err(Error::InvalidKey(InvalidKeyFailure::DerError("too short".into())));
    }
    if der[0] != 0x30 {
        return Err(Error::InvalidKey(InvalidKeyFailure::DerError("expected SEQUENCE".into())));
    }
    let mut pos = 2;
    if der.len() < pos + 2 {
        return Err(Error::InvalidKey(InvalidKeyFailure::DerError("truncated".into())));
    }
    if der[pos] != 0x02 {
        return Err(Error::InvalidKey(InvalidKeyFailure::DerError("expected INTEGER r".into())));
    }
    pos += 1;
    let r_len = der[pos] as usize;
    pos += 1;
    if der.len() < pos + r_len {
        return Err(Error::InvalidKey(InvalidKeyFailure::DerError("r truncated".into())));
    }
    let r = normalize_scalar_48(&der[pos..pos + r_len]);
    pos += r_len;
    if der.len() < pos + 2 {
        return Err(Error::InvalidKey(InvalidKeyFailure::DerError("s header truncated".into())));
    }
    if der[pos] != 0x02 {
        return Err(Error::InvalidKey(InvalidKeyFailure::DerError("expected INTEGER s".into())));
    }
    pos += 1;
    let s_len = der[pos] as usize;
    pos += 1;
    if der.len() < pos + s_len {
        return Err(Error::InvalidKey(InvalidKeyFailure::DerError("s truncated".into())));
    }
    let s = normalize_scalar_48(&der[pos..pos + s_len]);
    let mut sig = [0u8; 96];
    sig[..48].copy_from_slice(&r);
    sig[48..].copy_from_slice(&s);
    Ok(sig)
}

// ── Tests ──────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x25519_mlkem768_key_exchange() {
        let provider = DefaultCryptoProvider::new();

        let client_kx = provider.create_kx_pair(KeyExchangeGroup::X25519MlKem768).unwrap();
        let client_share = client_kx.public_key_bytes();
        assert_eq!(client_share.len(), 1216, "client key share must be 1216 bytes");

        let mut server_kx = provider.create_kx_pair(KeyExchangeGroup::X25519MlKem768).unwrap();
        server_kx.set_peer_public_key(&client_share).unwrap();
        let server_share = server_kx.public_key_bytes();
        assert_eq!(server_share.len(), 1120, "server key share must be 1120 bytes");

        let client_ss = client_kx.shared_secret(&server_share).unwrap();
        let server_ss = server_kx.shared_secret(&client_share).unwrap();

        assert_eq!(client_ss, server_ss, "shared secrets must match");
        assert_eq!(client_ss.len(), 64, "shared secret must be 64 bytes (32 X25519 + 32 ML-KEM)");
    }
}
