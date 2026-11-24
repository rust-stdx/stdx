#[cfg(feature = "payment_attempt_records")]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentAttemptRecordCustomerPresence {
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_attempt_records")]
impl PaymentAttemptRecordCustomerPresence {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentAttemptRecordCustomerPresence::OffSession => "off_session",
            PaymentAttemptRecordCustomerPresence::OnSession => "on_session",
            PaymentAttemptRecordCustomerPresence::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_attempt_records")]
impl<'json> json::Decode<'json> for PaymentAttemptRecordCustomerPresence {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentAttemptRecordCustomerPresence::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "off_session" => PaymentAttemptRecordCustomerPresence::OffSession,
            "on_session" => PaymentAttemptRecordCustomerPresence::OnSession,
            other => PaymentAttemptRecordCustomerPresence::Unknown(other.to_string()),
        })
    }
}
