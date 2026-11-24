#[cfg(feature = "payment_attempt_records")]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentAttemptRecordObject {
    PaymentAttemptRecord,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_attempt_records")]
impl PaymentAttemptRecordObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentAttemptRecordObject::PaymentAttemptRecord => "payment_attempt_record",
            PaymentAttemptRecordObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_attempt_records")]
impl Default for PaymentAttemptRecordObject {
    fn default() -> Self {
        PaymentAttemptRecordObject::Unknown(String::new())
    }
}

#[cfg(feature = "payment_attempt_records")]
impl<'json> json::Decode<'json> for PaymentAttemptRecordObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentAttemptRecordObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "payment_attempt_record" => PaymentAttemptRecordObject::PaymentAttemptRecord,
            other => PaymentAttemptRecordObject::Unknown(other.to_string()),
        })
    }
}
