use crypto::encoding as crypto_encoding;

use crate::{CryptoProvider, Error, MAX_CERTS, SignatureScheme};

#[cfg(not(target_os = "macos"))]
const DEFAULT_ROOT_DIRS: &[&str] = &[
    "/etc/ssl/cert",
    "/etc/ssl/certs",
    "/etc/pki/tls/certs",
    "/usr/local/share/certs",
    "/usr/share/ca-certificates/mozilla",
];

#[cfg(target_os = "macos")]
const DEFAULT_ROOT_DIRS: &[&str] = &["/etc/ssl", "/usr/local/etc/openssl/certs"];

#[async_trait::async_trait]
pub trait CertificateVerifier {
    async fn verify_certificate(&self, cert: &ReceivedCertificate, server_name: Option<&str>) -> Result<(), Error>;
}

/// A certificate received from the peer during the TLS handshake.
pub enum ReceivedCertificate<'a> {
    /// X.509 certificate chain, end-entity first.
    X509 {
        chain: heapless::Vec<ParsedCertificate<'a>, MAX_CERTS>,
    },
    /// Raw public key (RFC 7250).
    RawPublicKey {
        public_key: &'a [u8],
        scheme: SignatureScheme,
    },
}

/// A pre-parsed X.509 certificate with all commonly accessed fields
/// extracted in a single DER walk.
pub struct ParsedCertificate<'a> {
    /// Full DER encoding.
    pub der: &'a [u8],
    /// SubjectPublicKeyInfo DER.
    pub spki: &'a [u8],
    /// Raw public key bytes (BIT STRING content, minus unused-bits byte).
    pub public_key: &'a [u8],
    /// Issuer Distinguished Name (value of the Name SEQUENCE).
    pub issuer_dn: &'a [u8],
    /// Subject Distinguished Name (value of the Name SEQUENCE).
    pub subject_dn: &'a [u8],
    /// TBSCertificate raw bytes (tag + length + value) — the signed portion.
    pub tbs: &'a [u8],
    /// Signature value bytes (BIT STRING content).
    pub signature_value: &'a [u8],
    /// Signature algorithm OID.
    pub sig_alg_oid: &'a [u8],
    /// SPKI algorithm OID.
    pub spki_alg_oid: &'a [u8],
    /// Whether Basic Constraints cA is TRUE (`None` = extension absent).
    pub is_ca: Option<bool>,
    /// Whether EKU includes serverAuth (`None` = extension absent).
    pub has_server_auth_eku: Option<bool>,
    /// notBefore as Unix timestamp (seconds since epoch).
    pub not_before: u64,
    /// notAfter as Unix timestamp (seconds since epoch).
    pub not_after: u64,
}

impl<'a> ParsedCertificate<'a> {
    /// Parse a DER-encoded X.509 certificate, extracting all fields at once.
    pub fn from_der(der: &'a [u8]) -> Result<Self, Error> {
        let spki = x509::extract_spki_from_cert(der).map_err(|_| Error::CertificateParseFailed)?;
        let public_key = x509::extract_key_from_spki(spki).map_err(|_| Error::CertificateParseFailed)?;
        let issuer_dn = x509::extract_issuer_dn(der).map_err(|_| Error::CertificateParseFailed)?;
        let subject_dn = x509::extract_subject_dn(der).map_err(|_| Error::CertificateParseFailed)?;
        let tbs = x509::extract_tbs_cert(der).map_err(|_| Error::CertificateParseFailed)?;
        let signature_value = x509::extract_signature_value(der).map_err(|_| Error::CertificateParseFailed)?;
        let sig_alg_oid = x509::extract_signature_algorithm_oid(der).map_err(|_| Error::CertificateParseFailed)?;
        let spki_alg_oid = x509::extract_spki_algorithm_oid(spki).map_err(|_| Error::CertificateParseFailed)?;
        let is_ca = x509::is_ca(der);
        let has_server_auth_eku = x509::has_eku_server_auth(der);
        let (nb, na) = x509::parse_validity(der).map_err(|_| Error::CertificateParseFailed)?;

        Ok(Self {
            der,
            spki,
            public_key,
            issuer_dn,
            subject_dn,
            tbs,
            signature_value,
            sig_alg_oid,
            spki_alg_oid,
            is_ca,
            has_server_auth_eku,
            not_before: nb.to_unix_seconds(),
            not_after: na.to_unix_seconds(),
        })
    }
}

/// A trusted root certificate authority with pre-parsed fields.
///
/// All fields use fixed-capacity [`heapless::Vec`] for zero-allocation
/// storage. Use [`RootCa::from_der`] to parse a DER-encoded certificate.
#[derive(Clone)]
pub struct RootCa {
    pub subject_dn: heapless::Vec<u8, 256>,
    pub spki: heapless::Vec<u8, 512>,
    pub spki_alg_oid: heapless::Vec<u8, 16>,
    pub not_before: u64,
    pub not_after: u64,
}

impl RootCa {
    /// Parse a DER-encoded X.509 certificate into a root trust anchor.
    pub fn from_der(der: &[u8]) -> Result<Self, Error> {
        let parsed = ParsedCertificate::from_der(der)?;
        let mut subject_dn = heapless::Vec::new();
        subject_dn
            .extend_from_slice(parsed.subject_dn)
            .map_err(|_| Error::CertificateParseFailed)?;
        let mut spki = heapless::Vec::new();
        spki.extend_from_slice(parsed.spki)
            .map_err(|_| Error::CertificateParseFailed)?;
        let mut spki_alg_oid = heapless::Vec::new();
        spki_alg_oid
            .extend_from_slice(parsed.spki_alg_oid)
            .map_err(|_| Error::CertificateParseFailed)?;
        Ok(Self {
            subject_dn,
            spki,
            spki_alg_oid,
            not_before: parsed.not_before,
            not_after: parsed.not_after,
        })
    }
}

/// A raw public key pinned as a trust anchor (RFC 7250).
///
/// Used with [`DefaultCertificateVerifier::with_raw_keys`] to validate
/// raw-public-key connections. Each entry holds the raw key bytes and
/// the [`SignatureScheme`] the key belongs to.
#[derive(Clone)]
pub struct RawPublicKey {
    pub public_key: alloc::borrow::Cow<'static, [u8]>,
    pub scheme: SignatureScheme,
}

impl RawPublicKey {
    pub fn new(scheme: SignatureScheme, public_key: impl Into<alloc::borrow::Cow<'static, [u8]>>) -> Self {
        Self {
            public_key: public_key.into(),
            scheme,
        }
    }
}

// ── Clock trait ──

/// A wall clock used to check certificate validity periods.
///
/// Returns the current Unix timestamp (seconds since epoch). This trait
/// exists so that no_std environments can inject their own time source
/// (hardware RTC, NTP, etc.) instead of relying on `std::time::SystemTime`.
pub trait Clock: Send + Sync {
    fn now(&self) -> u64;
}

/// A [`Clock`] backed by `std::time::SystemTime`.
///
/// Available only when the `std` feature is enabled.
#[cfg(feature = "std")]
pub struct SystemClock;

#[cfg(feature = "std")]
impl Clock for SystemClock {
    fn now(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

// ── Default certificate verifier ──

/// Default certificate validator that can handle both X509 chains and raw public keys.
///
/// Validates X.509 chains and raw public keys against configured trust
/// anchors.  Use [`with_roots`][Self::with_roots] to seed X.509 root CAs
/// and [`with_raw_keys`][Self::with_raw_keys] to pin raw public keys
/// (RFC 7250).
///
/// Call [`danger_with_no_verification`][Self::danger_with_no_verification] to skip
/// all validation. Intended for testing only.
///
/// This is the recommended `CertificateVerifier` unless you are working with very tiny embedded
/// systems that only need one or more pinned keys.
///
/// # Examples
///
/// ```ignore
/// use tls2::{CertificateVerifier, ClientConfig, DefaultCertificateVerifier, RawPublicKey, SignatureScheme};
///
/// // Full X.509 validation with system roots (requires `std` feature)
/// let verifier = DefaultCertificateVerifier::new(DefaultCryptoProvider).with_system_roots();
///
/// // Pin raw public keys only, reject everything else
/// let pinned = RawPublicKey::new(SignatureScheme::Ed25519, b"...key bytes...");
/// let verifier = DefaultCertificateVerifier::new().with_raw_keys([pinned]);
///
/// // Testing mode — accept anything
/// let verifier = DefaultCertificateVerifier::new().danger_with_no_verification();
/// ```
#[cfg(feature = "default-certificate-verifier")]
#[derive(Clone)]
pub struct DefaultCertificateVerifier<C: CryptoProvider> {
    crypto: C,
    roots: Option<alloc::vec::Vec<RootCa>>,
    raw_keys: Option<alloc::vec::Vec<RawPublicKey>>,
    accept_any: bool,
    clock: Option<alloc::sync::Arc<dyn Clock>>,
}

#[cfg(feature = "default-certificate-verifier")]
impl<C: CryptoProvider> DefaultCertificateVerifier<C> {
    /// Create a new verifier with no trust anchors configured.
    ///
    /// By default:
    /// - No X.509 roots are configured — X.509 chains are rejected.
    /// - No raw public keys are pinned — raw public keys are rejected.
    /// - Validation is not bypassed.
    ///
    /// Use the builder methods ([`with_roots`][Self::with_roots],
    /// [`with_system_roots`][Self::with_system_roots],
    /// [`with_raw_keys`][Self::with_raw_keys],
    /// [`danger_with_no_verification`][Self::danger_with_no_verification]) to configure
    /// the verifier.
    pub fn new(crypto: C) -> Self {
        Self {
            crypto,
            roots: None,
            raw_keys: None,
            accept_any: false,
            clock: None,
        }
    }

    /// Load root CAs from the operating system's trust store.

    pub fn with_system_roots(mut self) -> Self {
        self.roots = Some(load_roots(DEFAULT_ROOT_DIRS));
        self
    }

    /// Add custom root trust anchors.
    pub fn with_roots(mut self, custom_roots: impl IntoIterator<Item = RootCa>) -> Self {
        let iter = custom_roots.into_iter();
        let (iter_size_hint, _) = iter.size_hint();
        let mut roots = self
            .roots
            .take()
            .unwrap_or(alloc::vec::Vec::with_capacity(iter_size_hint));

        roots.reserve(iter_size_hint); // basically a no-op if the Vec is slaready correctly sized
        roots.extend(iter);
        self.roots = Some(roots);
        self
    }

    /// Pin raw public keys as trust anchors (RFC 7250).
    ///
    /// When raw keys are configured, only received raw public keys
    /// that match one of the pinned entries are accepted.  X.509
    /// chains are not affected by this list.
    pub fn with_raw_keys(mut self, keys: impl IntoIterator<Item = RawPublicKey>) -> Self {
        let iter = keys.into_iter();
        let (iter_size_hint, _) = iter.size_hint();
        let mut raw_keys = self
            .raw_keys
            .take()
            .unwrap_or(alloc::vec::Vec::with_capacity(iter_size_hint));

        raw_keys.reserve(iter_size_hint); // basically a no-op if the Vec is slaready correctly sized
        raw_keys.extend(iter);
        self.raw_keys = Some(raw_keys);
        self
    }

    /// Skip all certificate validation.
    ///
    /// Every certificate — X.509 chain or raw public key — is
    /// accepted without any checks.  **Insecure; intended for
    /// testing only.**
    pub fn danger_with_no_verification(mut self) -> Self {
        self.accept_any = true;
        self
    }

    /// Use a custom clock for certificate validity checks.
    ///
    /// When no clock is set, [`DefaultCertificateVerifier`] falls back to
    /// `std::time::SystemTime` if the `std` feature is enabled.  Without
    /// `std` and without a custom clock, validation returns
    /// [`Error::CertificateClockMissing`].
    pub fn with_clock(mut self, clock: alloc::sync::Arc<dyn Clock>) -> Self {
        self.clock = Some(clock);
        self
    }
}

#[cfg(feature = "default-certificate-verifier")]
#[async_trait::async_trait]
impl<C: CryptoProvider> CertificateVerifier for DefaultCertificateVerifier<C> {
    async fn verify_certificate(&self, cert: &ReceivedCertificate, server_name: Option<&str>) -> Result<(), Error> {
        if self.accept_any {
            return Ok(());
        }

        match cert {
            ReceivedCertificate::RawPublicKey {
                public_key,
                scheme,
            } => {
                let keys = self.raw_keys.as_ref().ok_or(Error::CertificateNoTrustedRootFound {
                    searched_roots: 0,
                })?;
                for key in keys {
                    if key.scheme == *scheme && key.public_key.as_ref() == *public_key {
                        return Ok(());
                    }
                }
                Err(Error::CertificateNoTrustedRootFound {
                    searched_roots: keys.len(),
                })
            }
            ReceivedCertificate::X509 {
                chain,
            } => {
                if self.roots.is_none() || self.roots.as_ref().unwrap().is_empty() {
                    return Err(Error::CertificateNoTrustedRootFound {
                        searched_roots: 0,
                    });
                }
                self.validate_chain(chain, server_name)
            }
        }
    }
}

// ── Chain validation (private) ──

#[cfg(feature = "default-certificate-verifier")]
impl<C: CryptoProvider> DefaultCertificateVerifier<C> {
    fn validate_chain(&self, chain: &[ParsedCertificate], server_name: Option<&str>) -> Result<(), Error> {
        if chain.is_empty() {
            return Err(Error::CertificateEmptyChain);
        }

        let server_name = server_name.ok_or(Error::CertificateServerNameRequired)?;

        self.validate_ee_extensions(&chain[0], server_name)?;

        let now = match &self.clock {
            Some(clock) => clock.now(),
            #[cfg(feature = "std")]
            None => std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            #[cfg(not(feature = "std"))]
            None => return Err(Error::CertificateClockMissing),
        };

        for i in 0..chain.len() {
            let cert = &chain[i];
            let is_ee = i == 0;
            let is_last = i == chain.len() - 1;

            let (issuer_spki, issuer_subject_dn, issuer_spki_alg_oid): (&[u8], &[u8], &[u8]) = {
                if i + 1 < chain.len() {
                    let issuer = &chain[i + 1];
                    (issuer.spki, issuer.subject_dn, issuer.spki_alg_oid)
                } else {
                    match self.find_root(cert.issuer_dn, now) {
                        Ok(root) => (&root.spki[..], &root.subject_dn[..], &root.spki_alg_oid[..]),
                        Err(_) => {
                            let root = self.find_root_by_spki(cert.spki, now)?;
                            (&root.spki[..], &root.subject_dn[..], &root.spki_alg_oid[..])
                        }
                    }
                }
            };

            if !x509::dn_equal(cert.issuer_dn, issuer_subject_dn) {
                let is_self_key = !is_ee && self.is_own_root_key(cert.spki, now);
                if !is_self_key {
                    return Err(Error::CertificateIssuerSubjectDnMismatch);
                }
            }

            let is_cross_signed = !is_ee
                && is_last
                && self.is_own_root_key(cert.spki, now)
                && !x509::dn_equal(cert.issuer_dn, issuer_subject_dn);
            if !is_cross_signed {
                self.verify_cert_signature(cert, issuer_spki, issuer_spki_alg_oid)
                    .map_err(|_| Error::CertificateSignatureVerificationFailed)?;
            }

            if now < cert.not_before {
                return Err(Error::CertificateNotYetValid);
            }
            if now > cert.not_after {
                return Err(Error::CertificateExpired);
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

        self.crypto.verify(scheme, public_key, cert.tbs, cert.signature_value)
    }

    // now is an Unix timestamp in second
    fn find_root(&self, issuer_dn: &[u8], now: u64) -> Result<&RootCa, Error> {
        if self.roots.is_none() {
            return Err(Error::CertificateNoTrustedRootFound {
                searched_roots: 0,
            });
        }

        let roots = self.roots.as_ref().unwrap();

        for root in roots {
            if now < root.not_before || now > root.not_after {
                continue;
            }
            if x509::dn_equal(&root.subject_dn[..], issuer_dn) {
                return Ok(root);
            }
        }
        Err(Error::CertificateNoTrustedRootFound {
            searched_roots: roots.len(),
        })
    }

    // now is an Unix timestamp in second
    fn find_root_by_spki(&self, spki: &[u8], now: u64) -> Result<&RootCa, Error> {
        if self.roots.is_none() {
            return Err(Error::CertificateNoRootFoundBySpkiMatching);
        }

        let roots = self.roots.as_ref().unwrap();
        for root in roots {
            if now < root.not_before || now > root.not_after {
                continue;
            }

            if &root.spki[..] == spki {
                return Ok(root);
            }
        }
        Err(Error::CertificateNoRootFoundBySpkiMatching)
    }

    fn is_own_root_key(&self, spki: &[u8], now: u64) -> bool {
        self.find_root_by_spki(spki, now).is_ok()
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
pub fn load_roots(paths: &[&str]) -> alloc::vec::Vec<RootCa> {
    let mut roots = alloc::vec::Vec::with_capacity(120);
    for dir in paths {
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
        if ext != "crt" && ext != "pem" && ext != "cer" && (!ext.is_empty() || !path.is_file()) {
            continue;
        }
        load_certs_from_file(roots, &path);
    }
}

/// Try to load one or more certificates per file.
/// If the file contains PEM-encoded certificates, it loads them all.
/// Otherwise, it tries to parse the file as a binary DER-encoded certificate.
fn load_certs_from_file(roots: &mut alloc::vec::Vec<RootCa>, path: &std::path::Path) {
    let raw = match std::fs::read(path) {
        Ok(d) => d,
        Err(_) => return,
    };

    if raw.trim_ascii().starts_with(b"-----") {
        for block in crypto_encoding::pem::decode(&raw) {
            let Ok(block) = block else { continue };
            let root = match RootCa::from_der(&block.contents) {
                Ok(r) => r,
                Err(_) => continue,
            };
            if roots
                .iter()
                .any(|r| x509::dn_equal(&r.subject_dn[..], &root.subject_dn[..]))
            {
                continue;
            }
            let _ = roots.push(root);
        }
    } else {
        if let Ok(root) = RootCa::from_der(&raw) {
            if !roots
                .iter()
                .any(|r| x509::dn_equal(&r.subject_dn[..], &root.subject_dn[..]))
            {
                roots.push(root);
            }
        }
    }
}

// ── Tests ──

#[cfg(all(test, feature = "crypto-default-provider"))]
mod tests {
    use super::*;
    use crate::{SignatureScheme, crypto_default_provider::DefaultCryptoProvider};

    fn tokio_runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Runtime::new().unwrap()
    }

    fn rpk(scheme: SignatureScheme, key: &[u8]) -> ReceivedCertificate<'_> {
        ReceivedCertificate::RawPublicKey {
            public_key: key,
            scheme,
        }
    }

    #[test]
    fn raw_key_match_accepts() {
        let pinned = RawPublicKey::new(SignatureScheme::Ed25519, b"\x00\x01\x02\x03\x04");
        let verifier = DefaultCertificateVerifier::new(DefaultCryptoProvider).with_raw_keys([pinned]);

        let rt = tokio_runtime();
        let result =
            rt.block_on(verifier.verify_certificate(&rpk(SignatureScheme::Ed25519, b"\x00\x01\x02\x03\x04"), None));
        assert!(result.is_ok(), "matching raw key should be accepted");
    }

    #[test]
    fn raw_key_mismatch_key_rejects() {
        let pinned = RawPublicKey::new(SignatureScheme::Ed25519, b"\x00\x01\x02\x03\x04");
        let verifier = DefaultCertificateVerifier::new(DefaultCryptoProvider).with_raw_keys([pinned]);

        let rt = tokio_runtime();
        let result =
            rt.block_on(verifier.verify_certificate(&rpk(SignatureScheme::Ed25519, b"\xff\xff\xff\xff\xff"), None));
        assert!(result.is_err(), "mismatched key bytes should be rejected");
    }

    #[test]
    fn raw_key_mismatch_scheme_rejects() {
        let pinned = RawPublicKey::new(SignatureScheme::Ed25519, b"\x00\x01\x02\x03\x04");
        let verifier = DefaultCertificateVerifier::new(DefaultCryptoProvider).with_raw_keys([pinned]);

        let rt = tokio_runtime();
        let result = rt.block_on(
            verifier.verify_certificate(&rpk(SignatureScheme::EcdsaP256Sha256, b"\x00\x01\x02\x03\x04"), None),
        );
        assert!(result.is_err(), "mismatched scheme should be rejected");
    }

    #[test]
    fn raw_key_no_keys_rejects() {
        let verifier = DefaultCertificateVerifier::new(DefaultCryptoProvider);

        let rt = tokio_runtime();
        let result =
            rt.block_on(verifier.verify_certificate(&rpk(SignatureScheme::Ed25519, b"\x00\x01\x02\x03\x04"), None));
        assert!(result.is_err(), "no pinned keys should reject raw public key");
    }

    #[test]
    fn no_verification_accepts_raw_key() {
        let verifier = DefaultCertificateVerifier::new(DefaultCryptoProvider).danger_with_no_verification();

        let rt = tokio_runtime();
        let result =
            rt.block_on(verifier.verify_certificate(&rpk(SignatureScheme::Ed25519, b"\x00\x01\x02\x03\x04"), None));
        assert!(result.is_ok(), "with_no_verification should accept raw public key");
    }
}
