#[cfg(feature = "account_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountLinkParamsCollect {
    CurrentlyDue,
    EventuallyDue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "account_links")]
impl CreateAccountLinkParamsCollect {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountLinkParamsCollect::CurrentlyDue => "currently_due",
            CreateAccountLinkParamsCollect::EventuallyDue => "eventually_due",
            CreateAccountLinkParamsCollect::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "account_links")]
impl<'json> json::Decode<'json> for CreateAccountLinkParamsCollect {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountLinkParamsCollect::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "currently_due" => CreateAccountLinkParamsCollect::CurrentlyDue,
            "eventually_due" => CreateAccountLinkParamsCollect::EventuallyDue,
            other => CreateAccountLinkParamsCollect::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "account_links")]
impl crate::params::FormEncode for CreateAccountLinkParamsCollect {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "account_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountLinkParamsType {
    AccountOnboarding,
    AccountUpdate,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "account_links")]
impl CreateAccountLinkParamsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountLinkParamsType::AccountOnboarding => "account_onboarding",
            CreateAccountLinkParamsType::AccountUpdate => "account_update",
            CreateAccountLinkParamsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "account_links")]
impl<'json> json::Decode<'json> for CreateAccountLinkParamsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountLinkParamsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account_onboarding" => CreateAccountLinkParamsType::AccountOnboarding,
            "account_update" => CreateAccountLinkParamsType::AccountUpdate,
            other => CreateAccountLinkParamsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "account_links")]
impl crate::params::FormEncode for CreateAccountLinkParamsType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
