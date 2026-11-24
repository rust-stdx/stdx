#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
#[derive(Debug, Clone, PartialEq)]
pub enum ConfirmationTokenSetupFutureUsage {
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
impl ConfirmationTokenSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ConfirmationTokenSetupFutureUsage::OffSession => "off_session",
            ConfirmationTokenSetupFutureUsage::OnSession => "on_session",
            ConfirmationTokenSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
impl<'json> json::Decode<'json> for ConfirmationTokenSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ConfirmationTokenSetupFutureUsage::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "off_session" => ConfirmationTokenSetupFutureUsage::OffSession,
            "on_session" => ConfirmationTokenSetupFutureUsage::OnSession,
            other => ConfirmationTokenSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}
