#[cfg(feature = "webpki-validator")]
use alloc::{boxed::Box, sync::Arc, vec::Vec};

#[cfg(feature = "webpki-validator")]
use async_trait::async_trait;
#[cfg(feature = "webpki-validator")]
use bytes::Bytes;

#[cfg(all(feature = "webpki-validator", feature = "std"))]
use crate::config::SystemClock;
#[cfg(feature = "webpki-validator")]
use crate::{
    Error,
    config::{CertificateValidator, Clock, ReceivedCertificate},
    crypto::{CryptoProvider, SignatureScheme},
    errors::{
        CertificateChainFailure, CertificateParseFailure, CertificateValidationFailure, InternalFailure, PeerSide,
    },
};

#[derive(Clone)]
pub struct RootCa {
    pub subject: &'static [u8],
    pub spki: &'static [u8],
}

/// A [`CertificateValidator`] that validates X.509 certificate chains against
/// a set of root CA trust anchors.
///
/// Loads root CAs from the operating system's trust store and validates the
/// certificate chain, subject name (SAN), validity period, key usage, and
/// basic constraints.
///
/// The validator uses a [`CryptoProvider`] for signature verification across
/// the chain.
///
/// # Raw public keys
///
/// This validator rejects [`ReceivedCertificate::RawPublicKey`] variants.
/// Raw public key trust decisions (e.g. key pinning) are application‑specific
/// and must use a custom validator.
#[cfg(feature = "webpki-validator")]
pub struct WebPkiValidator {
    roots: Vec<RootCa>,
    crypto: Arc<dyn CryptoProvider>,
    clock: Arc<dyn Clock>,
}

#[cfg(feature = "webpki-validator")]
impl core::fmt::Debug for WebPkiValidator {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("WebPkiValidator")
            .field("roots", &self.roots.len())
            .finish()
    }
}

#[cfg(feature = "webpki-validator")]
impl Clone for WebPkiValidator {
    fn clone(&self) -> Self {
        Self {
            roots: self.roots.clone(),
            crypto: Arc::clone(&self.crypto),
            clock: Arc::clone(&self.clock),
        }
    }
}

#[cfg(feature = "webpki-validator")]
impl WebPkiValidator {
    /// Create a validator with root CAs loaded from the operating system's
    /// trust store, using the system clock for validity checks.
    ///
    /// `crypto` is used to verify signatures across the certificate chain.
    ///
    /// Available only when the `std` feature is enabled.
    #[cfg(feature = "std")]
    pub fn with_default_roots(crypto: Arc<dyn CryptoProvider>) -> Self {
        Self {
            roots: load_system_roots(),
            crypto,
            clock: Arc::new(SystemClock),
        }
    }

    /// Create a validator with custom root trust anchors, using the system
    /// clock for validity checks.
    ///
    /// Available only when the `std` feature is enabled.
    #[cfg(feature = "std")]
    pub fn with_custom_roots(crypto: Arc<dyn CryptoProvider>, roots: Vec<RootCa>) -> Self {
        Self {
            roots,
            crypto,
            clock: Arc::new(SystemClock),
        }
    }

    /// Create a validator with custom root trust anchors and a custom clock.
    ///
    /// This constructor works in both `std` and `no_std` environments.
    /// The caller is responsible for providing a [`Clock`] implementation
    /// that returns the current Unix timestamp in seconds.
    pub fn with_custom_roots_and_clock(
        crypto: Arc<dyn CryptoProvider>,
        roots: Vec<RootCa>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            roots,
            crypto,
            clock,
        }
    }
}

#[cfg(feature = "webpki-validator")]
#[async_trait]
impl CertificateValidator for WebPkiValidator {
    async fn validate(&self, cert: &ReceivedCertificate, server_name: Option<&str>) -> Result<(), Error> {
        match cert {
            ReceivedCertificate::X509 {
                chain, ..
            } => self.validate_chain(chain, server_name),
            ReceivedCertificate::RawPublicKey {
                ..
            } => Err(Error::CertificateValidationFailed(
                CertificateValidationFailure::RawPublicKeyRequiresCustomValidator,
            )),
        }
    }
}

#[cfg(feature = "webpki-validator")]
impl WebPkiValidator {
    fn validate_chain(&self, chain: &[Bytes], server_name: Option<&str>) -> Result<(), Error> {
        if chain.is_empty() {
            return Err(Error::CertificateValidationFailed(CertificateValidationFailure::EmptyChain));
        }

        let server_name = server_name
            .ok_or_else(|| Error::CertificateValidationFailed(CertificateValidationFailure::ServerNameRequired))?;

        // Validate EE certificate content (SAN, EKU, key usage, basic constraints).
        self.validate_ee_extensions(&chain[0], server_name)?;

        // Verify signatures and constraints through the chain.
        for i in 0..chain.len() {
            let cert_der = &chain[i];
            let is_ee = i == 0;
            let is_last = i == chain.len() - 1;

            // Get the issuer's SPKI and subject DN.
            let (issuer_spki, issuer_subject_dn): (&[u8], &[u8]) = {
                if i + 1 < chain.len() {
                    // Next cert in the chain is the issuer.
                    let issuer_der = &chain[i + 1];
                    let spki_full = x509::extract_spki_from_cert(issuer_der).map_err(|_| {
                        Error::CertificateValidationFailed(CertificateValidationFailure::ParseError(
                        CertificateParseFailure::IssuerSpkiParse { chain_index: i + 1 },
                        ))
                    })?;
                    let subject = x509::extract_subject_dn(issuer_der).map_err(|_| {
                        Error::CertificateValidationFailed(CertificateValidationFailure::ParseError(
                        CertificateParseFailure::IssuerSubjectDnParse { chain_index: i + 1 },
                        ))
                    })?;
                    (spki_full, subject)
                } else {
                    // Signed by a trusted root.
                    let cert_issuer_dn = x509::extract_issuer_dn(cert_der).map_err(|_| {
                        Error::CertificateValidationFailed(CertificateValidationFailure::ParseError(
                        CertificateParseFailure::CertIssuerDnParse { chain_index: i },
                        ))
                    })?;
                    match self.find_root(cert_issuer_dn) {
                        Ok(root) => (root.spki, root.subject),
                        Err(_) => {
                            // No DN match. Try SPKI matching: the last cert might be
                            // a cross-signed intermediate whose SPKI matches a trusted
                            // root (same key, different DN).
                            let root = self.find_root_by_spki(cert_der)?;
                            (root.spki, root.subject)
                        }
                    }
                }
            };
            let cert_issuer_dn = x509::extract_issuer_dn(cert_der).map_err(|_| {
                Error::CertificateValidationFailed(CertificateValidationFailure::ParseError(
                    CertificateParseFailure::CertIssuerDn,
                ))
            })?;
            if !x509::dn_equal(cert_issuer_dn, issuer_subject_dn) {
                // Cross-signed case: if this is the last intermediate in the chain
                // (not the EE cert) and its SPKI matches a trusted root, the DN
                // mismatch is expected — the root's key is the same, only the
                // issuer DN differs due to cross-signing.
                let is_self_key = !is_ee && self.is_own_root_key(cert_der);
                if !is_self_key {
                    return Err(Error::CertificateValidationFailed(
                        CertificateValidationFailure::ChainValidation(CertificateChainFailure::IssuerSubjectDnMismatch),
                    ));
                }
            }

            // Verify signature (skip for cross-signed root certs only:
            // never for the EE cert, and only when matched by SPKI).
            let is_cross_signed = !is_ee
                && is_last
                && self.is_own_root_key(cert_der)
                && !x509::dn_equal(cert_issuer_dn, issuer_subject_dn);
            if !is_cross_signed {
                self.verify_cert_signature(cert_der, issuer_spki).map_err(|_| {
                    Error::CertificateValidationFailed(CertificateValidationFailure::ChainValidation(
                        CertificateChainFailure::SignatureVerificationFailed { chain_index: i },
                    ))
                })?;
            }

            // Check validity period.
            let (nb, na) = x509::parse_validity(cert_der).map_err(|_| {
                Error::CertificateValidationFailed(CertificateValidationFailure::ParseError(
                    CertificateParseFailure::Validity,
                ))
            })?;
            let now = self.clock.now();
            if now < nb.to_unix_seconds() {
                return Err(Error::CertificateValidationFailed(
                    CertificateValidationFailure::ChainValidation(CertificateChainFailure::CertificateNotYetValid),
                ));
            }
            if now > na.to_unix_seconds() {
                return Err(Error::CertificateValidationFailed(
                    CertificateValidationFailure::ChainValidation(CertificateChainFailure::CertificateExpired),
                ));
            }

            // Intermediate certs must be CAs.
            if !is_ee {
                if x509::is_ca(cert_der) != Some(true) {
                    return Err(Error::CertificateValidationFailed(
                        CertificateValidationFailure::ChainValidation(CertificateChainFailure::IntermediateNotCa),
                    ));
                }
            }
        }

        Ok(())
    }

    fn validate_ee_extensions(&self, ee_der: &[u8], server_name: &str) -> Result<(), Error> {
        let dns_names = x509::parse_san_dns_names(ee_der).map_err(|_| {
            Error::CertificateValidationFailed(CertificateValidationFailure::ParseError(
                CertificateParseFailure::SanParse,
            ))
        })?;
        if !dns_names.iter().any(|n| dns_name_matches(n, server_name)) {
            return Err(Error::CertificateValidationFailed(
                CertificateValidationFailure::SubjectNameMismatch,
            ));
        }

        if x509::is_ca(ee_der) == Some(true) {
            return Err(Error::CertificateValidationFailed(
                CertificateValidationFailure::ChainValidation(CertificateChainFailure::EndEntityMustNotBeCa),
            ));
        }

        if let Some(has_server_auth) = x509::has_eku_server_auth(ee_der) {
            if !has_server_auth {
                return Err(Error::CertificateValidationFailed(
                    CertificateValidationFailure::ChainValidation(CertificateChainFailure::EkuDoesNotIncludeServerAuth),
                ));
            }
        }

        Ok(())
    }

    fn verify_cert_signature(&self, cert_der: &[u8], issuer_spki: &[u8]) -> Result<(), Error> {
        let tbs_data = extract_tbs(cert_der).map_err(|_| {
            Error::CertificateValidationFailed(CertificateValidationFailure::ParseError(
                CertificateParseFailure::TbsParse,
            ))
        })?;

        let signature = extract_signature_value(cert_der).map_err(|_| {
            Error::CertificateValidationFailed(CertificateValidationFailure::ParseError(
                CertificateParseFailure::SignatureParse,
            ))
        })?;

        let scheme = determine_signature_scheme(cert_der, issuer_spki)?;

        let public_key = x509::extract_key_from_spki(issuer_spki).map_err(|_| {
            Error::CertificateValidationFailed(CertificateValidationFailure::ParseError(
                CertificateParseFailure::IssuerPublicKeyParse,
            ))
        })?;

        self.crypto.verify_signature(scheme, public_key, tbs_data, signature)
    }

    fn find_root(&self, issuer_dn: &[u8]) -> Result<&RootCa, Error> {
        for root in &self.roots {
            if x509::dn_equal(root.subject, issuer_dn) {
                return Ok(root);
            }
        }
        Err(Error::CertificateValidationFailed(
            CertificateValidationFailure::ChainValidation(CertificateChainFailure::NoTrustedRootFound {
                searched_roots: self.roots.len(),
            }),
        ))
    }

    /// Find a root whose SPKI matches the given cert's SPKI.
    ///
    /// This handles cross-signed intermediates: a cert may have the same
    /// public key (SPKI) as a trusted root but a different issuer DN.
    fn find_root_by_spki(&self, cert_der: &[u8]) -> Result<&RootCa, Error> {
        let cert_spki = x509::extract_spki_from_cert(cert_der).map_err(|_| {
            Error::CertificateValidationFailed(CertificateValidationFailure::ParseError(
                CertificateParseFailure::CertSpkiForKeyMatching,
            ))
        })?;
        for root in &self.roots {
            if root.spki == cert_spki {
                return Ok(root);
            }
        }
        Err(Error::CertificateValidationFailed(
            CertificateValidationFailure::ChainValidation(CertificateChainFailure::NoRootFoundBySpkiMatching),
        ))
    }

    /// Check if this cert's SPKI matches any trusted root's SPKI.
    fn is_own_root_key(&self, cert_der: &[u8]) -> bool {
        self.find_root_by_spki(cert_der).is_ok()
    }
}

// ── TLV helpers ────────────────────────────────────────────────────────────

fn extract_tbs(cert_der: &[u8]) -> Result<&[u8], Error> {
    let outer = x509_parse_tlv(cert_der).map_err(|()| {
        Error::CertificateValidationFailed(CertificateValidationFailure::ParseError(
            CertificateParseFailure::InvalidCertDer,
        ))
    })?;
    let tbs = x509_parse_tlv(outer.2).map_err(|()| {
        Error::CertificateValidationFailed(CertificateValidationFailure::ParseError(
            CertificateParseFailure::InvalidTbs,
        ))
    })?;
    Ok(tbs.0)
}

fn extract_signature_value(cert_der: &[u8]) -> Result<&[u8], Error> {
    let outer = x509_parse_tlv(cert_der).map_err(|()| {
        Error::CertificateValidationFailed(CertificateValidationFailure::ParseError(
            CertificateParseFailure::InvalidCertDer,
        ))
    })?;
    let tbs = x509_parse_tlv(outer.2).map_err(|()| {
        Error::CertificateValidationFailed(CertificateValidationFailure::ParseError(
            CertificateParseFailure::InvalidTbs,
        ))
    })?;
    let after_tbs = &outer.2[tbs.0.len()..];
    let sig_alg = x509_parse_tlv(after_tbs).map_err(|()| {
        Error::CertificateValidationFailed(CertificateValidationFailure::ParseError(
            CertificateParseFailure::InvalidSignatureAlgorithm,
        ))
    })?;
    let after_sig_alg = &after_tbs[sig_alg.0.len()..];
    let sig = x509_parse_tlv(after_sig_alg).map_err(|()| {
        Error::CertificateValidationFailed(CertificateValidationFailure::ParseError(
            CertificateParseFailure::InvalidSignatureValue,
        ))
    })?;
    if sig.2.is_empty() {
        return Err(Error::CertificateValidationFailed(CertificateValidationFailure::ParseError(
            CertificateParseFailure::EmptySignature,
        )));
    }
    Ok(&sig.2[1..])
}

fn determine_signature_scheme(cert_der: &[u8], issuer_spki: &[u8]) -> Result<SignatureScheme, Error> {
    let sig_oid = x509::extract_signature_algorithm_oid(cert_der).map_err(|_| {
        Error::CertificateValidationFailed(CertificateValidationFailure::ParseError(
            CertificateParseFailure::SignatureAlgorithm,
        ))
    })?;

    let spki_alg_oid = x509::extract_spki_algorithm_oid(issuer_spki).map_err(|_| {
        Error::CertificateValidationFailed(CertificateValidationFailure::ParseError(
            CertificateParseFailure::SpkiAlgorithm,
        ))
    })?;

    if sig_oid == x509::OID_ED25519 {
        return Ok(SignatureScheme::Ed25519);
    }
    if sig_oid == x509::OID_ECDSA_SHA256 && spki_alg_oid == x509::OID_EC_PUBLIC_KEY_ALG {
        return Ok(SignatureScheme::EcdsaP256Sha256);
    }
    if sig_oid == x509::OID_ECDSA_SHA384 && spki_alg_oid == x509::OID_EC_PUBLIC_KEY_ALG {
        return Ok(SignatureScheme::EcdsaP384Sha384);
    }
    if sig_oid == x509::OID_RSA_SHA256 {
        return Ok(SignatureScheme::RsaPkcs1Sha256);
    }
    if sig_oid == x509::OID_RSA_SHA384 {
        return Ok(SignatureScheme::RsaPkcs1Sha384);
    }
    if sig_oid == x509::OID_RSA_SHA512 {
        return Ok(SignatureScheme::RsaPkcs1Sha512);
    }
    if sig_oid == x509::OID_RSA_PSS {
        return Ok(SignatureScheme::RsaPssRsaSha256);
    }

    Err(Error::CertificateValidationFailed(
        CertificateValidationFailure::ChainValidation(CertificateChainFailure::UnsupportedSignatureAlgorithm),
    ))
}

fn x509_parse_tlv(data: &[u8]) -> Result<(&[u8], u8, &[u8]), ()> {
    if data.len() < 2 {
        return Err(());
    }
    let tag = data[0];
    let len_byte = data[1];
    let (len, len_size) = if len_byte & 0x80 == 0 {
        (len_byte as usize, 1)
    } else {
        let num_bytes = (len_byte & 0x7f) as usize;
        if num_bytes == 0 || data.len() < 2 + num_bytes {
            return Err(());
        }
        let mut l = 0usize;
        for i in 0..num_bytes {
            l = (l << 8) | data[2 + i] as usize;
        }
        (l, 1 + num_bytes)
    };
    let start = 1 + len_size;
    let end = start.checked_add(len).ok_or(())?;
    if end > data.len() {
        return Err(());
    }
    Ok((&data[..end], tag, &data[start..end]))
}

fn dns_name_matches(san_entry: &[u8], server_name: &str) -> bool {
    let Ok(san_str) = core::str::from_utf8(san_entry) else {
        return false;
    };
    server_name_matches_wildcard(san_str, server_name)
}

fn server_name_matches_wildcard(dns_name: &str, server_name: &str) -> bool {
    let dns_name = dns_name.to_ascii_lowercase();
    let server_name = server_name.to_ascii_lowercase();

    if let Some(rest) = dns_name.strip_prefix("*.") {
        let Some(dot_pos) = server_name.find('.') else {
            return false;
        };
        let suffix = &server_name[dot_pos..];
        rest.eq_ignore_ascii_case(suffix)
            && server_name[..dot_pos].len() > 0
            && server_name[dot_pos + 1..].contains('.') == rest.contains('.')
    } else {
        dns_name == server_name
    }
}

// ── System root loading ────────────────────────────────────────────────────

/// Load root CAs from the operating system's trust store.
#[cfg(feature = "std")]
fn load_system_roots() -> Vec<RootCa> {
    // we pre-alloc 60 because there is around 118 Mozilla's root certs
    // so we expect system certs to be around that number.
    let mut roots = Vec::with_capacity(60);
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
fn load_certs_from_dir(roots: &mut Vec<RootCa>, dir: &str) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        // Accept .crt, .pem, and hash-link files (no extension)
        if ext != "crt" && ext != "pem" && (!ext.is_empty() || !path.is_file()) {
            continue;
        }
        try_load_cert(roots, &path);
    }
}

#[cfg(feature = "std")]
fn try_load_cert(roots: &mut Vec<RootCa>, path: &std::path::Path) {
    let raw = match std::fs::read(path) {
        Ok(d) => d,
        Err(_) => return,
    };

    // Parse PEM or raw DER.
    let der = if raw.starts_with(b"-----") {
        let mut block_iter = crypto::encoding::pem::decode(&raw);
        match block_iter.next() {
            Some(Ok(block)) => block.contents,
            _ => return,
        }
    } else {
        raw
    };

    let subject = match x509::extract_subject_dn(&der) {
        Ok(s) => s,
        Err(_) => return,
    };
    let spki = match x509::extract_spki_from_cert(&der) {
        Ok(s) => s,
        Err(_) => return,
    };

    // Deduplicate.
    if roots.iter().any(|r| x509::dn_equal(r.subject, subject)) {
        return;
    }

    // Leak for 'static lifetime (system roots live for program duration).
    let subject_static: &'static [u8] = Box::leak(subject.to_vec().into_boxed_slice());
    let spki_static: &'static [u8] = Box::leak(spki.to_vec().into_boxed_slice());

    roots.push(RootCa {
        subject: subject_static,
        spki: spki_static,
    });
}
