use crate::{CipherSuite, CryptoProvider, Hash, errors::Error};

/// HKDF-Expand-Label as defined in RFC 8446 §7.1.
///
/// Derives keying material from a secret using a labeled context.
/// `out` must be sized to the desired output length.
pub fn hkdf_expand_label(
    provider: &impl CryptoProvider,
    suite: CipherSuite,
    secret: &Hash,
    label: &[u8],
    context: &[u8],
    out: &mut [u8],
) -> Result<(), Error> {
    provider.hkdf_expand_label(out, suite, secret, label, context)
}

/// Derive-Secret as defined in RFC 8446 §7.1.
///
/// Derive-Secret(Secret, Label, Messages) =
///     HKDF-Expand-Label(Secret, Label, Transcript-Hash(Messages), Hash.length)
pub fn derive_secret(
    provider: &impl CryptoProvider,
    suite: CipherSuite,
    secret: &Hash,
    label: &[u8],
    transcript_hash: &Hash,
) -> Result<Hash, Error> {
    let hash_size = suite.hash_size();
    let mut out = Hash::new_zeroed(hash_size as u8);
    hkdf_expand_label(provider, suite, secret, label, transcript_hash, &mut *out)?;
    Ok(out)
}

/// Derive handshake traffic keys from a traffic secret.
///
/// Derives key = HKDF-Expand-Label(secret, "key", "", Nkey)
/// Derives iv  = HKDF-Expand-Label(secret, "iv", "", 12)
/// Returns the AEAD key and IV directly.
pub fn derive_traffic_keys<C: CryptoProvider + ?Sized>(
    provider: &C,
    suite: CipherSuite,
    traffic_secret: &Hash,
) -> Result<(C::AeadKey, [u8; 12]), Error> {
    let key_size = suite.key_size();
    let mut key_out = [0u8; 32];
    let mut iv_out = [0u8; 12];
    hkdf_expand_label(provider, suite, traffic_secret, b"key", b"", &mut key_out[..key_size])?;
    hkdf_expand_label(provider, suite, traffic_secret, b"iv", b"", &mut iv_out)?;
    let key = provider.new_aead_key(suite, &key_out[..key_size]);
    Ok((key, iv_out))
}

/// Derive the Finished key from a traffic secret.
///
/// finished_key = HKDF-Expand-Label(traffic_secret, "finished", "", Hash.length)
pub fn derive_finished_key(
    provider: &impl CryptoProvider,
    suite: CipherSuite,
    traffic_secret: &Hash,
) -> Result<Hash, Error> {
    let hash_size = suite.hash_size();
    let mut out = Hash::new_zeroed(hash_size as u8);
    hkdf_expand_label(provider, suite, traffic_secret, b"finished", b"", &mut *out)?;
    Ok(out)
}

/// Compute the TLS 1.3 Finished verify_data.
///
/// verify_data = HMAC(finished_key, transcript_hash)
pub fn compute_finished(
    provider: &impl CryptoProvider,
    suite: CipherSuite,
    finished_key: &Hash,
    transcript_hash: &Hash,
) -> Result<Hash, Error> {
    provider.hmac(suite, finished_key, transcript_hash)
}

/// Derive the PSK for a NewSessionTicket.
///
/// PSK = HKDF-Expand-Label(resumption_secret, "resumption", ticket_nonce, Hash.length)
pub fn derive_ticket_psk(
    provider: &impl CryptoProvider,
    suite: CipherSuite,
    resumption_secret: &Hash,
    ticket_nonce: &[u8],
) -> Result<Hash, Error> {
    let hash_size = suite.hash_size();
    let mut out = Hash::new_zeroed(hash_size as u8);
    hkdf_expand_label(provider, suite, resumption_secret, b"resumption", ticket_nonce, &mut *out)?;
    Ok(out)
}

/// KeyUpdate: new_secret = HKDF-Expand-Label(old_traffic_secret, "traffic upd", "", Hash.length)
pub fn key_update_secret(provider: &impl CryptoProvider, suite: CipherSuite, old_secret: &Hash) -> Result<Hash, Error> {
    let hash_size = suite.hash_size();
    let mut out = Hash::new_zeroed(hash_size as u8);
    hkdf_expand_label(provider, suite, old_secret, b"traffic upd", b"", &mut *out)?;
    Ok(out)
}

/// Compute the transcript hash of an empty string: Hash("").
pub fn compute_empty_hash(provider: &impl CryptoProvider, suite: CipherSuite) -> Result<Hash, Error> {
    provider.hash(suite, &[])
}
