#[cfg(feature = "apple_pay")]
#[derive(Debug, Clone, PartialEq)]
pub enum ApplePayDomainObject {
    ApplePayDomain,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "apple_pay")]
impl ApplePayDomainObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ApplePayDomainObject::ApplePayDomain => "apple_pay_domain",
            ApplePayDomainObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "apple_pay")]
impl Default for ApplePayDomainObject {
    fn default() -> Self {
        ApplePayDomainObject::Unknown(String::new())
    }
}

#[cfg(feature = "apple_pay")]
impl<'json> json::Decode<'json> for ApplePayDomainObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ApplePayDomainObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "apple_pay_domain" => ApplePayDomainObject::ApplePayDomain,
            other => ApplePayDomainObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "apple_pay")]
#[derive(Debug, Clone, Default)]
pub struct ListApplePayDomainsParams {
    pub domain_name: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "apple_pay")]
impl crate::params::FormEncode for ListApplePayDomainsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("domain_name", self.domain_name.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}
