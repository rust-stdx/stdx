#[cfg(feature = "payouts")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePayoutParamsMethod {
    Instant,
    Standard,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payouts")]
impl CreatePayoutParamsMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePayoutParamsMethod::Instant => "instant",
            CreatePayoutParamsMethod::Standard => "standard",
            CreatePayoutParamsMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payouts")]
impl<'json> json::Decode<'json> for CreatePayoutParamsMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePayoutParamsMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "instant" => CreatePayoutParamsMethod::Instant,
            "standard" => CreatePayoutParamsMethod::Standard,
            other => CreatePayoutParamsMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payouts")]
impl crate::params::FormEncode for CreatePayoutParamsMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
