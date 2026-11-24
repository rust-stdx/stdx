#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoDataDocumentReportExpirationDate {
    #[doc = "Numerical day between 1 and 31."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<i64>,
    #[doc = "Numerical month between 1 and 12."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub month: Option<i64>,
    #[doc = "The four-digit year."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoRelatedPerson {
    #[doc = "Token referencing the associated Account of the related Person resource."]
    #[json(default)]
    pub account: String,
    #[doc = "Token referencing the related Person resource."]
    #[json(default)]
    pub person: String,
}

#[cfg(any(feature = "events", feature = "identity"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct GelatoReportIdNumberOptions {}

#[cfg(feature = "identity")]
#[derive(Debug, Clone, Default)]
pub struct GetIdentityVerificationReportParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "identity")]
impl crate::params::FormEncode for GetIdentityVerificationReportParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "identity")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListIdentityVerificationReportsParamsType {
    Document,
    IdNumber,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "identity")]
impl ListIdentityVerificationReportsParamsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListIdentityVerificationReportsParamsType::Document => "document",
            ListIdentityVerificationReportsParamsType::IdNumber => "id_number",
            ListIdentityVerificationReportsParamsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "identity")]
impl<'json> json::Decode<'json> for ListIdentityVerificationReportsParamsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListIdentityVerificationReportsParamsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "document" => ListIdentityVerificationReportsParamsType::Document,
            "id_number" => ListIdentityVerificationReportsParamsType::IdNumber,
            other => ListIdentityVerificationReportsParamsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "identity")]
impl crate::params::FormEncode for ListIdentityVerificationReportsParamsType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
