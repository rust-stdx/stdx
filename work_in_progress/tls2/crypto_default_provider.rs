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
    KeyExchangeSecretKey, ParsedCertificate, ReceivedCertificate, RootCa, SIGNATURE_MAX_SIZE, SignatureScheme,
    errors::Error,
};

/// Default crypto provider backed by the `crypto` crate.
///
/// Supports two modes:
/// - **EE-only** (no roots configured): accepts any X.509 cert, extracts the
///   public key. Suitable for testing.
/// - **Full chain** (roots configured via builder): validates the full chain
///   including SAN, validity, EKU, Basic Constraints, and signature verification.
///
/// Raw public keys are always accepted.
///
/// # Examples
///
/// ```ignore
/// use tls2::crypto_default_provider::DefaultCryptoProvider;
///
/// // EE-only mode (no root validation)
/// let crypto = DefaultCryptoProvider::new();
///
/// // Full validation with system roots (requires `std` feature)
/// let crypto = DefaultCryptoProvider::new().with_system_roots();
///
/// // Full validation with custom roots
/// let crypto = DefaultCryptoProvider::new().with_roots(&[root_ca]);
/// ```
#[derive(Clone)]
pub struct DefaultCryptoProvider {
    roots: Option<alloc::vec::Vec<RootCa>>,
}

impl DefaultCryptoProvider {
    /// Create a new provider in EE-only mode (no root validation).
    pub fn new() -> Self {
        Self {
            roots: None,
        }
    }

    /// Load root CAs from the operating system's trust store.
    /// Requires the `std` feature.
    #[cfg(feature = "std")]
    pub fn with_system_roots(mut self) -> Self {
        self.roots = Some(load_system_roots());
        self
    }

    /// Add custom root trust anchors.
    pub fn with_roots(mut self, custom_roots: &[RootCa]) -> Self {
        let mut roots = if let Some(roots) = self.roots {
            roots
        } else {
            alloc::vec::Vec::with_capacity(custom_roots.len())
        };

        for r in custom_roots {
            roots.push(r.clone());
        }

        self.roots = Some(roots);
        self
    }
}

impl Default for DefaultCryptoProvider {
    fn default() -> Self {
        Self::new()
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
                out[..Sha256::OUTPUT_SIZE].copy_from_slice(h.as_ref());
            }
            CipherSuite::TlsAes256GcmSha384 => {
                let h = Sha384::hash(data);
                out[..Sha384::OUTPUT_SIZE].copy_from_slice(h.as_ref());
            }
        }
        Ok(())
    }

    fn hmac(&self, suite: CipherSuite, key: &[u8], data: &[u8], out: &mut [u8]) -> Result<(), Error> {
        match suite {
            CipherSuite::TlsAes128GcmSha256 | CipherSuite::TlsChaCha20Poly1305Sha256 => {
                let h = Hmac::<Sha256>::mac(key, data);
                out[..Sha256::OUTPUT_SIZE].copy_from_slice(h.as_ref());
            }
            CipherSuite::TlsAes256GcmSha384 => {
                let h = Hmac::<Sha384>::mac(key, data);
                out[..Sha384::OUTPUT_SIZE].copy_from_slice(h.as_ref());
            }
        }
        Ok(())
    }

    fn hkdf_extract(&self, out: &mut [u8], suite: CipherSuite, salt: &[u8], ikm: &[u8]) -> Result<(), Error> {
        match suite {
            CipherSuite::TlsAes128GcmSha256 | CipherSuite::TlsChaCha20Poly1305Sha256 => {
                let h = hkdf::extract::<Sha256>(Some(salt), ikm);
                out[..Sha256::OUTPUT_SIZE].copy_from_slice(h.as_ref());
            }
            CipherSuite::TlsAes256GcmSha384 => {
                let h = hkdf::extract::<Sha384>(Some(salt), ikm);
                out[..Sha384::OUTPUT_SIZE].copy_from_slice(h.as_ref());
            }
        }
        Ok(())
    }

    fn hkdf_expand_label(
        &self,
        out: &mut [u8],
        suite: CipherSuite,
        secret: &[u8],
        label: &[u8],
        context: &[u8],
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

    fn verify_certificate(&self, cert: &ReceivedCertificate, server_name: Option<&str>) -> Result<(), Error> {
        match cert {
            ReceivedCertificate::RawPublicKey {
                ..
            } => Ok(()),
            ReceivedCertificate::X509 {
                chain,
            } => {
                if self.roots.is_none() || self.roots.as_ref().unwrap().is_empty() {
                    return Ok(());
                }
                self.validate_chain(chain, server_name)
            }
        }
    }
}

// ── Chain validation (private) ──

impl DefaultCryptoProvider {
    fn validate_chain(&self, chain: &[ParsedCertificate], server_name: Option<&str>) -> Result<(), Error> {
        if chain.is_empty() {
            return Err(Error::CertificateEmptyChain);
        }

        let server_name = server_name.ok_or(Error::CertificateServerNameRequired)?;

        self.validate_ee_extensions(&chain[0], server_name)?;

        for i in 0..chain.len() {
            let cert = &chain[i];
            let is_ee = i == 0;
            let is_last = i == chain.len() - 1;

            let (issuer_spki, issuer_subject_dn, issuer_spki_alg_oid): (&[u8], &[u8], &[u8]) = {
                if i + 1 < chain.len() {
                    let issuer = &chain[i + 1];
                    (issuer.spki, issuer.subject_dn, issuer.spki_alg_oid)
                } else {
                    match self.find_root(cert.issuer_dn) {
                        Ok(root) => (&root.spki[..], &root.subject_dn[..], &root.spki_alg_oid[..]),
                        Err(_) => {
                            let root = self.find_root_by_spki(cert.spki)?;
                            (&root.spki[..], &root.subject_dn[..], &root.spki_alg_oid[..])
                        }
                    }
                }
            };

            if !x509::dn_equal(cert.issuer_dn, issuer_subject_dn) {
                let is_self_key = !is_ee && self.is_own_root_key(cert.spki);
                if !is_self_key {
                    return Err(Error::CertificateIssuerSubjectDnMismatch);
                }
            }

            let is_cross_signed = !is_ee
                && is_last
                && self.is_own_root_key(cert.spki)
                && !x509::dn_equal(cert.issuer_dn, issuer_subject_dn);
            if !is_cross_signed {
                self.verify_cert_signature(cert, issuer_spki, issuer_spki_alg_oid)
                    .map_err(|_| Error::CertificateSignatureVerificationFailed)?;
            }

            #[cfg(feature = "std")]
            {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                if now < cert.not_before {
                    return Err(Error::CertificateNotYetValid);
                }
                if now > cert.not_after {
                    return Err(Error::CertificateExpired);
                }
            }

            if !is_ee && cert.is_ca != Some(true) {
                return Err(Error::CertificateIntermediateNotCa);
            }
        }

        Ok(())
    }

    fn validate_ee_extensions(&self, ee: &ParsedCertificate, server_name: &str) -> Result<(), Error> {
        let matched = x509::check_san_dns_name(ee.der, server_name).map_err(|_| Error::CertificateParseFailed)?;
        if !matched {
            return Err(Error::CertificateSubjectNameMismatch);
        }

        if ee.is_ca == Some(true) {
            return Err(Error::CertificateEndEntityMustNotBeCa);
        }

        if let Some(false) = ee.has_server_auth_eku {
            return Err(Error::CertificateEkuDoesNotIncludeServerAuth);
        }

        Ok(())
    }

    fn verify_cert_signature(
        &self,
        cert: &ParsedCertificate,
        issuer_spki: &[u8],
        issuer_spki_alg_oid: &[u8],
    ) -> Result<(), Error> {
        let scheme = determine_signature_scheme(cert.sig_alg_oid, issuer_spki_alg_oid)?;

        let public_key = x509::extract_key_from_spki(issuer_spki).map_err(|_| Error::CertificateParseFailed)?;

        self.verify(scheme, public_key, cert.tbs, cert.signature_value)
    }

    fn find_root(&self, issuer_dn: &[u8]) -> Result<&RootCa, Error> {
        if self.roots.is_none() {
            return Err(Error::CertificateNoTrustedRootFound {
                searched_roots: 0,
            });
        }

        let roots = self.roots.as_ref().unwrap();

        for root in roots {
            if x509::dn_equal(&root.subject_dn[..], issuer_dn) {
                return Ok(root);
            }
        }
        Err(Error::CertificateNoTrustedRootFound {
            searched_roots: roots.len(),
        })
    }

    fn find_root_by_spki(&self, spki: &[u8]) -> Result<&RootCa, Error> {
        if self.roots.is_none() {
            return Err(Error::CertificateNoRootFoundBySpkiMatching);
        }

        let roots = self.roots.as_ref().unwrap();
        for root in roots {
            if &root.spki[..] == spki {
                return Ok(root);
            }
        }
        Err(Error::CertificateNoRootFoundBySpkiMatching)
    }

    fn is_own_root_key(&self, spki: &[u8]) -> bool {
        self.find_root_by_spki(spki).is_ok()
    }
}

fn determine_signature_scheme(sig_alg_oid: &[u8], spki_alg_oid: &[u8]) -> Result<SignatureScheme, Error> {
    if sig_alg_oid == x509::OID_ED25519 {
        return Ok(SignatureScheme::Ed25519);
    }
    if sig_alg_oid == x509::OID_ECDSA_SHA256 && spki_alg_oid == x509::OID_EC_PUBLIC_KEY_ALG {
        return Ok(SignatureScheme::EcdsaP256Sha256);
    }
    if sig_alg_oid == x509::OID_ECDSA_SHA384 && spki_alg_oid == x509::OID_EC_PUBLIC_KEY_ALG {
        return Ok(SignatureScheme::EcdsaP384Sha384);
    }
    if sig_alg_oid == x509::OID_RSA_SHA256 {
        return Ok(SignatureScheme::RsaPkcs1Sha256);
    }
    if sig_alg_oid == x509::OID_RSA_SHA384 {
        return Ok(SignatureScheme::RsaPkcs1Sha384);
    }
    if sig_alg_oid == x509::OID_RSA_SHA512 {
        return Ok(SignatureScheme::RsaPkcs1Sha512);
    }
    if sig_alg_oid == x509::OID_RSA_PSS {
        return Ok(SignatureScheme::RsaPssRsaSha256);
    }
    Err(Error::CertificateUnsupportedSignatureAlgorithm)
}

// ── System root loading ──

#[cfg(feature = "std")]
fn load_system_roots() -> alloc::vec::Vec<RootCa> {
    let mut roots = alloc::vec::Vec::with_capacity(120);
    let dirs = [
        "/usr/share/ca-certificates/mozilla",
        "/etc/ssl/certs",
        "/etc/pki/tls/certs",
    ];
    for dir in &dirs {
        load_certs_from_dir(&mut roots, dir);
    }
    roots
}

#[cfg(feature = "std")]
fn load_certs_from_dir(roots: &mut alloc::vec::Vec<RootCa>, dir: &str) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext != "crt" && ext != "pem" && (!ext.is_empty() || !path.is_file()) {
            continue;
        }
        try_load_cert(roots, &path);
    }
}

#[cfg(feature = "std")]
fn try_load_cert(roots: &mut alloc::vec::Vec<RootCa>, path: &std::path::Path) {
    let raw = match std::fs::read(path) {
        Ok(d) => d,
        Err(_) => return,
    };

    let data_owner: std::vec::Vec<u8>;
    let der: &[u8] = if raw.starts_with(b"-----") {
        let mut block_iter = crypto::encoding::pem::decode(&raw);
        match block_iter.next() {
            Some(Ok(block)) => {
                data_owner = block.contents;
                &data_owner
            }
            _ => return,
        }
    } else {
        &raw
    };

    let root = match RootCa::from_der(der) {
        Ok(r) => r,
        Err(_) => return,
    };

    if roots
        .iter()
        .any(|r| x509::dn_equal(&r.subject_dn[..], &root.subject_dn[..]))
    {
        return;
    }

    let _ = roots.push(root);
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
