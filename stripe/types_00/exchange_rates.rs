#[cfg(feature = "exchange_rates")]
#[derive(Debug, Clone, PartialEq)]
pub enum ExchangeRateObject {
    ExchangeRate,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "exchange_rates")]
impl ExchangeRateObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ExchangeRateObject::ExchangeRate => "exchange_rate",
            ExchangeRateObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "exchange_rates")]
impl Default for ExchangeRateObject {
    fn default() -> Self {
        ExchangeRateObject::Unknown(String::new())
    }
}

#[cfg(feature = "exchange_rates")]
impl<'json> json::Decode<'json> for ExchangeRateObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ExchangeRateObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exchange_rate" => ExchangeRateObject::ExchangeRate,
            other => ExchangeRateObject::Unknown(other.to_string()),
        })
    }
}
