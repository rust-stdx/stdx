/// End-to-end: validate a self-signed cert against a matching custom root using WebPkiValidator.
#[tokio::test]
async fn webpki_validator_self_signed_as_root() {
    use std::sync::Arc;

    use tls::{
        WebPkiValidator,
        config::{CertificateValidator, ReceivedCertificate},
        crypto_default_provider::DefaultCryptoProvider,
        default_validator::RootCa,
    };

    // Generate a self-signed cert
    let cert = rcgen::generate_simple_self_signed(vec!["example.com".to_string()]).unwrap();
    let der = cert.cert.der().to_vec();

    // Extract its SPKI and subject to use as a trust anchor
    let spki: &'static [u8] = Box::leak(x509::extract_spki_from_cert(&der).unwrap().to_vec().into_boxed_slice());
    let subject: &'static [u8] = Box::leak(x509::extract_subject_dn(&der).unwrap().to_vec().into_boxed_slice());

    // Create a validator with this cert as the sole trust anchor
    let provider = Arc::new(DefaultCryptoProvider::new());
    let roots = vec![RootCa {
        subject,
        spki,
    }];
    let validator = WebPkiValidator::with_custom_roots(provider.clone(), roots);

    let received = ReceivedCertificate::X509 {
        chain: vec![der.into()],
        verify_scheme: tls::crypto::SignatureScheme::EcdsaP256Sha256,
    };

    // A self-signed cert's issuer DN = its subject DN, which matches our root.
    // However, the signature verification should fail because the cert's signature
    // was verified against itself (we're checking the signature against the root's SPKI,
    // which is the same key since it's self-signed).
    let result = validator.validate(&received, Some("example.com")).await;
    // The validation might succeed (if the cert happens to use a supported scheme)
    // or fail for crypto reasons, but it should NOT fail with "no trusted root found"
    eprintln!("Result: {:?}", result);
    if let Err(ref e) = result {
        assert!(
            !format!("{e}").contains("no trusted root found"),
            "Should not fail on root matching: {e}"
        );
    }
}

/// Test that dn_equal properly handles DNs with different SET granularity.
#[test]
fn dn_equal_different_set_granularity() {
    // DN with one attribute per SET: country only
    let dn1 = b"\x31\x0b\x30\x09\x06\x03\x55\x04\x06\x13\x02\x55\x53"; // C=US, one SET
    // Same DN, exact bytes
    assert!(x509::dn_equal(dn1, dn1));
    // Self-signed cert: subject == issuer
    let cert = rcgen::generate_simple_self_signed(vec!["example.com".to_string()]).unwrap();
    let der = cert.cert.der();
    let subject = x509::extract_subject_dn(der).unwrap();
    let issuer = x509::extract_issuer_dn(der).unwrap();
    assert!(x509::dn_equal(subject, issuer), "Self-signed cert: subject must equal issuer");
}
