#[cfg(feature = "payment_attempt_records")]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentAttemptRecordReportedBy {
    SelfValue,
    Stripe,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_attempt_records")]
impl PaymentAttemptRecordReportedBy {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentAttemptRecordReportedBy::SelfValue => "self",
            PaymentAttemptRecordReportedBy::Stripe => "stripe",
            PaymentAttemptRecordReportedBy::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_attempt_records")]
impl Default for PaymentAttemptRecordReportedBy {
    fn default() -> Self {
        PaymentAttemptRecordReportedBy::Unknown(String::new())
    }
}

#[cfg(feature = "payment_attempt_records")]
impl<'json> json::Decode<'json> for PaymentAttemptRecordReportedBy {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentAttemptRecordReportedBy::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "self" => PaymentAttemptRecordReportedBy::SelfValue,
            "stripe" => PaymentAttemptRecordReportedBy::Stripe,
            other => PaymentAttemptRecordReportedBy::Unknown(other.to_string()),
        })
    }
}
