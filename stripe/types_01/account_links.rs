#[cfg(feature = "account_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountLinkParamsCollectionOptionsFutureRequirements {
    Include,
    Omit,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "account_links")]
impl CreateAccountLinkParamsCollectionOptionsFutureRequirements {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountLinkParamsCollectionOptionsFutureRequirements::Include => "include",
            CreateAccountLinkParamsCollectionOptionsFutureRequirements::Omit => "omit",
            CreateAccountLinkParamsCollectionOptionsFutureRequirements::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "account_links")]
impl<'json> json::Decode<'json> for CreateAccountLinkParamsCollectionOptionsFutureRequirements {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountLinkParamsCollectionOptionsFutureRequirements::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "include" => CreateAccountLinkParamsCollectionOptionsFutureRequirements::Include,
            "omit" => CreateAccountLinkParamsCollectionOptionsFutureRequirements::Omit,
            other => CreateAccountLinkParamsCollectionOptionsFutureRequirements::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "account_links")]
impl crate::params::FormEncode for CreateAccountLinkParamsCollectionOptionsFutureRequirements {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
