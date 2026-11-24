#[cfg(feature = "ephemeral_keys")]
#[derive(Debug, Clone, PartialEq)]
pub enum EphemeralKeyObject {
    EphemeralKey,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "ephemeral_keys")]
impl EphemeralKeyObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            EphemeralKeyObject::EphemeralKey => "ephemeral_key",
            EphemeralKeyObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "ephemeral_keys")]
impl Default for EphemeralKeyObject {
    fn default() -> Self {
        EphemeralKeyObject::Unknown(String::new())
    }
}

#[cfg(feature = "ephemeral_keys")]
impl<'json> json::Decode<'json> for EphemeralKeyObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(EphemeralKeyObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ephemeral_key" => EphemeralKeyObject::EphemeralKey,
            other => EphemeralKeyObject::Unknown(other.to_string()),
        })
    }
}
