#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
#[derive(Debug, Clone, PartialEq)]
pub enum ConfirmationTokenObject {
    ConfirmationToken,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
impl ConfirmationTokenObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ConfirmationTokenObject::ConfirmationToken => "confirmation_token",
            ConfirmationTokenObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
impl Default for ConfirmationTokenObject {
    fn default() -> Self {
        ConfirmationTokenObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
impl<'json> json::Decode<'json> for ConfirmationTokenObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ConfirmationTokenObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "confirmation_token" => ConfirmationTokenObject::ConfirmationToken,
            other => ConfirmationTokenObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "confirmation_tokens")]
#[derive(Debug, Clone, Default)]
pub struct GetConfirmationTokenParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "confirmation_tokens")]
impl crate::params::FormEncode for GetConfirmationTokenParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
