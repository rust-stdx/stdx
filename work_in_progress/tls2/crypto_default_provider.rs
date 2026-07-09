use crypto::{
    Aead as CryptoAead, Hasher as CryptoHasher,
    aes::{Aes128Gcm, Aes256Gcm},
    chacha::ChaCha20Poly1305,
    curve25519::x25519,
    hkdf,
    hmac::Hmac,
    mlkem::{CIPHERTEXT_SIZE_768, generate_keypair_768_derand},
    p256, random_fill,
    sha2::{Sha256, Sha384},
};
use heapless::Vec;

use crate::{
    CipherSuite, CryptoProvider, Hash, KEY_EXCHANGE_PUBLIC_KEY_MAX_SIZE, KEY_EXCHANGE_SECRET_KEY_MAX_SIZE,
    KEY_EXCHANGE_SHARED_SECRET_MAX_SIZE, KeyExchangeGroup, KeyExchangePublicKey, KeyExchangeSecretKey,
    SIGNATURE_MAX_SIZE, SignatureScheme, errors::Error,
};

/// Default crypto provider backed by the `crypto` crate.
///
/// Provides AEAD, key exchange, hash, and signature operations.
///
/// # Examples
///
/// ```ignore
/// use tls2::crypto_default_provider::DefaultCryptoProvider;
///
/// let crypto = DefaultCryptoProvider;
/// ```
#[derive(Clone)]
pub struct DefaultCryptoProvider;

/// Incremental hash state for the default provider.
///
/// Wraps either SHA-256 (32-bit state) or SHA-384 (64-bit state) depending
/// on the negotiated cipher suite.
#[derive(Clone)]
pub enum Hasher {
    Sha256(crypto::sha2::Sha256),
    Sha384(crypto::sha2::Sha384),
}

/// Cached AEAD key for the default provider.
///
/// Wraps the expanded cipher state for AES-128-GCM, AES-256-GCM, or
/// ChaCha20-Poly1305 so key expansion happens only once.
#[cfg_attr(feature = "zeroize", derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop))]
pub enum AeadKey {
    Aes128Gcm(Aes128Gcm),
    Aes256Gcm(Aes256Gcm),
    ChaCha20Poly1305(ChaCha20Poly1305),
}

impl CryptoProvider for DefaultCryptoProvider {
    type Hasher = Hasher;
    type AeadKey = AeadKey;

    #[inline]
    fn cipher_suites() -> &'static [CipherSuite] {
        // runtime / compile detection of CPU instructions to return the list of supported ciphersuites
        // in the correct order
        static AES_FIRST: &[CipherSuite] = &[
            CipherSuite::TlsAes256GcmSha384,
            CipherSuite::TlsChaCha20Poly1305Sha256,
            CipherSuite::TlsAes128GcmSha256,
        ];
        static CHACHA_FIRST: &[CipherSuite] = &[
            CipherSuite::TlsChaCha20Poly1305Sha256,
            CipherSuite::TlsAes256GcmSha384,
            CipherSuite::TlsAes128GcmSha256,
        ];

        #[cfg(target_arch = "x86_64")]
        {
            #[cfg(feature = "std")]
            if std::arch::is_x86_feature_detected!("aes") {
                return AES_FIRST;
            }
            #[cfg(all(not(feature = "std"), target_feature = "aes"))]
            return AES_FIRST;
        }

        #[cfg(target_arch = "aarch64")]
        {
            #[cfg(feature = "std")]
            if std::arch::is_aarch64_feature_detected!("aes") {
                return AES_FIRST;
            }
            #[cfg(all(not(feature = "std"), target_feature = "aes"))]
            return AES_FIRST;
        }

        CHACHA_FIRST
    }

    #[inline]
    fn signature_schemes() -> &'static [SignatureScheme] {
        &[
            SignatureScheme::Ed25519,
            SignatureScheme::EcdsaP256Sha256,
            SignatureScheme::EcdsaP384Sha384,
            SignatureScheme::RsaPssRsaSha256,
            SignatureScheme::RsaPkcs1Sha256,
        ]
    }

    #[inline]
    fn key_exchange_groups() -> &'static [KeyExchangeGroup] {
        &[
            KeyExchangeGroup::X25519MlKem768,
            KeyExchangeGroup::X25519,
            KeyExchangeGroup::Secp256r1,
        ]
    }

    fn new_aead_key(&self, suite: CipherSuite, key: &[u8]) -> Self::AeadKey {
        match suite {
            CipherSuite::TlsAes128GcmSha256 => AeadKey::Aes128Gcm(Aes128Gcm::new(key.try_into().unwrap())),
            CipherSuite::TlsAes256GcmSha384 => AeadKey::Aes256Gcm(Aes256Gcm::new(key.try_into().unwrap())),
            CipherSuite::TlsChaCha20Poly1305Sha256 => {
                AeadKey::ChaCha20Poly1305(ChaCha20Poly1305::new(key.try_into().unwrap()))
            }
        }
    }

    fn new_hash(&self, suite: CipherSuite) -> Self::Hasher {
        match suite {
            CipherSuite::TlsAes128GcmSha256 | CipherSuite::TlsChaCha20Poly1305Sha256 => {
                Hasher::Sha256(<Sha256 as CryptoHasher>::new())
            }
            CipherSuite::TlsAes256GcmSha384 => Hasher::Sha384(<Sha384 as CryptoHasher>::new()),
        }
    }

    fn hash_update(&self, state: &mut Self::Hasher, data: &[u8]) {
        match state {
            Hasher::Sha256(s) => s.update(data),
            Hasher::Sha384(s) => s.update(data),
        }
    }

    fn hash_finalize(&self, state: Self::Hasher) -> Result<Hash, Error> {
        match state {
            Hasher::Sha256(s) => {
                let h = s.sum();
                Ok(Hash::from_slice(h.as_ref()))
            }
            Hasher::Sha384(s) => {
                let h = s.sum();
                Ok(Hash::from_slice(h.as_ref()))
            }
        }
    }

    fn secure_random(&self, buf: &mut [u8]) {
        random_fill(buf);
    }

    fn hash(&self, suite: CipherSuite, data: &[u8]) -> Result<Hash, Error> {
        match suite {
            CipherSuite::TlsAes128GcmSha256 | CipherSuite::TlsChaCha20Poly1305Sha256 => {
                Ok(Hash::from_slice(Sha256::hash(data).as_ref()))
            }
            CipherSuite::TlsAes256GcmSha384 => Ok(Hash::from_slice(Sha384::hash(data).as_ref())),
        }
    }

    fn hmac(&self, suite: CipherSuite, key: &Hash, data: &[u8]) -> Result<Hash, Error> {
        match suite {
            CipherSuite::TlsAes128GcmSha256 | CipherSuite::TlsChaCha20Poly1305Sha256 => {
                Ok(Hash::from_slice(Hmac::<Sha256>::mac(key, data).as_ref()))
            }
            CipherSuite::TlsAes256GcmSha384 => Ok(Hash::from_slice(Hmac::<Sha384>::mac(key, data).as_ref())),
        }
    }

    fn hkdf_extract(&self, suite: CipherSuite, salt: &Hash, ikm: &[u8]) -> Result<Hash, Error> {
        match suite {
            CipherSuite::TlsAes128GcmSha256 | CipherSuite::TlsChaCha20Poly1305Sha256 => {
                Ok(Hash::from_slice(hkdf::extract::<Sha256>(Some(salt), ikm).as_ref()))
            }
            CipherSuite::TlsAes256GcmSha384 => Ok(Hash::from_slice(hkdf::extract::<Sha384>(Some(salt), ikm).as_ref())),
        }
    }

    fn hkdf_expand_label(
        &self,
        out: &mut [u8],
        suite: CipherSuite,
        secret: &Hash,
        label: &[u8],
        context: &[u8],
    ) -> Result<(), Error> {
        let len = out.len();
        let hkdf_label = build_hkdf_label(label, context, len);
        match suite {
            CipherSuite::TlsAes128GcmSha256 | CipherSuite::TlsChaCha20Poly1305Sha256 => {
                hkdf::expand::<Sha256>(out, secret, &hkdf_label).unwrap();
            }
            CipherSuite::TlsAes256GcmSha384 => {
                hkdf::expand::<Sha384>(out, secret, &hkdf_label).unwrap();
            }
        }
        Ok(())
    }

    fn aead_encrypt(
        &self,
        key: &Self::AeadKey,
        nonce: &[u8],
        aad: &[u8],
        data: &mut [u8],
        plaintext_len: usize,
    ) -> Result<usize, Error> {
        let total = plaintext_len + 16;
        match key {
            AeadKey::Aes128Gcm(cipher) => {
                let tag = cipher.encrypt_in_place(&mut data[..plaintext_len], nonce, aad);
                data[plaintext_len..total].copy_from_slice(tag.as_ref());
            }
            AeadKey::Aes256Gcm(cipher) => {
                let tag = cipher.encrypt_in_place(&mut data[..plaintext_len], nonce, aad);
                data[plaintext_len..total].copy_from_slice(tag.as_ref());
            }
            AeadKey::ChaCha20Poly1305(cipher) => {
                let tag = cipher.encrypt_in_place(&mut data[..plaintext_len], nonce, aad);
                data[plaintext_len..total].copy_from_slice(tag.as_ref());
            }
        }
        Ok(total)
    }

    fn aead_decrypt(&self, key: &Self::AeadKey, nonce: &[u8], aad: &[u8], data: &mut [u8]) -> Result<usize, Error> {
        if data.len() < 16 {
            return Err(Error::AeadError);
        }
        let ct_len = data.len() - 16;
        let (ct, tag) = data.split_at_mut(ct_len);
        match key {
            AeadKey::Aes128Gcm(cipher) => {
                cipher
                    .decrypt_in_place(ct, nonce, aad, tag)
                    .map_err(|_| Error::AeadError)?;
            }
            AeadKey::Aes256Gcm(cipher) => {
                cipher
                    .decrypt_in_place(ct, nonce, aad, tag)
                    .map_err(|_| Error::AeadError)?;
            }
            AeadKey::ChaCha20Poly1305(cipher) => {
                cipher
                    .decrypt_in_place(ct, nonce, aad, tag)
                    .map_err(|_| Error::AeadError)?;
            }
        }
        Ok(ct_len)
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
            KeyExchangeGroup::X25519MlKem768 => {
                // for X25519MlKem768 we keep the same order as for the public keys and shared secret:
                // first the ml_kem seed (64 bytes) and then the x25519 seed (32 bytes)
                let mut seeds = [0u8; KEY_EXCHANGE_SECRET_KEY_MAX_SIZE];
                self.secure_random(&mut seeds);

                let (_, mlkem_pk) =
                    generate_keypair_768_derand(&seeds[..64].try_into().map_err(|_| Error::CryptoError)?);
                let x25519_sk =
                    x25519::SecretKey::from_bytes(&seeds[64..96].try_into().map_err(|_| Error::CryptoError)?);
                let x25519_pk = x25519_sk.public_key();

                let mut pub_bytes: Vec<u8, KEY_EXCHANGE_PUBLIC_KEY_MAX_SIZE> = Vec::new();
                pub_bytes
                    .extend_from_slice(&mlkem_pk.to_bytes())
                    .map_err(|_| Error::CryptoError)?;
                pub_bytes
                    .extend_from_slice(&x25519_pk.to_bytes())
                    .map_err(|_| Error::CryptoError)?;
                Ok((
                    KeyExchangeSecretKey::new(group, &seeds),
                    KeyExchangePublicKey::new(group, &pub_bytes),
                ))
            }
            KeyExchangeGroup::Secp256r1 => {
                let mut private_key_bytes = [0u8; 32];
                self.secure_random(&mut private_key_bytes);
                let private_key = p256::SecretKey::from_bytes(&private_key_bytes).map_err(|_| Error::CryptoError)?;
                let public_key = private_key.public_key();
                Ok((
                    KeyExchangeSecretKey::new(group, &private_key.to_bytes()),
                    KeyExchangePublicKey::new(group, &public_key.to_bytes()),
                ))
            }
            _ => Err(Error::UnsupportedKeyExchangeGroup),
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
            KeyExchangeGroup::X25519MlKem768 => {
                let seeds: &[u8; KEY_EXCHANGE_SECRET_KEY_MAX_SIZE] =
                    secret.bytes().try_into().map_err(|_| Error::CryptoError)?;

                let (mlkem_sk, _) =
                    generate_keypair_768_derand(&seeds[..64].try_into().map_err(|_| Error::CryptoError)?);
                let x25519_sk =
                    x25519::SecretKey::from_bytes(&seeds[64..96].try_into().map_err(|_| Error::CryptoError)?);

                let mlkem_ct: &[u8; CIPHERTEXT_SIZE_768] = peer_public
                    .get(..CIPHERTEXT_SIZE_768)
                    .and_then(|s| s.try_into().ok())
                    .ok_or(Error::CryptoError)?;
                let peer_x25519_pk = x25519::PublicKey::from_bytes(
                    &peer_public[CIPHERTEXT_SIZE_768..]
                        .try_into()
                        .map_err(|_| Error::CryptoError)?,
                );
                let shared_secret_mlkem = mlkem_sk.decapsulate(mlkem_ct).map_err(|_| Error::CryptoError)?;
                let shared_secret_x25519 = x25519_sk.ecdh(&peer_x25519_pk);
                let mut out = Vec::new();
                out.extend_from_slice(&shared_secret_mlkem)
                    .map_err(|_| Error::CryptoError)?;
                out.extend_from_slice(&shared_secret_x25519)
                    .map_err(|_| Error::CryptoError)?;
                Ok(out)
            }
            KeyExchangeGroup::Secp256r1 => {
                let sk_bytes: &[u8; 32] = secret.bytes().try_into().map_err(|_| Error::CryptoError)?;
                let private_key = p256::SecretKey::from_bytes(sk_bytes).map_err(|_| Error::CryptoError)?;
                let shared_secret = private_key
                    .ecdh(&p256::PublicKey::from_bytes(peer_public).map_err(|_| Error::CryptoError)?)
                    .map_err(|_| Error::CryptoError)?;
                let mut out = Vec::new();
                out.extend_from_slice(&shared_secret).unwrap();
                Ok(out)
            }
            _ => Err(Error::UnsupportedKeyExchangeGroup),
        }
    }

    fn sign(
        &self,
        scheme: SignatureScheme,
        secret_key: &[u8],
        data: &[u8],
    ) -> Result<Vec<u8, SIGNATURE_MAX_SIZE>, Error> {
        match scheme {
            SignatureScheme::Ed25519 => {
                let seed: &[u8; 32] = secret_key.try_into().map_err(|_| Error::CryptoError)?;
                let secret_key = crypto::curve25519::ed25519::SecretKey::from_bytes(seed);
                let signature = secret_key.sign(data);
                Ok(Vec::from_slice(&signature).unwrap())
            }
            SignatureScheme::EcdsaP256Sha256 => {
                let key: &[u8; 32] = secret_key.try_into().map_err(|_| Error::CryptoError)?;
                let private_key = crypto::p256::SecretKey::from_bytes(key).map_err(|_| Error::CryptoError)?;
                let raw_sig = private_key.sign(data).map_err(|_| Error::CryptoError)?;
                p256_raw_to_der_signature(&raw_sig)
            }
            SignatureScheme::EcdsaP384Sha384 => {
                let key: &[u8; 48] = secret_key.try_into().map_err(|_| Error::CryptoError)?;
                let private_key = crypto::p384::PrivateKey::from_bytes(key).map_err(|_| Error::CryptoError)?;
                let raw_sig = private_key.sign(data).map_err(|_| Error::CryptoError)?;
                p384_raw_to_der_signature(&raw_sig)
            }
            _ => Err(Error::CryptoError),
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
                let sig = p256_der_to_raw_signature(signature).map_err(|_| Error::DecodeError)?;
                pk.verify(data, &sig).map_err(|_| Error::InvalidSignature)
            }
            SignatureScheme::EcdsaP384Sha384 => {
                let pk = crypto::p384::PublicKey::from_bytes(public_key).map_err(|_| Error::CryptoError)?;
                let sig = p384_der_to_raw_signature(signature).map_err(|_| Error::DecodeError)?;
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
}

// ── HKDF helpers ──

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

fn p256_der_to_raw_signature(der: &[u8]) -> Result<[u8; 64], Error> {
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

fn p384_der_to_raw_signature(der: &[u8]) -> Result<[u8; 96], Error> {
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

fn p256_raw_to_der_signature(raw: &[u8; 64]) -> Result<Vec<u8, SIGNATURE_MAX_SIZE>, Error> {
    let r = &raw[..32];
    let s = &raw[32..];

    let r_offset = r.iter().position(|&b| b != 0).unwrap_or(r.len());
    let r_stripped = &r[r_offset..];
    let (r_enc, r_enc_len) = if r_stripped.is_empty() {
        ([0x00u8; 34], 1)
    } else if r_stripped[0] & 0x80 != 0 {
        let mut arr = [0u8; 34];
        arr[0] = 0x00;
        arr[1..=r_stripped.len()].copy_from_slice(&r_stripped);
        (arr, r_stripped.len() + 1)
    } else {
        let mut arr = [0u8; 34];
        arr[..r_stripped.len()].copy_from_slice(&r_stripped);
        (arr, r_stripped.len())
    };

    let s_offset = s.iter().position(|&b| b != 0).unwrap_or(s.len());
    let s_stripped = &s[s_offset..];
    let (s_enc, s_enc_len) = if s_stripped.is_empty() {
        ([0x00u8; 34], 1)
    } else if s_stripped[0] & 0x80 != 0 {
        let mut arr = [0u8; 34];
        arr[0] = 0x00;
        arr[1..=s_stripped.len()].copy_from_slice(&s_stripped);
        (arr, s_stripped.len() + 1)
    } else {
        let mut arr = [0u8; 34];
        arr[..s_stripped.len()].copy_from_slice(&s_stripped);
        (arr, s_stripped.len())
    };

    let total_len = 2 + r_enc_len + 2 + s_enc_len;
    let mut der = Vec::new();
    der.push(0x30).unwrap();
    der.push(total_len as u8).unwrap();
    der.push(0x02).unwrap();
    der.push(r_enc_len as u8).unwrap();
    der.extend_from_slice(&r_enc[..r_enc_len]).unwrap();
    der.push(0x02).unwrap();
    der.push(s_enc_len as u8).unwrap();
    der.extend_from_slice(&s_enc[..s_enc_len]).unwrap();
    Ok(der)
}

fn p384_raw_to_der_signature(raw: &[u8; 96]) -> Result<Vec<u8, SIGNATURE_MAX_SIZE>, Error> {
    let r = &raw[..48];
    let s = &raw[48..];

    let r_offset = r.iter().position(|&b| b != 0).unwrap_or(r.len());
    let r_stripped = &r[r_offset..];
    let (r_enc, r_enc_len) = if r_stripped.is_empty() {
        ([0x00u8; 50], 1)
    } else if r_stripped[0] & 0x80 != 0 {
        let mut arr = [0u8; 50];
        arr[0] = 0x00;
        arr[1..=r_stripped.len()].copy_from_slice(r_stripped);
        (arr, r_stripped.len() + 1)
    } else {
        let mut arr = [0u8; 50];
        arr[..r_stripped.len()].copy_from_slice(r_stripped);
        (arr, r_stripped.len())
    };

    let s_offset = s.iter().position(|&b| b != 0).unwrap_or(s.len());
    let s_stripped = &s[s_offset..];
    let (s_enc, s_enc_len) = if s_stripped.is_empty() {
        ([0x00u8; 50], 1)
    } else if s_stripped[0] & 0x80 != 0 {
        let mut arr = [0u8; 50];
        arr[0] = 0x00;
        arr[1..=s_stripped.len()].copy_from_slice(s_stripped);
        (arr, s_stripped.len() + 1)
    } else {
        let mut arr = [0u8; 50];
        arr[..s_stripped.len()].copy_from_slice(s_stripped);
        (arr, s_stripped.len())
    };

    let total_len = 2 + r_enc_len + 2 + s_enc_len;
    let mut der = Vec::new();
    der.push(0x30).unwrap();
    der.push(total_len as u8).unwrap();
    der.push(0x02).unwrap();
    der.push(r_enc_len as u8).unwrap();
    der.extend_from_slice(&r_enc[..r_enc_len]).unwrap();
    der.push(0x02).unwrap();
    der.push(s_enc_len as u8).unwrap();
    der.extend_from_slice(&s_enc[..s_enc_len]).unwrap();
    Ok(der)
}

#[cfg(test)]
mod tests {
    use crypto::{
        curve25519::x25519,
        mlkem::{PUBLIC_KEY_SIZE_768, PublicKey768},
    };

    use super::*;

    #[test]
    fn x25519_mlkem768_key_exchange() {
        let provider = DefaultCryptoProvider;
        let group = KeyExchangeGroup::X25519MlKem768;

        // ── Client side: generate keypair, send ClientHello ──
        let (client_secret, client_public) = provider.key_exchange_generate_keypair(group).unwrap();
        assert_eq!(client_public.bytes().len(), 1216, "client key share must be 1216 bytes");

        // ── Server side: parse client's share, encapsulate ──
        let client_mlkem_pk =
            PublicKey768::from_bytes(client_public.bytes()[..PUBLIC_KEY_SIZE_768].try_into().unwrap());
        let (server_ct, server_mlkem_ss) = client_mlkem_pk.encapsulate();

        let server_x25519_sk = x25519::SecretKey::generate();
        let server_x25519_pk = server_x25519_sk.public_key();
        let server_x25519_ss = server_x25519_sk.ecdh(&x25519::PublicKey::from_bytes(
            &client_public.bytes()[PUBLIC_KEY_SIZE_768..].try_into().unwrap(),
        ));
        let server_ss_full: [u8; 64] = {
            let mut s = [0u8; 64];
            s[..32].copy_from_slice(&server_mlkem_ss);
            s[32..].copy_from_slice(&server_x25519_ss);
            s
        };

        // Build server's key share (1120 bytes: ct + X25519 pk)
        let mut server_share: Vec<u8, KEY_EXCHANGE_PUBLIC_KEY_MAX_SIZE> = Vec::new();
        server_share.extend_from_slice(&server_ct).unwrap();
        server_share.extend_from_slice(&server_x25519_pk.to_bytes()).unwrap();
        assert_eq!(server_share.len(), 1120, "server key share must be 1120 bytes");

        // ── Client side: compute shared secret from server's response ──
        let client_ss = provider.key_exchange(&client_secret, &server_share).unwrap();
        assert_eq!(client_ss.len(), 64, "shared secret must be 64 bytes");
        assert_eq!(&client_ss[..], &server_ss_full, "shared secrets must match");
    }

    #[test]
    fn secp256r1_key_exchange() {
        let provider = DefaultCryptoProvider;
        let group = KeyExchangeGroup::Secp256r1;

        // Two parties each generate a keypair
        let (alice_secret, alice_public) = provider.key_exchange_generate_keypair(group).unwrap();
        let (bob_secret, bob_public) = provider.key_exchange_generate_keypair(group).unwrap();

        assert_eq!(alice_public.bytes().len(), 65, "P-256 public key must be 65 bytes");
        assert_eq!(bob_public.bytes().len(), 65, "P-256 public key must be 65 bytes");

        // Each computes the shared secret from the other's public key
        let alice_ss = provider.key_exchange(&alice_secret, bob_public.bytes()).unwrap();
        let bob_ss = provider.key_exchange(&bob_secret, alice_public.bytes()).unwrap();

        assert_eq!(alice_ss.len(), 32, "P-256 shared secret must be 32 bytes");
        assert_eq!(alice_ss, bob_ss, "shared secrets must match");
    }

    #[test]
    fn ecdsa_p256_sign_verify_roundtrip() {
        let provider = DefaultCryptoProvider;
        let data = b"TLS 1.3 test message";

        let seed = [42u8; 32];
        let private_key = crypto::p256::SecretKey::from_bytes(&seed).unwrap();
        let public_key = private_key.public_key().to_bytes();

        let signature = provider.sign(SignatureScheme::EcdsaP256Sha256, &seed, data).unwrap();
        provider
            .verify(SignatureScheme::EcdsaP256Sha256, &public_key, data, &signature)
            .unwrap();
    }

    #[test]
    fn ecdsa_p384_sign_verify_roundtrip() {
        let provider = DefaultCryptoProvider;
        let data = b"TLS 1.3 test message";

        let seed = [42u8; 48];
        let private_key = crypto::p384::PrivateKey::from_bytes(&seed).unwrap();
        let public_key = private_key.public_key().to_bytes();

        let signature = provider.sign(SignatureScheme::EcdsaP384Sha384, &seed, data).unwrap();
        provider
            .verify(SignatureScheme::EcdsaP384Sha384, &public_key, data, &signature)
            .unwrap();
    }

    #[test]
    fn ecdsa_p256_der_encoding_roundtrip() {
        let raw = [0u8; 64];
        let der = p256_raw_to_der_signature(&raw).unwrap();
        let decoded = p256_der_to_raw_signature(&der).unwrap();
        assert_eq!(raw, decoded);

        let raw = [0xffu8; 64];
        let der = p256_raw_to_der_signature(&raw).unwrap();
        let decoded = p256_der_to_raw_signature(&der).unwrap();
        assert_eq!(raw, decoded);

        let mut raw = [0u8; 64];
        raw[0] = 0x12;
        raw[32] = 0x34;
        let der = p256_raw_to_der_signature(&raw).unwrap();
        let decoded = p256_der_to_raw_signature(&der).unwrap();
        assert_eq!(raw, decoded);
    }

    #[test]
    fn ecdsa_p384_der_encoding_roundtrip() {
        let raw = [0u8; 96];
        let der = p384_raw_to_der_signature(&raw).unwrap();
        let decoded = p384_der_to_raw_signature(&der).unwrap();
        assert_eq!(raw, decoded);

        let raw = [0xffu8; 96];
        let der = p384_raw_to_der_signature(&raw).unwrap();
        let decoded = p384_der_to_raw_signature(&der).unwrap();
        assert_eq!(raw, decoded);

        let mut raw = [0u8; 96];
        raw[0] = 0x12;
        raw[48] = 0x34;
        let der = p384_raw_to_der_signature(&raw).unwrap();
        let decoded = p384_der_to_raw_signature(&der).unwrap();
        assert_eq!(raw, decoded);
    }
}
