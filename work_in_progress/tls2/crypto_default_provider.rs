use crypto::{
    Aead as _, Hasher,
    aes::{Aes128Gcm, Aes256Gcm},
    chacha::ChaCha20Poly1305,
    curve25519::x25519,
    hkdf,
    hmac::Hmac,
    random_fill,
    sha2::{Sha256, Sha384},
};
use heapless::Vec;

use crate::{
    CipherSuite, CryptoProvider, KEY_EXCHANGE_SHARED_SECRET_MAX_SIZE, KeyExchangeGroup, KeyExchangePublicKey,
    KeyExchangeSecretKey, SIGNATURE_MAX_SIZE, SignatureScheme, errors::Error,
};

/// Default crypto provider backed by the `crypto` crate.
#[derive(Clone)]
pub struct DefaultCryptoProvider;

impl DefaultCryptoProvider {
    pub fn new() -> Self {
        Self
    }
}

impl CryptoProvider for DefaultCryptoProvider {
    const CIPHER_SUITES: &[CipherSuite] = &[
        CipherSuite::TlsAes128GcmSha256,
        CipherSuite::TlsChaCha20Poly1305Sha256,
        CipherSuite::TlsAes256GcmSha384,
    ];
    const KEY_EXCHANGE_GROUPS: &[KeyExchangeGroup] = &[KeyExchangeGroup::X25519];
    const SIGNATURE_SCHEMES: &[SignatureScheme] = &[
        SignatureScheme::Ed25519,
        SignatureScheme::EcdsaP256Sha256,
        SignatureScheme::EcdsaP384Sha384,
        SignatureScheme::RsaPssRsaSha256,
        SignatureScheme::RsaPkcs1Sha256,
    ];

    fn secure_random(&self, buf: &mut [u8]) {
        random_fill(buf);
    }

    fn hash(&self, suite: CipherSuite, data: &[u8], out: &mut [u8]) -> Result<(), Error> {
        match suite {
            CipherSuite::TlsAes128GcmSha256 | CipherSuite::TlsChaCha20Poly1305Sha256 => {
                let h = Sha256::hash(data);
                out[..32].copy_from_slice(h.as_ref());
            }
            CipherSuite::TlsAes256GcmSha384 => {
                let h = Sha384::hash(data);
                out[..48].copy_from_slice(h.as_ref());
            }
        }
        Ok(())
    }

    fn hmac(&self, suite: CipherSuite, key: &[u8], data: &[u8], out: &mut [u8]) -> Result<(), Error> {
        match suite {
            CipherSuite::TlsAes128GcmSha256 | CipherSuite::TlsChaCha20Poly1305Sha256 => {
                let h = Hmac::<Sha256>::mac(key, data);
                out[..32].copy_from_slice(h.as_ref());
            }
            CipherSuite::TlsAes256GcmSha384 => {
                let h = Hmac::<Sha384>::mac(key, data);
                out[..48].copy_from_slice(h.as_ref());
            }
        }
        Ok(())
    }

    fn hkdf_extract(&self, suite: CipherSuite, salt: &[u8], ikm: &[u8], out: &mut [u8]) -> Result<(), Error> {
        match suite {
            CipherSuite::TlsAes128GcmSha256 | CipherSuite::TlsChaCha20Poly1305Sha256 => {
                let h = hkdf::extract::<Sha256>(Some(salt), ikm);
                out[..32].copy_from_slice(h.as_ref());
            }
            CipherSuite::TlsAes256GcmSha384 => {
                let h = hkdf::extract::<Sha384>(Some(salt), ikm);
                out[..48].copy_from_slice(h.as_ref());
            }
        }
        Ok(())
    }

    fn hkdf_expand_label(
        &self,
        suite: CipherSuite,
        secret: &[u8],
        label: &[u8],
        context: &[u8],
        out: &mut [u8],
    ) -> Result<(), Error> {
        let len = out.len();
        let hkdf_label = build_hkdf_label(label, context, len);
        match suite {
            CipherSuite::TlsAes128GcmSha256 | CipherSuite::TlsChaCha20Poly1305Sha256 => match len {
                12 => {
                    let a = hkdf::expand::<Sha256, 12>(secret, &hkdf_label).unwrap();
                    out.copy_from_slice(&a);
                }
                16 => {
                    let a = hkdf::expand::<Sha256, 16>(secret, &hkdf_label).unwrap();
                    out.copy_from_slice(&a);
                }
                32 => {
                    let a = hkdf::expand::<Sha256, 32>(secret, &hkdf_label).unwrap();
                    out.copy_from_slice(&a);
                }
                48 => {
                    let a = hkdf::expand::<Sha256, 48>(secret, &hkdf_label).unwrap();
                    out.copy_from_slice(&a);
                }
                _ => return Err(Error::CryptoError),
            },
            CipherSuite::TlsAes256GcmSha384 => match len {
                12 => {
                    let a = hkdf::expand::<Sha384, 12>(secret, &hkdf_label).unwrap();
                    out.copy_from_slice(&a);
                }
                16 => {
                    let a = hkdf::expand::<Sha384, 16>(secret, &hkdf_label).unwrap();
                    out.copy_from_slice(&a);
                }
                32 => {
                    let a = hkdf::expand::<Sha384, 32>(secret, &hkdf_label).unwrap();
                    out.copy_from_slice(&a);
                }
                48 => {
                    let a = hkdf::expand::<Sha384, 48>(secret, &hkdf_label).unwrap();
                    out.copy_from_slice(&a);
                }
                _ => return Err(Error::CryptoError),
            },
        }
        Ok(())
    }

    fn aead_encrypt(
        &self,
        suite: CipherSuite,
        key: &[u8],
        nonce: &[u8],
        aad: &[u8],
        data: &mut [u8],
        plaintext_len: usize,
    ) -> Result<usize, Error> {
        match suite {
            CipherSuite::TlsAes128GcmSha256 => {
                let k: &[u8; 16] = key.try_into().map_err(|_| Error::CryptoError)?;
                let cipher = Aes128Gcm::new(k);
                let tag = cipher.encrypt_in_place(&mut data[..plaintext_len], nonce, aad);
                let total = plaintext_len + 16;
                data[plaintext_len..total].copy_from_slice(tag.as_ref());
                Ok(total)
            }
            CipherSuite::TlsAes256GcmSha384 => {
                let k: &[u8; 32] = key.try_into().map_err(|_| Error::CryptoError)?;
                let cipher = Aes256Gcm::new(k);
                let tag = cipher.encrypt_in_place(&mut data[..plaintext_len], nonce, aad);
                let total = plaintext_len + 16;
                data[plaintext_len..total].copy_from_slice(tag.as_ref());
                Ok(total)
            }
            CipherSuite::TlsChaCha20Poly1305Sha256 => {
                let k: &[u8; 32] = key.try_into().map_err(|_| Error::CryptoError)?;
                let cipher = ChaCha20Poly1305::new(k);
                let tag = cipher.encrypt_in_place(&mut data[..plaintext_len], nonce, aad);
                let total = plaintext_len + 16;
                data[plaintext_len..total].copy_from_slice(tag.as_ref());
                Ok(total)
            }
        }
    }

    fn aead_decrypt(
        &self,
        suite: CipherSuite,
        key: &[u8],
        nonce: &[u8],
        aad: &[u8],
        data: &mut [u8],
    ) -> Result<usize, Error> {
        if data.len() < 16 {
            return Err(Error::AeadError);
        }
        let ct_len = data.len() - 16;
        let (ct, tag) = data.split_at_mut(ct_len);
        match suite {
            CipherSuite::TlsAes128GcmSha256 => {
                let k: &[u8; 16] = key.try_into().map_err(|_| Error::CryptoError)?;
                let cipher = Aes128Gcm::new(k);
                cipher
                    .decrypt_in_place(ct, nonce, aad, tag)
                    .map_err(|_| Error::AeadError)?;
                Ok(ct_len)
            }
            CipherSuite::TlsAes256GcmSha384 => {
                let k: &[u8; 32] = key.try_into().map_err(|_| Error::CryptoError)?;
                let cipher = Aes256Gcm::new(k);
                cipher
                    .decrypt_in_place(ct, nonce, aad, tag)
                    .map_err(|_| Error::AeadError)?;
                Ok(ct_len)
            }
            CipherSuite::TlsChaCha20Poly1305Sha256 => {
                let k: &[u8; 32] = key.try_into().map_err(|_| Error::CryptoError)?;
                let cipher = ChaCha20Poly1305::new(k);
                cipher
                    .decrypt_in_place(ct, nonce, aad, tag)
                    .map_err(|_| Error::AeadError)?;
                Ok(ct_len)
            }
        }
    }

    fn key_exchange_generate_keypair(
        &self,
        group: KeyExchangeGroup,
    ) -> Result<(KeyExchangeSecretKey, KeyExchangePublicKey), Error> {
        match group {
            KeyExchangeGroup::X25519 => {
                let sk = x25519::SecretKey::generate();
                let pk = sk.public_key();
                Ok((
                    KeyExchangeSecretKey::new(group, &sk.to_bytes()),
                    KeyExchangePublicKey::new(group, &pk.to_bytes()),
                ))
            }
            _ => Err(Error::CryptoError),
        }
    }

    fn key_exchange(
        &self,
        secret: &KeyExchangeSecretKey,
        peer_public: &[u8],
    ) -> Result<Vec<u8, KEY_EXCHANGE_SHARED_SECRET_MAX_SIZE>, Error> {
        match secret.group() {
            KeyExchangeGroup::X25519 => {
                let sk_bytes: &[u8; 32] = secret.bytes().try_into().map_err(|_| Error::CryptoError)?;
                let pk_bytes: &[u8; 32] = peer_public.try_into().map_err(|_| Error::CryptoError)?;
                let sk = x25519::SecretKey::from_bytes(sk_bytes);
                let pk = x25519::PublicKey::from_bytes(pk_bytes);
                let ss = sk.ecdh(&pk);
                let mut out = Vec::new();
                out.extend_from_slice(&ss).unwrap();
                Ok(out)
            }
            _ => Err(Error::CryptoError),
        }
    }

    fn sign(
        &self,
        scheme: SignatureScheme,
        secret_key: &[u8],
        data: &[u8],
        sig_out: &mut [u8],
    ) -> Result<Vec<u8, SIGNATURE_MAX_SIZE>, Error> {
        match scheme {
            SignatureScheme::Ed25519 => {
                let seed: &[u8; 32] = secret_key.try_into().map_err(|_| Error::CryptoError)?;
                let sk = crypto::curve25519::ed25519::SecretKey::from_bytes(seed);
                let sig = sk.sign(data);
                sig_out[..64].copy_from_slice(&sig);
                let mut v = Vec::new();
                v.extend_from_slice(&sig).unwrap();
                Ok(v)
            }
            _ => {
                let mut v = Vec::new();
                v.extend_from_slice(sig_out).unwrap();
                Ok(v)
            }
        }
    }

    fn verify(&self, scheme: SignatureScheme, public_key: &[u8], data: &[u8], signature: &[u8]) -> Result<(), Error> {
        match scheme {
            SignatureScheme::Ed25519 => {
                let pk: &[u8; 32] = public_key.try_into().map_err(|_| Error::CryptoError)?;
                let pk = crypto::curve25519::ed25519::PublicKey::from_bytes(pk).map_err(|_| Error::CryptoError)?;
                let sig: &[u8; 64] = signature.try_into().map_err(|_| Error::InvalidSignature)?;
                pk.verify(data, sig).map_err(|_| Error::InvalidSignature)
            }
            SignatureScheme::EcdsaP256Sha256 => {
                let pk = crypto::p256::PublicKey::from_bytes(public_key).map_err(|_| Error::CryptoError)?;
                let sig = der_to_raw_p256_signature(signature).map_err(|_| Error::DecodeError)?;
                pk.verify(data, &sig).map_err(|_| Error::InvalidSignature)
            }
            SignatureScheme::EcdsaP384Sha384 => {
                let pk = crypto::p384::PublicKey::from_bytes(public_key).map_err(|_| Error::CryptoError)?;
                let sig = der_to_raw_p384_signature(signature).map_err(|_| Error::DecodeError)?;
                pk.verify(data, &sig).map_err(|_| Error::InvalidSignature)
            }
            SignatureScheme::RsaPkcs1Sha256 => {
                crypto::rsa::verify_pkcs1_sha256(public_key, signature, data).map_err(|_| Error::InvalidSignature)
            }
            SignatureScheme::RsaPssRsaSha256 => {
                crypto::rsa::verify_pss_sha256(public_key, signature, data).map_err(|_| Error::InvalidSignature)
            }
            _ => Err(Error::InvalidSignature),
        }
    }

    fn validate_cert_chain(
        &self,
        chain: &[&[u8]],
        _server_name: Option<&str>,
        public_key_out: &mut [u8],
    ) -> Result<(SignatureScheme, usize), Error> {
        let der = chain.first().ok_or(Error::InvalidCertificate)?;
        let spki = x509::extract_spki_from_cert(der).map_err(|_| Error::InvalidCertificate)?;
        let key = x509::extract_key_from_spki(spki).map_err(|_| Error::InvalidCertificate)?;

        if let Ok(pk) = crypto::p256::PublicKey::from_bytes(key) {
            let bytes = pk.to_bytes();
            let len = bytes.len();
            public_key_out[..len].copy_from_slice(&bytes);
            return Ok((SignatureScheme::EcdsaP256Sha256, len));
        }
        if let Ok(pk) = crypto::p384::PublicKey::from_bytes(key) {
            let bytes = pk.to_bytes();
            let len = bytes.len();
            public_key_out[..len].copy_from_slice(&bytes);
            return Ok((SignatureScheme::EcdsaP384Sha384, len));
        }
        if key.len() == 32 {
            public_key_out[..32].copy_from_slice(key);
            return Ok((SignatureScheme::Ed25519, 32));
        }
        if key.len() <= 294 {
            let len = key.len();
            public_key_out[..len].copy_from_slice(key);
            return Ok((SignatureScheme::RsaPkcs1Sha256, len));
        }

        Err(Error::InvalidCertificate)
    }
}

// ── Helpers ──

fn build_hkdf_label(label: &[u8], context: &[u8], out_len: usize) -> Vec<u8, 130> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&(out_len as u16).to_be_bytes()).unwrap();
    buf.push((6 + label.len()) as u8).unwrap();
    buf.extend_from_slice(b"tls13 ").unwrap();
    buf.extend_from_slice(label).unwrap();
    buf.push(context.len() as u8).unwrap();
    buf.extend_from_slice(context).unwrap();
    buf
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

fn der_to_raw_p256_signature(der: &[u8]) -> Result<[u8; 64], Error> {
    if der.len() < 8 || der[0] != 0x30 {
        return Err(Error::DecodeError);
    }
    let mut pos = 2;
    if pos + 2 > der.len() || der[pos] != 0x02 {
        return Err(Error::DecodeError);
    }
    pos += 1;
    let r_len = der[pos] as usize;
    pos += 1;
    if pos + r_len > der.len() {
        return Err(Error::DecodeError);
    }
    let r = normalize_scalar(&der[pos..pos + r_len]);
    pos += r_len;
    if pos + 2 > der.len() || der[pos] != 0x02 {
        return Err(Error::DecodeError);
    }
    pos += 1;
    let s_len = der[pos] as usize;
    pos += 1;
    if pos + s_len > der.len() {
        return Err(Error::DecodeError);
    }
    let s = normalize_scalar(&der[pos..pos + s_len]);
    let mut sig = [0u8; 64];
    sig[..32].copy_from_slice(&r);
    sig[32..].copy_from_slice(&s);
    Ok(sig)
}

fn der_to_raw_p384_signature(der: &[u8]) -> Result<[u8; 96], Error> {
    if der.len() < 8 || der[0] != 0x30 {
        return Err(Error::DecodeError);
    }
    let mut pos = 2;
    if pos + 2 > der.len() || der[pos] != 0x02 {
        return Err(Error::DecodeError);
    }
    pos += 1;
    let r_len = der[pos] as usize;
    pos += 1;
    if pos + r_len > der.len() {
        return Err(Error::DecodeError);
    }
    let r = normalize_scalar_48(&der[pos..pos + r_len]);
    pos += r_len;
    if pos + 2 > der.len() || der[pos] != 0x02 {
        return Err(Error::DecodeError);
    }
    pos += 1;
    let s_len = der[pos] as usize;
    pos += 1;
    if pos + s_len > der.len() {
        return Err(Error::DecodeError);
    }
    let s = normalize_scalar_48(&der[pos..pos + s_len]);
    let mut sig = [0u8; 96];
    sig[..48].copy_from_slice(&r);
    sig[48..].copy_from_slice(&s);
    Ok(sig)
}
