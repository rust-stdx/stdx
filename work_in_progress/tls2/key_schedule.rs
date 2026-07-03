use crate::{CipherSuite, CryptoProvider, errors::Error};

/// HKDF-Expand-Label as defined in RFC 8446 §7.1.
///
/// Derives keying material from a secret using a labeled context.
/// `out` must be sized to the desired output length.
pub fn hkdf_expand_label(
    provider: &impl CryptoProvider,
    suite: CipherSuite,
    secret: &[u8],
    label: &[u8],
    context: &[u8],
    out: &mut [u8],
) -> Result<(), Error> {
    provider.hkdf_expand_label(suite, secret, label, context, out)
}

/// Derive-Secret as defined in RFC 8446 §7.1.
///
/// Derive-Secret(Secret, Label, Messages) =
///     HKDF-Expand-Label(Secret, Label, Transcript-Hash(Messages), Hash.length)
pub fn derive_secret(
    provider: &impl CryptoProvider,
    suite: CipherSuite,
    secret: &[u8],
    label: &[u8],
    transcript_hash: &[u8],
    out: &mut [u8],
) -> Result<(), Error> {
    hkdf_expand_label(provider, suite, secret, label, transcript_hash, out)
}

/// Derive handshake traffic keys from a traffic secret.
///
/// Derives key = HKDF-Expand-Label(secret, "key", "", Nkey)
/// Derives iv  = HKDF-Expand-Label(secret, "iv", "", 12)
pub fn derive_traffic_keys(
    provider: &impl CryptoProvider,
    suite: CipherSuite,
    traffic_secret: &[u8],
    key_out: &mut [u8],
    iv_out: &mut [u8; 12],
) -> Result<(), Error> {
    let key_size = suite.key_size();

    hkdf_expand_label(provider, suite, traffic_secret, b"key", b"", &mut key_out[..key_size])?;
    hkdf_expand_label(provider, suite, traffic_secret, b"iv", b"", iv_out)
}

/// Derive the Finished key from a traffic secret.
///
/// finished_key = HKDF-Expand-Label(traffic_secret, "finished", "", Hash.length)
pub fn derive_finished_key(
    provider: &impl CryptoProvider,
    suite: CipherSuite,
    traffic_secret: &[u8],
    out: &mut [u8],
) -> Result<(), Error> {
    let hash_size = suite.hash_size();
    hkdf_expand_label(provider, suite, traffic_secret, b"finished", b"", &mut out[..hash_size])
}

/// Compute the TLS 1.3 Finished verify_data.
///
/// verify_data = HMAC(finished_key, transcript_hash)
pub fn compute_finished(
    provider: &impl CryptoProvider,
    suite: CipherSuite,
    finished_key: &[u8],
    transcript_hash: &[u8],
    out: &mut [u8],
) -> Result<(), Error> {
    let hash_size = suite.hash_size();
    provider.hmac(suite, finished_key, transcript_hash, &mut out[..hash_size])
}

/// Derive the PSK for a NewSessionTicket.
///
/// PSK = HKDF-Expand-Label(resumption_secret, "resumption", ticket_nonce, Hash.length)
pub fn derive_ticket_psk(
    provider: &impl CryptoProvider,
    suite: CipherSuite,
    resumption_secret: &[u8],
    ticket_nonce: &[u8],
    out: &mut [u8],
) -> Result<(), Error> {
    let hash_size = suite.hash_size();
    hkdf_expand_label(
        provider,
        suite,
        resumption_secret,
        b"resumption",
        ticket_nonce,
        &mut out[..hash_size],
    )
}

/// KeyUpdate: new_secret = HKDF-Expand-Label(old_traffic_secret, "traffic upd", "", Hash.length)
pub fn key_update_secret(
    provider: &impl CryptoProvider,
    suite: CipherSuite,
    old_secret: &[u8],
    new_out: &mut [u8],
) -> Result<(), Error> {
    let hash_size = suite.hash_size();
    hkdf_expand_label(provider, suite, old_secret, b"traffic upd", b"", &mut new_out[..hash_size])
}

/// Compute the transcript hash of an empty string: Hash("").
pub fn compute_empty_hash(provider: &impl CryptoProvider, suite: CipherSuite, out: &mut [u8]) -> Result<(), Error> {
    let hash_size = suite.hash_size();
    provider.hash(suite, &[], &mut out[..hash_size])
}
