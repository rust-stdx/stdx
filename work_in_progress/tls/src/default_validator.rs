#[cfg(feature = "webpki-validator")]
use alloc::{format, vec::Vec};

#[cfg(feature = "webpki-validator")]
use async_trait::async_trait;

#[cfg(feature = "webpki-validator")]
use crate::Error;
#[cfg(feature = "webpki-validator")]
use crate::config::{CertificateValidator, ReceivedCertificate};

/// A [`CertificateValidator`] that uses `rustls-webpki` for X.509 chain
/// validation against a set of root trust anchors.
///
/// Loads Mozilla's CA bundle via [`webpki_roots::TLS_SERVER_ROOTS`] by default
/// and validates the certificate chain, subject name (SAN), and validity
/// period.
///
/// # Raw public keys
///
/// This validator rejects [`ReceivedCertificate::RawPublicKey`] variants.
/// Raw public key trust decisions (e.g. key pinning) are application-specific
/// and must use a custom validator.
#[cfg(feature = "webpki-validator")]
#[derive(Clone, Debug)]
pub struct WebPkiValidator {
    roots: Vec<rustls_pki_types::TrustAnchor<'static>>,
}

#[cfg(feature = "webpki-validator")]
impl WebPkiValidator {
    /// Create a validator with Mozilla's default root CA certificate bundle.
    ///
    /// Uses the trust anchors from the [`webpki-roots`] crate.
    pub fn with_default_roots() -> Self {
        Self {
            roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
        }
    }

    /// Create a validator with custom root trust anchors.
    pub fn with_custom_roots(roots: Vec<rustls_pki_types::TrustAnchor<'static>>) -> Self {
        Self {
            roots,
        }
    }
}

#[cfg(feature = "webpki-validator")]
fn system_time_now() -> std::time::Duration {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
}

#[cfg(feature = "webpki-validator")]
#[async_trait]
impl CertificateValidator for WebPkiValidator {
    async fn validate(&self, cert: &ReceivedCertificate, server_name: Option<&str>) -> Result<(), Error> {
        match cert {
            ReceivedCertificate::X509 {
                chain, ..
            } => {
                if chain.is_empty() {
                    return Err(Error::CertificateValidationFailed("empty certificate chain".into()));
                }

                let server_name = server_name.ok_or_else(|| {
                    Error::CertificateValidationFailed("server name (SNI) required for X.509 validation".into())
                })?;

                let cert_der = rustls_pki_types::CertificateDer::from(&chain[0][..]);
                let ee = webpki::EndEntityCert::try_from(&cert_der)
                    .map_err(|e| Error::CertificateValidationFailed(format!("failed to parse end-entity cert: {e}")))?;

                let intermediates: Vec<rustls_pki_types::CertificateDer<'_>> = chain[1..]
                    .iter()
                    .map(|d| rustls_pki_types::CertificateDer::from(d.as_slice()))
                    .collect();

                let time = rustls_pki_types::UnixTime::since_unix_epoch(system_time_now());

                let server_name_dns = rustls_pki_types::ServerName::try_from(server_name)
                    .map_err(|e| Error::CertificateValidationFailed(format!("invalid server name: {e}")))?;

                ee.verify_is_valid_for_subject_name(&server_name_dns).map_err(|e| {
                    Error::CertificateValidationFailed(format!("subject name (SAN) validation failed: {e}"))
                })?;

                ee.verify_for_usage(
                    webpki::ALL_VERIFICATION_ALGS,
                    &self.roots,
                    &intermediates,
                    time,
                    webpki::KeyUsage::server_auth(),
                    None,
                    None,
                )
                .map_err(|e| Error::CertificateValidationFailed(format!("chain validation failed: {e}")))?;

                Ok(())
            }
            ReceivedCertificate::RawPublicKey {
                ..
            } => Err(Error::CertificateValidationFailed(
                "raw public key validation requires a custom CertificateValidator".into(),
            )),
        }
    }
}
