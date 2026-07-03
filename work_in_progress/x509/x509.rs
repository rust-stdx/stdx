#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidDer,
    InvalidCertificate,
    InvalidSpki,
    InvalidPublicKey,
    Truncated,
    InvalidOid,
    InvalidExtension,
    InvalidValidity,
    InvalidTime,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDer => f.write_str("invalid DER encoding"),
            Self::InvalidCertificate => f.write_str("invalid X.509 certificate structure"),
            Self::InvalidSpki => f.write_str("invalid SubjectPublicKeyInfo"),
            Self::InvalidPublicKey => f.write_str("invalid public key BIT STRING"),
            Self::Truncated => f.write_str("truncated DER input"),
            Self::InvalidOid => f.write_str("invalid OID encoding"),
            Self::InvalidExtension => f.write_str("invalid extension"),
            Self::InvalidValidity => f.write_str("invalid validity period"),
            Self::InvalidTime => f.write_str("invalid time encoding"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[derive(Debug)]
struct Tlv<'a> {
    tag: u8,
    raw: &'a [u8],
    value: &'a [u8],
}

fn read_tlv(data: &[u8]) -> Result<Tlv<'_>, Error> {
    if data.is_empty() {
        return Err(Error::Truncated);
    }
    let tag = data[0];
    let consumed = 1;

    if data.len() <= consumed {
        return Err(Error::Truncated);
    }
    let len_byte = data[consumed];

    let (len, len_size) = if len_byte & 0x80 == 0 {
        (len_byte as usize, 1)
    } else {
        let num_bytes = (len_byte & 0x7f) as usize;
        if num_bytes == 0 || num_bytes > core::mem::size_of::<usize>() {
            return Err(Error::InvalidDer);
        }
        if data.len() <= consumed + num_bytes {
            return Err(Error::Truncated);
        }
        if num_bytes > 1 && data[consumed + 1] == 0 {
            return Err(Error::InvalidDer);
        }
        let mut l = 0usize;
        for i in 0..num_bytes {
            l = (l << 8) | data[consumed + 1 + i] as usize;
        }
        if l < 128 {
            return Err(Error::InvalidDer);
        }
        (l, 1 + num_bytes)
    };

    let start = consumed + len_size;
    let end = start.checked_add(len).ok_or(Error::InvalidDer)?;
    if end > data.len() {
        return Err(Error::Truncated);
    }

    Ok(Tlv {
        tag,
        raw: &data[..end],
        value: &data[start..end],
    })
}

fn skip_tlv(data: &[u8], offset: &mut usize) -> Result<(), Error> {
    let tlv = read_tlv(&data[*offset..])?;
    *offset += tlv.raw.len();
    Ok(())
}

fn skip_tag(data: &[u8], offset: &mut usize, expected_tag: u8) -> Result<(), Error> {
    if *offset >= data.len() || data[*offset] != expected_tag {
        return Err(Error::InvalidCertificate);
    }
    skip_tlv(data, offset)
}

/// Extract raw public key bytes from a SubjectPublicKeyInfo DER blob.
///
/// The SPKI DER contains a BIT STRING that holds the raw key material
/// (uncompressed point for P-256, 32-byte key for Ed25519, etc.).
/// This walks the DER structure to locate the BIT STRING and returns
/// its content (minus the leading unused-bits byte).
pub fn extract_key_from_spki(spki_der: &[u8]) -> Result<&[u8], Error> {
    let spki = read_tlv(spki_der)?;
    if spki.tag != 0x30 {
        return Err(Error::InvalidSpki);
    }
    // Two formats:
    // Full:  30 [len] 30 [algid_len] ... 03 [key_len] ...
    // Stripped (webpki-roots): 30 [algid_len] ... 03 [key_len] ...
    if spki.value.first() == Some(&0x30) {
        // Full format: skip the outer AlgorithmIdentifier SEQUENCE.
        let mut offset = 0;
        skip_tlv(spki.value, &mut offset)?;
        let inner = &spki.value[offset..];
        let bs = read_tlv(inner)?;
        if bs.tag != 0x03 {
            return Err(Error::InvalidSpki);
        }
        if bs.value.is_empty() {
            return Err(Error::InvalidPublicKey);
        }
        Ok(&bs.value[1..])
    } else {
        // Stripped format: spki IS the AlgorithmIdentifier; BIT STRING follows.
        let after = &spki_der[spki.raw.len()..];
        if after.is_empty() {
            return Err(Error::InvalidPublicKey);
        }
        let bs = read_tlv(after)?;
        if bs.tag != 0x03 {
            return Err(Error::InvalidSpki);
        }
        if bs.value.is_empty() {
            return Err(Error::InvalidPublicKey);
        }
        Ok(&bs.value[1..])
    }
}

/// Extract the SubjectPublicKeyInfo DER from an X.509 certificate.
///
/// Returns the raw DER bytes of the `subjectPublicKeyInfo` field inside the
/// `tbsCertificate` SEQUENCE.
pub fn extract_spki_from_cert<'a>(cert_der: &'a [u8]) -> Result<&'a [u8], Error> {
    let fields = walk_tbs(cert_der)?;
    Ok(fields.spki)
}

/// Extract raw public key bytes from an X.509 certificate (DER-encoded).
///
/// Returns the key material (e.g. uncompressed P-256 point, Ed25519 key
/// bytes) stripped of the unused-bits byte from the BIT STRING encoding.
#[cfg(feature = "alloc")]
pub fn extract_public_key_from_cert(cert_der: &[u8]) -> Result<Vec<u8>, Error> {
    let spki = extract_spki_from_cert(cert_der)?;
    let key = extract_key_from_spki(spki)?;
    Ok(key.to_vec())
}

// ── OID constants ──────────────────────────────────────────────────────────

/// 2.5.29.17 – Subject Alternative Name
pub const OID_SAN: &[u8] = &[0x55, 0x1d, 0x11];
/// 2.5.29.15 – Key Usage
pub const OID_KEY_USAGE: &[u8] = &[0x55, 0x1d, 0x0f];
/// 2.5.29.37 – Extended Key Usage
pub const OID_EKU: &[u8] = &[0x55, 0x1d, 0x25];
/// 2.5.29.19 – Basic Constraints
pub const OID_BASIC_CONSTRAINTS: &[u8] = &[0x55, 0x1d, 0x13];
/// 1.3.6.1.5.5.7.3.1 – serverAuth EKU
pub const OID_EKU_SERVER_AUTH: &[u8] = &[0x2b, 0x06, 0x01, 0x05, 0x05, 0x07, 0x03, 0x01];

// Signature algorithm OIDs
/// 1.3.101.112 – id-Ed25519
pub const OID_ED25519: &[u8] = &[0x2b, 0x65, 0x70];
/// 1.2.840.10045.4.3.2 – ecdsa-with-SHA256
pub const OID_ECDSA_SHA256: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x03, 0x02];
/// 1.2.840.10045.4.3.3 – ecdsa-with-SHA384
pub const OID_ECDSA_SHA384: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x03, 0x03];
// EC public key OID (not signature, but used in SPKI)
pub const OID_EC_PUBLIC_KEY: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01];
/// 1.2.840.113549.1.1.11 – sha256WithRSAEncryption
pub const OID_RSA_SHA256: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0b];
/// 1.2.840.113549.1.1.12 – sha384WithRSAEncryption
pub const OID_RSA_SHA384: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0c];
/// 1.2.840.113549.1.1.13 – sha512WithRSAEncryption
pub const OID_RSA_SHA512: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0d];
/// 1.2.840.113549.1.1.10 – id-RSASSA-PSS
pub const OID_RSA_PSS: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0a];
/// 1.2.840.10045.2.1 – ecPublicKey (algorithm OID in SPKI)
pub const OID_EC_PUBLIC_KEY_ALG: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01];

// ── TBS navigation ─────────────────────────────────────────────────────────

/// Fields of the TBSCertificate SEQUENCE returned by `walk_tbs`.
struct TbsFields<'a> {
    /// Raw bytes of the issuer Name (SEQUENCE OF SET OF AttributeTypeAndValue).
    issuer_dn: &'a [u8],
    /// Raw bytes of the subject Name.
    subject_dn: &'a [u8],
    /// Raw bytes of the subjectPublicKeyInfo.
    spki: &'a [u8],
}

/// Walk the top-level fields of an X.509 TBSCertificate and return field
/// references.
fn walk_tbs(cert_der: &[u8]) -> Result<TbsFields<'_>, Error> {
    let cert = read_tlv(cert_der)?;
    if cert.tag != 0x30 {
        return Err(Error::InvalidCertificate);
    }
    let tbs = read_tlv(cert.value)?;
    if tbs.tag != 0x30 {
        return Err(Error::InvalidCertificate);
    }
    let inner = tbs.value;
    let mut offset = 0;

    if offset >= inner.len() {
        return Err(Error::InvalidCertificate);
    }

    // [0] version (optional, context-specific constructed)
    if inner[offset] == 0xa0 {
        skip_tlv(inner, &mut offset)?;
    }

    // serialNumber (INTEGER)
    skip_tag(inner, &mut offset, 0x02)?;
    // signature (AlgorithmIdentifier SEQUENCE)
    skip_tag(inner, &mut offset, 0x30)?;

    // issuer (Name SEQUENCE)
    if offset >= inner.len() || inner[offset] != 0x30 {
        return Err(Error::InvalidCertificate);
    }
    let issuer_tlv = read_tlv(&inner[offset..])?;
    let issuer_dn = &inner[offset..offset + issuer_tlv.raw.len()];

    // Extract issuer raw bytes (including tag+length)
    offset += issuer_tlv.raw.len();

    // validity (SEQUENCE)
    skip_tag(inner, &mut offset, 0x30)?;

    // subject (Name SEQUENCE)
    if offset >= inner.len() || inner[offset] != 0x30 {
        return Err(Error::InvalidCertificate);
    }
    let subject_tlv = read_tlv(&inner[offset..])?;
    let subject_dn = &inner[offset..offset + subject_tlv.raw.len()];
    offset += subject_tlv.raw.len();

    // Skip optional context-specific fields: [1] issuerUniqueID,
    // [2] subjectUniqueID, [3] extensions
    while offset < inner.len() && (inner[offset] & 0xa0) == 0xa0 {
        skip_tlv(inner, &mut offset)?;
    }

    // subjectPublicKeyInfo (SEQUENCE)
    if offset >= inner.len() || inner[offset] != 0x30 {
        return Err(Error::InvalidCertificate);
    }
    let spki_tlv = read_tlv(&inner[offset..])?;
    let spki = &inner[offset..offset + spki_tlv.raw.len()];

    Ok(TbsFields {
        issuer_dn,
        subject_dn,
        spki,
    })
}

/// Find the value of an extension by OID.
///
/// Returns the raw bytes of the extension's `extnValue` (the content inside
/// the OCTET STRING), or `None` if the extension is not present.
///
/// The certificate must contain the `[3] extensions` field.
pub fn find_extension<'a>(cert_der: &'a [u8], oid: &[u8]) -> Option<&'a [u8]> {
    let (tbs_inner, extensions_start) = find_extensions_section(cert_der)?;
    // extensions is EXPLICIT [3], so the wrapper's value is the Extensions SEQUENCE.
    let wrapper = read_tlv(&tbs_inner[extensions_start..]).ok()?;
    let ext_seq = read_tlv(wrapper.value).ok()?;
    find_oid_in_extensions(ext_seq.value, oid)
}

/// Locate the `[3] extensions` field in the TBSCertificate and return
/// `(tbs_inner, offset_to_extensions_a3)`.
fn find_extensions_section(cert_der: &[u8]) -> Option<(&[u8], usize)> {
    let cert = read_tlv(cert_der).ok()?;
    if cert.tag != 0x30 {
        return None;
    }
    let tbs = read_tlv(cert.value).ok()?;
    if tbs.tag != 0x30 {
        return None;
    }
    let inner = tbs.value;
    let mut offset = 0;

    if offset < inner.len() && inner[offset] == 0xa0 {
        let _ = skip_tlv(inner, &mut offset);
    }
    let _ = skip_tag(inner, &mut offset, 0x02);
    let _ = skip_tag(inner, &mut offset, 0x30);
    let _ = skip_tag(inner, &mut offset, 0x30);
    let _ = skip_tag(inner, &mut offset, 0x30);
    let _ = skip_tag(inner, &mut offset, 0x30);

    // Skip optional [1] issuerUniqueID, [2] subjectUniqueID (IMPLICIT).
    while offset < inner.len()
        && (inner[offset] == 0x81 || inner[offset] == 0x82 || inner[offset] == 0xa1 || inner[offset] == 0xa2)
    {
        let _ = skip_tlv(inner, &mut offset);
    }

    // subjectPublicKeyInfo (SEQUENCE)
    let _ = skip_tag(inner, &mut offset, 0x30);

    // [3] extensions
    if offset < inner.len() && inner[offset] == 0xa3 {
        Some((inner, offset))
    } else {
        None
    }
}

fn find_oid_in_extensions<'a>(extensions: &'a [u8], oid: &[u8]) -> Option<&'a [u8]> {
    let mut offset = 0;
    while offset < extensions.len() {
        let ext = read_tlv(&extensions[offset..]).ok()?;
        offset += ext.raw.len();
        if ext.tag != 0x30 {
            continue;
        }
        let mut off = 0;
        // extnID (OID)
        let oid_tlv = read_tlv(&ext.value[off..]).ok()?;
        if oid_tlv.tag != 0x06 {
            continue;
        }
        off += oid_tlv.raw.len();

        // critical (BOOLEAN, optional, tag 0x01)
        if off < ext.value.len() && ext.value[off] == 0x01 {
            let bool_tlv = read_tlv(&ext.value[off..]).ok()?;
            off += bool_tlv.raw.len();
        }

        // extnValue (OCTET STRING)
        if off >= ext.value.len() || ext.value[off] != 0x04 {
            continue;
        }
        let val_tlv = read_tlv(&ext.value[off..]).ok()?;

        if oid_tlv.value == oid {
            return Some(val_tlv.value);
        }
    }
    None
}

// ── DN extraction ──────────────────────────────────────────────────────────

/// Extract the raw DER-encoded issuer Distinguished Name from an X.509
/// certificate.
///
/// Returns the content of the issuer Name SEQUENCE (the RDNs), without the
/// outer SEQUENCE tag and length bytes.
pub fn extract_issuer_dn(cert_der: &[u8]) -> Result<&[u8], Error> {
    let fields = walk_tbs(cert_der)?;
    // fields.issuer_dn includes the outer SEQUENCE TLV; return just the value.
    let issuer_tlv = read_tlv(fields.issuer_dn)?;
    Ok(issuer_tlv.value)
}

/// Extract the raw DER-encoded subject Distinguished Name from an X.509
/// certificate.
///
/// Returns the content of the subject Name SEQUENCE (the RDNs), without the
/// outer SEQUENCE tag and length bytes.
pub fn extract_subject_dn(cert_der: &[u8]) -> Result<&[u8], Error> {
    let fields = walk_tbs(cert_der)?;
    let subject_tlv = read_tlv(fields.subject_dn)?;
    Ok(subject_tlv.value)
}

/// Compare two Distinguished Name byte slices for equality, normalising the
/// SET/SET-OF structure.
///
/// X.509 DNs can encode the same logical set of attributes with different SET
/// granularity (e.g. a single SET with two attributes vs two SETs with one
/// attribute each).  This function flattens both DNs into a list of `(OID,
/// value)` pairs, sorts them, and compares the sorted lists.
///
/// Returns `true` if the DNs are semantically equal.
pub fn dn_equal(a: &[u8], b: &[u8]) -> bool {
    let pairs_a = flatten_dn(a);
    let pairs_b = flatten_dn(b);
    if pairs_a.len() != pairs_b.len() {
        return false;
    }
    for (pa, pb) in pairs_a.iter().zip(pairs_b.iter()) {
        if pa.0 != pb.0 || pa.1 != pb.1 {
            return false;
        }
    }
    true
}

/// Debug helper: format a DN as a string of "OID=value" pairs.
#[cfg(feature = "alloc")]
pub fn debug_dn_pairs(dn: &[u8]) -> alloc::string::String {
    let pairs = flatten_dn(dn);
    let parts: alloc::vec::Vec<_> = pairs
        .into_iter()
        .map(|(oid, val)| {
            let oid_name = oid_to_name(oid);
            let val_str = if val.iter().all(|b| b.is_ascii_graphic() || *b == b' ') {
                alloc::string::String::from_utf8_lossy(val).into_owned()
            } else {
                alloc::format!("{:02x?}", val)
            };
            alloc::format!("{oid_name}={val_str}")
        })
        .collect();
    parts.join(", ")
}

fn oid_to_name(oid: &[u8]) -> &'static str {
    match oid {
        &[0x55, 0x04, 0x03] => "CN",
        &[0x55, 0x04, 0x06] => "C",
        &[0x55, 0x04, 0x07] => "L",
        &[0x55, 0x04, 0x08] => "ST",
        &[0x55, 0x04, 0x0a] => "O",
        &[0x55, 0x04, 0x0b] => "OU",
        _ => "(unknown OID)",
    }
}

/// Flatten a DN content (SETs of SEQUENCEs of AttributeTypeAndValue) into
/// sorted (OID, value) pairs.
///
/// Each element is `(OID bytes, attribute value bytes)`.  The result is
/// sorted to enable order-independent comparison.
fn flatten_dn(dn: &[u8]) -> alloc::vec::Vec<(&[u8], &[u8])> {
    let mut pairs = alloc::vec::Vec::new();
    let mut offset = 0;
    while offset < dn.len() && dn[offset] == 0x31 {
        // Each RDN is a SET (0x31)
        let rdn = match read_tlv(&dn[offset..]) {
            Ok(t) => t,
            Err(_) => break,
        };
        offset += rdn.raw.len();

        // Within each RDN, extract SEQUENCEs (AttributeTypeAndValue)
        let mut rdn_off = 0;
        while rdn_off < rdn.value.len() && rdn.value[rdn_off] == 0x30 {
            let attr = match read_tlv(&rdn.value[rdn_off..]) {
                Ok(t) => t,
                Err(_) => break,
            };
            rdn_off += attr.raw.len();

            // AttributeTypeAndValue ::= SEQUENCE { type OID, value ANY }
            if let Some((oid, val)) = parse_attribute(attr.value) {
                pairs.push((oid, val));
            }
        }
    }
    // Sort by OID then by value for stable comparison
    pairs.sort_by(|a, b| a.0.cmp(b.0).then(a.1.cmp(b.1)));
    pairs
}

/// Parse a single AttributeTypeAndValue SEQUENCE into (OID, value).
fn parse_attribute(attr_value: &[u8]) -> Option<(&[u8], &[u8])> {
    let mut off = 0;
    let oid_tlv = read_tlv(&attr_value[off..]).ok()?;
    if oid_tlv.tag != 0x06 {
        return None;
    }
    off += oid_tlv.raw.len();
    if off >= attr_value.len() {
        return None;
    }
    // The value is whatever TLV follows the OID (can be PrintableString,
    // UTF8String, IA5String, TeletexString, BMPString, etc.)
    let val_tlv = read_tlv(&attr_value[off..]).ok()?;
    Some((oid_tlv.value, val_tlv.value))
}

// ── Signature algorithm ────────────────────────────────────────────────────

/// Extract the signature algorithm OID from an X.509 certificate.
///
/// Returns the raw OID bytes from the `signatureAlgorithm` field of the
/// certificate (NOT the TBSCertificate's inner signature field — those are
/// the same per RFC 5280 but the outer one is what covers the signed data).
pub fn extract_signature_algorithm_oid(cert_der: &[u8]) -> Result<&[u8], Error> {
    let cert = read_tlv(cert_der)?;
    if cert.tag != 0x30 {
        return Err(Error::InvalidCertificate);
    }
    let tbs = read_tlv(cert.value)?;
    if tbs.tag != 0x30 {
        return Err(Error::InvalidCertificate);
    }
    // After TBSCertificate comes signatureAlgorithm (SEQUENCE)
    let sig_alg_offset = tbs.raw.len();
    if sig_alg_offset >= cert.value.len() {
        return Err(Error::InvalidCertificate);
    }
    let sig_alg = read_tlv(&cert.value[sig_alg_offset..])?;
    if sig_alg.tag != 0x30 {
        return Err(Error::InvalidCertificate);
    }
    // First element of AlgorithmIdentifier is the OID
    let oid = read_tlv(sig_alg.value)?;
    if oid.tag != 0x06 {
        return Err(Error::InvalidCertificate);
    }
    Ok(oid.value)
}

/// Extract the raw TBSCertificate bytes (tag + length + value) from a
/// DER-encoded X.509 certificate.
///
/// This is the portion of the certificate that is covered by the signature.
pub fn extract_tbs_cert(cert_der: &[u8]) -> Result<&[u8], Error> {
    let cert = read_tlv(cert_der)?;
    if cert.tag != 0x30 {
        return Err(Error::InvalidCertificate);
    }
    let tbs = read_tlv(cert.value)?;
    if tbs.tag != 0x30 {
        return Err(Error::InvalidCertificate);
    }
    Ok(tbs.raw)
}

/// Extract the signature value from a DER-encoded X.509 certificate.
///
/// Returns the raw signature bytes (the content of the BIT STRING, with the
/// leading unused-bits byte stripped).
pub fn extract_signature_value(cert_der: &[u8]) -> Result<&[u8], Error> {
    let cert = read_tlv(cert_der)?;
    if cert.tag != 0x30 {
        return Err(Error::InvalidCertificate);
    }
    let tbs = read_tlv(cert.value)?;
    if tbs.tag != 0x30 {
        return Err(Error::InvalidCertificate);
    }
    let after_tbs = &cert.value[tbs.raw.len()..];
    let sig_alg = read_tlv(after_tbs)?;
    if sig_alg.tag != 0x30 {
        return Err(Error::InvalidCertificate);
    }
    let after_sig_alg = &after_tbs[sig_alg.raw.len()..];
    let sig = read_tlv(after_sig_alg)?;
    if sig.tag != 0x03 {
        return Err(Error::InvalidCertificate);
    }
    if sig.value.is_empty() {
        return Err(Error::InvalidCertificate);
    }
    Ok(&sig.value[1..])
}

// ── SAN (Subject Alternative Name) ─────────────────────────────────────────

/// Parse DNS names from the Subject Alternative Name extension.
///
/// Returns the raw bytes of each dNSName entry.
#[cfg(feature = "alloc")]
pub fn parse_san_dns_names(cert_der: &[u8]) -> Result<Vec<Vec<u8>>, Error> {
    let san_value = find_extension(cert_der, OID_SAN).ok_or(Error::InvalidExtension)?;
    let general_names = read_tlv(san_value)?;
    if general_names.tag != 0x30 {
        return Err(Error::InvalidExtension);
    }
    let mut dns_names = Vec::new();
    let mut offset = 0;
    while offset < general_names.value.len() {
        if general_names.value[offset] == 0x82 {
            // [2] dNSName (context-specific, implicit, IA5String)
            let dns = read_tlv(&general_names.value[offset..])?;
            dns_names.push(dns.value.to_vec());
            offset += dns.raw.len();
        } else {
            skip_tlv(general_names.value, &mut offset)?;
        }
    }
    Ok(dns_names)
}

/// Check whether any SAN dNSName matches the given server name, without
/// allocating.
///
/// Supports wildcard DNS names (`*.example.com`). Returns `Ok(true)` on
/// match, `Ok(false)` on no match (or missing SAN extension), `Err` on
/// parse failure.
pub fn check_san_dns_name(cert_der: &[u8], server_name: &str) -> Result<bool, Error> {
    let san_value = match find_extension(cert_der, OID_SAN) {
        Some(v) => v,
        None => return Ok(false),
    };
    let general_names = read_tlv(san_value)?;
    if general_names.tag != 0x30 {
        return Err(Error::InvalidExtension);
    }
    let mut offset = 0;
    while offset < general_names.value.len() {
        if general_names.value[offset] == 0x82 {
            let dns = read_tlv(&general_names.value[offset..])?;
            if dns_name_matches(dns.value, server_name) {
                return Ok(true);
            }
            offset += dns.raw.len();
        } else {
            skip_tlv(general_names.value, &mut offset)?;
        }
    }
    Ok(false)
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
            && !server_name[..dot_pos].is_empty()
            && server_name[dot_pos + 1..].contains('.') == rest.contains('.')
    } else {
        dns_name == server_name
    }
}

// ── Key Usage ──────────────────────────────────────────────────────────────

/// Key usage bit positions
pub mod key_usage {
    pub const DIGITAL_SIGNATURE: u8 = 0;
    pub const KEY_ENCIPHERMENT: u8 = 2;
    pub const KEY_CERT_SIGN: u8 = 5;
}

/// Parse the Key Usage extension and return the raw bit mask as a `u16`.
///
/// Returns `None` if the extension is not present.
pub fn parse_key_usage(cert_der: &[u8]) -> Option<u16> {
    let ku_value = find_extension(cert_der, OID_KEY_USAGE)?;
    let bs = read_tlv(ku_value).ok()?;
    if bs.tag != 0x03 {
        return None;
    }
    let unused = bs.value.first().copied().unwrap_or(0);
    let bits = &bs.value[1..];
    let mut mask: u16 = 0;
    for (i, &byte) in bits.iter().enumerate() {
        mask |= (byte as u16) << (8 * i);
    }
    // Clear the unused bits at the most significant end
    if unused > 0 {
        mask &= !(0xffff << (16 - unused as usize));
    }
    Some(mask)
}

// ── Extended Key Usage ─────────────────────────────────────────────────────

/// Check whether the Extended Key Usage extension includes `serverAuth`
/// (1.3.6.1.5.5.7.3.1).
///
/// Returns `None` if the EKU extension is absent, `Some(true)` if serverAuth
/// is present, `Some(false)` if it is not.
pub fn has_eku_server_auth(cert_der: &[u8]) -> Option<bool> {
    let eku_value = find_extension(cert_der, OID_EKU)?;
    let seq = read_tlv(eku_value).ok()?;
    if seq.tag != 0x30 {
        return Some(false);
    }
    let mut offset = 0;
    while offset < seq.value.len() {
        let oid = read_tlv(&seq.value[offset..]).ok()?;
        if oid.tag == 0x06 && oid.value == OID_EKU_SERVER_AUTH {
            return Some(true);
        }
        offset += oid.raw.len();
    }
    Some(false)
}

// ── Basic Constraints ──────────────────────────────────────────────────────

/// Check whether the certificate is a CA via the Basic Constraints extension.
///
/// Returns `None` if the extension is absent (cert is NOT a CA per RFC 5280),
/// `Some(true)` if `cA` is TRUE, `Some(false)` if `cA` is FALSE.
pub fn is_ca(cert_der: &[u8]) -> Option<bool> {
    let bc_value = find_extension(cert_der, OID_BASIC_CONSTRAINTS)?;
    let seq = read_tlv(bc_value).ok()?;
    if seq.tag != 0x30 {
        return Some(false);
    }
    // cA BOOLEAN (tag 0x01, value 0xff for TRUE, 0x00 for FALSE)
    if seq.value.first() == Some(&0x01) {
        let bool_tlv = read_tlv(seq.value).ok()?;
        return Some(bool_tlv.value == [0xff]);
    }
    Some(false)
}

// ── Validity / Time ────────────────────────────────────────────────────────

/// Parsed X.509 time (UTCTime or GeneralizedTime).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X509Time {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl X509Time {
    /// Convert to Unix timestamp (seconds since 1970-01-01 00:00:00 UTC).
    pub fn to_unix_seconds(self) -> u64 {
        let days = days_from_civil(self.year as i32, self.month, self.day);
        let unix_epoch_days = days_from_civil(1970, 1, 1);
        let day_offset = (days - unix_epoch_days) as u64;
        day_offset * 86400 + self.hour as u64 * 3600 + self.minute as u64 * 60 + self.second as u64
    }
}

/// Parse the validity period from an X.509 certificate.
///
/// Returns `(not_before, not_after)` as `X509Time` values.
pub fn parse_validity(cert_der: &[u8]) -> Result<(X509Time, X509Time), Error> {
    let cert = read_tlv(cert_der)?;
    if cert.tag != 0x30 {
        return Err(Error::InvalidCertificate);
    }
    let tbs = read_tlv(cert.value)?;
    if tbs.tag != 0x30 {
        return Err(Error::InvalidCertificate);
    }
    let inner = tbs.value;
    let mut offset = 0;

    // version (optional)
    if offset < inner.len() && inner[offset] == 0xa0 {
        skip_tlv(inner, &mut offset)?;
    }
    skip_tag(inner, &mut offset, 0x02)?; // serialNumber
    skip_tag(inner, &mut offset, 0x30)?; // signature
    skip_tag(inner, &mut offset, 0x30)?; // issuer

    // validity (SEQUENCE)
    if offset >= inner.len() || inner[offset] != 0x30 {
        return Err(Error::InvalidValidity);
    }
    let validity = read_tlv(&inner[offset..])?;

    let mut voff = 0;
    let not_before = parse_x509_time_element(&validity.value, &mut voff)?;
    let not_after = parse_x509_time_element(&validity.value, &mut voff)?;

    Ok((not_before, not_after))
}

fn parse_x509_time_element(data: &[u8], offset: &mut usize) -> Result<X509Time, Error> {
    // UTCTime (tag 0x17) or GeneralizedTime (tag 0x18)
    if *offset >= data.len() {
        return Err(Error::InvalidTime);
    }
    let tag = data[*offset];
    if tag != 0x17 && tag != 0x18 {
        return Err(Error::InvalidTime);
    }
    let tlv = read_tlv(&data[*offset..])?;
    *offset += tlv.raw.len();

    let bytes = tlv.value;

    if tag == 0x17 {
        // UTCTime: YYMMDDHHMMSSZ (13 bytes) or YYMMDDHHMMZ (11 bytes)
        if bytes.len() < 11 || bytes.last() != Some(&b'Z') {
            return Err(Error::InvalidTime);
        }
        let yy = parse_two_digits(bytes, 0)?;
        let year = if yy >= 50 { 1900 + yy as u16 } else { 2000 + yy as u16 };
        Ok(X509Time {
            year,
            month: parse_two_digits(bytes, 2)?,
            day: parse_two_digits(bytes, 4)?,
            hour: parse_two_digits(bytes, 6)?,
            minute: parse_two_digits(bytes, 8)?,
            second: if bytes.len() >= 13 {
                parse_two_digits(bytes, 10)?
            } else {
                0
            },
        })
    } else {
        // GeneralizedTime: YYYYMMDDHHMMSSZ (15 bytes)
        if bytes.len() < 15 || bytes.last() != Some(&b'Z') {
            return Err(Error::InvalidTime);
        }
        Ok(X509Time {
            year: parse_four_digits(bytes, 0)?,
            month: parse_two_digits(bytes, 4)?,
            day: parse_two_digits(bytes, 6)?,
            hour: parse_two_digits(bytes, 8)?,
            minute: parse_two_digits(bytes, 10)?,
            second: parse_two_digits(bytes, 12)?,
        })
    }
}

fn parse_two_digits(bytes: &[u8], pos: usize) -> Result<u8, Error> {
    if pos + 1 >= bytes.len() {
        return Err(Error::InvalidTime);
    }
    let hi = digit(bytes[pos])?;
    let lo = digit(bytes[pos + 1])?;
    Ok(hi * 10 + lo)
}

fn parse_four_digits(bytes: &[u8], pos: usize) -> Result<u16, Error> {
    let hi = parse_two_digits(bytes, pos)? as u16;
    let lo = parse_two_digits(bytes, pos + 2)? as u16;
    Ok(hi * 100 + lo)
}

fn digit(b: u8) -> Result<u8, Error> {
    if b.is_ascii_digit() {
        Ok(b - b'0')
    } else {
        Err(Error::InvalidTime)
    }
}

/// Number of days since 0000-03-01 (proleptic Gregorian).
///
/// Based on Howard Hinnant's algorithm.
fn days_from_civil(y: i32, m: u8, d: u8) -> i32 {
    let y = y as i32 - i32::from(m <= 2);
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u32;
    let doy = (153 * (if m as u32 > 2 { m as u32 - 3 } else { m as u32 + 9 }) + 2) / 5 + d as u32 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era as i32 * 146097 + doe as i32
}

// ── SPKI algorithm ─────────────────────────────────────────────────────────

/// Extract the algorithm OID from a SubjectPublicKeyInfo DER blob.
///
/// This is the OID inside the AlgorithmIdentifier, e.g. `1.2.840.10045.2.1`
/// for EC public keys or `1.3.101.112` for Ed25519.
pub fn extract_spki_algorithm_oid(spki_der: &[u8]) -> Result<&[u8], Error> {
    let top = read_tlv(spki_der)?;
    if top.tag != 0x30 {
        return Err(Error::InvalidSpki);
    }
    // If the content starts with 0x30, we have the full SPKI SEQUENCE
    // wrapping AlgorithmIdentifier + BIT STRING. Unwrap one level.
    // If it starts with 0x06, we have the AlgorithmIdentifier directly
    // (e.g. from webpki-roots TrustAnchor format).
    let alg_id = if top.value.first() == Some(&0x30) {
        read_tlv(top.value)?
    } else {
        top
    };
    if alg_id.tag != 0x30 {
        return Err(Error::InvalidSpki);
    }
    let oid = read_tlv(alg_id.value)?;
    if oid.tag != 0x06 {
        return Err(Error::InvalidSpki);
    }
    Ok(oid.value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode_hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn parse_empty() {
        assert_eq!(read_tlv(b"").unwrap_err(), Error::Truncated);
    }

    #[test]
    fn parse_truncated_tag() {
        assert_eq!(read_tlv(b"\x30").unwrap_err(), Error::Truncated);
    }

    #[test]
    fn parse_truncated_value() {
        assert_eq!(read_tlv(b"\x30\x05").unwrap_err(), Error::Truncated);
    }

    #[test]
    fn parse_null() {
        let tlv = read_tlv(b"\x05\x00").unwrap();
        assert_eq!(tlv.tag, 0x05);
        assert!(tlv.value.is_empty());
        assert_eq!(tlv.raw, b"\x05\x00");
    }

    #[test]
    fn parse_integer() {
        let tlv = read_tlv(b"\x02\x03\x01\x00\x01").unwrap();
        assert_eq!(tlv.tag, 0x02);
        assert_eq!(tlv.value, b"\x01\x00\x01");
        assert_eq!(tlv.raw, b"\x02\x03\x01\x00\x01");
    }

    #[test]
    fn parse_long_form_length() {
        let payload = [0x42u8; 256];
        let mut raw = vec![0x04, 0x82, 0x01, 0x00];
        raw.extend_from_slice(&payload);
        let tlv = read_tlv(&raw).unwrap();
        assert_eq!(tlv.tag, 0x04);
        assert_eq!(tlv.value.len(), 256);
    }

    #[test]
    fn parse_sequence() {
        let tlv = read_tlv(b"\x30\x06\x02\x01\x01\x02\x01\x02").unwrap();
        assert_eq!(tlv.tag, 0x30);
        assert_eq!(tlv.value, b"\x02\x01\x01\x02\x01\x02");
    }

    #[test]
    fn spki_roundtrip() {
        let der = decode_hex(
            "3059301306072a8648ce3d020106082a8648ce3d030107034200\
             04deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef\
             deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef\
             deadbeefdeadbeefdeadbeefdeadbeef",
        );
        let key = extract_key_from_spki(&der).unwrap();
        assert_eq!(key.len(), 65);
        assert_eq!(key[0], 0x04);
    }

    #[test]
    fn extract_from_real_cert() {
        let der = include_bytes!("tests/p256_cert.der");
        let spki = extract_spki_from_cert(der).unwrap();
        let key = extract_key_from_spki(spki).unwrap();
        assert_eq!(key.len(), 65);
        assert_eq!(key[0], 0x04);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn extract_pk_alloc() {
        let der = include_bytes!("tests/p256_cert.der");
        let pk = extract_public_key_from_cert(der).unwrap();
        assert_eq!(pk.len(), 65);
        assert_eq!(pk[0], 0x04);
    }

    #[test]
    fn reject_invalid_spki() {
        assert_eq!(extract_key_from_spki(b"\x05\x00").unwrap_err(), Error::InvalidSpki);
    }

    #[test]
    fn reject_truncated_cert() {
        assert_eq!(extract_spki_from_cert(b"\x30\x03\x02\x01").unwrap_err(), Error::Truncated);
    }

    #[test]
    fn time_parsing_utc_time() {
        let utc = b"\x17\x0d230101120000Z";
        let mut offset = 0;
        let t = parse_x509_time_element(utc, &mut offset).unwrap();
        assert_eq!(t.year, 2023);
        assert_eq!(t.month, 1);
        assert_eq!(t.day, 1);
        assert_eq!(t.hour, 12);
        assert_eq!(t.minute, 0);
        assert_eq!(t.second, 0);
        // 2023-01-01 12:00:00 UTC = 1672574400
        assert_eq!(t.to_unix_seconds(), 1672574400);
    }

    #[test]
    fn time_parsing_generalized_time() {
        let gt = b"\x18\x0f20230101120000Z";
        let mut offset = 0;
        let t = parse_x509_time_element(gt, &mut offset).unwrap();
        assert_eq!(t.year, 2023);
        assert_eq!(t.month, 1);
        assert_eq!(t.day, 1);
        assert_eq!(t.hour, 12);
        assert_eq!(t.minute, 0);
        assert_eq!(t.second, 0);
        assert_eq!(t.to_unix_seconds(), 1672574400);
    }

    #[test]
    fn time_parsing_pre_2000_utc() {
        // UTCTime with YY >= 50 => 19YY
        let utc = b"\x17\x0d990101000000Z";
        let mut offset = 0;
        let t = parse_x509_time_element(utc, &mut offset).unwrap();
        assert_eq!(t.year, 1999);
        assert_eq!(t.month, 1);
        assert_eq!(t.day, 1);
    }

    #[test]
    fn validity_from_cert() {
        let der = include_bytes!("tests/p256_cert.der");
        let (nb, na) = parse_validity(der).unwrap();
        assert!(nb.year > 2000);
        assert!(na.year > nb.year);
    }

    #[test]
    fn extract_dns_names() {
        // A cert with SAN containing "example.com" and "www.example.com"
        // For simplicity, test the basic structure
        let der = include_bytes!("tests/p256_cert.der");
        // This test cert may or may not have SAN; just check it doesn't crash
        let _ = parse_san_dns_names(der);
    }
}
