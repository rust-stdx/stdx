#[cfg(feature = "country_specs")]
#[derive(Debug, Clone, PartialEq)]
pub enum CountrySpecObject {
    CountrySpec,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "country_specs")]
impl CountrySpecObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CountrySpecObject::CountrySpec => "country_spec",
            CountrySpecObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "country_specs")]
impl Default for CountrySpecObject {
    fn default() -> Self {
        CountrySpecObject::Unknown(String::new())
    }
}

#[cfg(feature = "country_specs")]
impl<'json> json::Decode<'json> for CountrySpecObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CountrySpecObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "country_spec" => CountrySpecObject::CountrySpec,
            other => CountrySpecObject::Unknown(other.to_string()),
        })
    }
}
