use crate::{Error, MAX_CERTS, SignatureScheme};

#[async_trait::async_trait]
pub trait CertificateVerifier {
    async fn verfiy_certificate(&self, cert: &ReceivedCertificate, server_name: Option<&str>) -> Result<(), Error>;
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
        })
    }
}
