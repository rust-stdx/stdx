//! COSE_Sign1 and COSE_Sign support.

use core::fmt;

use cbor::{Decoder, Encoder, Tag, Type, Write};

use crate::{
    Algorithm, CoseKeySet, Error, Header, Headers, KeyOp, KeyType, Result, VerifyOptions,
    codec::{Cursor, cbor_len_array, cbor_len_bytes, cbor_len_str, cbor_len_tag, decoder, take_raw},
};

/// A signer: a key that can produce a COSE signature.
///
/// Signers write into a caller-provided buffer, so signing works without an
/// allocator. With the `alloc` feature, the provided `sign` method allocates a
/// `Vec` of exactly [`Signer::signature_len`] bytes.
pub trait Signer {
    /// Returns the COSE algorithm implemented by this signer.
    fn algorithm(&self) -> Algorithm;

    /// Returns the exact signature size in bytes.
    fn signature_len(&self) -> usize;

    /// Signs `message`, writing the signature into `out`.
    ///
    /// Returns the number of bytes written, or [`Error::BufferTooSmall`].
    fn sign_into(&self, message: &[u8], out: &mut [u8]) -> Result<usize>;

    /// Signs `message` and returns the signature in a new `Vec`.
    #[cfg(feature = "alloc")]
    fn sign(&self, message: &[u8]) -> Result<alloc::vec::Vec<u8>> {
        let mut out = alloc::vec![0u8; self.signature_len()];
        let len = self.sign_into(message, &mut out)?;
        out.truncate(len);
        Ok(out)
    }
}

/// A verifier: a key that can verify a COSE signature.
pub trait Verifier {
    /// Returns the COSE algorithm implemented by this verifier.
    fn algorithm(&self) -> Algorithm;

    /// Verifies `signature` over `message`.
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<()>;
}

/// The protected-header bytes as they must appear in `Sig_structure`.
#[inline]
pub(crate) fn protected_for_structure(protected: &[u8]) -> &[u8] {
    if protected == [0xa0] { &[] } else { protected }
}

fn sig_structure_len(
    context: &str,
    body_protected: &[u8],
    sign_protected: Option<&[u8]>,
    external_aad: &[u8],
    payload: &[u8],
) -> usize {
    let field_count = if sign_protected.is_some() { 5 } else { 4 };
    let mut len = cbor_len_array(field_count)
        + cbor_len_str(context.len())
        + cbor_len_bytes(protected_for_structure(body_protected).len());
    if let Some(sign_protected) = sign_protected {
        len += cbor_len_bytes(protected_for_structure(sign_protected).len());
    }
    len + cbor_len_bytes(external_aad.len()) + cbor_len_bytes(payload.len())
}

fn encode_sig_structure_into(
    out: &mut [u8],
    context: &str,
    body_protected: &[u8],
    sign_protected: Option<&[u8]>,
    external_aad: &[u8],
    payload: &[u8],
) -> Result<usize> {
    let required = sig_structure_len(context, body_protected, sign_protected, external_aad, payload);
    if out.len() < required {
        return Err(Error::BufferTooSmall {
            required,
            available: out.len(),
        });
    }

    let mut cursor = Cursor::new(&mut out[..required]);
    {
        let mut e = Encoder::new(&mut cursor);
        e.array(if sign_protected.is_some() { 5 } else { 4 })?;
        e.text(context)?;
        e.bytes(protected_for_structure(body_protected))?;
        if let Some(sign_protected) = sign_protected {
            e.bytes(protected_for_structure(sign_protected))?;
        }
        e.bytes(external_aad)?;
        e.bytes(payload)?;
    }
    if cursor.position() != required {
        return Err(Error::Encode);
    }
    Ok(required)
}

fn sign1_message_len(
    tagged: bool,
    protected: &[u8],
    unprotected: &[u8],
    payload: Option<&[u8]>,
    signature_len: usize,
) -> usize {
    let mut len =
        cbor_len_array(4) + cbor_len_bytes(protected.len()) + unprotected.len() + cbor_len_bytes(signature_len);
    if tagged {
        len += cbor_len_tag(18);
    }
    len += match payload {
        Some(payload) => cbor_len_bytes(payload.len()),
        None => 1,
    };
    len
}

fn encode_sign1_into(
    out: &mut [u8],
    tagged: bool,
    protected: &[u8],
    unprotected: &[u8],
    payload: Option<&[u8]>,
    signature: &[u8],
) -> Result<usize> {
    let required = sign1_message_len(tagged, protected, unprotected, payload, signature.len());
    if out.len() < required {
        return Err(Error::BufferTooSmall {
            required,
            available: out.len(),
        });
    }

    let mut cursor = Cursor::new(&mut out[..required]);
    {
        let mut e = Encoder::new(&mut cursor);
        if tagged {
            e.tag(Tag::new(18))?;
        }
        e.array(4)?;
        e.bytes(protected)?;
        e.writer_mut().write_all(unprotected).map_err(|_| Error::Encode)?;
        match payload {
            Some(payload) => {
                e.bytes(payload)?;
            }
            None => {
                e.null()?;
            }
        }
        e.bytes(signature)?;
    }
    if cursor.position() != required {
        return Err(Error::Encode);
    }
    Ok(required)
}

/// A parsed COSE_Sign1 message.
pub struct Sign1<'a> {
    protected: &'a [u8],
    unprotected: &'a [u8],
    payload: Option<&'a [u8]>,
    signature: &'a [u8],
    tagged: bool,
}

impl fmt::Debug for Sign1<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sign1")
            .field("tagged", &self.tagged)
            .field("payload_len", &self.payload.map(|p| p.len()))
            .field("signature_len", &self.signature.len())
            .finish()
    }
}

impl<'a> Sign1<'a> {
    /// Parses a COSE_Sign1 message.
    ///
    /// Both tagged (`#6.18`) and untagged messages are accepted. Header maps are
    /// parsed lazily by [`Sign1::headers`] or by the verification methods.
    ///
    /// # Errors
    ///
    /// Returns [`Error::WrongTag`] for a tag other than 18,
    /// [`Error::InvalidStructure`] for the wrong array shape or field type,
    /// [`Error::IndefiniteLength`] for indefinite-length containers,
    /// [`Error::Decode`] for malformed CBOR, and [`Error::TrailingBytes`] if
    /// extra bytes follow the message.
    pub fn from_slice(input: &'a [u8]) -> Result<Self> {
        let mut d = decoder(input);
        let tagged = d.datatype()? == Type::Tag;
        if tagged {
            let tag = d.tag()?;
            if tag.get() != 18 {
                return Err(Error::WrongTag {
                    expected: 18,
                    actual: tag.get(),
                });
            }
        }
        if d.array()?.ok_or(Error::IndefiniteLength)? != 4 {
            return Err(Error::InvalidStructure);
        }
        let protected = d.bytes()?;
        if d.datatype()? != Type::Map {
            return Err(Error::InvalidStructure);
        }
        let unprotected = take_raw(input, &mut d)?;
        let payload = if d.datatype()? == Type::Null {
            d.null()?;
            None
        } else {
            Some(d.bytes()?)
        };
        let signature = d.bytes()?;
        if d.position() != input.len() {
            return Err(Error::TrailingBytes);
        }
        Ok(Self {
            protected,
            unprotected,
            payload,
            signature,
            tagged,
        })
    }

    /// Returns true if the message carries the `#6.18` tag.
    #[inline]
    pub fn is_tagged(&self) -> bool {
        self.tagged
    }

    /// Returns the protected header bytes.
    #[inline]
    pub fn protected(&self) -> &'a [u8] {
        self.protected
    }

    /// Returns the raw encoded unprotected header map.
    #[inline]
    pub fn unprotected(&self) -> &'a [u8] {
        self.unprotected
    }

    /// Returns the attached payload, or `None` for a detached payload.
    #[inline]
    pub fn payload(&self) -> Option<&'a [u8]> {
        self.payload
    }

    /// Returns the signature bytes.
    #[inline]
    pub fn signature(&self) -> &'a [u8] {
        self.signature
    }

    /// Parses and validates both header buckets.
    pub fn headers(&self) -> Result<Headers<'a>> {
        let protected = Header::from_slice(self.protected)?;
        let unprotected = Header::from_slice(self.unprotected)?;
        Headers::new(protected, unprotected)
    }

    /// Verifies the attached payload without allocating.
    pub fn verify_with_scratch(
        &self,
        verifier: &dyn Verifier,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<()> {
        let payload = self.payload.ok_or(Error::DetachedPayloadMissing)?;
        self.verify_payload(payload, verifier, external_aad, options, scratch)
    }

    /// Verifies a detached payload without allocating.
    pub fn verify_detached_with_scratch(
        &self,
        payload: &[u8],
        verifier: &dyn Verifier,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<()> {
        if self.payload.is_some() {
            return Err(Error::PayloadNotDetached);
        }
        self.verify_payload(payload, verifier, external_aad, options, scratch)
    }

    fn verify_payload(
        &self,
        payload: &[u8],
        verifier: &dyn Verifier,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<()> {
        let headers = self.headers()?;
        headers.validate_with(options.understood_header_labels)?;
        let algorithm = check_algorithm(&headers, verifier.algorithm(), options)?;
        debug_assert!(algorithm.is_signature());
        if let Some(expected) = algorithm.signature_len()
            && self.signature.len() != expected
        {
            return Err(Error::InvalidSignature);
        }

        let len = encode_sig_structure_into(scratch, "Signature1", self.protected, None, external_aad, payload)?;
        verifier.verify(&scratch[..len], self.signature)
    }

    /// Verifies with a key set without allocating.
    pub fn verify_with_key_set_scratch(
        &self,
        key_set: &CoseKeySet<'_>,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<()> {
        let payload = self.payload.ok_or(Error::DetachedPayloadMissing)?;
        self.verify_payload_with_key_set(payload, key_set, external_aad, options, scratch)
    }

    /// Verifies a detached payload with a key set without allocating.
    pub fn verify_detached_with_key_set_scratch(
        &self,
        payload: &[u8],
        key_set: &CoseKeySet<'_>,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<()> {
        if self.payload.is_some() {
            return Err(Error::PayloadNotDetached);
        }
        self.verify_payload_with_key_set(payload, key_set, external_aad, options, scratch)
    }

    fn verify_payload_with_key_set(
        &self,
        payload: &[u8],
        key_set: &CoseKeySet<'_>,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<()> {
        let headers = self.headers()?;
        headers.validate_with(options.understood_header_labels)?;
        let algorithm = headers.algorithm().ok_or(Error::MissingAlgorithm)?;
        check_algorithm_policy(&headers, algorithm, options)?;
        if !algorithm.is_signature() {
            return Err(Error::UnsupportedAlgorithm(algorithm));
        }
        if let Some(expected) = algorithm.signature_len()
            && self.signature.len() != expected
        {
            return Err(Error::InvalidSignature);
        }

        let kid = headers.kid();
        let len = encode_sig_structure_into(scratch, "Signature1", self.protected, None, external_aad, payload)?;
        let tbs = &scratch[..len];

        let limit = options.max_key_candidates;
        let mut attempts = 0usize;
        for key in key_set.iter() {
            let Ok(key) = key else {
                continue;
            };
            if let Some(kid) = kid
                && key.kid() != Some(kid)
            {
                continue;
            }
            if key.validate_for(algorithm, KeyOp::Verify).is_err() {
                continue;
            }
            if let Some(limit) = limit
                && attempts >= limit
            {
                return Err(Error::TooManyAttempts {
                    limit,
                });
            }
            attempts += 1;
            if verify_with_cose_key(&key, algorithm, tbs, self.signature).is_ok() {
                return Ok(());
            }
        }

        Err(Error::InvalidSignature)
    }

    /// Returns the exact encoded message length.
    pub fn encoded_len(&self) -> usize {
        sign1_message_len(
            self.tagged,
            self.protected,
            self.unprotected,
            self.payload,
            self.signature.len(),
        )
    }

    /// Encodes the message into `out`.
    pub fn encode_into(&self, out: &mut [u8]) -> Result<usize> {
        encode_sign1_into(out, self.tagged, self.protected, self.unprotected, self.payload, self.signature)
    }

    /// Encodes the message into a new `Vec`.
    #[cfg(feature = "alloc")]
    pub fn to_vec(&self) -> alloc::vec::Vec<u8> {
        let mut out = alloc::vec![0u8; self.encoded_len()];
        let len = self.encode_into(&mut out).expect("capacity was computed exactly");
        out.truncate(len);
        out
    }

    /// Verifies the attached payload, allocating scratch space.
    #[cfg(feature = "alloc")]
    pub fn verify(&self, verifier: &dyn Verifier, external_aad: &[u8], options: &VerifyOptions<'_>) -> Result<()> {
        let payload = self.payload.ok_or(Error::DetachedPayloadMissing)?;
        let mut scratch = alloc::vec![
            0u8;
            sig_structure_len("Signature1", self.protected, None, external_aad, payload)
        ];
        self.verify_with_scratch(verifier, external_aad, options, &mut scratch)
    }

    /// Verifies a detached payload, allocating scratch space.
    #[cfg(feature = "alloc")]
    pub fn verify_detached(
        &self,
        payload: &[u8],
        verifier: &dyn Verifier,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
    ) -> Result<()> {
        if self.payload.is_some() {
            return Err(Error::PayloadNotDetached);
        }
        let mut scratch = alloc::vec![
            0u8;
            sig_structure_len("Signature1", self.protected, None, external_aad, payload)
        ];
        self.verify_detached_with_scratch(payload, verifier, external_aad, options, &mut scratch)
    }

    /// Verifies with a key set, allocating scratch space.
    #[cfg(feature = "alloc")]
    pub fn verify_with_key_set(
        &self,
        key_set: &CoseKeySet<'_>,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
    ) -> Result<()> {
        let payload = self.payload.ok_or(Error::DetachedPayloadMissing)?;
        let mut scratch = alloc::vec![
            0u8;
            sig_structure_len("Signature1", self.protected, None, external_aad, payload)
        ];
        self.verify_with_key_set_scratch(key_set, external_aad, options, &mut scratch)
    }

    /// Verifies a detached payload with a key set, allocating scratch space.
    #[cfg(feature = "alloc")]
    pub fn verify_detached_with_key_set(
        &self,
        payload: &[u8],
        key_set: &CoseKeySet<'_>,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
    ) -> Result<()> {
        if self.payload.is_some() {
            return Err(Error::PayloadNotDetached);
        }
        let mut scratch = alloc::vec![
            0u8;
            sig_structure_len("Signature1", self.protected, None, external_aad, payload)
        ];
        self.verify_detached_with_key_set_scratch(payload, key_set, external_aad, options, &mut scratch)
    }
}

fn check_algorithm(headers: &Headers<'_>, key_algorithm: Algorithm, options: &VerifyOptions<'_>) -> Result<Algorithm> {
    let algorithm = headers.algorithm().ok_or(Error::MissingAlgorithm)?;
    check_algorithm_policy(headers, algorithm, options)?;
    if !algorithm.is_compatible(key_algorithm) {
        return Err(Error::AlgorithmMismatch {
            header: algorithm,
            key: key_algorithm,
        });
    }
    if !algorithm.is_signature() {
        return Err(Error::UnsupportedAlgorithm(algorithm));
    }
    Ok(algorithm)
}

fn check_algorithm_policy(headers: &Headers<'_>, algorithm: Algorithm, options: &VerifyOptions<'_>) -> Result<()> {
    if options.require_protected_alg && headers.protected().algorithm().is_none() {
        return Err(Error::AlgorithmNotProtected);
    }
    if let Some(allowed) = options.allowed_algorithms
        && !allowed.contains(&algorithm)
    {
        return Err(Error::AlgorithmNotAllowed(algorithm));
    }
    Ok(())
}

fn verify_with_cose_key(key: &crate::CoseKey<'_>, algorithm: Algorithm, tbs: &[u8], signature: &[u8]) -> Result<()> {
    let kty = key.kty()?;
    match (kty, algorithm) {
        (KeyType::Okp, Algorithm::Ed25519 | Algorithm::EdDSA) => {
            let public = crypto::curve25519::ed25519::PublicKey::try_from(key)?;
            Verifier::verify(&public, tbs, signature)
        }
        (KeyType::Ec2, Algorithm::ESP256 | Algorithm::ES256) => {
            let public = crypto::p256::PublicKey::try_from(key)?;
            Verifier::verify(&public, tbs, signature)
        }
        (KeyType::Ec2, Algorithm::ESP384 | Algorithm::ES384) => {
            let public = crypto::p384::PublicKey::try_from(key)?;
            Verifier::verify(&public, tbs, signature)
        }
        (KeyType::Ec2, Algorithm::ESP512 | Algorithm::ES512) => {
            let public = crypto::p521::PublicKey::try_from(key)?;
            Verifier::verify(&public, tbs, signature)
        }
        (KeyType::Akp, Algorithm::MlDsa44) => {
            let public = crypto::mldsa::MlDsa44PublicKey::try_from(key)?;
            Verifier::verify(&public, tbs, signature)
        }
        (KeyType::Akp, Algorithm::MlDsa65) => {
            let public = crypto::mldsa::MlDsa65PublicKey::try_from(key)?;
            Verifier::verify(&public, tbs, signature)
        }
        (KeyType::Akp, Algorithm::MlDsa87) => {
            let public = crypto::mldsa::MlDsa87PublicKey::try_from(key)?;
            Verifier::verify(&public, tbs, signature)
        }
        (KeyType::Rsa, _) => {
            let public = key.rsa_public_key(algorithm, KeyOp::Verify)?;
            Verifier::verify(&public, tbs, signature)
        }
        _ => Err(Error::InvalidKey),
    }
}

/// Builder for a COSE_Sign1 message.
pub struct Sign1Builder<'a> {
    protected: Header<'a>,
    unprotected: Header<'a>,
    payload: Option<&'a [u8]>,
    detached: bool,
    external_aad: &'a [u8],
    tagged: bool,
}

impl<'a> Sign1Builder<'a> {
    /// Creates a builder with empty headers, no payload, no external AAD, and
    /// tagging enabled.
    #[inline]
    pub const fn new() -> Self {
        Self {
            protected: Header::empty_protected(),
            unprotected: Header::empty_unprotected(),
            payload: None,
            detached: false,
            external_aad: &[],
            tagged: true,
        }
    }

    /// Sets the protected header.
    #[inline]
    pub fn protected(mut self, header: Header<'a>) -> Self {
        self.protected = header;
        self
    }

    /// Sets the unprotected header.
    #[inline]
    pub fn unprotected(mut self, header: Header<'a>) -> Self {
        self.unprotected = header;
        self
    }

    /// Sets an attached payload.
    #[inline]
    pub fn payload(mut self, payload: &'a [u8]) -> Self {
        self.payload = Some(payload);
        self.detached = false;
        self
    }

    /// Sets a detached payload. The payload is signed but encoded as `nil`.
    #[inline]
    pub fn detached_payload(mut self, payload: &'a [u8]) -> Self {
        self.payload = Some(payload);
        self.detached = true;
        self
    }

    /// Sets externally supplied authenticated data.
    #[inline]
    pub fn external_aad(mut self, external_aad: &'a [u8]) -> Self {
        self.external_aad = external_aad;
        self
    }

    /// Enables or disables the `#6.18` tag.
    #[inline]
    pub fn tagged(mut self, tagged: bool) -> Self {
        self.tagged = tagged;
        self
    }

    /// Signs and encodes into `out` without allocating.
    ///
    /// This supports signatures up to 132 bytes (ECDSA P-256/P-384/P-521 and
    /// Ed25519). For larger signatures such as ML-DSA, use
    /// [`Sign1Builder::sign_into_with_signature`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::MissingPayload`] when no payload was set,
    /// [`Error::MissingAlgorithm`]/[`Error::AlgorithmMismatch`] when the
    /// headers and signer disagree, [`Error::BufferTooSmall`] when `out` or
    /// the internal signature buffer is too small, and
    /// [`Error::InvalidSignature`] if the signer writes the wrong length.
    pub fn sign_into(&self, signer: &dyn Signer, out: &mut [u8]) -> Result<usize> {
        if signer.signature_len() > 132 {
            return Err(Error::BufferTooSmall {
                required: signer.signature_len(),
                available: 132,
            });
        }
        let mut signature = [0u8; 132];
        self.sign_into_with_signature(signer, out, &mut signature)
    }

    /// Signs and encodes into `out`, using a caller-provided signature buffer.
    ///
    /// # Errors
    ///
    /// Returns [`Error::MissingPayload`] when no payload was set,
    /// [`Error::MissingAlgorithm`]/[`Error::AlgorithmMismatch`] when the
    /// headers and signer disagree, [`Error::BufferTooSmall`] when `out` or
    /// `signature` is too small, and [`Error::InvalidSignature`] if the
    /// signer writes the wrong length.
    pub fn sign_into_with_signature(&self, signer: &dyn Signer, out: &mut [u8], signature: &mut [u8]) -> Result<usize> {
        let headers = Headers::new(
            Header::from_slice(self.protected.as_bytes())?,
            Header::from_slice(self.unprotected.as_bytes())?,
        )?;
        let algorithm = headers.algorithm().ok_or(Error::MissingAlgorithm)?;
        if !algorithm.is_compatible(signer.algorithm()) {
            return Err(Error::AlgorithmMismatch {
                header: algorithm,
                key: signer.algorithm(),
            });
        }
        if !algorithm.is_signature() {
            return Err(Error::UnsupportedAlgorithm(algorithm));
        }

        let payload = self.payload.ok_or(Error::MissingPayload)?;
        let signature_len = signer.signature_len();
        if signature.len() < signature_len {
            return Err(Error::BufferTooSmall {
                required: signature_len,
                available: signature.len(),
            });
        }

        let tbs_len = sig_structure_len("Signature1", self.protected.as_bytes(), None, self.external_aad, payload);
        let message_payload = if self.detached { None } else { Some(payload) };
        let message_len = sign1_message_len(
            self.tagged,
            self.protected.as_bytes(),
            self.unprotected.as_bytes(),
            message_payload,
            signature_len,
        );
        let required = tbs_len.max(message_len);
        if out.len() < required {
            return Err(Error::BufferTooSmall {
                required,
                available: out.len(),
            });
        }

        encode_sig_structure_into(
            &mut out[..tbs_len],
            "Signature1",
            self.protected.as_bytes(),
            None,
            self.external_aad,
            payload,
        )?;

        let signed = signer.sign_into(&out[..tbs_len], &mut signature[..signature_len])?;
        if signed != signature_len {
            return Err(Error::InvalidSignature);
        }

        encode_sign1_into(
            out,
            self.tagged,
            self.protected.as_bytes(),
            self.unprotected.as_bytes(),
            message_payload,
            &signature[..signature_len],
        )
    }

    /// Signs and encodes into a new `Vec`.
    #[cfg(feature = "alloc")]
    pub fn sign(&self, signer: &dyn Signer) -> Result<alloc::vec::Vec<u8>> {
        let payload = self.payload.ok_or(Error::MissingPayload)?;
        let message_payload = if self.detached { None } else { Some(payload) };
        let tbs_len = sig_structure_len("Signature1", self.protected.as_bytes(), None, self.external_aad, payload);
        let message_len = sign1_message_len(
            self.tagged,
            self.protected.as_bytes(),
            self.unprotected.as_bytes(),
            message_payload,
            signer.signature_len(),
        );
        let required = tbs_len.max(message_len);
        let mut out = alloc::vec![0u8; required];

        if signer.signature_len() <= 132 {
            let len = self.sign_into(signer, &mut out)?;
            out.truncate(len);
        } else {
            let mut signature = alloc::vec![0u8; signer.signature_len()];
            let len = self.sign_into_with_signature(signer, &mut out, &mut signature)?;
            out.truncate(len);
        }
        Ok(out)
    }
}

impl Default for Sign1Builder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// COSE_Sign
// ---------------------------------------------------------------------------

/// A single signature inside a COSE_Sign message.
pub struct SignatureView<'a> {
    protected: &'a [u8],
    unprotected: &'a [u8],
    signature: &'a [u8],
}

impl<'a> SignatureView<'a> {
    fn from_slice(input: &'a [u8]) -> Result<Self> {
        let mut d = decoder(input);
        if d.array()?.ok_or(Error::IndefiniteLength)? != 3 {
            return Err(Error::InvalidStructure);
        }
        let protected = d.bytes()?;
        if d.datatype()? != Type::Map {
            return Err(Error::InvalidStructure);
        }
        let unprotected = take_raw(input, &mut d)?;
        let signature = d.bytes()?;
        if d.position() != input.len() {
            return Err(Error::TrailingBytes);
        }
        Ok(Self {
            protected,
            unprotected,
            signature,
        })
    }

    /// Returns the protected header bytes.
    #[inline]
    pub fn protected(&self) -> &'a [u8] {
        self.protected
    }

    /// Returns the raw unprotected header map.
    #[inline]
    pub fn unprotected(&self) -> &'a [u8] {
        self.unprotected
    }

    /// Returns the signature bytes.
    #[inline]
    pub fn signature(&self) -> &'a [u8] {
        self.signature
    }

    /// Parses and validates the signer headers.
    pub fn headers(&self) -> Result<Headers<'a>> {
        let protected = Header::from_slice(self.protected)?;
        let unprotected = Header::from_slice(self.unprotected)?;
        Headers::new(protected, unprotected)
    }
}

/// Iterator over the signatures of a COSE_Sign message.
pub struct Signatures<'a> {
    input: &'a [u8],
    d: Decoder<'a>,
    remaining: usize,
}

impl<'a> Iterator for Signatures<'a> {
    type Item = Result<SignatureView<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let raw = match take_raw(self.input, &mut self.d) {
            Ok(raw) => raw,
            Err(err) => {
                self.remaining = 0;
                return Some(Err(err));
            }
        };
        Some(SignatureView::from_slice(raw))
    }
}

/// A parsed COSE_Sign message.
pub struct Sign<'a> {
    protected: &'a [u8],
    unprotected: &'a [u8],
    payload: Option<&'a [u8]>,
    signatures: &'a [u8],
    signature_count: usize,
    tagged: bool,
}

impl<'a> Sign<'a> {
    /// Parses a COSE_Sign message.
    ///
    /// # Errors
    ///
    /// Returns [`Error::WrongTag`] for a tag other than 98,
    /// [`Error::InvalidStructure`] for the wrong array shape, an empty
    /// signature array, more than 64 signatures, or a bad field type,
    /// [`Error::IndefiniteLength`] for indefinite-length containers,
    /// [`Error::Decode`] for malformed CBOR, and [`Error::TrailingBytes`] if
    /// extra bytes follow the message.
    pub fn from_slice(input: &'a [u8]) -> Result<Self> {
        let mut d = decoder(input);
        let tagged = d.datatype()? == Type::Tag;
        if tagged {
            let tag = d.tag()?;
            if tag.get() != 98 {
                return Err(Error::WrongTag {
                    expected: 98,
                    actual: tag.get(),
                });
            }
        }
        if d.array()?.ok_or(Error::IndefiniteLength)? != 4 {
            return Err(Error::InvalidStructure);
        }
        let protected = d.bytes()?;
        if d.datatype()? != Type::Map {
            return Err(Error::InvalidStructure);
        }
        let unprotected = take_raw(input, &mut d)?;
        let payload = if d.datatype()? == Type::Null {
            d.null()?;
            None
        } else {
            Some(d.bytes()?)
        };
        if d.datatype()? != Type::Array {
            return Err(Error::InvalidStructure);
        }
        let signatures = take_raw(input, &mut d)?;
        if d.position() != input.len() {
            return Err(Error::TrailingBytes);
        }

        let mut sd = decoder(signatures);
        let signature_count = sd.array()?.ok_or(Error::IndefiniteLength)?;
        if signature_count == 0 || signature_count > 64 {
            return Err(Error::InvalidStructure);
        }

        Ok(Self {
            protected,
            unprotected,
            payload,
            signatures,
            signature_count,
            tagged,
        })
    }

    /// Returns true if the message carries the `#6.98` tag.
    #[inline]
    pub fn is_tagged(&self) -> bool {
        self.tagged
    }

    /// Returns the body protected header bytes.
    #[inline]
    pub fn protected(&self) -> &'a [u8] {
        self.protected
    }

    /// Returns the raw body unprotected header map.
    #[inline]
    pub fn unprotected(&self) -> &'a [u8] {
        self.unprotected
    }

    /// Returns the attached payload, or `None` for a detached payload.
    #[inline]
    pub fn payload(&self) -> Option<&'a [u8]> {
        self.payload
    }

    /// Returns the number of signatures.
    #[inline]
    pub fn signature_count(&self) -> usize {
        self.signature_count
    }

    /// Parses and validates the body headers.
    pub fn headers(&self) -> Result<Headers<'a>> {
        let protected = Header::from_slice(self.protected)?;
        let unprotected = Header::from_slice(self.unprotected)?;
        Headers::new(protected, unprotected)
    }

    /// Iterates over signatures.
    pub fn signatures(&self) -> Signatures<'a> {
        let input = self.signatures;
        let mut d = decoder(input);
        let remaining = match d.array() {
            Ok(Some(len)) => len,
            _ => 0,
        };
        Signatures {
            input,
            d,
            remaining,
        }
    }

    fn signature_at(&self, index: usize) -> Result<SignatureView<'a>> {
        self.signatures().nth(index).ok_or(Error::InvalidStructure)?
    }

    /// Verifies one signature without allocating.
    pub fn verify_with_scratch(
        &self,
        index: usize,
        verifier: &dyn Verifier,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<()> {
        let payload = self.payload.ok_or(Error::DetachedPayloadMissing)?;
        self.verify_payload(index, payload, verifier, external_aad, options, scratch)
    }

    /// Verifies a detached payload without allocating.
    pub fn verify_detached_with_scratch(
        &self,
        index: usize,
        payload: &[u8],
        verifier: &dyn Verifier,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<()> {
        if self.payload.is_some() {
            return Err(Error::PayloadNotDetached);
        }
        self.verify_payload(index, payload, verifier, external_aad, options, scratch)
    }

    fn verify_payload(
        &self,
        index: usize,
        payload: &[u8],
        verifier: &dyn Verifier,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<()> {
        let body_headers = self.headers()?;
        body_headers.validate_with(options.understood_header_labels)?;
        let signature = self.signature_at(index)?;
        let signer_headers = signature.headers()?;
        signer_headers.validate_with(options.understood_header_labels)?;
        let algorithm = signer_headers.algorithm().ok_or(Error::MissingAlgorithm)?;
        check_algorithm_policy(&signer_headers, algorithm, options)?;
        if !algorithm.is_compatible(verifier.algorithm()) {
            return Err(Error::AlgorithmMismatch {
                header: algorithm,
                key: verifier.algorithm(),
            });
        }
        if !algorithm.is_signature() {
            return Err(Error::UnsupportedAlgorithm(algorithm));
        }
        if let Some(expected) = algorithm.signature_len()
            && signature.signature.len() != expected
        {
            return Err(Error::InvalidSignature);
        }

        let len = encode_sig_structure_into(
            scratch,
            "Signature",
            self.protected,
            Some(signature.protected),
            external_aad,
            payload,
        )?;
        verifier.verify(&scratch[..len], signature.signature)
    }

    /// Tries signatures in order without allocating. Returns the successful
    /// index.
    ///
    /// Stops after [`VerifyOptions::max_signature_attempts`] signatures
    /// (8 by default) and returns [`Error::TooManyAttempts`] if the budget is
    /// exhausted before a signature verifies.
    pub fn verify_any_with_scratch(
        &self,
        verifier: &dyn Verifier,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<usize> {
        let payload = self.payload.ok_or(Error::DetachedPayloadMissing)?;
        self.verify_any_payload_with_scratch(payload, verifier, external_aad, options, scratch)
    }

    /// Tries every signature against a detached payload without allocating.
    pub fn verify_any_detached_with_scratch(
        &self,
        payload: &[u8],
        verifier: &dyn Verifier,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<usize> {
        if self.payload.is_some() {
            return Err(Error::PayloadNotDetached);
        }
        self.verify_any_payload_with_scratch(payload, verifier, external_aad, options, scratch)
    }

    fn verify_any_payload_with_scratch(
        &self,
        payload: &[u8],
        verifier: &dyn Verifier,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<usize> {
        let limit = options.max_signature_attempts;
        for index in 0..self.signature_count() {
            if let Some(limit) = limit
                && index >= limit
            {
                return Err(Error::TooManyAttempts {
                    limit,
                });
            }
            if self
                .verify_payload(index, payload, verifier, external_aad, options, scratch)
                .is_ok()
            {
                return Ok(index);
            }
        }
        Err(Error::InvalidSignature)
    }

    /// Verifies one signature with a key set without allocating.
    pub fn verify_with_key_set_scratch(
        &self,
        index: usize,
        key_set: &CoseKeySet<'_>,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<()> {
        let payload = self.payload.ok_or(Error::DetachedPayloadMissing)?;
        self.verify_payload_with_key_set(index, payload, key_set, external_aad, options, scratch)
    }

    /// Verifies one signature against a detached payload with a key set
    /// without allocating.
    pub fn verify_detached_with_key_set_scratch(
        &self,
        index: usize,
        payload: &[u8],
        key_set: &CoseKeySet<'_>,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<()> {
        if self.payload.is_some() {
            return Err(Error::PayloadNotDetached);
        }
        self.verify_payload_with_key_set(index, payload, key_set, external_aad, options, scratch)
    }

    fn verify_payload_with_key_set(
        &self,
        index: usize,
        payload: &[u8],
        key_set: &CoseKeySet<'_>,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<()> {
        let body_headers = self.headers()?;
        body_headers.validate_with(options.understood_header_labels)?;
        let signature = self.signature_at(index)?;
        let signer_headers = signature.headers()?;
        signer_headers.validate_with(options.understood_header_labels)?;
        let algorithm = signer_headers.algorithm().ok_or(Error::MissingAlgorithm)?;
        check_algorithm_policy(&signer_headers, algorithm, options)?;
        if !algorithm.is_signature() {
            return Err(Error::UnsupportedAlgorithm(algorithm));
        }
        if let Some(expected) = algorithm.signature_len()
            && signature.signature.len() != expected
        {
            return Err(Error::InvalidSignature);
        }

        let kid = signer_headers.kid();
        let len = encode_sig_structure_into(
            scratch,
            "Signature",
            self.protected,
            Some(signature.protected),
            external_aad,
            payload,
        )?;
        let tbs = &scratch[..len];

        let limit = options.max_key_candidates;
        let mut attempts = 0usize;
        for key in key_set.iter() {
            let Ok(key) = key else {
                continue;
            };
            if let Some(kid) = kid
                && key.kid() != Some(kid)
            {
                continue;
            }
            if key.validate_for(algorithm, KeyOp::Verify).is_err() {
                continue;
            }
            if let Some(limit) = limit
                && attempts >= limit
            {
                return Err(Error::TooManyAttempts {
                    limit,
                });
            }
            attempts += 1;
            if verify_with_cose_key(&key, algorithm, tbs, signature.signature).is_ok() {
                return Ok(());
            }
        }

        Err(Error::InvalidSignature)
    }

    /// Returns the exact encoded message length.
    pub fn encoded_len(&self) -> usize {
        let mut len =
            cbor_len_array(4) + cbor_len_bytes(self.protected.len()) + self.unprotected.len() + self.signatures.len();
        if self.tagged {
            len += cbor_len_tag(98);
        }
        len += match self.payload {
            Some(payload) => cbor_len_bytes(payload.len()),
            None => 1,
        };
        len
    }

    /// Encodes the message into `out`.
    pub fn encode_into(&self, out: &mut [u8]) -> Result<usize> {
        let required = self.encoded_len();
        if out.len() < required {
            return Err(Error::BufferTooSmall {
                required,
                available: out.len(),
            });
        }

        let mut cursor = Cursor::new(&mut out[..required]);
        {
            let mut e = Encoder::new(&mut cursor);
            if self.tagged {
                e.tag(Tag::new(98))?;
            }
            e.array(4)?;
            e.bytes(self.protected)?;
            e.writer_mut().write_all(self.unprotected).map_err(|_| Error::Encode)?;
            match self.payload {
                Some(payload) => {
                    e.bytes(payload)?;
                }
                None => {
                    e.null()?;
                }
            }
            e.writer_mut().write_all(self.signatures).map_err(|_| Error::Encode)?;
        }
        if cursor.position() != required {
            return Err(Error::Encode);
        }
        Ok(required)
    }

    /// Encodes the message into a new `Vec`.
    #[cfg(feature = "alloc")]
    pub fn to_vec(&self) -> alloc::vec::Vec<u8> {
        let mut out = alloc::vec![0u8; self.encoded_len()];
        let len = self.encode_into(&mut out).expect("capacity was computed exactly");
        out.truncate(len);
        out
    }

    /// Verifies one signature, allocating scratch space.
    #[cfg(feature = "alloc")]
    pub fn verify(
        &self,
        index: usize,
        verifier: &dyn Verifier,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
    ) -> Result<()> {
        let payload = self.payload.ok_or(Error::DetachedPayloadMissing)?;
        let signature = self.signature_at(index)?;
        let mut scratch = alloc::vec![
            0u8;
            sig_structure_len(
                "Signature",
                self.protected,
                Some(signature.protected),
                external_aad,
                payload
            )
        ];
        self.verify_with_scratch(index, verifier, external_aad, options, &mut scratch)
    }

    /// Verifies one signature against a detached payload, allocating scratch
    /// space.
    #[cfg(feature = "alloc")]
    pub fn verify_detached(
        &self,
        index: usize,
        payload: &[u8],
        verifier: &dyn Verifier,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
    ) -> Result<()> {
        if self.payload.is_some() {
            return Err(Error::PayloadNotDetached);
        }
        let signature = self.signature_at(index)?;
        let mut scratch = alloc::vec![
            0u8;
            sig_structure_len(
                "Signature",
                self.protected,
                Some(signature.protected),
                external_aad,
                payload
            )
        ];
        self.verify_detached_with_scratch(index, payload, verifier, external_aad, options, &mut scratch)
    }

    /// Tries signatures in order, allocating scratch space.
    ///
    /// Stops after [`VerifyOptions::max_signature_attempts`] signatures
    /// (8 by default) and returns [`Error::TooManyAttempts`] if the budget is
    /// exhausted before a signature verifies.
    #[cfg(feature = "alloc")]
    pub fn verify_any(
        &self,
        verifier: &dyn Verifier,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
    ) -> Result<usize> {
        let payload = self.payload.ok_or(Error::DetachedPayloadMissing)?;
        let mut max_len = 0;
        for signature in self.signatures() {
            let signature = signature?;
            max_len = max_len.max(sig_structure_len(
                "Signature",
                self.protected,
                Some(signature.protected),
                external_aad,
                payload,
            ));
        }
        let mut scratch = alloc::vec![0u8; max_len];
        self.verify_any_with_scratch(verifier, external_aad, options, &mut scratch)
    }

    /// Tries every signature against a detached payload, allocating scratch
    /// space.
    #[cfg(feature = "alloc")]
    pub fn verify_any_detached(
        &self,
        payload: &[u8],
        verifier: &dyn Verifier,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
    ) -> Result<usize> {
        if self.payload.is_some() {
            return Err(Error::PayloadNotDetached);
        }
        let mut max_len = 0;
        for signature in self.signatures() {
            let signature = signature?;
            max_len = max_len.max(sig_structure_len(
                "Signature",
                self.protected,
                Some(signature.protected),
                external_aad,
                payload,
            ));
        }
        let mut scratch = alloc::vec![0u8; max_len];
        self.verify_any_detached_with_scratch(payload, verifier, external_aad, options, &mut scratch)
    }

    /// Verifies one signature with a key set, allocating scratch space.
    #[cfg(feature = "alloc")]
    pub fn verify_with_key_set(
        &self,
        index: usize,
        key_set: &CoseKeySet<'_>,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
    ) -> Result<()> {
        let payload = self.payload.ok_or(Error::DetachedPayloadMissing)?;
        let signature = self.signature_at(index)?;
        let mut scratch = alloc::vec![
            0u8;
            sig_structure_len(
                "Signature",
                self.protected,
                Some(signature.protected),
                external_aad,
                payload
            )
        ];
        self.verify_with_key_set_scratch(index, key_set, external_aad, options, &mut scratch)
    }

    /// Verifies one signature against a detached payload with a key set,
    /// allocating scratch space.
    #[cfg(feature = "alloc")]
    pub fn verify_detached_with_key_set(
        &self,
        index: usize,
        payload: &[u8],
        key_set: &CoseKeySet<'_>,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
    ) -> Result<()> {
        if self.payload.is_some() {
            return Err(Error::PayloadNotDetached);
        }
        let signature = self.signature_at(index)?;
        let mut scratch = alloc::vec![
            0u8;
            sig_structure_len(
                "Signature",
                self.protected,
                Some(signature.protected),
                external_aad,
                payload
            )
        ];
        self.verify_detached_with_key_set_scratch(index, payload, key_set, external_aad, options, &mut scratch)
    }

    /// Signs a COSE_Sign message into `out` without allocating.
    ///
    /// The payload is always attached. Use [`SignBuilder`] for detached
    /// payloads.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidStructure`] for an empty slot list,
    /// [`Error::MissingAlgorithm`]/[`Error::AlgorithmMismatch`] for signer
    /// header problems, [`Error::BufferTooSmall`] when `out` or a signature
    /// buffer is too small, and [`Error::InvalidSignature`] if a signer writes
    /// the wrong length.
    pub fn sign_into<'b>(
        body_protected: Header<'b>,
        body_unprotected: Header<'b>,
        payload: &'b [u8],
        external_aad: &[u8],
        tagged: bool,
        slots: &mut [SignatureSlot<'b>],
        out: &mut [u8],
    ) -> Result<usize> {
        if slots.is_empty() {
            return Err(Error::InvalidStructure);
        }
        Headers::new(
            Header::from_slice(body_protected.as_bytes())?,
            Header::from_slice(body_unprotected.as_bytes())?,
        )?;

        let mut max_tbs = 0;
        for slot in slots.iter() {
            let headers = Headers::new(
                Header::from_slice(slot.protected.as_bytes())?,
                Header::from_slice(slot.unprotected.as_bytes())?,
            )?;
            let algorithm = headers.algorithm().ok_or(Error::MissingAlgorithm)?;
            if !algorithm.is_compatible(slot.signer.algorithm()) {
                return Err(Error::AlgorithmMismatch {
                    header: algorithm,
                    key: slot.signer.algorithm(),
                });
            }
            if !algorithm.is_signature() {
                return Err(Error::UnsupportedAlgorithm(algorithm));
            }
            if slot.signature.len() < slot.signer.signature_len() {
                return Err(Error::BufferTooSmall {
                    required: slot.signer.signature_len(),
                    available: slot.signature.len(),
                });
            }
            max_tbs = max_tbs.max(sig_structure_len(
                "Signature",
                body_protected.as_bytes(),
                Some(slot.protected.as_bytes()),
                external_aad,
                payload,
            ));
        }

        let mut message_len = cbor_len_array(4)
            + cbor_len_bytes(body_protected.as_bytes().len())
            + body_unprotected.as_bytes().len()
            + cbor_len_bytes(payload.len())
            + cbor_len_array(slots.len() as u64);
        if tagged {
            message_len += cbor_len_tag(98);
        }
        for slot in slots.iter() {
            message_len += cbor_len_array(3)
                + cbor_len_bytes(slot.protected.as_bytes().len())
                + slot.unprotected.as_bytes().len()
                + cbor_len_bytes(slot.signer.signature_len());
        }

        let required = max_tbs.max(message_len);
        if out.len() < required {
            return Err(Error::BufferTooSmall {
                required,
                available: out.len(),
            });
        }

        for slot in slots.iter_mut() {
            let tbs_len = sig_structure_len(
                "Signature",
                body_protected.as_bytes(),
                Some(slot.protected.as_bytes()),
                external_aad,
                payload,
            );
            encode_sig_structure_into(
                &mut out[..tbs_len],
                "Signature",
                body_protected.as_bytes(),
                Some(slot.protected.as_bytes()),
                external_aad,
                payload,
            )?;
            let len = slot
                .signer
                .sign_into(&out[..tbs_len], &mut slot.signature[..slot.signer.signature_len()])?;
            if len != slot.signer.signature_len() {
                return Err(Error::InvalidSignature);
            }
            slot.signature_len = len;
        }

        let mut cursor = Cursor::new(&mut out[..message_len]);
        {
            let mut e = Encoder::new(&mut cursor);
            if tagged {
                e.tag(Tag::new(98))?;
            }
            e.array(4)?;
            e.bytes(body_protected.as_bytes())?;
            e.writer_mut()
                .write_all(body_unprotected.as_bytes())
                .map_err(|_| Error::Encode)?;
            e.bytes(payload)?;
            e.array(slots.len())?;
            for slot in slots.iter() {
                e.array(3)?;
                e.bytes(slot.protected.as_bytes())?;
                e.writer_mut()
                    .write_all(slot.unprotected.as_bytes())
                    .map_err(|_| Error::Encode)?;
                e.bytes(&slot.signature[..slot.signature_len])?;
            }
        }
        if cursor.position() != message_len {
            return Err(Error::Encode);
        }
        Ok(message_len)
    }
}

/// One signer slot for zero-allocation COSE_Sign signing.
pub struct SignatureSlot<'a> {
    /// Signer protected header.
    pub protected: Header<'a>,
    /// Signer unprotected header.
    pub unprotected: Header<'a>,
    /// The signer.
    pub signer: &'a dyn Signer,
    /// Buffer that receives the signature.
    pub signature: &'a mut [u8],
    /// Number of signature bytes written by [`Sign::sign_into`].
    pub signature_len: usize,
}

impl<'a> SignatureSlot<'a> {
    /// Creates a signature slot.
    pub fn new(
        signer: &'a dyn Signer,
        protected: Header<'a>,
        unprotected: Header<'a>,
        signature: &'a mut [u8],
    ) -> Self {
        Self {
            protected,
            unprotected,
            signer,
            signature,
            signature_len: 0,
        }
    }
}

/// Owned builder for a COSE_Sign message (alloc only).
#[cfg(feature = "alloc")]
pub struct SignBuilder<'a> {
    protected: Header<'a>,
    unprotected: Header<'a>,
    payload: Option<&'a [u8]>,
    detached: bool,
    external_aad: &'a [u8],
    tagged: bool,
    signatures: alloc::vec::Vec<PendingSignature<'a>>,
}

#[cfg(feature = "alloc")]
struct PendingSignature<'a> {
    signer: &'a dyn Signer,
    protected: Header<'a>,
    unprotected: Header<'a>,
}

#[cfg(feature = "alloc")]
impl<'a> SignBuilder<'a> {
    /// Creates an empty builder.
    pub const fn new() -> Self {
        Self {
            protected: Header::empty_protected(),
            unprotected: Header::empty_unprotected(),
            payload: None,
            detached: false,
            external_aad: &[],
            tagged: true,
            signatures: alloc::vec::Vec::new(),
        }
    }

    /// Sets the body protected header.
    pub fn protected(mut self, header: Header<'a>) -> Self {
        self.protected = header;
        self
    }

    /// Sets the body unprotected header.
    pub fn unprotected(mut self, header: Header<'a>) -> Self {
        self.unprotected = header;
        self
    }

    /// Sets an attached payload.
    pub fn payload(mut self, payload: &'a [u8]) -> Self {
        self.payload = Some(payload);
        self.detached = false;
        self
    }

    /// Sets a detached payload.
    pub fn detached_payload(mut self, payload: &'a [u8]) -> Self {
        self.payload = Some(payload);
        self.detached = true;
        self
    }

    /// Sets external AAD.
    pub fn external_aad(mut self, external_aad: &'a [u8]) -> Self {
        self.external_aad = external_aad;
        self
    }

    /// Enables or disables the `#6.98` tag.
    pub fn tagged(mut self, tagged: bool) -> Self {
        self.tagged = tagged;
        self
    }

    /// Registers a signer.
    ///
    /// The signature is computed later by [`SignBuilder::sign`] using the
    /// final payload, body protected header, and external AAD. Changing those
    /// values after this call therefore affects the signature correctly,
    /// rather than silently producing an invalid message.
    ///
    /// Returns an error if the signer headers are malformed, the signer does
    /// not match the header algorithm, or the algorithm is not a signature
    /// algorithm.
    pub fn add_signature(
        mut self,
        signer: &'a dyn Signer,
        protected: Header<'a>,
        unprotected: Header<'a>,
    ) -> Result<Self> {
        let headers = Headers::new(
            Header::from_slice(protected.as_bytes())?,
            Header::from_slice(unprotected.as_bytes())?,
        )?;
        let algorithm = headers.algorithm().ok_or(Error::MissingAlgorithm)?;
        if !algorithm.is_compatible(signer.algorithm()) {
            return Err(Error::AlgorithmMismatch {
                header: algorithm,
                key: signer.algorithm(),
            });
        }
        if !algorithm.is_signature() {
            return Err(Error::UnsupportedAlgorithm(algorithm));
        }
        drop(headers);

        self.signatures.push(PendingSignature {
            signer,
            protected,
            unprotected,
        });
        Ok(self)
    }

    /// Signs and encodes the message into a new `Vec`.
    pub fn sign(self) -> Result<alloc::vec::Vec<u8>> {
        if self.signatures.is_empty() {
            return Err(Error::InvalidStructure);
        }
        Headers::new(
            Header::from_slice(self.protected.as_bytes())?,
            Header::from_slice(self.unprotected.as_bytes())?,
        )?;
        let payload = self.payload.ok_or(Error::MissingPayload)?;
        let message_payload = if self.detached { None } else { Some(payload) };

        let mut max_tbs = 0usize;
        let mut message_len = cbor_len_array(4)
            + cbor_len_bytes(self.protected.as_bytes().len())
            + self.unprotected.as_bytes().len()
            + cbor_len_array(self.signatures.len() as u64);
        if self.tagged {
            message_len += cbor_len_tag(98);
        }
        message_len += match message_payload {
            Some(payload) => cbor_len_bytes(payload.len()),
            None => 1,
        };
        for signature in &self.signatures {
            max_tbs = max_tbs.max(sig_structure_len(
                "Signature",
                self.protected.as_bytes(),
                Some(signature.protected.as_bytes()),
                self.external_aad,
                payload,
            ));
            message_len += cbor_len_array(3)
                + cbor_len_bytes(signature.protected.as_bytes().len())
                + signature.unprotected.as_bytes().len()
                + cbor_len_bytes(signature.signer.signature_len());
        }

        let required = max_tbs.max(message_len);
        let mut out = alloc::vec![0u8; required];
        let mut signatures = alloc::vec::Vec::with_capacity(self.signatures.len());
        for signature in &self.signatures {
            let tbs_len = sig_structure_len(
                "Signature",
                self.protected.as_bytes(),
                Some(signature.protected.as_bytes()),
                self.external_aad,
                payload,
            );
            encode_sig_structure_into(
                &mut out[..tbs_len],
                "Signature",
                self.protected.as_bytes(),
                Some(signature.protected.as_bytes()),
                self.external_aad,
                payload,
            )?;
            let mut signature_bytes = alloc::vec![0u8; signature.signer.signature_len()];
            let len = signature.signer.sign_into(&out[..tbs_len], &mut signature_bytes)?;
            if len != signature_bytes.len() {
                return Err(Error::InvalidSignature);
            }
            signatures.push(signature_bytes);
        }

        let mut cursor = Cursor::new(&mut out[..message_len]);
        {
            let mut e = Encoder::new(&mut cursor);
            if self.tagged {
                e.tag(Tag::new(98))?;
            }
            e.array(4)?;
            e.bytes(self.protected.as_bytes())?;
            e.writer_mut()
                .write_all(self.unprotected.as_bytes())
                .map_err(|_| Error::Encode)?;
            match message_payload {
                Some(payload) => {
                    e.bytes(payload)?;
                }
                None => {
                    e.null()?;
                }
            }
            e.array(self.signatures.len())?;
            for (signature, signature_bytes) in self.signatures.iter().zip(signatures.iter()) {
                e.array(3)?;
                e.bytes(signature.protected.as_bytes())?;
                e.writer_mut()
                    .write_all(signature.unprotected.as_bytes())
                    .map_err(|_| Error::Encode)?;
                e.bytes(signature_bytes)?;
            }
        }
        if cursor.position() != message_len {
            return Err(Error::Encode);
        }
        out.truncate(message_len);
        Ok(out)
    }
}

#[cfg(feature = "alloc")]
impl Default for SignBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
