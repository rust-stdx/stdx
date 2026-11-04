//! COSE_Mac0 and COSE_Mac support.

use core::fmt;

use cbor::{Decoder, Encoder, Tag, Type, Write};

use crate::{
    Algorithm, CoseKeySet, Error, Header, Headers, KeyOp, Result, VerifyOptions,
    codec::{
        Cursor, cbor_len_array, cbor_len_bytes, cbor_len_int, cbor_len_str, cbor_len_tag, cbor_len_uint, decode_bytes,
        decoder, find_value, take_raw,
    },
    sign::protected_for_structure,
};

/// A MAC: a key that can create and verify COSE authentication tags.
pub trait Macer {
    /// Returns the COSE algorithm implemented by this key.
    fn algorithm(&self) -> Algorithm;

    /// Returns the exact tag size in bytes.
    fn tag_len(&self) -> usize;

    /// Computes the MAC of `message`, writing the tag into `out`.
    fn mac_into(&self, message: &[u8], out: &mut [u8]) -> Result<usize>;

    /// Verifies `tag` over `message`.
    fn verify(&self, message: &[u8], tag: &[u8]) -> Result<()>;

    /// Computes the MAC and returns it in a new `Vec`.
    #[cfg(feature = "alloc")]
    fn mac(&self, message: &[u8]) -> Result<alloc::vec::Vec<u8>> {
        let mut out = alloc::vec![0u8; self.tag_len()];
        let len = self.mac_into(message, &mut out)?;
        out.truncate(len);
        Ok(out)
    }
}

fn mac_structure_len(context: &str, protected: &[u8], external_aad: &[u8], payload: &[u8]) -> usize {
    cbor_len_array(4)
        + cbor_len_str(context.len())
        + cbor_len_bytes(protected_for_structure(protected).len())
        + cbor_len_bytes(external_aad.len())
        + cbor_len_bytes(payload.len())
}

fn encode_mac_structure_into(
    out: &mut [u8],
    context: &str,
    protected: &[u8],
    external_aad: &[u8],
    payload: &[u8],
) -> Result<usize> {
    let required = mac_structure_len(context, protected, external_aad, payload);
    if out.len() < required {
        return Err(Error::BufferTooSmall {
            required,
            available: out.len(),
        });
    }
    let mut cursor = Cursor::new(&mut out[..required]);
    {
        let mut e = Encoder::new(&mut cursor);
        e.array(4)?;
        e.text(context)?;
        e.bytes(protected_for_structure(protected))?;
        e.bytes(external_aad)?;
        e.bytes(payload)?;
    }
    if cursor.position() != required {
        return Err(Error::Encode);
    }
    Ok(required)
}

fn mac0_message_len(
    tagged: bool,
    protected: &[u8],
    unprotected: &[u8],
    payload: Option<&[u8]>,
    tag_len: usize,
) -> usize {
    let mut len = cbor_len_array(4) + cbor_len_bytes(protected.len()) + unprotected.len() + cbor_len_bytes(tag_len);
    if tagged {
        len += cbor_len_tag(17);
    }
    len += match payload {
        Some(payload) => cbor_len_bytes(payload.len()),
        None => 1,
    };
    len
}

fn encode_mac0_into(
    out: &mut [u8],
    tagged: bool,
    protected: &[u8],
    unprotected: &[u8],
    payload: Option<&[u8]>,
    tag: &[u8],
) -> Result<usize> {
    let required = mac0_message_len(tagged, protected, unprotected, payload, tag.len());
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
            e.tag(Tag::new(17))?;
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
        e.bytes(tag)?;
    }
    if cursor.position() != required {
        return Err(Error::Encode);
    }
    Ok(required)
}

fn check_mac_algorithm(
    headers: &Headers<'_>,
    macer_algorithm: Algorithm,
    options: &VerifyOptions<'_>,
) -> Result<Algorithm> {
    let algorithm = headers.algorithm().ok_or(Error::MissingAlgorithm)?;
    check_mac_algorithm_policy(headers, algorithm, options)?;
    if !algorithm.is_mac() {
        return Err(Error::UnsupportedAlgorithm(algorithm));
    }
    if algorithm != macer_algorithm {
        return Err(Error::AlgorithmMismatch {
            header: algorithm,
            key: macer_algorithm,
        });
    }
    Ok(algorithm)
}

fn check_mac_algorithm_policy(headers: &Headers<'_>, algorithm: Algorithm, options: &VerifyOptions<'_>) -> Result<()> {
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

/// A parsed COSE_Mac0 message.
pub struct Mac0<'a> {
    protected: &'a [u8],
    unprotected: &'a [u8],
    payload: Option<&'a [u8]>,
    tag: &'a [u8],
    tagged: bool,
}

impl fmt::Debug for Mac0<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mac0")
            .field("tagged", &self.tagged)
            .field("payload_len", &self.payload.map(|p| p.len()))
            .field("tag_len", &self.tag.len())
            .finish()
    }
}

impl<'a> Mac0<'a> {
    /// Parses a COSE_Mac0 message.
    ///
    /// Both tagged (`#6.17`) and untagged messages are accepted. Header maps
    /// are parsed lazily by [`Mac0::headers`] or by the verification methods.
    ///
    /// # Errors
    ///
    /// Returns [`Error::WrongTag`] for a tag other than 17,
    /// [`Error::InvalidStructure`] for the wrong array shape or field type,
    /// [`Error::IndefiniteLength`] for indefinite-length containers,
    /// [`Error::Decode`] for malformed CBOR, and [`Error::TrailingBytes`] if
    /// extra bytes follow the message.
    pub fn from_slice(input: &'a [u8]) -> Result<Self> {
        let mut d = decoder(input);
        let tagged = d.datatype()? == Type::Tag;
        if tagged {
            let tag = d.tag()?;
            if tag.get() != 17 {
                return Err(Error::WrongTag {
                    expected: 17,
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
        let tag = d.bytes()?;
        if d.position() != input.len() {
            return Err(Error::TrailingBytes);
        }
        Ok(Self {
            protected,
            unprotected,
            payload,
            tag,
            tagged,
        })
    }

    /// Returns true if the message carries the `#6.17` tag.
    #[inline]
    pub fn is_tagged(&self) -> bool {
        self.tagged
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

    /// Returns the attached payload, or `None` for a detached payload.
    #[inline]
    pub fn payload(&self) -> Option<&'a [u8]> {
        self.payload
    }

    /// Returns the MAC tag.
    #[inline]
    pub fn tag(&self) -> &'a [u8] {
        self.tag
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
        macer: &dyn Macer,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<()> {
        let payload = self.payload.ok_or(Error::DetachedPayloadMissing)?;
        self.verify_payload(payload, macer, external_aad, options, scratch)
    }

    /// Verifies a detached payload without allocating.
    pub fn verify_detached_with_scratch(
        &self,
        payload: &[u8],
        macer: &dyn Macer,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<()> {
        if self.payload.is_some() {
            return Err(Error::PayloadNotDetached);
        }
        self.verify_payload(payload, macer, external_aad, options, scratch)
    }

    fn verify_payload(
        &self,
        payload: &[u8],
        macer: &dyn Macer,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<()> {
        let headers = self.headers()?;
        headers.validate_with(options.understood_header_labels)?;
        let algorithm = check_mac_algorithm(&headers, macer.algorithm(), options)?;
        if self.tag.len() != algorithm.tag_len().unwrap_or(0) {
            return Err(Error::InvalidTag);
        }
        let len = encode_mac_structure_into(scratch, "MAC0", self.protected, external_aad, payload)?;
        macer.verify(&scratch[..len], self.tag)
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
        check_mac_algorithm_policy(&headers, algorithm, options)?;
        if !algorithm.is_mac() {
            return Err(Error::UnsupportedAlgorithm(algorithm));
        }
        if self.tag.len() != algorithm.tag_len().unwrap_or(0) {
            return Err(Error::InvalidTag);
        }
        let kid = headers.kid();
        let len = encode_mac_structure_into(scratch, "MAC0", self.protected, external_aad, payload)?;
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
            if key.validate_for(algorithm, KeyOp::MacVerify).is_err() {
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
            if let Ok(symmetric) = key.symmetric_key(algorithm, KeyOp::MacVerify)
                && symmetric.verify(tbs, self.tag).is_ok()
            {
                return Ok(());
            }
        }
        Err(Error::InvalidTag)
    }

    /// Returns the exact encoded message length.
    pub fn encoded_len(&self) -> usize {
        mac0_message_len(self.tagged, self.protected, self.unprotected, self.payload, self.tag.len())
    }

    /// Encodes the message into `out`.
    pub fn encode_into(&self, out: &mut [u8]) -> Result<usize> {
        encode_mac0_into(out, self.tagged, self.protected, self.unprotected, self.payload, self.tag)
    }

    /// Encodes the message into a new `Vec`.
    #[cfg(feature = "alloc")]
    pub fn to_vec(&self) -> alloc::vec::Vec<u8> {
        let mut out = alloc::vec![0u8; self.encoded_len()];
        let len = self.encode_into(&mut out).expect("capacity was exact");
        out.truncate(len);
        out
    }

    /// Verifies the attached payload, allocating scratch space.
    #[cfg(feature = "alloc")]
    pub fn verify(&self, macer: &dyn Macer, external_aad: &[u8], options: &VerifyOptions<'_>) -> Result<()> {
        let payload = self.payload.ok_or(Error::DetachedPayloadMissing)?;
        let mut scratch = alloc::vec![0u8; mac_structure_len("MAC0", self.protected, external_aad, payload)];
        self.verify_with_scratch(macer, external_aad, options, &mut scratch)
    }

    /// Verifies a detached payload, allocating scratch space.
    #[cfg(feature = "alloc")]
    pub fn verify_detached(
        &self,
        payload: &[u8],
        macer: &dyn Macer,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
    ) -> Result<()> {
        if self.payload.is_some() {
            return Err(Error::PayloadNotDetached);
        }
        let mut scratch = alloc::vec![0u8; mac_structure_len("MAC0", self.protected, external_aad, payload)];
        self.verify_detached_with_scratch(payload, macer, external_aad, options, &mut scratch)
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
        let mut scratch = alloc::vec![0u8; mac_structure_len("MAC0", self.protected, external_aad, payload)];
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
        let mut scratch = alloc::vec![0u8; mac_structure_len("MAC0", self.protected, external_aad, payload)];
        self.verify_detached_with_key_set_scratch(payload, key_set, external_aad, options, &mut scratch)
    }
}

/// Builder for COSE_Mac0.
pub struct Mac0Builder<'a> {
    protected: Header<'a>,
    unprotected: Header<'a>,
    payload: Option<&'a [u8]>,
    detached: bool,
    external_aad: &'a [u8],
    tagged: bool,
}

impl<'a> Mac0Builder<'a> {
    /// Creates an empty builder.
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

    /// Sets a detached payload.
    #[inline]
    pub fn detached_payload(mut self, payload: &'a [u8]) -> Self {
        self.payload = Some(payload);
        self.detached = true;
        self
    }

    /// Sets external AAD.
    #[inline]
    pub fn external_aad(mut self, external_aad: &'a [u8]) -> Self {
        self.external_aad = external_aad;
        self
    }

    /// Enables or disables the `#6.17` tag.
    #[inline]
    pub fn tagged(mut self, tagged: bool) -> Self {
        self.tagged = tagged;
        self
    }

    /// Computes the MAC and encodes into `out` without allocating.
    ///
    /// # Errors
    ///
    /// Returns [`Error::MissingPayload`] when no payload was set,
    /// [`Error::MissingAlgorithm`]/[`Error::AlgorithmMismatch`] when the
    /// headers and MAC key disagree, [`Error::BufferTooSmall`] when `out` is
    /// too small, and [`Error::InvalidTag`] if the MAC key writes the wrong
    /// tag length.
    pub fn mac_into(&self, macer: &dyn Macer, out: &mut [u8]) -> Result<usize> {
        let headers = Headers::new(
            Header::from_slice(self.protected.as_bytes())?,
            Header::from_slice(self.unprotected.as_bytes())?,
        )?;
        let algorithm = headers.algorithm().ok_or(Error::MissingAlgorithm)?;
        if algorithm != macer.algorithm() {
            return Err(Error::AlgorithmMismatch {
                header: algorithm,
                key: macer.algorithm(),
            });
        }
        if !algorithm.is_mac() {
            return Err(Error::UnsupportedAlgorithm(algorithm));
        }
        let payload = self.payload.ok_or(Error::MissingPayload)?;
        let tag_len = algorithm.tag_len().ok_or(Error::UnsupportedAlgorithm(algorithm))?;
        let message_payload = if self.detached { None } else { Some(payload) };

        let tbs_len = mac_structure_len("MAC0", self.protected.as_bytes(), self.external_aad, payload);
        let message_len = mac0_message_len(
            self.tagged,
            self.protected.as_bytes(),
            self.unprotected.as_bytes(),
            message_payload,
            tag_len,
        );
        let required = tbs_len.max(message_len);
        if out.len() < required {
            return Err(Error::BufferTooSmall {
                required,
                available: out.len(),
            });
        }

        encode_mac_structure_into(
            &mut out[..tbs_len],
            "MAC0",
            self.protected.as_bytes(),
            self.external_aad,
            payload,
        )?;

        let mut tag = [0u8; 64];
        let tag_written = macer.mac_into(&out[..tbs_len], &mut tag[..tag_len])?;
        if tag_written != tag_len {
            return Err(Error::InvalidTag);
        }

        encode_mac0_into(
            out,
            self.tagged,
            self.protected.as_bytes(),
            self.unprotected.as_bytes(),
            message_payload,
            &tag[..tag_len],
        )
    }

    /// Computes the MAC and encodes into a new `Vec`.
    #[cfg(feature = "alloc")]
    pub fn mac(&self, macer: &dyn Macer) -> Result<alloc::vec::Vec<u8>> {
        let payload = self.payload.ok_or(Error::MissingPayload)?;
        let message_payload = if self.detached { None } else { Some(payload) };
        let tbs_len = mac_structure_len("MAC0", self.protected.as_bytes(), self.external_aad, payload);
        let message_len = mac0_message_len(
            self.tagged,
            self.protected.as_bytes(),
            self.unprotected.as_bytes(),
            message_payload,
            macer.tag_len(),
        );
        let mut out = alloc::vec![0u8; tbs_len.max(message_len)];
        let len = self.mac_into(macer, &mut out)?;
        out.truncate(len);
        Ok(out)
    }
}

impl Default for Mac0Builder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// COSE_Mac
// ---------------------------------------------------------------------------

/// A recipient in a COSE_Mac message.
pub struct Recipient<'a> {
    protected: &'a [u8],
    unprotected: &'a [u8],
    ciphertext: Option<&'a [u8]>,
    recipients: Option<&'a [u8]>,
}

fn kid_in(bytes: &[u8]) -> Option<&[u8]> {
    let raw = find_value(bytes, crate::Label::Int(4)).ok()??;
    decode_bytes(raw).ok()
}

impl<'a> Recipient<'a> {
    fn from_slice(input: &'a [u8]) -> Result<Self> {
        let mut d = decoder(input);
        let len = d.array()?.ok_or(Error::IndefiniteLength)?;
        if len != 3 && len != 4 {
            return Err(Error::InvalidStructure);
        }
        let protected = d.bytes()?;
        if d.datatype()? != Type::Map {
            return Err(Error::InvalidStructure);
        }
        let unprotected = take_raw(input, &mut d)?;
        let ciphertext = if d.datatype()? == Type::Null {
            d.null()?;
            None
        } else {
            Some(d.bytes()?)
        };
        let recipients = if len == 4 {
            if d.datatype()? != Type::Array {
                return Err(Error::InvalidStructure);
            }
            Some(take_raw(input, &mut d)?)
        } else {
            None
        };
        if d.position() != input.len() {
            return Err(Error::TrailingBytes);
        }
        let recipient = Self {
            protected,
            unprotected,
            ciphertext,
            recipients,
        };
        // Validate the recipient header buckets eagerly. `Mac` verification
        // relies on `kid`/`alg` being unambiguous; accepting malformed maps
        // here would allow different parsers to disagree on the recipient.
        recipient.headers()?;
        Ok(recipient)
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

    /// Returns the encrypted key, or `None` for `nil`.
    #[inline]
    pub fn ciphertext(&self) -> Option<&'a [u8]> {
        self.ciphertext
    }

    /// Parses and validates the recipient headers.
    pub fn headers(&self) -> Result<Headers<'a>> {
        let protected = Header::from_slice(self.protected)?;
        let unprotected = Header::from_slice(self.unprotected)?;
        Headers::new(protected, unprotected)
    }

    /// Returns the recipient algorithm, preferring the protected bucket.
    pub fn algorithm(&self) -> Option<Algorithm> {
        let protected = Header::from_slice(self.protected).ok();
        let unprotected = Header::from_slice(self.unprotected).ok();
        protected
            .and_then(|header| header.algorithm())
            .or_else(|| unprotected.and_then(|header| header.algorithm()))
    }

    /// Returns the recipient `kid`, preferring the protected bucket.
    pub fn kid(&self) -> Option<&'a [u8]> {
        kid_in(self.protected).or_else(|| kid_in(self.unprotected))
    }

    /// Iterates over nested recipients.
    pub fn recipients(&self) -> Recipients<'a> {
        match self.recipients {
            Some(input) => {
                let mut d = decoder(input);
                let remaining = match d.array() {
                    Ok(Some(len)) => len,
                    _ => 0,
                };
                Recipients {
                    input,
                    d,
                    remaining,
                }
            }
            None => Recipients {
                input: &[],
                d: decoder(&[]),
                remaining: 0,
            },
        }
    }
}

/// Iterator over COSE_Mac recipients.
pub struct Recipients<'a> {
    input: &'a [u8],
    d: Decoder<'a>,
    remaining: usize,
}

impl<'a> Iterator for Recipients<'a> {
    type Item = Result<Recipient<'a>>;

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
        Some(Recipient::from_slice(raw))
    }
}

/// A parsed COSE_Mac message.
pub struct Mac<'a> {
    protected: &'a [u8],
    unprotected: &'a [u8],
    payload: Option<&'a [u8]>,
    tag: &'a [u8],
    recipients: &'a [u8],
    recipient_count: usize,
    tagged: bool,
}

impl<'a> Mac<'a> {
    /// Parses a COSE_Mac message.
    ///
    /// Verification requires exactly one direct recipient; see
    /// [`Mac::verify`]. Recipient headers are validated lazily when the
    /// recipient is inspected or when the message is verified.
    ///
    /// # Errors
    ///
    /// Returns [`Error::WrongTag`] for a tag other than 97,
    /// [`Error::InvalidStructure`] for the wrong array shape or field type,
    /// [`Error::IndefiniteLength`] for indefinite-length containers,
    /// [`Error::Decode`] for malformed CBOR, and [`Error::TrailingBytes`] if
    /// extra bytes follow the message.
    pub fn from_slice(input: &'a [u8]) -> Result<Self> {
        let mut d = decoder(input);
        let tagged = d.datatype()? == Type::Tag;
        if tagged {
            let tag = d.tag()?;
            if tag.get() != 97 {
                return Err(Error::WrongTag {
                    expected: 97,
                    actual: tag.get(),
                });
            }
        }
        if d.array()?.ok_or(Error::IndefiniteLength)? != 5 {
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
        let tag = d.bytes()?;
        if d.datatype()? != Type::Array {
            return Err(Error::InvalidStructure);
        }
        let recipients = take_raw(input, &mut d)?;
        if d.position() != input.len() {
            return Err(Error::TrailingBytes);
        }
        let mut rd = decoder(recipients);
        let recipient_count = rd.array()?.ok_or(Error::IndefiniteLength)?;
        if recipient_count == 0 || recipient_count > 64 {
            return Err(Error::InvalidStructure);
        }
        Ok(Self {
            protected,
            unprotected,
            payload,
            tag,
            recipients,
            recipient_count,
            tagged,
        })
    }

    /// Returns true if the message carries the `#6.97` tag.
    #[inline]
    pub fn is_tagged(&self) -> bool {
        self.tagged
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

    /// Returns the attached payload, or `None` for a detached payload.
    #[inline]
    pub fn payload(&self) -> Option<&'a [u8]> {
        self.payload
    }

    /// Returns the MAC tag.
    #[inline]
    pub fn tag(&self) -> &'a [u8] {
        self.tag
    }

    /// Parses and validates both body header buckets.
    pub fn headers(&self) -> Result<Headers<'a>> {
        let protected = Header::from_slice(self.protected)?;
        let unprotected = Header::from_slice(self.unprotected)?;
        Headers::new(protected, unprotected)
    }

    /// Iterates over recipients.
    pub fn recipients(&self) -> Recipients<'a> {
        let input = self.recipients;
        let mut d = decoder(input);
        let remaining = match d.array() {
            Ok(Some(len)) => len,
            _ => 0,
        };
        Recipients {
            input,
            d,
            remaining,
        }
    }

    /// Returns the number of recipients.
    #[inline]
    pub fn recipient_count(&self) -> usize {
        self.recipient_count
    }

    /// Returns the single direct recipient required by this crate.
    ///
    /// This enforces the RFC 9052 direct-encryption rules strictly:
    /// exactly one recipient, `alg` equal to `direct`, an empty protected
    /// header, an empty ciphertext byte string, and no nested recipients.
    fn direct_recipient(&self) -> Result<Recipient<'a>> {
        if self.recipient_count != 1 {
            return Err(Error::InvalidStructure);
        }
        let recipient = self.recipients().next().ok_or(Error::InvalidStructure)??;

        if recipient.algorithm() != Some(Algorithm::Direct) {
            return Err(Error::InvalidStructure);
        }
        if !Header::from_slice(recipient.protected)?.is_empty() {
            return Err(Error::InvalidStructure);
        }
        if !matches!(recipient.ciphertext(), Some([])) {
            return Err(Error::InvalidStructure);
        }
        if recipient.recipients.is_some() {
            return Err(Error::InvalidStructure);
        }
        Ok(recipient)
    }

    /// Verifies the attached payload without allocating.
    pub fn verify_with_scratch(
        &self,
        macer: &dyn Macer,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<()> {
        let payload = self.payload.ok_or(Error::DetachedPayloadMissing)?;
        self.verify_payload(payload, macer, external_aad, options, scratch)
    }

    /// Verifies a detached payload without allocating.
    pub fn verify_detached_with_scratch(
        &self,
        payload: &[u8],
        macer: &dyn Macer,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<()> {
        if self.payload.is_some() {
            return Err(Error::PayloadNotDetached);
        }
        self.verify_payload(payload, macer, external_aad, options, scratch)
    }

    fn verify_payload(
        &self,
        payload: &[u8],
        macer: &dyn Macer,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
        scratch: &mut [u8],
    ) -> Result<()> {
        let headers = self.headers()?;
        headers.validate_with(options.understood_header_labels)?;
        let algorithm = check_mac_algorithm(&headers, macer.algorithm(), options)?;
        if self.tag.len() != algorithm.tag_len().unwrap_or(0) {
            return Err(Error::InvalidTag);
        }
        self.direct_recipient()?;
        let len = encode_mac_structure_into(scratch, "MAC", self.protected, external_aad, payload)?;
        macer.verify(&scratch[..len], self.tag)
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
        check_mac_algorithm_policy(&headers, algorithm, options)?;
        if !algorithm.is_mac() {
            return Err(Error::UnsupportedAlgorithm(algorithm));
        }
        if self.tag.len() != algorithm.tag_len().unwrap_or(0) {
            return Err(Error::InvalidTag);
        }
        let recipient = self.direct_recipient()?;
        let kid = recipient.kid();
        let len = encode_mac_structure_into(scratch, "MAC", self.protected, external_aad, payload)?;
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
            if key.validate_for(algorithm, KeyOp::MacVerify).is_err() {
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
            if let Ok(symmetric) = key.symmetric_key(algorithm, KeyOp::MacVerify)
                && symmetric.verify(tbs, self.tag).is_ok()
            {
                return Ok(());
            }
        }
        Err(Error::InvalidTag)
    }

    /// Returns the exact encoded message length.
    pub fn encoded_len(&self) -> usize {
        let mut len = cbor_len_array(5)
            + cbor_len_bytes(self.protected.len())
            + self.unprotected.len()
            + cbor_len_bytes(self.tag.len())
            + self.recipients.len();
        if self.tagged {
            len += cbor_len_tag(97);
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
                e.tag(Tag::new(97))?;
            }
            e.array(5)?;
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
            e.bytes(self.tag)?;
            e.writer_mut().write_all(self.recipients).map_err(|_| Error::Encode)?;
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
        let len = self.encode_into(&mut out).expect("capacity was exact");
        out.truncate(len);
        out
    }

    /// Verifies the attached payload, allocating scratch space.
    #[cfg(feature = "alloc")]
    pub fn verify(&self, macer: &dyn Macer, external_aad: &[u8], options: &VerifyOptions<'_>) -> Result<()> {
        let payload = self.payload.ok_or(Error::DetachedPayloadMissing)?;
        let mut scratch = alloc::vec![0u8; mac_structure_len("MAC", self.protected, external_aad, payload)];
        self.verify_with_scratch(macer, external_aad, options, &mut scratch)
    }

    /// Verifies a detached payload, allocating scratch space.
    #[cfg(feature = "alloc")]
    pub fn verify_detached(
        &self,
        payload: &[u8],
        macer: &dyn Macer,
        external_aad: &[u8],
        options: &VerifyOptions<'_>,
    ) -> Result<()> {
        if self.payload.is_some() {
            return Err(Error::PayloadNotDetached);
        }
        let mut scratch = alloc::vec![0u8; mac_structure_len("MAC", self.protected, external_aad, payload)];
        self.verify_detached_with_scratch(payload, macer, external_aad, options, &mut scratch)
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
        let mut scratch = alloc::vec![0u8; mac_structure_len("MAC", self.protected, external_aad, payload)];
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
        let mut scratch = alloc::vec![0u8; mac_structure_len("MAC", self.protected, external_aad, payload)];
        self.verify_detached_with_key_set_scratch(payload, key_set, external_aad, options, &mut scratch)
    }
}

/// Builder for COSE_Mac with one direct recipient.
pub struct MacBuilder<'a> {
    protected: Header<'a>,
    unprotected: Header<'a>,
    payload: Option<&'a [u8]>,
    detached: bool,
    external_aad: &'a [u8],
    tagged: bool,
    recipient_kid: Option<&'a [u8]>,
}

impl<'a> MacBuilder<'a> {
    /// Creates an empty builder.
    #[inline]
    pub const fn new() -> Self {
        Self {
            protected: Header::empty_protected(),
            unprotected: Header::empty_unprotected(),
            payload: None,
            detached: false,
            external_aad: &[],
            tagged: true,
            recipient_kid: None,
        }
    }

    /// Sets the body protected header.
    #[inline]
    pub fn protected(mut self, header: Header<'a>) -> Self {
        self.protected = header;
        self
    }

    /// Sets the body unprotected header.
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

    /// Sets a detached payload.
    #[inline]
    pub fn detached_payload(mut self, payload: &'a [u8]) -> Self {
        self.payload = Some(payload);
        self.detached = true;
        self
    }

    /// Sets external AAD.
    #[inline]
    pub fn external_aad(mut self, external_aad: &'a [u8]) -> Self {
        self.external_aad = external_aad;
        self
    }

    /// Enables or disables the `#6.97` tag.
    #[inline]
    pub fn tagged(mut self, tagged: bool) -> Self {
        self.tagged = tagged;
        self
    }

    /// Sets the `kid` of the single direct recipient.
    #[inline]
    pub fn key_id(mut self, kid: &'a [u8]) -> Self {
        self.recipient_kid = Some(kid);
        self
    }

    fn recipient_len(&self) -> usize {
        let map_len = if self.recipient_kid.is_some() { 2 } else { 1 };
        cbor_len_array(3)
            + cbor_len_bytes(0)
            + cbor_len_uint(map_len)
            + cbor_len_int(1)
            + cbor_len_int(-6)
            + if let Some(kid) = self.recipient_kid {
                cbor_len_int(4) + cbor_len_bytes(kid.len())
            } else {
                0
            }
            + cbor_len_bytes(0)
    }

    /// Computes the MAC and encodes into `out` without allocating.
    ///
    /// # Errors
    ///
    /// Returns [`Error::MissingPayload`] when no payload was set,
    /// [`Error::MissingAlgorithm`]/[`Error::AlgorithmMismatch`] when the
    /// headers and MAC key disagree, [`Error::BufferTooSmall`] when `out` is
    /// too small, [`Error::InvalidKey`] for an empty recipient `kid`, and
    /// [`Error::InvalidTag`] if the MAC key writes the wrong tag length.
    pub fn mac_into(&self, macer: &dyn Macer, out: &mut [u8]) -> Result<usize> {
        if self.recipient_kid.is_some_and(|kid| kid.is_empty()) {
            return Err(Error::InvalidKey);
        }
        let headers = Headers::new(
            Header::from_slice(self.protected.as_bytes())?,
            Header::from_slice(self.unprotected.as_bytes())?,
        )?;
        let algorithm = headers.algorithm().ok_or(Error::MissingAlgorithm)?;
        if algorithm != macer.algorithm() {
            return Err(Error::AlgorithmMismatch {
                header: algorithm,
                key: macer.algorithm(),
            });
        }
        if !algorithm.is_mac() {
            return Err(Error::UnsupportedAlgorithm(algorithm));
        }
        let payload = self.payload.ok_or(Error::MissingPayload)?;
        let tag_len = algorithm.tag_len().ok_or(Error::UnsupportedAlgorithm(algorithm))?;
        let message_payload = if self.detached { None } else { Some(payload) };

        let tbs_len = mac_structure_len("MAC", self.protected.as_bytes(), self.external_aad, payload);
        let message_len = cbor_len_array(5)
            + cbor_len_bytes(self.protected.as_bytes().len())
            + self.unprotected.as_bytes().len()
            + cbor_len_bytes(tag_len)
            + cbor_len_array(1)
            + self.recipient_len()
            + if self.tagged { cbor_len_tag(97) } else { 0 }
            + match message_payload {
                Some(payload) => cbor_len_bytes(payload.len()),
                None => 1,
            };

        let required = tbs_len.max(message_len);
        if out.len() < required {
            return Err(Error::BufferTooSmall {
                required,
                available: out.len(),
            });
        }

        encode_mac_structure_into(
            &mut out[..tbs_len],
            "MAC",
            self.protected.as_bytes(),
            self.external_aad,
            payload,
        )?;
        let mut tag = [0u8; 64];
        let written = macer.mac_into(&out[..tbs_len], &mut tag[..tag_len])?;
        if written != tag_len {
            return Err(Error::InvalidTag);
        }

        let mut cursor = Cursor::new(&mut out[..message_len]);
        {
            let mut e = Encoder::new(&mut cursor);
            if self.tagged {
                e.tag(Tag::new(97))?;
            }
            e.array(5)?;
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
            e.bytes(&tag[..tag_len])?;
            e.array(1)?;
            e.array(3)?;
            e.bytes(&[])?;
            e.map(if self.recipient_kid.is_some() { 2 } else { 1 })?;
            e.i64(1)?;
            e.i64(-6)?;
            if let Some(kid) = self.recipient_kid {
                e.i64(4)?;
                e.bytes(kid)?;
            }
            e.bytes(&[])?;
        }
        if cursor.position() != message_len {
            return Err(Error::Encode);
        }
        Ok(message_len)
    }

    /// Computes the MAC and encodes into a new `Vec`.
    #[cfg(feature = "alloc")]
    pub fn mac(&self, macer: &dyn Macer) -> Result<alloc::vec::Vec<u8>> {
        let payload = self.payload.ok_or(Error::MissingPayload)?;
        let message_payload = if self.detached { None } else { Some(payload) };
        let tbs_len = mac_structure_len("MAC", self.protected.as_bytes(), self.external_aad, payload);
        let message_len = cbor_len_array(5)
            + cbor_len_bytes(self.protected.as_bytes().len())
            + self.unprotected.as_bytes().len()
            + cbor_len_bytes(macer.tag_len())
            + cbor_len_array(1)
            + self.recipient_len()
            + if self.tagged { cbor_len_tag(97) } else { 0 }
            + match message_payload {
                Some(payload) => cbor_len_bytes(payload.len()),
                None => 1,
            };
        let mut out = alloc::vec![0u8; tbs_len.max(message_len)];
        let len = self.mac_into(macer, &mut out)?;
        out.truncate(len);
        Ok(out)
    }
}

impl Default for MacBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
